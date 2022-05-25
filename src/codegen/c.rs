use std::{io::Write};

use crate::ast::{Ast, Expression, Statement, Operator, Typ};

use super::{Codegen, Typename};

pub struct C;

const NAME_REPLACE: &[(&str, &str)] = &[("ඬ", "main")];

impl Typename for C {
    fn typename(t: &Typ) -> &'static str {
        match t {
            Typ::Num => "int",
            Typ::Str => "char*",
            Typ::Bool => "int",
            Typ::Void => "void",
        }
    }
}

impl<W> Codegen<W, [Ast]> for C where W: Write {
    fn gen(&mut self, s: &[Ast], buf: &mut W) -> std::io::Result<()> {
        writeln!(
			buf,
			r#"// C code generated from suslang
#include <stdio.h>
#define report printf
"#
		)?;
		for ast in s {
			self.gen(ast, buf)?;
		}
		Ok(())
    }
}

impl<W> Codegen<W, Ast> for C where W: Write {
    fn gen(&mut self, s: &Ast, buf: &mut W) -> std::io::Result<()> {
        match s {
            Ast::Func(name, typ, args, block) => {
				let mut name = name.clone();
				for (name_r, replace) in NAME_REPLACE {
					if name == *name_r {
						name = replace.to_string();
						break;
					}
				}
				write!(buf, "{} {}(", Self::typename(typ), name)?;
				let args_len = args.len();
				for (i, (name, typ)) in args.iter().enumerate() {
					write!(
						buf,
						"{} {}{}",
						Self::typename(typ),
						name,
						if i == args_len - 1 { "" } else { ", " }
					)?;
				}
				writeln!(buf, ") {{")?;
			
				for line in block {
					write!(buf, "\t")?;
					self.gen(line, buf)?;
				}
				writeln!(buf, "}}")?;
				Ok(())
			},
        }
    }
}

impl<W> Codegen<W, Statement> for C where W: Write {
    fn gen(&mut self, s: &Statement, buf: &mut W) -> std::io::Result<()> {
        match s {
			Statement::Return(n) => {
				write!(buf, "return ")?;
				n.as_ref().map(|x| self.gen(x, buf));
				writeln!(buf, ";")?;
			}
			Statement::Expr(m) => {
				self.gen(m, buf)?;
				writeln!(buf, ";")?;
			}
			Statement::If(cond, b, e) => {
				write!(buf, "if (")?;
				self.gen(cond, buf)?;
				writeln!(buf, ") {{")?;
				for s in b {
					write!(buf, "\t")?;
					self.gen(s, buf)?;
				}
				if let Some(e) = e {
					writeln!(buf, "}} else {{")?;
					for s in e {
						write!(buf, "\t")?;
						self.gen(s, buf)?;
					}
				}
				writeln!(buf, "}}")?;
			}
			Statement::Declare(name, typ) => writeln!(buf, "{} {};", Self::typename(typ), name).unwrap(),
			Statement::Define(name, expr) => {
				write!(buf, "{} = ", name)?;
				self.gen(expr, buf)?;
				writeln!(buf, ";")?;
			}
		}
		Ok(())
    }
}

impl<W> Codegen<W, Expression> for C where W: Write {
    fn gen(&mut self, expr: &Expression, buf: &mut W) -> std::io::Result<()> {
        match expr {
			Expression::Call(func, args) => {
				let mut func = func.clone();
				for (name_r, replace) in NAME_REPLACE {
					if func == *name_r {
						func = replace.to_string();
						break;
					}
				}
				write!(buf, "{}(", func)?;
				let args_len = args.len();
				for (i, arg) in args.iter().enumerate() {
					self.gen(arg, buf)?;
					if i != args_len - 1 {
						write!(buf, ", ")?;
					}
				}
				write!(buf, ")")?;
			}
			Expression::NumLit(s) => write!(buf, "{}", s)?,
			Expression::StringLit(s) => write!(buf, "\"{}\"", s)?,
			Expression::Operation(Operator::Lt, lhs, rhs) => {
				self.gen(lhs.as_ref(), buf)?;
				write!(buf, " < ")?;
				self.gen(rhs.as_ref(), buf)?;
			}
			Expression::Operation(Operator::Add, lhs, rhs) => {
				self.gen(lhs.as_ref(), buf)?;
				write!(buf, " + ")?;
				self.gen(rhs.as_ref(), buf)?;
			}
			Expression::Operation(Operator::Sub, lhs, rhs) => {
				self.gen(lhs.as_ref(), buf)?;
				write!(buf, " - ")?;
				self.gen(rhs.as_ref(), buf)?;
			}
			Expression::Variable(x) => write!(buf, "{}", x)?,
			x => todo!("{:?}", x)
		};
		Ok(())
    }
}


