use std::io::Write;

use crate::ast::{Ast, Expression, Operator, Statement};

use super::Codegen;

pub struct Js;

impl<W: Write> Codegen<W, [Ast]> for Js {
	#[allow(clippy::only_used_in_recursion)]
    fn gen(&mut self, s: &[Ast], buf: &mut W) -> std::io::Result<()> {
		writeln!(buf, r#"// suslang automagically generated code
function report(s, ...a) {{
	console.log(s.trimEnd(), a);
}}
"#)?;
        for s in s {
			self.gen(s, buf)?;
		}
		writeln!(buf, "ඬ()")?;
		Ok(())
    }
}

impl<W: Write> Codegen<W, Ast> for Js {
    fn gen(&mut self, s: &Ast, buf: &mut W) -> std::io::Result<()> {
        match s {
            Ast::Func(name, _, args, block) => {
				write!(buf, "function {}(", name)?;
				for a in args.iter().flat_map(|(s, _)| [None, Some(s)]).skip(1) {
					if let Some(a) = a {
						write!(buf, "{}", a)?;
					}else{
						write!(buf, ", ")?;
					}
				}
				writeln!(buf, "){{")?;
				for s in block {
					self.gen(s, buf)?;
				}
				writeln!(buf, "}}")?;
			},
        }
		Ok(())
    }
}

impl<W: Write> Codegen<W, Statement> for Js {
    fn gen(&mut self, s: &Statement, buf: &mut W) -> std::io::Result<()> {
        match s {
            Statement::If(cond, block, else_block) => {
				write!(buf, "if (")?;
				self.gen(cond, buf)?;
				writeln!(buf, ") {{")?;
				for s in block {
					self.gen(s, buf)?;
				}
				if let Some(else_block) = else_block {
					writeln!(buf, "}} else {{")?;
					for s in else_block {
						self.gen(s, buf)?;
					}
				}
				writeln!(buf, "}}")?;

			},
            Statement::Return(e) => {
                write!(buf, "return ")?;
                if let Some(e) = e {
                    self.gen(e, buf)?;
                };
                writeln!(buf, ";")?;
            }
            Statement::Expr(e) => {
				self.gen(e, buf)?;
                writeln!(buf, ";")?;
			},
            Statement::Declare(name, _) => writeln!(buf, "let {};", name)?,
            Statement::Define(name, e) => {
				write!(buf, "{} = ", name)?;
				self.gen(e, buf)?;
				writeln!(buf, ";")?;
			},
        }
        Ok(())
    }
}



impl<W: Write> Codegen<W, Expression> for Js {
    fn gen(&mut self, s: &Expression, buf: &mut W) -> std::io::Result<()> {
        match s {
            Expression::Call(name, args) => {
                write!(buf, "{}(", name)?;
                for t in args.iter().flat_map(|x| [None, Some(x)]).skip(1) {
                    if let Some(x) = t {
                        self.gen(x, buf)?;
                    } else {
                        write!(buf, ", ")?;
                    }
                }
				write!(buf, ")")?;
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
            Expression::Operation(Operator::Mod, lhs, rhs) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " % ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(Operator::Lt, lhs, rhs) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " < ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(Operator::GEt, lhs, rhs) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " >= ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(Operator::Eq, lhs, rhs) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " == ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::StringLit(s) => write!(buf, "\"{}\"", s)?,
            Expression::NumLit(n) => write!(buf, "{}", n)?,
            Expression::BoolLit(b) => write!(buf, "{}", b)?,
            Expression::Variable(n) => write!(buf, "{}", n)?,
        };
        Ok(())
    }
}
