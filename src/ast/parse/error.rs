use std::error::Error;

use nom_supreme::error::GenericErrorTree;

use super::{context::Context, spans::Span};

pub type ParseError<I> =
    GenericErrorTree<I, &'static str, Context, Box<dyn Error + Send + Sync + 'static>>;

pub type IResult<'a, Err, Extra> = nom::IResult<Span<'a>, Span<'a, Extra>, Err>;
