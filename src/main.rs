use std::path::PathBuf;

use nom::error::VerboseError;
use nom_supreme::error::ErrorTree;
use suslang::{
    ast::parse::{error::ParseError, identifier, spans::load_file_str},
    codegen, codegen_file,
};

fn main() {
    println!("Hello, world!");
    let helloworld = include_str!("../examples/bools.sus");

    let ast = suslang::parse_str(helloworld);

    suslang::typecheck(&ast);

    codegen_file("tmp.scm", &mut codegen::Scm, ast.as_slice());
    codegen_file("tmp.c", &mut codegen::C, ast.as_slice());
    codegen_file("tmp.js", &mut codegen::Js, ast.as_slice());
    codegen_file("tmp.py", &mut codegen::Py::new(), ast.as_slice());

    let file = dbg!(load_file_str(&"test.sus", "1hola HO"));
    dbg!(identifier::<ParseError<_>>(file));
}
