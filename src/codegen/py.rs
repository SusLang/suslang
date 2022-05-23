use std::{io::Write};

use crate::ast::{Ast, Expression, Statement, Operator, Typ};

use super::Codegen;

pub struct Py;


impl<W> Codegen<W, [Ast]> for Py where W: Write {
    fn gen(&mut self, s: &[Ast], buf: &mut W) -> std::io::Result<()> {
        writeln!(
			buf,
			r#"# Python code generated from suslang
def report
"# // creo que quieres report = print y ya esta
		)?;
		for ast in s {
			self.gen(ast, buf)?;
		}
		writeln!(buf, r#"if __name__ == "__main__":
    ඬ()"#)?;
		Ok(())
    }
}