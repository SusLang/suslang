use std::{fs::File, io::{BufWriter, Write}, process::Command};

use crate::{tokens::tokenize, ast::{Ast, Parse}};

mod ast;
mod tokens;
mod codegen;

fn main() {
    println!("Hello, world!");
    let helloworld = include_str!("../examples/helloworld.sus");


    let tok = tokenize(helloworld);
    println!("{:?}", tok);

    let mut tok = tok.into_iter().peekable();
    let ast = Ast::parse(&mut tok).unwrap();
    println!("{:?}", ast);


    let f = File::create("tmp.c").unwrap();

    let mut buf = BufWriter::new(f);
    codegen::gen_c(vec![ast], &mut buf).unwrap();

    buf.flush().unwrap();

    Command::new("gcc").arg("tmp.c").arg("-o").arg("sus.out").output().unwrap();

}
