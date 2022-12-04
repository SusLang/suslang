use suslang::{codegen_file, codegen};

fn main() {
    println!("Hello, world!");
    let helloworld = include_str!("../examples/day1.sus");


    let ast = suslang::parse_str(helloworld);

    suslang::typecheck(&ast);

    codegen_file("tmp.scm", &mut codegen::Scm, ast.as_slice());
    codegen_file("tmp.c", &mut codegen::C, ast.as_slice());
    codegen_file("tmp.js", &mut codegen::Js, ast.as_slice());
    codegen_file("tmp.py", &mut codegen::Py::new(), ast.as_slice());

}
