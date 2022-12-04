use std::io::Write;

use crate::ast::{Ast, Expression, Statement, Operator, Typ};

use super::Codegen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Py {
	tab_count: usize
}

impl Py {
	#[allow(dead_code)]
	pub fn new() -> Self {
		Self::default()
	}
}

const fn default_value(typ: &Typ) -> &'static str {
    match typ {
        Typ::Num => "0",
        Typ::Str =>  "\"\"",
        Typ::Bool => "False",
        Typ::Void => ""
    }
}

// static TAB_COUNT: AtomicUsize = AtomicUsize::new(0);

// fn add_tabs() {
// 	TAB_COUNT.fetch_add(1, Ordering::SeqCst);
// }

// fn sub_tabs() {
// 	TAB_COUNT.fetch_sub(1, Ordering::SeqCst);
// }

// fn get_tabs() -> usize {
// 	TAB_COUNT.load(Ordering::SeqCst)
// }

impl<W> Codegen<W, [Ast]> for Py where W: Write {
    fn gen(&mut self, s: &[Ast], buf: &mut W) -> std::io::Result<()> {
        writeln!(
			buf,
			r#"# Python code generated from suslang
import sys

def report(s, *args):
	# print(repr(s), args)
	print((s if s is str else str(s)) % args, end="")
def getelement(arr, index):
	return arr[index]
def setelement(arr, index, value):
	arr[index] = value
# string library, will move elsewhere later
def length(s):
	return len(s)
def replace(s, old, new):
	return s.replace(old, new)
def split(s, sep):
	return s.split(sep)
def openfile(file):
	f = open(file)
	l = f.readlines()
	f.close()
	return l
"#
		)?;
		for ast in s {
			self.gen(ast, buf)?;
		}
		writeln!(buf, r#"
if __name__ == "__main__":
	sys.setrecursionlimit(1000000000)
	ඬ()"#)?;
		Ok(())
    }
}

impl<W> Codegen<W, Ast> for Py where W: Write {
    fn gen(&mut self, s: &Ast, buf: &mut W) -> std::io::Result<()> {
		// let mut var_tab_count: &usize = &0;
        match s {
            Ast::Func(name, _typ, args, block) => {
				write!(buf, "def {}(", name)?;
				for a in args.iter().flat_map(|(s, _)| [None, Some(s)]).skip(1) {
					if let Some(a) = a {
						write!(buf, "{}", a)?;
					}else{
						write!(buf, ", ")?;
					}
				}
				writeln!(buf, "):")?;
				// var_tab_count = var_tab_count + 1;
				self.tab_count += 1;
				for line in block {
					self.gen(line, buf)?;
				}
				self.tab_count -= 1;
				Ok(())
			}
		}
	}
}

impl<W> Codegen<W, Statement> for Py where W: Write {
    fn gen(&mut self, s: &Statement, buf: &mut W) -> std::io::Result<()> {
        match s {
			Statement::Return(n) => {
				write!(buf, "{}return ", "\t".repeat(self.tab_count))?;
				if let Some(n) = n {
                    self.gen(n, buf)?;
                };
				writeln!(buf)?;
			}
			Statement::Expr(m) => {
				write!(buf, "{}", "\t".repeat(self.tab_count))?;
				self.gen(m, buf)?;
				writeln!(buf)?;
			}
			Statement::If(cond, b, e) => {
				write!(buf, "{}if ", "\t".repeat(self.tab_count))?;
				self.gen(cond, buf)?;
				writeln!(buf, ":")?;
				self.tab_count += 1;
				for s in b {
					self.gen(s, buf)?;
				}
				// tab_count = tab_count - 1;
				if let Some(e) = e {
					// tab_count = tab_count + 1;
					writeln!(buf, "{}else:", "\t".repeat(self.tab_count - 1))?;
					for s in e {
						self.gen(s, buf)?;
					}
				}
				self.tab_count -= 1;
				writeln!(buf)?;
			}
			Statement::Declare(name, typ) => writeln!(buf, "{}{} = {}", "\t".repeat(self.tab_count), name, default_value(typ)).unwrap(),
			Statement::Define(name, expr) => {
				write!(buf, "{}{} = ", "\t".repeat(self.tab_count), name)?;
				self.gen(expr, buf)?;
				writeln!(buf)?;
			}
		}
		Ok(())
	}
}

impl<W> Codegen<W, Expression> for Py where W: Write {
    fn gen(&mut self, expr: &Expression, buf: &mut W) -> std::io::Result<()> {
        match expr {
			Expression::Call(func, args) => {
				write!(buf, "{}(", func)?;
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
				// write!(buf, "{}", "\t".repeat(get_tabs()))?;
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " + ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(Operator::Sub, lhs, rhs) => {
				// write!(buf, "{}", "\t".repeat(get_tabs()))?;
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " - ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
			Expression::Operation(Operator::Mod, lhs, rhs) => {
				// write!(buf, "{}", "\t".repeat(get_tabs()))?;
				self.gen(lhs.as_ref(), buf)?;
				write!(buf, " % ")?;
				self.gen(rhs.as_ref(), buf)?;
			}
		
            Expression::Operation(Operator::Lt, lhs, rhs) => {
				// write!(buf, "{}", "\t".repeat(get_tabs()))?;
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " < ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
			Expression::Operation(Operator::GEt, lhs, rhs) => {
				// write!(buf, "{}", "\t".repeat(get_tabs()))?;
				self.gen(lhs.as_ref(), buf)?;
				write!(buf, " >= ")?;
				self.gen(rhs.as_ref(), buf)?;
			}
			Expression::Operation(Operator::Eq, lhs, rhs) => {
				// write!(buf, "{}", "\t".repeat(get_tabs()))?;
				self.gen(lhs.as_ref(), buf)?;
				write!(buf, " == ")?;
				self.gen(rhs.as_ref(), buf)?;
			}
            Expression::StringLit(s) => write!(buf, "\"{}\"", s)?,
            Expression::NumLit(n) => write!(buf, "{}", n)?,
            Expression::BoolLit(b) => write!(buf, "{}", if *b {"True"} else {"False"})?,
            Expression::Variable(n) => write!(buf, "{}", n)?,
		};
		Ok(())
	}
}
