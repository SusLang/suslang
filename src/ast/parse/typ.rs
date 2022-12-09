use crate::ast::Typ;

use super::{
    context::Context,
    error::IResult,
    spans::{spanned_value, Span},
};

use nom::{branch::alt, bytes::complete::tag, error::ParseError, Parser};
use nom_supreme::{context::ContextError, ParserExt};

pub fn parse_type<'a, E>(i: Span<'a>) -> IResult<'a, E, Typ>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>, Context> + 'a,
{
    alt((
        spanned_value(&Typ::Bool, tag("bool")),
        spanned_value(&Typ::Num, tag("number")),
        spanned_value(&Typ::Void, tag("void")),
        spanned_value(&Typ::Str, tag("string")),
    ))
    .context(Context::Type)
    .parse(i)
}
