mod c;

use std::{io::Write};

pub trait Codegen<W: Write, T> {
    fn gen(&mut self, s: &T, buf: &mut W) -> std::io::Result<()>;
}

pub use c::C;


