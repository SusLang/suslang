use std::iter::Peekable;

use crate::tokens::Token;

pub trait Parse: Sized {
    fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Typ {
    Num,
    Str,
    Bool,
    Void,
}

impl std::fmt::Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Num => "int",
                Self::Str => "char*",
                Self::Bool => "int",
                Self::Void => "void",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // Add,
    // Sub,
    // Div,
    // Mul,
    // Mod,
    // Gt,
    // Get,
    // Lt,
    // Let,
    // Eq,
    // Not,
    // NotEq
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Call(String, Vec<Self>),
    // Operation(Operator, Box<Expression>, Box<Expression>),
    StringLit(String),
    NumLit(i32),
}

impl Parse for Expression {
    fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String> {
        match tokens.peek().copied() {
            Some(Token("complete")) => {
                tokens.next();
                let name = tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?;
                let mut args = Vec::new();
                if tokens.peek() == Some(&Token("with")) {
                    loop {
                        tokens.next();
                        let e = Self::parse(tokens)?;
                        args.push(e);
                        if tokens.peek() != Some(&Token("and")) {
                            break;
                        }
                    }

                    // while let Some(x) = tokens.next() {
                    // 	if x == Token("")
                    // }
                }

                Ok(Self::Call(name.0.to_string(), args))
            }
            Some(Token("\"")) => {
                tokens.next();
                let s = tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?;
                tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())
                    .and_then(|x| match x {
                        Token("\"") => Ok(()),
                        x => Err(format!("UNexpected closing string token: {:?}", x)),
                    })?;

                Ok(Self::StringLit(s.0.to_string()))
            }
            Some(x) if x.0.chars().all(|n| n.is_digit(10)) => {
                tokens.next();
                Ok(Self::NumLit(x.0.parse().map_err(|x| {
                    format!("Error parsing number literal: {}", x)
                })?))
            }
            Some(_) => {
                todo!("????????????????????????????????????EXPRESSION?????????????????????????")
            }
            None => Err("Expected tokens".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    // If(Expression, Block, Option<Block>),
    Return(Option<Expression>),
    Expr(Expression),
}

impl Parse for Statement {
    fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String> {
        match tokens.peek() {
            Some(Token("eject")) => {
                tokens.next();

                let expr = if tokens.peek() == Some(&Token("ඞ")) {
                    None
                } else {
                    Some(Expression::parse(tokens)?)
                };

                Ok(Self::Return(expr))
            }
            Some(_) => Expression::parse(tokens).map(Self::Expr),
            None => Err("Expected tokens".to_string()),
        }
    }
}

pub type Block = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    Func(String, Typ, Vec<(String, Typ)>, Block),
}

impl Parse for Ast {
    fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String> {
        match tokens.peek() {
            Some(Token("task")) => {
                tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?; // task
                let name = tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?;
                tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?; // with
                                                                          // TODO Leer argumentos
                let args = Vec::new();
                tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?; // ➤
                let typ = tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())
                    .and_then(|tok| match tok.0 {
                        "void" => Ok(Typ::Void),
                        "number" => Ok(Typ::Num),
                        "string" => Ok(Typ::Str),
                        "bool" => Ok(Typ::Bool),
                        x => Err(format!("Unexpected type: {}", x)),
                    })?;

                tokens
                    .next()
                    .ok_or_else(|| "Error, unexpected EOF".to_string())?; // \n

                let mut lines = Vec::new();

                loop {
                    let mut v = Vec::new();
                    if tokens.peek() != Some(&Token("ච")) {
                        break;
                    }
                    tokens.next();
                    for t in tokens.by_ref() {
                        if t == Token("\n") {
                            break;
                        } else {
                            v.push(t);
                        }
                    }
                    lines.push(v);
                }
                println!("{:?} {:?}", name, typ);
                let block = lines
                    .into_iter()
                    .map(|line| {
                        let mut tok = line.into_iter().peekable();
                        let statement = Statement::parse(&mut tok);
                        // println!("{:?} {:?}", statement, tok.next());
                        assert_eq!(tok.next(), Some(Token("ඞ")));
                        assert_eq!(tok.next(), None);
                        statement
                    })
                    .collect::<Result<Block, String>>()?;

                Ok(Self::Func(name.0.to_string(), typ, args, block))
            }
            Some(x) => Err(format!("Error, unexpected token {:?}", x)),
            None => Err("Error, unexpected EOF".to_string()),
        }
    }
}
