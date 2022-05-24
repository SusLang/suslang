mod c;
mod js;
mod scm;
mod py;

use std::{io::Write};

pub trait Codegen<W: Write, T: ?Sized> {
    fn gen(&mut self, s: &T, buf: &mut W) -> std::io::Result<()>;
}

pub trait Typename {
    fn typename(t: &Typ) -> &'static str;
}

pub trait Codegeneable<W, C> { //idea de Juan
    fn gen(&self, codegen: &mut C, buf: &mut W) -> std::io::Result<()>;
}

impl <T, W, C> Codegeneable<W, C> for T where W: Write, C: Codegen<W, T> {
    fn gen(&self, codegen: &mut C, buf: &mut W) -> std::io::Result<()> {
        codegen.gen(self, buf)
    }
} 

pub use c::C;
pub use js::Js;
pub use scm::Scm;
pub use py::Py;

use crate::ast::Typ;

