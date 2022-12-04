use std::error::Error;

use nom_supreme::error::GenericErrorTree;

use super::context::Context;

pub type ParseError<I> =
    GenericErrorTree<I, &'static str, Context, Box<dyn Error + Send + Sync + 'static>>;
