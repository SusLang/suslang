use std::collections::HashMap;

use crate::ast::{parse::spans::Span, Ast};

pub struct Module<'a> {
    items: Vec<Span<'a, Ast<'a>>>,
    submodules: HashMap<String, Module<'a>>,
}
