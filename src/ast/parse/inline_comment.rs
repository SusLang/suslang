use std::ops::RangeTo;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::recognize,
    error::ParseError,
    multi::many0,
    sequence::{delimited, tuple},
    AsChar, Compare, FindSubstring, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};

const START_INLINE_COMMENT: &str = "(*";
const END_INLINE_COMMENT: &str = "*)";

pub fn inline_comment<
    'a,
    I: Clone
        + nom::Offset
        + nom::Slice<RangeTo<usize>>
        + InputTake
        + Compare<&'a str>
        + FindSubstring<&'a str>,
    E: ParseError<I>,
>(
    i: I,
) -> nom::IResult<I, I, E> {
    recognize(tuple((
        tag(START_INLINE_COMMENT),
        take_until(END_INLINE_COMMENT),
        tag(END_INLINE_COMMENT),
    )))(i)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, I, I2, I3, P, O, E>(parser: P) -> impl FnMut(I) -> nom::IResult<I, O, E> + 'a
where
    P: Parser<I, O, E> + 'a,
    I: Clone
        + nom::Offset
        + nom::Slice<RangeTo<usize>>
        + InputTake
        + Compare<&'a str>
        + FindSubstring<&'a str>
        + InputLength
        + InputIter<Item = I2>
        + InputTakeAtPosition<Item = I3>
        + 'a,
    E: ParseError<I> + 'a,
    O: 'a,
    I2: Clone + AsChar,
    I3: Clone + AsChar,
{
    delimited(
        many0(alt((multispace1, inline_comment))),
        parser,
        many0(alt((multispace1, inline_comment))),
    )
}
