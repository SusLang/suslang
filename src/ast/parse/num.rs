use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    Parser,
};
use nom_supreme::{context::ContextError, ParserExt};

use super::{
    context::Context,
    error::IResult,
    spans::{MapExt, Span},
};

fn hexadecimal_value<'a, E>(input: Span<'a>) -> IResult<'a, E, i32>
where
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, ParseIntError>
        + ContextError<Span<'a>, Context>,
{
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            )))
            .context(Context::HexNum),
        ),
        |out: Span| {
            i32::from_str_radix(&str::replace(out.fragment(), "_", ""), 16).map(|r| out.map(|()| r))
        },
    )(input)
}

fn octal_value<'a, E>(input: Span<'a>) -> IResult<'a, E, i32>
where
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, ParseIntError>
        + ContextError<Span<'a>, Context>,
{
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_')))))
                .context(Context::OctNum),
        ),
        |out: Span| {
            i32::from_str_radix(&str::replace(out.fragment(), "_", ""), 8).map(|r| out.map(|()| r))
        },
    )(input)
}

fn binary_value<'a, E>(input: Span<'a>) -> IResult<'a, E, i32>
where
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, ParseIntError>
        + ContextError<Span<'a>, Context>,
{
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))).context(Context::BinNum),
        ),
        |out: Span| {
            i32::from_str_radix(&str::replace(out.fragment(), "_", ""), 2).map(|r| out.map(|()| r))
        },
    )(input)
}

fn decimal_value<'a, E>(input: Span<'a>) -> IResult<'a, E, i32>
where
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, ParseIntError>
        + ContextError<Span<'a>, Context>,
{
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))
            .context(Context::DecNum),
        |out: Span| {
            str::replace(out.fragment(), "_", "")
                .parse::<i32>()
                .map(|r| out.map(|()| r))
        },
    )(input)
}

pub fn num_lit<'a, E>(input: Span<'a>) -> IResult<'a, E, i32>
where
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, ParseIntError>
        + ContextError<Span<'a>, Context>,
{
    alt((hexadecimal_value, octal_value, binary_value, decimal_value))
        .context(Context::NumLit)
        .parse(input)
}
