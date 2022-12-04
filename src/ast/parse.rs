use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, recognize},
    error::ParseError,
    multi::many0_count,
    sequence::pair,
    IResult, Parser,
};
use nom_supreme::{context::ContextError, ParserExt};

use self::{
    context::Context,
    spans::{MapExt, Span},
};

use super::Expression;

pub mod context;
pub mod error;
pub mod spans;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier<'a>(pub &'a str);

pub fn identifier<'a, E>(input: Span<'a>) -> IResult<Span<'a>, Span<'a, Identifier<'a>>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>, Context>,
{
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        |span: Span| span.map(|()| Identifier(span.fragment())),
    )
    .context(Context::Identifier)
    .parse(input)
}

// pub fn parse_expr(i: Span) -> IResult<Span, Expression> {}
