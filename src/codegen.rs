mod c;
mod js;
mod py;
mod scm;

use std::io::Write;

pub trait Codegen<W: Write, T: ?Sized> {
    fn gen(&mut self, s: &T, buf: &mut W) -> std::io::Result<()>;
}

trait Typename {
    fn typename(t: &Typ) -> &'static str;
}

trait Codegeneable<W, C> {
    fn gen(&self, codegen: &mut C, buf: &mut W) -> std::io::Result<()>;
}

impl<T, W, C> Codegeneable<W, C> for T
where
    W: Write,
    C: Codegen<W, T>,
{
    fn gen(&self, codegen: &mut C, buf: &mut W) -> std::io::Result<()> {
        codegen.gen(self, buf)
    }
}

pub use c::C;
pub use js::Js;
pub use py::Py;
pub use scm::Scm;

use crate::ast::Typ;
