use nom::{
    branch::alt,
    character::complete::{char, digit1, satisfy},
    combinator::{map, recognize},
    error::ParseError,
    multi::many0_count,
    sequence::pair,
    Parser,
};
use nom_supreme::{context::ContextError, ParserExt};

use self::{
    context::Context,
    error::IResult,
    spans::{MapExt, Span},
};

use super::{Expression, Operator};

pub mod context;
pub mod error;
pub mod expression;
pub mod spans;
pub mod statement;

mod inline_comment;
mod num;
mod operator;
mod opt;
mod string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier<'a>(pub &'a str);

pub fn identifier<'a, E>(input: Span<'a>) -> IResult<'a, E, Identifier>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>, Context>,
{
    map(
        recognize(pair(
            alt((
                recognize(satisfy(char::is_alphabetic)),
                recognize(char('_')),
            )),
            many0_count(alt((
                recognize(satisfy(char::is_alphabetic)),
                digit1,
                recognize(char('_')),
            ))),
        )),
        |span: Span| span.map(|()| Identifier(span.fragment())),
    )
    .context(Context::Identifier)
    .parse(input)
}
