use suslang::{codegen_file, codegen};

fn main() {
    println!("Hello, world!");
    let helloworld = include_str!("../examples/fibonacci.sus");


    let ast = suslang::parse_str(helloworld);

    suslang::typecheck(&ast);

    codegen_file("tmp.scm", &mut codegen::Scm, ast.as_slice());

}

/*fn codegen_file<W: Write, T>(from: &str, to: &str, gen: fn(&[Ast], &mut W) -> std::io::Result<()> , ast: &Vec<Ast>) {
    let f = File::create(from).unwrap();
    let mut buf = BufWriter::new(f);
    gen(ast.as_slice(), &mut buf).unwrap();
    buf.flush().unwrap();
}*/




