mod c;
mod js;
mod scm;

use std::{io::Write};

pub trait Codegen<W: Write, T: ?Sized> {
    fn gen(&mut self, s: &T, buf: &mut W) -> std::io::Result<()>;
}

pub trait Typename {
    fn typename(t: &Typ) -> &'static str;
}

pub use c::C;
pub use js::Js;
pub use scm::Scm;

use crate::ast::Typ;

