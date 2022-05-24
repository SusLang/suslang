// Replace with Typ if functions is added as a type

use std::collections::HashMap;

use crate::ast::{Ast, Typ};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Function(Vec<Type>, Box<Type>),
    Void,
    Number,
    Bool,
    String,
}

impl From<Typ> for Type {
    fn from(t: Typ) -> Self {
        match t {
            Typ::Num => Self::Number,
            Typ::Str => Self::String,
            Typ::Bool => Self::Bool,
            Typ::Void => Self::Void,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Scope<'a> {
    top_level: HashMap<&'a str, Type>,
    data: Vec<HashMap<&'a str, Type>>,
}

impl<'a> Scope<'a> {
    fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, s: &'a str, t: Type) {
        self.data
            .last_mut()
            .unwrap_or(&mut self.top_level)
            .insert(s, t);
    }

    pub fn push(&mut self) {
        self.data.push(Default::default());
    }

    pub fn pop(&mut self) {
        self.data.pop();
    }

    pub fn get(&self, s: &str) -> Option<&Type> {
        self.data
            .iter()
            .rev()
            .chain(std::iter::once(&self.top_level))
            .map(|x| x.get(s))
            .find(|x| x.is_some())
            .flatten()
    }
}

fn typecheck(a: &[Ast]) {
    let mut scopes = Scope::new();
    for a in a {
        match a {
            Ast::Func(name, ret, args, _) => scopes.add(
                name,
                Type::Function(
                    args.iter().map(|(_, t)| Type::from(*t)).collect(),
                    Box::new(Type::from(*ret)),
                ),
            ),
        }
    }

	for a in a {
        match a {
            Ast::Func(name, ret, args, body) => {
				// TODO typecheck bodies
			},
        }
    }
}
