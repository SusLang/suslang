#[cfg(feature = "backend-c")]
mod c;
#[cfg(feature = "backend-c")]
pub use c::C;

#[cfg(feature = "backend-js")]
mod js;
#[cfg(feature = "backend-js")]
pub use js::Js;

#[cfg(feature = "backend-python")]
mod py;
#[cfg(feature = "backend-python")]
pub use py::Py;

#[cfg(feature = "backend-scm")]
mod scm;
#[cfg(feature = "backend-scm")]
pub use scm::Scm;

use std::io::Write;

pub trait Codegen<W: Write, T: ?Sized> {
    fn gen(&mut self, s: &T, buf: &mut W) -> std::io::Result<()>;
}

impl<'a, W, T, C> Codegen<W, Span<'a, T>> for C
where
    W: Write,
    C: Codegen<W, T>,
{
    fn gen(&mut self, s: &Span<'a, T>, buf: &mut W) -> std::io::Result<()> {
        self.gen(&s.extra.data, buf)
    }
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

use crate::ast::{parse::spans::Span, Typ};
