use crate::{
    ast::{parse::spans::Span, Ast},
    module::Module,
};

pub fn link<'a>(module_graph: &Module<'a>) -> Vec<Span<'a, Ast<'a>>> {
    let mut res = Vec::new();
    res
}
