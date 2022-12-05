use std::num::ParseIntError;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::success,
    error::{FromExternalError, ParseError},
    multi::{many0, many_m_n},
    sequence::preceded,
};
use nom_supreme::context::ContextError;

use crate::ast::{Block, Statement};

use super::{
    context::Context,
    error::IResult,
    expression::parse_expr,
    spans::{spanned, spanned_map, Span},
};

pub fn parse_block<'a, E>(
    suslevel: usize,
) -> impl FnMut(Span<'a>) -> IResult<'a, E, Vec<Span<'a, Statement>>>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    spanned(many0(parse_statement(suslevel)))
}

pub fn parse_statement<'a, E>(suslevel: usize) -> impl FnMut(Span<'a>) -> IResult<'a, E, Statement>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>,
{
    preceded(
        many_m_n(suslevel, suslevel, char('ච')),
        alt((spanned_map(parse_expr, Statement::Expr),)),
    )
}

#[cfg(test)]
mod tests {
    use crate::ast::{parse::spans::load_file_str, Expression, Operator, Statement};

    #[allow(unused_imports)]
    use super::*;

    const TEST_DATA: &str = r#"
sus? < 2 5
චcomplete report with "hey\n"ඞ
sus? < 2 5
චcomplete report with "hey\n"ඞ
eject 0ඞ"#;

    #[test]
    fn parse_block() {
        let data = load_file_str(&"test.sus", TEST_DATA);
        let b = super::parse_block::<super::super::error::ParseError<Span>>(0)(data).map(
            |(rest, s)| {
                (
                    *rest.fragment(),
                    s.extra
                        .data
                        .into_iter()
                        .map(|x| x.extra.data)
                        .collect::<Vec<_>>(),
                )
            },
        );
        let expected = (
            "",
            vec![
                Statement::If(
                    Expression::Operation(
                        Operator::Lt,
                        Box::new(Expression::NumLit(2)),
                        Box::new(Expression::NumLit(5)),
                    ),
                    vec![Statement::Expr(Expression::Call(
                        "report".into(),
                        vec![Expression::StringLit("hey\n".into())],
                    ))],
                    None,
                ),
                Statement::If(
                    Expression::Operation(
                        Operator::Lt,
                        Box::new(Expression::NumLit(2)),
                        Box::new(Expression::NumLit(5)),
                    ),
                    vec![Statement::Expr(Expression::Call(
                        "report".into(),
                        vec![Expression::StringLit("hey\n".into())],
                    ))],
                    None,
                ),
                Statement::Return(Some(Expression::NumLit(0))),
            ],
        );
        assert!(b.is_ok());
        let unwrapped = b.unwrap();
        assert_eq!(
            unwrapped, expected,
            "Expected: {expected:#?}\n But got: {unwrapped:#?}"
        );
    }
}
