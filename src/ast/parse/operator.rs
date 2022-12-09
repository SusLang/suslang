use nom::error::ParseError;
use nom_supreme::tag::{complete::tag, TagError};

use crate::ast::Operator;

use super::{
    error::IResult,
    spans::{spanned_char, spanned_value, Span},
};

pub fn binary_operator<'a, E>(i: Span<'a>) -> IResult<'a, E, Operator>
where
    E: ParseError<Span<'a>> + TagError<Span<'a>, &'static str>,
{
    nom::branch::alt((
        spanned_value(&Operator::Add, spanned_char('+')),
        spanned_value(&Operator::Sub, spanned_char('-')),
        spanned_value(&Operator::Lt, spanned_char('<')),
        spanned_value(&Operator::Mod, spanned_char('%')),
        spanned_value(&Operator::Eq, tag("==")),
        spanned_value(&Operator::GEt, tag(">=")),
    ))(i)
}
