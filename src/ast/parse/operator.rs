use nom::error::ParseError;

use crate::ast::Operator;

use super::{
    error::IResult,
    spans::{spanned_char, spanned_value, Span},
};

pub fn binary_operator<'a, E>(i: Span<'a>) -> IResult<'a, E, Operator>
where
    E: ParseError<Span<'a>>,
{
    nom::branch::alt((
        spanned_value(&Operator::Add, spanned_char('+')),
        spanned_value(&Operator::Sub, spanned_char('-')),
        spanned_value(&Operator::Lt, spanned_char('<')),
    ))(i)
}
