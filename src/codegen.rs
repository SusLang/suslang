use std::io::Write;

use crate::ast::{Ast, Block, Expression, Statement, Typ};

const NAME_REPLACE: &[(&str, &str)] = &[("ඬ", "main")];

pub fn gen_c<W: Write>(ast: Vec<Ast>, buf: &mut W) -> std::io::Result<()> {
    writeln!(
        buf,
        r#"#include <stdio.h>
void report(char* s) {{
    printf("%s\n", s);
}}
"#
    )?;
    for a in ast {
        match a {
            Ast::Func(name, typ, args, block) => gen_c_func(name, typ, args, block, buf),
        }?;
    }
    Ok(())
}

pub fn gen_c_func<W: Write>(
    mut name: String,
    typ: Typ,
    args: Vec<(String, Typ)>,
    block: Block,
    buf: &mut W,
) -> std::io::Result<()> {
    for (name_r, replace) in NAME_REPLACE {
        if name == *name_r {
            name = replace.to_string();
            break;
        }
    }
    write!(buf, "{} {}(", typ, name)?;
    let args_len = args.len();
    for (i, (name, typ)) in args.into_iter().enumerate() {
        write!(
            buf,
            "{} {}{}",
            typ,
            name,
            if i == args_len - 1 { "" } else { ", " }
        )?;
    }
    writeln!(buf, ") {{")?;

    for line in block {
        write!(buf, "\t")?;
        match line {
            Statement::Return(n) => {
                write!(buf, "return ")?;
                n.map(|x| gen_c_expr(x, buf));
                writeln!(buf, ";")?;
            }
            Statement::Expr(m) => {
                gen_c_expr(m, buf)?;
                writeln!(buf, ";")?;
            }
        }
    }
    writeln!(buf, "}}")?;
    Ok(())
}

pub fn gen_c_expr<W: Write>(expr: Expression, buf: &mut W) -> std::io::Result<()> {
    match expr {
        Expression::Call(mut func, args) => {
            for (name_r, replace) in NAME_REPLACE {
                if func == *name_r {
                    func = replace.to_string();
                    break;
                }
            }
            write!(buf, "{}(", func)?;
            let args_len = args.len();
            for (i, arg) in args.into_iter().enumerate() {
                gen_c_expr(arg, buf)?;
                if i != args_len - 1 {
                    write!(buf, ", ")?;
                }
            }
            write!(buf, ")")?;
        }
        Expression::NumLit(s) => write!(buf, "{}", s)?,
        Expression::StringLit(s) => write!(buf, "\"{}\"", s)?,
    };
    Ok(())
}
