use suslang::{
    ast::parse::{error::ParseError, parse_expr, spans::load_file_str},
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

    let file = dbg!(load_file_str(
        &"test.sus",
        r#"+ 1 complete report with "%d %s\n" and (complete add with - 1 3 and 0b10) and "AA""#
    ));
    println!("RESULT: {:#?}", parse_expr::<ParseError<_>>(file));
}
