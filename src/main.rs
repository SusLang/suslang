use std::{fs::File, io::{BufWriter, Write}};

use crate::{tokens::{tokenize, Token}, ast::{Ast, Parse}, codegen::Codegen};

mod ast;
mod tokens;
mod codegen;

fn main() {
    println!("Hello, world!");
    let helloworld = include_str!("../examples/ifs.sus");


    let tok = tokenize(helloworld);
    println!("{:?}", tok);

    let mut tok = tok.into_iter().peekable();
    
    let mut ast = Vec::new();
    while tok.peek().is_some() {
        if tok.peek() == Some(&Token("\n")) {
            tok.next();
            continue;
        }
        ast.push(Ast::parse(&mut tok).unwrap());
        if tok.peek() == Some(&Token("\n")) {
            tok.next();
            continue;
        }
    }
    println!("{:?}", ast);


    let f = File::create("tmp.c").unwrap();

    let mut buf = BufWriter::new(f);
    codegen::C.gen(ast.as_slice(), &mut buf).unwrap();

    buf.flush().unwrap();

    let f = File::create("tmp.js").unwrap();

    let mut buf = BufWriter::new(f);
    codegen::Js.gen(ast.as_slice(), &mut buf).unwrap();

    let f = File::create("tmp.scm").unwrap();

    let mut buf = BufWriter::new(f);
    codegen::Scm.gen(ast.as_slice(), &mut buf).unwrap();

    buf.flush().unwrap();

    let f = File::create("tmp.py").unwrap();

    let mut buf = BufWriter::new(f);
    codegen::Py.gen(ast.as_slice(), &mut buf).unwrap();

    buf.flush().unwrap();

    // Command::new("gcc").arg("tmp.c").arg("-o").arg("sus.out").output().unwrap();

}
