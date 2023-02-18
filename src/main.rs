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

fn check_module_graph(module_graph: &Module) {
    for module in module_graph.iter() {
        if let Err(report) = suslang::typecheck(&module.items, module_graph) {
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
    }
    
}

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

    check_module_graph(&module);

    let ast = linker::link(&module);

    codegen_file(output, codegen, ast.as_slice());
    drop(module);
    drop(fs);
}

fn check<A: AsRef<Path>>(input: &A) {
    let mut fs = Filesystem::new();
    let module = Module::new(input.as_ref().into(), &mut fs).unwrap();

    check_module_graph(&module);

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
    // #[arg(conflicts_with_all = ["output", "backend"])]
    // #[arg(long)]
    // check: bool,
    // #[arg(conflicts_with = "check")]
    // #[arg(requires = "backend")]
    // #[arg(required_unless_present = "check")]
    // output: Option<PathBuf>,
    // #[arg(requires = "output")]
    // #[arg(required_unless_present = "check")]
    // #[arg(short)]
    // backend: Option<Backends>,
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Subcommands {
    Check,
    Build { output: PathBuf, backend: Backends },
}

fn main() {
    let args = Arguments::parse();
    // dbg!(&args);
    // println!("Hello, world!");
    // let helloworld = include_str!("../examples/day1.sus");
    match args.subcommand {
        Subcommands::Check => {
            check(&args.input);
            println!("OK");
        },
        Subcommands::Build { output, backend } => {
            let mut codegen: Box<dyn Codegen<BufWriter<File>, [Span<Ast>]>> = match backend {
                Backends::C => Box::new(codegen::C),
                Backends::Js | Backends::Javascript => Box::new(codegen::Js),
                Backends::Py | Backends::Python => Box::new(codegen::Py::new()),
                Backends::Scm => Box::new(codegen::Scm),
            };
            compile_file(&args.input, &output, codegen.as_mut())
        }
    }
    // if let Some((output, backend)) = args.output.zip(args.backend) {
    //     let mut codegen: Box<dyn Codegen<BufWriter<File>, [Span<Ast>]>> = match backend {
    //         Backends::C => Box::new(codegen::C),
    //         Backends::Js | Backends::Javascript => Box::new(codegen::Js),
    //         Backends::Py | Backends::Python => Box::new(codegen::Py::new()),
    //         Backends::Scm => Box::new(codegen::Scm),
    //     };
    //     compile_file(&args.input, &output, codegen.as_mut())
    // }else if args.check {
    //     check(&args.input)
    // }
}
