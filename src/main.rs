use std::{fs::File, io::{BufWriter, Write}};

use crate::{tokens::{tokenize, Token}, ast::{Ast, Parse}};

mod ast;
mod tokens;
mod codegen;

fn main() {
    println!("Hello, world!");
    let helloworld = include_str!("../examples/fibonacci.sus");


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
    codegen::gen_c(ast, &mut buf).unwrap();

    buf.flush().unwrap();

    // Command::new("gcc").arg("tmp.c").arg("-o").arg("sus.out").output().unwrap();

}
