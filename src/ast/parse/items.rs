use std::num::ParseIntError;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, success},
    error::{FromExternalError, ParseError},
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    Parser,
};
use nom_supreme::{
    context::ContextError,
    tag::{complete::tag, TagError},
};

use crate::ast::{Ast, Typ};

use super::{
    context::Context,
    error::IResult,
    identifier,
    inline_comment::ws,
    spans::{spanned, Span},
    statement::parse_block,
    typ::parse_type,
};

pub fn parse_ast_item<'a, E>(i: Span<'a>) -> IResult<'a, E, Ast>
where
    E: ParseError<Span<'a>>
        + ContextError<Span<'a>, Context>
        + FromExternalError<Span<'a>, ParseIntError>
        + TagError<Span<'a>, &'static str>
        + 'a,
{
    /*
    task ඬ with ➤ number
    චcomplete report with "hello world"ඞ
    චeject 0ඞ
     */
    ws(spanned(alt((map(
        preceded(
            ws(tag("task")),
            tuple((
                ws(identifier),
                ws(tag("with")),
                separated_list0(
                    tag("and"),
                    separated_pair(ws(identifier), ws(char(':')), ws(parse_type)),
                ),
                alt((
                    ws(preceded(ws(char('➤')), ws(parse_type))),
                    spanned(success(Typ::Void)),
                )),
                parse_block(1),
            )),
        ),
        |(name, _, args, ret, block)| {
            Ast::Func(
                name.extra.data.0.to_string(),
                ret.extra.data,
                args.into_iter()
                    .map(|(name, typ)| (name.extra.data.0.into(), typ.extra.data))
                    .collect(),
                block.extra.data.into_iter().map(|s| s.extra.data).collect(),
            )
        },
    ),))))
    .parse(i)
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        parse::{
            error::ParseError,
            items::parse_ast_item,
            spans::{load_file_str, Span},
        },
        Ast, Expression, Statement, Typ,
    };

    #[test]
    fn test_parse_no_return_type() {
        const DATA: &str = r#"
task a with
චabcඞ"#;
        let data = load_file_str(&"test_parse_no_return_type.sus", DATA);
        let res = parse_ast_item::<ParseError<Span>>(data);
        dbg!(&res);
        let res = res.map(|(a, b)| (*a.fragment(), b.extra.data));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            (
                "",
                Ast::Func(
                    "a".into(),
                    Typ::Void,
                    vec![],
                    vec![Statement::Expr(Expression::Variable("abc".into()))]
                )
            )
        )
    }
}
