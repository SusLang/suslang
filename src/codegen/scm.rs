use std::io::Write;

use crate::ast::{parse::spans::Span, Ast, Expression, Operator, Statement, Typ};

use super::{Codegen, Codegeneable};

pub struct Scm;

const fn default_value(typ: Typ) -> &'static str {
    match typ {
        Typ::Num => "0",
        Typ::Str => r#""""#,
        Typ::Bool => "'false",
        Typ::Void => "void",
    }
}

pub struct Block<T>(pub T);

// impl<T> From<T> for Block<T> {
//     fn from(v: T) -> Self {
//         Self(v)
//     }
// }

// impl<T> From<Block<T>> for T {
//     fn from(v: Block<T>) -> Self {
//         v.0
//     }
// }

fn write_eval<W>(
    operator: &str,
    operands: &[&dyn super::Codegeneable<W, Scm>],
    buf: &mut W,
) -> std::io::Result<()>
where
    W: Write,
{
    write!(buf, "( {operator} ")?;
    for t in operands.iter() {
        (*t).gen(&mut Scm, buf)?;
        write!(buf, " ")?;
    }
    writeln!(buf, ")")?;
    Ok(())
}

impl<'a, W> Codegen<W, [Span<'a, Ast<'a>>]> for Scm
where
    W: Write,
{
    // main one
    fn gen(&mut self, s: &[Span<'a, Ast<'a>>], buf: &mut W) -> std::io::Result<()> {
        writeln!(
            buf,
            r#"; scheme code generated from suslang
( define ( report f )
    ( display f ) )"#
        )?;
        for ast in s {
            self.gen(ast, buf)?;
        }
        writeln!(
            buf,
            r#"(ඬ)
(exit)"#
        )?; // TODO change so that everything has to be inside ඬ to keep C standard
        Ok(())
    }
}

impl<'a, W> Codegen<W, Ast<'a>> for Scm
where
    W: Write,
{
    fn gen(&mut self, s: &Ast, buf: &mut W) -> std::io::Result<()> {
        match s {
            Ast::Mod(_) => (),
            Ast::Import(_) => todo!(),
            Ast::Func(name, _, args, blocks) => {
                write!(buf, "( define ( {} ", name.extra.data)?;
                for arg in args.iter() {
                    let (name, _) = &arg.extra.data;
                    write!(buf, "{} ", name.extra.data)?;
                }
                writeln!(buf, ")")?;

                for line in &blocks.extra.data {
                    write!(buf, "\t")?;
                    self.gen(line, buf)?;
                }
                writeln!(buf, ")")?;
            }
        }
        Ok(())
    }
}

impl<W> Codegen<W, [&dyn super::Codegeneable<W, Self>]> for Scm
where
    W: Write,
{
    fn gen(&mut self, s: &[&dyn super::Codegeneable<W, Self>], buf: &mut W) -> std::io::Result<()> {
        write!(buf, "( ")?;
        for c in s {
            (*c).gen(self, buf)?;
        }
        write!(buf, " )")?;
        Ok(())
    }
}

impl<W, T> Codegen<W, Vec<T>> for Scm
where
    W: Write,
    T: Codegeneable<W, Self>,
{
    fn gen(&mut self, s: &Vec<T>, buf: &mut W) -> std::io::Result<()> {
        write!(buf, " ")?;
        for c in s {
            (*c).gen(self, buf)?;
        }
        write!(buf, " ")?;
        Ok(())
    }
}

impl<'a, W> Codegen<W, Block<&[Span<'a, Statement<'a>>]>> for Scm
where
    W: Write,
{
    fn gen(&mut self, s: &Block<&[Span<'a, Statement<'a>>]>, buf: &mut W) -> std::io::Result<()> {
        write!(buf, " ( begin ")?;
        for c in s.0 {
            c.gen(self, buf)?;
        }
        write!(buf, " ) ")?;
        Ok(())
    }
}

impl<'a, W> Codegen<W, Statement<'a>> for Scm
where
    W: Write,
{
    fn gen(&mut self, s: &Statement, buf: &mut W) -> std::io::Result<()> {
        match s {
            Statement::Return(s) => {
                if let Some(r) = s {
                    self.gen(r, buf)?;
                } else {
                    write!(buf, "(void)")?;
                }
            }

            Statement::Expr(e) => {
                self.gen(e, buf)?;
            }

            Statement::If(cond, b, e) => {
                if let Some(b2) = e {
                    write_eval(
                        "if",
                        &[
                            cond,
                            &Block(b.extra.data.as_slice()),
                            &Block(b2.extra.data.as_slice()),
                        ],
                        buf,
                    )?;
                } else {
                    write_eval("if", &[cond, &Block(b.extra.data.as_slice())], buf)?;
                }
            }

            Statement::While(cond, b) => {
                write_eval("while", &[cond, &Block(b.extra.data.as_slice())], buf)?;
            }

            Statement::Declare(name, typ) => write_eval(
                "define",
                &[
                    &Expression::Variable(name.to_string()),
                    &Expression::Variable(default_value(typ.extra.data).to_string()),
                ],
                buf,
            )?,
            Statement::Define(name, val) => {
                write_eval("set!", &[&Expression::Variable(name.to_string()), val], buf)?
            } //x => todo!("{:?}", x)
        }
        Ok(())
    }
}

impl<'a, W> Codegen<W, Expression<'a>> for Scm
where
    W: Write,
{
    fn gen(&mut self, expr: &Expression, buf: &mut W) -> std::io::Result<()> {
        match expr {
            Expression::Call(name, args) => {
                /*write!(buf, "( {} ", name)?;
                for (t) in  args.iter() {
                    self.gen(t, buf)?;
                }
                writeln!(buf, ")")?;*/
                write_eval(
                    &name.extra.data,
                    args.iter()
                        .map(|x| x as &dyn Codegeneable<W, Self>)
                        .collect::<Vec<_>>()
                        .as_slice(),
                    buf,
                )?;
            }

            Expression::NumLit(s) => write!(buf, "{s}")?,
            Expression::StringLit(s) => write!(buf, "{s:?}")?,

            Expression::Variable(v) => write!(buf, " {v} ")?,

            Expression::BoolLit(b) => write!(buf, "{}", if *b { "'true" } else { "'false" })?,

            Expression::Operation(op, b1, b2) => match op.extra.data {
                Operator::Add => write_eval("+", &[b1.as_ref(), b2.as_ref()], buf)?,
                Operator::Sub => write_eval("-", &[b1.as_ref(), b2.as_ref()], buf)?,
                Operator::Mod => write_eval("modulo", &[b1.as_ref(), b2.as_ref()], buf)?,
                Operator::Lt => write_eval("<", &[b1.as_ref(), b2.as_ref()], buf)?,
                Operator::GEt => write_eval(">=", &[b1.as_ref(), b2.as_ref()], buf)?,
                Operator::Eq => write_eval("=", &[b1.as_ref(), b2.as_ref()], buf)?,

                #[allow(unreachable_patterns)]
                x => todo!("{:?}", x),
            },
            #[allow(unreachable_patterns)]
            x => todo!("{:?}", x),
        }
        Ok(())
    }
} //scheme *
