use std::{collections::HashMap, hash::Hash, path::PathBuf};

use crate::{
    ast::{
        parse::{
            error::ParseError,
            items::parse_items,
            spans::{load_file_str, MapExt, Span},
        },
        Ast,
    },
    fs::Filesystem,
    typecheck::Type,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Module<'a> {
    pub items: Vec<Span<'a, Ast<'a>>>,
    pub submodules: HashMap<String, Module<'a>>,
    pub exports: Option<HashMap<String, Type>>,
    pub path: ModuleUsePath,
}

impl<'a> Hash for Module<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.items.hash(state);
        // self.submodules.hash(state);
        // self.exports.hash(state);
        self.path.hash(state);
    }
}

impl<'a> Module<'a> {
    pub fn new<'b>(
        file: PathBuf,
        files: &'b mut Filesystem<'a>,
    ) -> Result<Self, LoadError<'a, Span<'a>>> {
        Self::new_inner(file, files, Vec::new())
    }

    fn new_inner<'b>(
        file: PathBuf,
        files: &'b mut Filesystem<'a>,
        path: ModuleUsePath,
    ) -> Result<Self, LoadError<'a, Span<'a>>> {
        let (file, contents) = files.load(file)?;
        let s = load_file_str(file, contents);
        let (_, items) = parse_items::<ParseError<_>>(s)?;
        let mut submodules = HashMap::new();
        for item in items.iter().map(|s| &s.extra.data) {
            if let Ast::Mod(name) = item {
                let mod_path = format!("{}.sus", &name.extra.data);
                let mod_path = if path.is_empty() {
                    file.with_file_name(&mod_path)
                } else {
                    file.with_extension("").join(&mod_path)
                };
                if !mod_path.is_file() {
                    return Err(LoadError::ModuleFileNotFound(
                        name.clone().map(|_| mod_path),
                    ));
                }
                let mut new_path = path.clone();
                new_path.push(name.extra.data.clone());
                let module = Self::new_inner(mod_path, files, new_path)?;
                submodules.insert(name.extra.data.clone(), module);
            }
        }

        let mut s = Self {
            items,
            submodules,
            exports: None,
            path,
        };
        s.load_exports();
        Ok(s)
    }

    fn load_exports(&mut self) {
        self.exports.get_or_insert_with(|| {
            let mut hm = HashMap::with_capacity(self.items.len());
            for item in &self.items {
                match &item.extra.data {
                    Ast::Func(name, ret, args, _) => {
                        hm.insert(
                            name.extra.data.clone(),
                            Type::Function(
                                args.iter()
                                    .map(|x| x.extra.data.1.extra.data.into())
                                    .collect(),
                                Box::new(ret.extra.data.into()),
                            ),
                        );
                    }
                    Ast::Mod(_) => (),
                    Ast::Import(_) => (),
                }
            }
            hm.shrink_to_fit();
            hm
        });
    }

    pub fn get_exports(&self) -> impl Iterator<Item = (&str, &Type)> {
        self.exports
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, b)| (a.as_str(), b))
    }

    pub fn get_module(&self, path: &[String]) -> Option<&Module<'a>> {
        if path.is_empty() {
            Some(self)
        } else {
            self.submodules
                .get(&path[0])
                .and_then(|sm| sm.get_module(&path[1..]))
        }
    }

    pub fn get_module_mut(&mut self, path: &[String]) -> Option<&mut Module<'a>> {
        if path.is_empty() {
            Some(self)
        } else {
            self.submodules
                .get_mut(&path[0])
                .and_then(|sm| sm.get_module_mut(&path[1..]))
        }
    }

    pub fn print_tree(&self) {
        self.print_tree_inner(0);
    }

    fn print_tree_inner(&self, level: usize) {
        for (name, module) in self.submodules.iter() {
            println!("{0:>1$}{name}", "", level);
            module.print_tree_inner(level + 1)
        }
    }
}

#[derive(Debug)]
pub enum LoadError<'a, T> {
    IOError(std::io::Error),
    ParseError(nom::Err<ParseError<T>>),
    ModuleFileNotFound(Span<'a, PathBuf>),
}

impl<'a, T> From<std::io::Error> for LoadError<'a, T> {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl<'a, T> From<nom::Err<ParseError<T>>> for LoadError<'a, T> {
    fn from(value: nom::Err<ParseError<T>>) -> Self {
        Self::ParseError(value)
    }
}
pub type ModuleUsePath = Vec<String>;
