use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use ast::{parse::spans::Span, Ast};
use codegen::Codegen;

// use crate::tokens::{tokenize, Token};

pub mod ast;
pub mod codegen;
pub mod error;
pub mod fs;
pub mod linker;
pub mod module;
mod scope;
// mod tokens;
mod typecheck;

pub use typecheck::typecheck;

// pub fn parse_str(text: &str) -> Vec<Ast> {
//     let tok = tokenize(text);
//     // println!("{:?}", tok);

//     let mut tok = tok.into_iter().peekable();

//     let mut ast = Vec::new();
//     while tok.peek().is_some() {
//         if tok.peek() == Some(&Token("\n")) {
//             tok.next();
//             continue;
//         }
//         ast.push(Ast::parse(&mut tok).unwrap());
//         if tok.peek() == Some(&Token("\n")) {
//             tok.next();
//             continue;
//         }
//     }
//     ast
// }

// pub fn parse<P: AsRef<Path>>(file: P) -> std::io::Result<Vec<Ast>> {
//     let mut s = String::new();
//     std::fs::File::open(file)?.read_to_string(&mut s)?;
//     Ok(parse_str(&s))
// }

pub fn codegen_file<'a, C, P>(file: P, cod: &mut C, ast: &[Span<'a, Ast<'a>>])
where
    C: Codegen<BufWriter<File>, [Span<'a, Ast<'a>>]>,
    P: AsRef<Path>,
{
    let f = File::create(file).unwrap();
    let mut buf = BufWriter::new(f);
    cod.gen(ast, &mut buf).unwrap();
    buf.flush().unwrap();
}
