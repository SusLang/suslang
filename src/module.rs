use std::{collections::HashMap, path::PathBuf};

use crate::{
    ast::{
        parse::{
            error::ParseError,
            items::parse_items,
            spans::{load_file_str, Span},
        },
        Ast,
    },
    fs::Filesystem,
};

#[derive(Debug)]
pub struct Module<'a> {
    items: Vec<Span<'a, Ast<'a>>>,
    submodules: HashMap<String, Module<'a>>,
}

impl<'a> Module<'a> {
    pub fn new<'b>(
        file: PathBuf,
        files: &'b mut Filesystem<'a>,
    ) -> Result<Self, LoadError<Span<'a>>> {
        Self::new_inner(file, files, true)
    }

    fn new_inner<'b>(
        file: PathBuf,
        files: &'b mut Filesystem<'a>,
        root: bool,
    ) -> Result<Self, LoadError<Span<'a>>> {
        let (file, contents) = files.load(file)?;
        let s = load_file_str(file, contents);
        let (_, items) = parse_items::<ParseError<_>>(s)?;
        let mut submodules = HashMap::new();
        for item in items.iter().map(|s| &s.extra.data) {
            if let Ast::Mod(name) = item {
                let mod_path = format!("{}.sus", &name.extra.data);
                let mod_path = if root {
                    file.with_file_name(mod_path)
                } else {
                    file.with_extension("").join(mod_path)
                };
                if !mod_path.is_file() {
                    return Err(LoadError::ModuleFileNotFound(mod_path));
                }
                let module = Self::new_inner(mod_path, files, false)?;
                submodules.insert(name.extra.data.clone(), module);
            }
        }

        Ok(Self { items, submodules })
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
pub enum LoadError<T> {
    IOError(std::io::Error),
    ParseError(nom::Err<ParseError<T>>),
    ModuleFileNotFound(PathBuf),
}

impl<T> From<std::io::Error> for LoadError<T> {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl<T> From<nom::Err<ParseError<T>>> for LoadError<T> {
    fn from(value: nom::Err<ParseError<T>>) -> Self {
        Self::ParseError(value)
    }
}
