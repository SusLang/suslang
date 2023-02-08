use std::io::Write;

use nom_locate::LocatedSpan;

use crate::ast::{
    parse::spans::{ExtraData, Span},
    Ast, Expression, Operator, Statement, Typ,
};

use super::{Codegen, Typename};

pub struct C;

const NAME_REPLACE: &[(&str, &str)] = &[("ඬ", "main")];

impl Typename for C {
    fn typename(t: &Typ) -> &'static str {
        match t {
            Typ::Num => "int",
            Typ::Str => "char*",
            Typ::Bool => "int",
            Typ::Void => "void",
        }
    }
}

impl<'a, W> Codegen<W, [Span<'a, Ast<'a>>]> for C
where
    W: Write,
{
    fn gen(&mut self, s: &[Span<'a, Ast<'a>>], buf: &mut W) -> std::io::Result<()> {
        writeln!(
            buf,
            r#"// C code generated from suslang
#include <stdio.h>
#define report printf
"#
        )?;
        for ast in s {
            self.gen(ast, buf)?;
        }
        Ok(())
    }
}

impl<'a, W> Codegen<W, Ast<'a>> for C
where
    W: Write,
{
    fn gen(&mut self, s: &Ast<'a>, buf: &mut W) -> std::io::Result<()> {
        match s {
            Ast::Func(name, typ, args, block) => {
                let mut name = name.extra.data.clone();
                for (name_r, replace) in NAME_REPLACE {
                    if name == *name_r {
                        name = replace.to_string();
                        break;
                    }
                }
                write!(buf, "{} {}(", Self::typename(&typ.extra.data), name)?;
                let args_len = args.len();
                for (i, arg) in args.iter().enumerate() {
                    let (name, typ) = &arg.extra.data;
                    write!(
                        buf,
                        "{} {}{}",
                        Self::typename(&typ.extra.data),
                        name,
                        if i == args_len - 1 { "" } else { ", " }
                    )?;
                }
                writeln!(buf, ") {{")?;

                for line in &block.extra.data {
                    write!(buf, "\t")?;
                    self.gen(line, buf)?;
                }
                writeln!(buf, "}}")?;
                Ok(())
            }
        }
    }
}

impl<'a, W> Codegen<W, Statement<'a>> for C
where
    W: Write,
{
    fn gen(&mut self, s: &Statement<'a>, buf: &mut W) -> std::io::Result<()> {
        match s {
            Statement::Return(n) => {
                write!(buf, "return ")?;
                n.as_ref().map(|x| self.gen(x, buf));
                writeln!(buf, ";")?;
            }
            Statement::Expr(m) => {
                self.gen(m, buf)?;
                writeln!(buf, ";")?;
            }
            Statement::If(cond, b, e) => {
                write!(buf, "if (")?;
                self.gen(cond, buf)?;
                writeln!(buf, ") {{")?;
                for s in &b.extra.data {
                    write!(buf, "\t")?;
                    self.gen(s, buf)?;
                }
                if let Some(e) = e {
                    writeln!(buf, "}} else {{")?;
                    for s in &e.extra.data {
                        write!(buf, "\t")?;
                        self.gen(s, buf)?;
                    }
                }
                writeln!(buf, "}}")?;
            }
            Statement::Declare(name, typ) => {
                writeln!(buf, "{} {};", Self::typename(&typ.extra.data), name).unwrap()
            }
            Statement::Define(name, expr) => {
                write!(buf, "{} = ", name)?;
                self.gen(expr, buf)?;
                writeln!(buf, ";")?;
            }
            Statement::While(cond, body) => {
                write!(buf, "while (")?;
                self.gen(cond, buf)?;
                writeln!(buf, ") {{")?;
                for s in &body.extra.data {
                    write!(buf, "\t")?;
                    self.gen(s, buf)?;
                }
                writeln!(buf, "}}")?;
            }
        }
        Ok(())
    }
}

impl<'a, W> Codegen<W, Expression<'a>> for C
where
    W: Write,
{
    fn gen(&mut self, expr: &Expression<'a>, buf: &mut W) -> std::io::Result<()> {
        match expr {
            Expression::Call(func, args) => {
                let mut func = func.extra.data.clone();
                for (name_r, replace) in NAME_REPLACE {
                    if func == *name_r {
                        func = replace.to_string();
                        break;
                    }
                }
                write!(buf, "{}(", func)?;
                let args_len = args.len();
                for (i, arg) in args.iter().enumerate() {
                    self.gen(arg, buf)?;
                    if i != args_len - 1 {
                        write!(buf, ", ")?;
                    }
                }
                write!(buf, ")")?;
            }
            Expression::NumLit(s) => write!(buf, "{}", s)?,
            Expression::StringLit(s) => write!(buf, "{:?}", s)?,
            Expression::Operation(
                LocatedSpan {
                    extra:
                        ExtraData {
                            data: Operator::Lt, ..
                        },
                    ..
                },
                lhs,
                rhs,
            ) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " < ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(
                LocatedSpan {
                    extra:
                        ExtraData {
                            data: Operator::GEt,
                            ..
                        },
                    ..
                },
                lhs,
                rhs,
            ) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " >= ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(
                LocatedSpan {
                    extra:
                        ExtraData {
                            data: Operator::Eq, ..
                        },
                    ..
                },
                lhs,
                rhs,
            ) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " == ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(
                LocatedSpan {
                    extra:
                        ExtraData {
                            data: Operator::Add,
                            ..
                        },
                    ..
                },
                lhs,
                rhs,
            ) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " + ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(
                LocatedSpan {
                    extra:
                        ExtraData {
                            data: Operator::Sub,
                            ..
                        },
                    ..
                },
                lhs,
                rhs,
            ) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " - ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Operation(
                LocatedSpan {
                    extra:
                        ExtraData {
                            data: Operator::Mod,
                            ..
                        },
                    ..
                },
                lhs,
                rhs,
            ) => {
                self.gen(lhs.as_ref(), buf)?;
                write!(buf, " % ")?;
                self.gen(rhs.as_ref(), buf)?;
            }
            Expression::Variable(x) => write!(buf, "{}", x)?,
            Expression::BoolLit(b) => write!(buf, "{}", i32::from(*b))?,
        };
        Ok(())
    }
}
