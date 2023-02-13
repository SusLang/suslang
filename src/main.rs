use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use clap::Parser;
use miette::GraphicalReportHandler;
use suslang::{
    ast::{parse::spans::Span, Ast},
    codegen::{self, Codegen},
    codegen_file,
    fs::Filesystem,
    linker::{self},
    module::Module,
};

fn compile_file<
    'a,
    A: AsRef<Path>,
    B: AsRef<Path>,
    C: Codegen<BufWriter<File>, [Span<'a, Ast<'a>>]> + ?Sized,
>(
    input: &A,
    output: &B,
    codegen: &mut C,
) {
    let mut fs = Filesystem::new();
    let module = Module::new(input.as_ref().into(), &mut fs).unwrap();

    // println!("ROOT: ");
    // module.print_tree();

    // dbg!(module.get_module(&["ifs".into()]));
    // dbg!(module.get_module(&["lib".into()]));
    // dbg!(module.get_module(&["lib".into(), "ifs".into()]));
    // dbg!(module.get_module(&["lib".into(), "a".into()]));
    // dbg!(module.get_module(&[]));

    // let res = parse_items::<ParseError<_>>(load_file_str(&"../examples/day1.sus", helloworld));
    // // println!("{res:#?}");
    // .into_iter()
    // .map(|s| s.extra.data)
    // .collect::<Vec<_>>();

    // let ast = suslang::parse_str(helloworld);

    if let Err(report) = suslang::typecheck(&module.items, &module) {
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

    codegen_file(output, codegen, ast.as_slice());

    // let file = dbg!(load_file_str(
    //     &"test.sus",
    //     r#"+ 1 complete report with "%d %s\n" and (complete add with - 1 3 and 0b10) and "AA""#
    // ));
    // println!("RESULT: {:#?}", parse_expr::<ParseError<_>>(file));
    drop(module);
    drop(fs);
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum Backends {
    C,
    Js,
    Javascript,
    Py,
    Python,
    Scm,
}

#[derive(Debug, clap::Parser)]
struct Arguments {
    input: PathBuf,
    output: PathBuf,
    backend: Backends,
}

fn main() {
    let args = Arguments::parse();
    // dbg!(&args);
    // println!("Hello, world!");
    // let helloworld = include_str!("../examples/day1.sus");
    let mut codegen: Box<dyn Codegen<BufWriter<File>, [Span<Ast>]>> = match args.backend {
        Backends::C => Box::new(codegen::C),
        Backends::Js | Backends::Javascript => Box::new(codegen::Js),
        Backends::Py | Backends::Python => Box::new(codegen::Py::new()),
        Backends::Scm => Box::new(codegen::Scm),
    };
    compile_file(&args.input, &args.output, codegen.as_mut())
}
