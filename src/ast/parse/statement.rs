use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, newline},
    combinator::{map, opt},
    error::{FromExternalError, ParseError},
    multi::{many0, many_m_n},
    sequence::{pair, preceded, terminated, tuple},
};
use nom_supreme::context::ContextError;

use crate::ast::{Block, Statement};

use super::{
    context::Context,
    error::IResult,
    expression::parse_expr,
    inline_comment::ws,
    spans::{spanned, spanned_map, Span},
};

pub fn parse_if<'a, E>(suslevel: usize) -> impl FnMut(Span<'a>) -> IResult<'a, E, Statement>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>
        + 'a,
{
    spanned(preceded(
        ws(tag("sus?")),
        map(
            tuple((
                parse_expr,
                parse_block(suslevel + 1),
                opt(preceded(
                    tuple((parse_tabs(suslevel), tag("clean?"), newline)),
                    parse_block(suslevel + 1),
                )),
            )),
            |(e, b, else_b)| {
                Statement::If(
                    e.extra.data,
                    b.extra.data.into_iter().map(|s| s.extra.data).collect(),
                    else_b.map(|else_b| {
                        else_b
                            .extra
                            .data
                            .into_iter()
                            .map(|s| s.extra.data)
                            .collect()
                    }),
                )
            },
        ),
    ))
}

pub fn parse_block<'a, E>(
    suslevel: usize,
) -> impl FnMut(Span<'a>) -> IResult<'a, E, Vec<Span<'a, Statement>>>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>
        + 'a,
{
    spanned(many0(parse_statement(suslevel)))
}

pub fn parse_tabs<'a, E>(
    suslevel: usize,
) -> impl FnMut(Span<'a>) -> nom::IResult<Span<'a>, Vec<char>, E>
where
    E: ParseError<Span<'a>>,
{
    many_m_n(suslevel, suslevel, char('ච'))
}

pub fn parse_statement<'a, E>(suslevel: usize) -> impl FnMut(Span<'a>) -> IResult<'a, E, Statement>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>
        + 'a,
{
    preceded(
        parse_tabs(suslevel),
        ws(alt((
            move |i: Span<'a>| parse_if(suslevel)(i),
            terminated(spanned_map(parse_expr, Statement::Expr), char('ඞ')),
        ))),
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

    #[test]
    fn parse_if() {
        const TEST_IF: &str = r#"sus? < 2 5
චcomplete report with "hey\n"ඞ
ඞ

"#;
        let res = super::parse_if::<super::super::error::ParseError<Span>>(0)(load_file_str(
            &"test_if.sus",
            TEST_IF,
        ));
        dbg!(&res);
        assert!(res.is_ok());
        assert_eq!(
            res.map(|(a, b)| (*a.fragment(), b.extra.data)).unwrap(),
            (
                "",
                Statement::If(
                    Expression::Operation(
                        Operator::Lt,
                        Box::new(Expression::NumLit(2)),
                        Box::new(Expression::NumLit(5))
                    ),
                    vec![Statement::Expr(Expression::Call(
                        "report".into(),
                        vec![Expression::StringLit("hey\n".into())]
                    ))],
                    None
                )
            )
        )
    }

    #[test]
    fn parse_statement() {
        const TEST_STATEMENT: &str = r#"චcomplete report with "hey\n"ඞ"#;
        let res = super::parse_block::<super::super::error::ParseError<Span>>(1)(load_file_str(
            &"test_statement.sus",
            TEST_STATEMENT,
        ));
        dbg!(&res);
        assert!(res.is_ok());
        assert_eq!(
            res.map(|(a, b)| (
                *a.fragment(),
                b.extra.data.into_iter().map(|s| s.extra.data).collect()
            ))
            .unwrap(),
            (
                "",
                vec![Statement::Expr(Expression::Call(
                    "report".into(),
                    vec![Expression::StringLit("hey\n".into())]
                ))]
            )
        )
    }
}
