use nom::{
    branch::alt,
    character::complete::{char, digit1, satisfy},
    combinator::{map, recognize},
    error::ParseError,
    multi::{many0_count, separated_list1},
    sequence::pair,
    Parser,
};
use nom_supreme::{
    context::ContextError,
    tag::{complete::tag, TagError},
    ParserExt,
};

use crate::module::ModuleUsePath;

use self::{
    context::Context,
    error::IResult,
    spans::{spanned, MapExt, Span},
};

use super::{Expression, Operator};

pub mod context;
pub mod error;
pub mod expression;
pub mod items;
pub mod spans;
pub mod statement;

mod inline_comment;
mod num;
mod operator;
mod string;
mod typ;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier<'a>(pub &'a str);

pub fn valid_alpha(c: char) -> bool {
    c != 'ඞ' && c != 'ච' && c.is_alphabetic()
}

pub fn identifier<'a, E>(input: Span<'a>) -> IResult<'a, E, Identifier>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>, Context>,
{
    map(
        recognize(pair(
            alt((recognize(satisfy(valid_alpha)), recognize(char('_')))),
            many0_count(alt((
                recognize(satisfy(valid_alpha)),
                digit1,
                recognize(char('_')),
            ))),
        )),
        |span: Span| span.map(|()| Identifier(span.fragment())),
    )
    .context(Context::Identifier)
    .parse(input)
}

pub fn path<'a, E>(input: Span<'a>) -> IResult<'a, E, ModuleUsePath>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>, Context> + TagError<Span<'a>, &'static str>,
{
    spanned(map(separated_list1(tag("<="), identifier), |x| {
        x.into_iter().map(|x| x.extra.data.0.into()).collect()
    }))
    .context(Context::Path)
    .parse(input)
}
