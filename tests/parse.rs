use suslang::{
    ast::{Ast, Expression, Statement, Typ},
    parse_str,
};

#[test]
fn parse_helloworld() {
    let s = include_str!("../examples/helloworld.sus");
    let ast = parse_str(s);
    assert_eq!(
        ast,
        vec![Ast::Func(
            "ඬ".into(),
            Typ::Num,
            vec![],
            vec![
                Statement::Expr(Expression::Call(
                    "report".into(),
                    vec![Expression::StringLit("hello world".into())]
                )),
                Statement::Return(Some(Expression::NumLit(0)))
            ]
        )]
    );
}
