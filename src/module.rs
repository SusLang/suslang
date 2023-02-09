use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

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
        let (file, contents) = files.load(file)?;
        let s = load_file_str(file, contents);
        let (_, items) = parse_items::<ParseError<_>>(s)?;
        let mut submodules = HashMap::new();
        for item in items.iter().map(|s| &s.extra.data) {
            if let Ast::Mod(name) = item {
                let module = Self::new(file.join(format!("{}.sus", &name.extra.data)), files)?;
                submodules.insert(name.extra.data.clone(), module);
            }
        }

        Ok(Self { items, submodules })
    }
}

#[derive(Debug)]
pub enum LoadError<T> {
    IOError(std::io::Error),
    ParseError(nom::Err<ParseError<T>>),
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
