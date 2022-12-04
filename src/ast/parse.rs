use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, satisfy},
    combinator::{map, opt, recognize},
    error::{FromExternalError, ParseError},
    multi::{many0_count, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    Parser,
};
use nom_supreme::{context::ContextError, ParserExt};

use self::{
    context::Context,
    error::IResult,
    inline_comment::ws,
    num::num_lit,
    operator::binary_operator,
    spans::{spanned, spanned_map, MapExt, Span},
    string::parse_string,
};

use super::{Expression, Operator};

pub mod context;
pub mod error;
mod inline_comment;
mod num;
mod operator;
pub mod spans;
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

pub fn parse_bool_lit<'a, E>(i: Span<'a>) -> IResult<E, bool>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>, Context>,
{
    alt((
        map(tag("sus"), |span: Span| span.map(|_| true)),
        map(tag("clean"), |span: Span| span.map(|_| false)),
    ))
    .context(Context::BoolLit)
    .parse(i)
}

pub fn parse_string_lit<'a, E>(i: Span<'a>) -> IResult<E, String>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    parse_string.context(Context::StringLit).parse(i)
}

fn parse_parens<'a, E>(i: Span<'a>) -> IResult<'a, E, Expression>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    delimited(char('('), ws(parse_expr), char(')')).parse(i)
}

fn parse_call<'a, E>(
    i: Span<'a>,
) -> IResult<'a, E, (Span<'a, Identifier>, Vec<Span<'a, Expression>>)>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    preceded(
        tag("complete"),
        ws(spanned(pair(
            ws(identifier),
            map(
                opt(preceded(
                    tag("with"),
                    separated_list1(tag("and"), ws(parse_expr)),
                )),
                |x: Option<Vec<_>>| x.unwrap_or_default(),
            ),
        ))),
    )
    .context(Context::Call)
    .parse(i)
}

#[allow(clippy::type_complexity)]
fn parse_binary_operation<'a, E>(
    i: Span<'a>,
) -> IResult<
    'a,
    E,
    (
        Span<'a, Operator>,
        Span<'a, Expression>,
        Span<'a, Expression>,
    ),
>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    spanned(tuple((ws(binary_operator), ws(parse_expr), ws(parse_expr))))
        .context(Context::BinaryOperation)
        .parse(i)
}

pub fn parse_expr<'a, E>(i: Span<'a>) -> IResult<E, Expression>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    alt((
        spanned_map(parse_bool_lit, Expression::BoolLit),
        spanned_map(parse_string_lit, Expression::StringLit),
        spanned_map(num_lit, Expression::NumLit),
        spanned_map(parse_call, |(ident, args)| {
            Expression::Call(
                ident.extra.data.0.to_string(),
                args.into_iter().map(|arg| arg.extra.data).collect(),
            )
        }),
        spanned_map(parse_binary_operation, |(operator, arg1, arg2)| {
            Expression::Operation(
                operator.extra.data,
                Box::new(arg1.extra.data),
                Box::new(arg2.extra.data),
            )
        }),
        spanned_map(identifier, |ident| {
            Expression::Variable(ident.0.to_string())
        }),
        parse_parens,
    ))
    .context(Context::Expression)
    .parse(i)
}
