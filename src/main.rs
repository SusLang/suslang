use miette::{ErrReport, GraphicalReportHandler, IntoDiagnostic};
use suslang::{
    ast::parse::{error::ParseError, items::parse_items, spans::load_file_str},
    codegen, codegen_file,
    error::TypeCheckError,
    fs::Filesystem,
    linker,
    module::Module,
};

fn main() {
    // println!("Hello, world!");
    // let helloworld = include_str!("../examples/day1.sus");

    let mut fs = Filesystem::new();
    let module = Module::new("examples/modules.sus".into(), &mut fs).unwrap();

    // println!("ROOT: ");
    // module.print_tree();

    // dbg!(module.get_module(&["ifs".into()]));
    // dbg!(module.get_module(&["lib".into()]));
    // dbg!(module.get_module(&["lib".into(), "ifs".into()]));
    // dbg!(module.get_module(&["lib".into(), "a".into()]));
    // dbg!(module.get_module(&[]));

    // let res = parse_items::<ParseError<_>>(load_file_str(&"../examples/day1.sus", helloworld));
    // // println!("{res:#?}");
    let ast = &module.items;
    // .into_iter()
    // .map(|s| s.extra.data)
    // .collect::<Vec<_>>();

    // let ast = suslang::parse_str(helloworld);

    if let Err(report) = suslang::typecheck(ast, &module) {
        let handler = GraphicalReportHandler::new();
        let mut buf = String::new();
        handler.render_report(&mut buf, &report).unwrap();
        println!("{buf}");
        // match report {
        //     TypeCheckError::ItemNotFound(e) => {
        //         let r = miette::Report::from(e);
        //         eprintln!("{r:?}");
        //     }
        //     e => eprintln!("{e}"),
        // }
        std::process::exit(1);
    }

    let ast = linker::link(&module);

    codegen_file("tmp.scm", &mut codegen::Scm, ast.as_slice());
    codegen_file("tmp.c", &mut codegen::C, ast.as_slice());
    codegen_file("tmp.js", &mut codegen::Js, ast.as_slice());
    codegen_file("tmp.py", &mut codegen::Py::new(), ast.as_slice());

    // let file = dbg!(load_file_str(
    //     &"test.sus",
    //     r#"+ 1 complete report with "%d %s\n" and (complete add with - 1 3 and 0b10) and "AA""#
    // ));
    // println!("RESULT: {:#?}", parse_expr::<ParseError<_>>(file));
    drop(module);
    drop(fs);
}
