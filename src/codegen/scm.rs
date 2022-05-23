use std::{io::Write};

use crate::ast::{Ast, Expression, Statement, Operator, Typ};

use super::Codegen;

pub struct Scm;

fn default_value (typ: Typ) -> &'static str {
    match typ {
        Typ::Num => "0",
        Typ::Str =>  "str",
        Typ::Bool => "false",
        Typ::Void => ""
    }
}

impl <W> Codegen<W, [Ast]> for  Scm  where W: Write { // main one
    fn gen(&mut self, s: &[Ast], buf: &mut W) -> std::io::Result<()> {
        writeln!(buf, r#"; scheme code generated from suslang
(define (report f)
    (display f)
    (display "\n"))"#)?;
         for ast in s {
            self.gen(ast, buf)?;
         }
         writeln!(buf, "(ඬ)")?;
         Ok(())
    }   
}

impl<W> Codegen<W, Ast> for Scm where W: Write {
    fn gen(&mut self, s: &Ast, buf: &mut W) -> std::io::Result<()> {
        match s {
            Ast::Func(name, typ, args, blocks) => {
                write!(buf, "(define ({}", name)?;
                for (name, typ) in args.iter() {
                    write!(buf, "{} ", name)?;
                }
                writeln!(buf, ")")?;

                for line in blocks {
                    write!(buf, "\t")?;
                    self.gen(line, buf)?;
                }
                writeln!(buf, ")")?;
            }
        }
        Ok(())
    }
}


impl<W> Codegen<W, Statement> for Scm where W: Write {
    fn gen(&mut self, s: &Statement, buf: &mut W) -> std::io::Result<()> {
        match s {
            Statement::Return(s) => {
                if let Some(r) = s {
                    self.gen(r, buf)?;
                } else {
                    write!(buf, "(void)")?;
                }
            }

            Statement::Expr(e) => {
                self.gen(e, buf)?;
            }

            _ => todo!()
        }
        Ok(())
    }
}

impl<W> Codegen<W, Expression> for Scm where W: Write {
    fn gen(&mut self, expr: &Expression, buf: &mut W) -> std::io::Result<()> {
        match expr {
            Expression::Call(name, args) => {
                write!(buf, "({} ", name)?;
                for (t) in  args.iter() {
                    self.gen(t, buf)?;
                }
                writeln!(buf, ")")?;
            }

			Expression::NumLit(s) => write!(buf, "{}", s)?,
			Expression::StringLit(s) => write!(buf, "\"{}\"", s)?,
            _ => todo!()
        }
        Ok(())
    }
}