use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::{map, opt},
    error::{FromExternalError, ParseError},
    multi::{many0, many_m_n},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    Parser,
};
use nom_supreme::{context::ContextError, ParserExt};

use crate::ast::Statement;

use super::{
    context::Context,
    error::IResult,
    expression::parse_expr,
    identifier,
    inline_comment::ws,
    // opt::opt,
    spans::{spanned, spanned_map, Span},
    typ::parse_type,
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
                    tuple((parse_tabs(suslevel), tag("clean?"), line_ending)),
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
    move |i: Span| {
        spanned(many0(parse_statement(suslevel)))
            .context(Context::Block)
            .parse(i)
    }
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
    move |i: Span<'a>| {
        preceded(
            parse_tabs(suslevel),
            ws(alt((
                parse_if(suslevel),
                terminated(
                    spanned(map(preceded(ws(tag("eject")), opt(ws(parse_expr))), |x| {
                        Statement::Return(x.map(|s| s.extra.data))
                    })),
                    char('ඞ'),
                )
                .context(Context::Eject),
                terminated(
                    spanned(map(
                        preceded(
                            ws(tag("crewmate")),
                            separated_pair(ws(identifier), char(':'), ws(parse_type)),
                        ),
                        |(name, typ)| Statement::Declare(name.extra.data.0.into(), typ.extra.data),
                    )),
                    char('ඞ'),
                )
                .context(Context::Declare),
                terminated(
                    spanned(map(
                        preceded(ws(tag("make")), pair(ws(identifier), ws(parse_expr))),
                        |(name, typ)| Statement::Define(name.extra.data.0.into(), typ.extra.data),
                    )),
                    char('ඞ'),
                )
                .context(Context::Declare),
                terminated(spanned_map(parse_expr, Statement::Expr), char('ඞ')),
            ))),
        )
        .context(Context::Statement)
        .parse(i)
    }
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
crewmate red: numberඞ
make red 5ඞ
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
                Statement::Declare("red".into(), crate::ast::Typ::Num),
                Statement::Define("red".into(), Expression::NumLit(5)),
                Statement::Return(Some(Expression::NumLit(0))),
            ],
        );
        println!("RESULT: {b:#?}");
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
    fn parse_statement_expr() {
        const TEST_STATEMENT: &str = r#"චcomplete report with "hey\n"ඞ"#;
        let res = super::parse_statement::<super::super::error::ParseError<Span>>(1)(
            load_file_str(&"test_statement.sus", TEST_STATEMENT),
        );
        dbg!(&res);
        assert!(res.is_ok());
        assert_eq!(
            res.map(|(a, b)| (*a.fragment(), b.extra.data)).unwrap(),
            (
                "",
                Statement::Expr(Expression::Call(
                    "report".into(),
                    vec![Expression::StringLit("hey\n".into())]
                ))
            )
        )
    }

    #[test]
    fn parse_statement_eject() {
        const TEST_STATEMENT1: &str = r#"චeject "hey\n"ඞ"#;
        let res = super::parse_block::<super::super::error::ParseError<Span>>(1)(load_file_str(
            &"test_statement.sus",
            TEST_STATEMENT1,
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
                vec![Statement::Return(Some(Expression::StringLit(
                    "hey\n".into()
                )))]
            )
        );

        const TEST_STATEMENT2: &str = r#"ejectඞ"#;
        let res = super::parse_statement::<super::super::error::ParseError<Span>>(0)(
            load_file_str(&"test_statement.sus", TEST_STATEMENT2),
        );
        // dbg!(terminated(
        //     spanned::<super::super::error::ParseError<Span>, _, _>(map(
        //         preceded(ws(tag("eject")), opt(ws(super::parse_expr))),
        //         |x| { Statement::Return(x.map(|s| s.extra.data)) }
        //     )),
        //     success(()) // char('ඞ'),
        // )(load_file_str(
        //     &"test_statement.sus",
        //     TEST_STATEMENT2
        // )));
        dbg!(&res);
        assert!(res.is_ok());
        assert_eq!(
            res.map(|(a, b)| (*a.fragment(), b.extra.data)).unwrap(),
            ("", Statement::Return(None))
        )
    }
}
