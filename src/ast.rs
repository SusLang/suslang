pub mod parse;

// use std::iter::Peekable;

// use crate::tokens::Token;

use std::fmt::Display;

use self::parse::spans::Span;

// fn read_block<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Vec<Token<'a>> {
//     let mut code_block = Vec::new();
//     loop {
//         if tokens.peek() != Some(&Token("ච")) {
//             break;
//         }
//         tokens.next();
//         for t in tokens.by_ref() {
//             code_block.push(t);
//             if t == Token("\n") {
//                 break;
//             }
//         }
//     }

//     code_block
// }

// // TODO ADD SPANS FOR AST

// pub trait Parse: Sized {
//     fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String>;
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Typ {
    Num,
    Str,
    Bool,
    Void,
    // function???
}

impl std::fmt::Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Num => "int",
                Self::Str => "string",
                Self::Bool => "boolean",
                Self::Void => "void",
            }
        )
    }
}

// impl Typ {
//     fn parse_tok(tok: Token) -> Result<Self, String> {
//         match tok.0 {
//             "void" => Ok(Self::Void),
//             "number" => Ok(Self::Num),
//             "string" => Ok(Self::Str),
//             "bool" => Ok(Self::Bool),
//             x => Err(format!("Unexpected type: {}", x)),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mod,
    // Div,
    // Mul,
    // Mod,
    // Gt,
    GEt,
    // Get,
    Lt,
    // Let,
    Eq,
    // Not,
    // NotEq
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operator::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mod => write!(f, "%"),
            GEt => write!(f, ">="),
            Lt => write!(f, "<"),
            Eq => write!(f, "=="),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Call(Span<'a, String>, Vec<Span<'a, Self>>),
    Operation(Span<'a, Operator>, Box<Span<'a, Self>>, Box<Span<'a, Self>>),
    StringLit(String),
    NumLit(i32),
    BoolLit(bool),
    Variable(String),
}

// impl Parse for Expression {
//     fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String> {
//         match tokens.peek().copied() {
//             Some(Token("complete")) => {
//                 tokens.next();
//                 let name = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?;
//                 let mut args = Vec::new();
//                 if tokens.peek() == Some(&Token("with")) {
//                     loop {
//                         tokens.next();
//                         let e = Self::parse(tokens)?;
//                         args.push(e);
//                         if tokens.peek() != Some(&Token("and")) {
//                             break;
//                         }
//                     }

//                     // while let Some(x) = tokens.next() {
//                     // 	if x == Token("")
//                     // }
//                 }

//                 Ok(Self::Call(name.0.to_string(), args))
//             }
//             Some(Token("\"")) => {
//                 tokens.next();
//                 let s = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?;
//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())
//                     .and_then(|x| match x {
//                         Token("\"") => Ok(()),
//                         x => Err(format!("UNexpected closing string token: {:?}", x)),
//                     })?;

//                 Ok(Self::StringLit(s.0.to_string()))
//             }
//             Some(Token("<")) => {
//                 tokens.next();

//                 let lhs = Self::parse(tokens)?;
//                 let rhs = Self::parse(tokens)?;

//                 Ok(Self::Operation(Operator::Lt, Box::new(lhs), Box::new(rhs)))
//             }
//             Some(Token(">=")) => {
//                 tokens.next();

//                 let lhs = Self::parse(tokens)?;
//                 let rhs = Self::parse(tokens)?;

//                 Ok(Self::Operation(Operator::GEt, Box::new(lhs), Box::new(rhs)))
//             }
//             Some(Token("==")) => {
//                 tokens.next();

//                 let lhs = Self::parse(tokens)?;
//                 let rhs = Self::parse(tokens)?;

//                 Ok(Self::Operation(Operator::Eq, Box::new(lhs), Box::new(rhs)))
//             }
//             // TODO the rest of the logical ops
//             Some(Token("+")) => {
//                 tokens.next();

//                 let lhs = Self::parse(tokens)?;
//                 let rhs = Self::parse(tokens)?;

//                 Ok(Self::Operation(Operator::Add, Box::new(lhs), Box::new(rhs)))
//             }
//             Some(Token("-")) => {
//                 tokens.next();

//                 let lhs = Self::parse(tokens)?;
//                 let rhs = Self::parse(tokens)?;

//                 Ok(Self::Operation(Operator::Sub, Box::new(lhs), Box::new(rhs)))
//             }
//             Some(Token("%")) => {
//                 tokens.next();

//                 let lhs = Self::parse(tokens)?;
//                 let rhs = Self::parse(tokens)?;

//                 Ok(Self::Operation(Operator::Mod, Box::new(lhs), Box::new(rhs)))
//             }
//             // TODO the rest of the arithmetic ops
//             Some(x) if x.0.chars().all(|n| n.is_ascii_digit()) => {
//                 tokens.next();
//                 Ok(Self::NumLit(x.0.parse().map_err(|x| {
//                     format!("Error parsing number literal: {}", x)
//                 })?))
//             }
//             Some(Token("sus")) => {
//                 tokens.next();
//                 Ok(Self::BoolLit(true))
//             }
//             Some(Token("clean")) => {
//                 tokens.next();
//                 Ok(Self::BoolLit(false))
//             }
//             Some(x) => {
//                 tokens.next();
//                 Ok(Self::Variable(x.0.to_string()))
//             }
//             None => Err("Expected tokens".to_string()),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    If(
        Span<'a, Expression<'a>>,
        Span<'a, Block<'a>>,
        Option<Span<'a, Block<'a>>>,
    ),
    While(Span<'a, Expression<'a>>, Span<'a, Block<'a>>),
    Return(Option<Span<'a, Expression<'a>>>),
    Expr(Span<'a, Expression<'a>>),
    Declare(Span<'a, String>, Span<'a, Typ>),
    Define(Span<'a, String>, Span<'a, Expression<'a>>),
}

// impl Parse for Statement {
//     fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String> {
//         match tokens.peek() {
//             Some(Token("eject")) => {
//                 tokens.next();

//                 let expr = if tokens.peek() == Some(&Token("ඞ")) {
//                     None
//                 } else {
//                     Some(Expression::parse(tokens)?)
//                 };

//                 Ok(Self::Return(expr))
//             }
//             Some(Token("sus?")) => {
//                 tokens.next();
//                 let expr = Expression::parse(tokens)?;
//                 println!("{:?}", expr);
//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())
//                     .and_then(|x| match x {
//                         Token("\n") => Ok(()),
//                         x => Err(format!("Expected EOL: {:?}", x)),
//                     })?;
//                 let mut code_block = read_block(tokens).into_iter().peekable();
//                 let mut block = Block::new();
//                 while code_block.peek().is_some() {
//                     let statement = Self::parse(&mut code_block)?;
//                     assert_eq!(code_block.next(), Some(Token("ඞ"))); //TODO: this line gives error when ther is and if inside an if (No EOL after if)
//                     assert!(matches!(code_block.next(), Some(Token("\n")) | None));
//                     block.push(statement);
//                 }
//                 println!("if {:?}", block);
//                 println!("next {:?}", tokens.peek());
//                 let mut else_block = None;
//                 if tokens.peek() == Some(&Token("clean?")) {
//                     tokens.next();
//                     tokens
//                         .next()
//                         .ok_or_else(|| "Error, unexpected EOF".to_string())
//                         .and_then(|x| match x {
//                             Token("\n") => Ok(()),
//                             x => Err(format!("Expected EOL: {:?}", x)),
//                         })?;
//                     let mut code_block = read_block(tokens).into_iter().peekable();
//                     let mut block = Block::new();
//                     while code_block.peek().is_some() {
//                         let statement = Self::parse(&mut code_block)?;
//                         // assert if the next token is a newline or an sus?
//                         assert_eq!(code_block.next(), Some(Token("ඞ")));
//                         assert!(matches!(code_block.next(), Some(Token("\n")) | None));
//                         block.push(statement);
//                     }
//                     else_block = Some(block)
//                 }
//                 println!("else {:?}", else_block);
//                 println!("next {:?}", tokens.peek());
//                 Ok(Self::If(expr, block, else_block))
//             }

//             Some(Token("crewmate")) => {
//                 // crewmate red: int
//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; // crewamte

//                 let name = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; //red

//                 assert_eq!(tokens.next(), Some(Token(":")));

//                 let typ = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())
//                     .and_then(Typ::parse_tok)?; // int

//                 //assert_eq!(tokens.next(), Some(Token("ඞ")));
//                 //assert!(matches!(tokens.next(), Some(Token("\n")) | None));

//                 Ok(Self::Declare(name.0.to_string(), typ))
//             }

//             Some(Token("make")) => {
//                 // make red 5
//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; // make

//                 let name = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; // red

//                 let expr = Expression::parse(tokens)?;

//                 Ok(Self::Define(name.0.to_string(), expr))
//             }

//             Some(_) => Expression::parse(tokens).map(Self::Expr),
//             None => Err("Expected tokens".to_string()),
//         }
//     }
// }

pub type Block<'a> = Vec<Span<'a, Statement<'a>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast<'a> {
    Func(
        Span<'a, String>,
        Span<'a, Typ>,
        Vec<Span<'a, (Span<'a, String>, Span<'a, Typ>)>>,
        Span<'a, Block<'a>>,
    ),
    Mod(Span<'a, String>),
    Import(Span<'a, Vec<String>>),
    //Declare(String, Typ),
    //Define(String, Statement)
}

// impl Parse for Ast {
//     fn parse<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self, String> {
//         match tokens.peek() {
//             Some(Token("task")) => {
//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; // task
//                 let name = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?;
//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; // with

//                 let mut args = Vec::new();

//                 while let Some(x) = tokens.next() {
//                     if x == Token("➤") {
//                         break;
//                     } else {
//                         // dbg!(tokens
//                         //     .next()
//                         //     .ok_or_else(|| "Error, unexpected EOF".to_string())?); // crewmate

//                         let name = tokens
//                             .next()
//                             .ok_or_else(|| "Error, unexpected EOF".to_string())?;

//                         tokens
//                             .next()
//                             .ok_or_else(|| "Error, unexpected EOF".to_string())?; // :

//                         let typ = tokens
//                             .next()
//                             .ok_or_else(|| "Error, unexpected EOF".to_string())
//                             .and_then(Typ::parse_tok)?;

//                         args.push((name.0.to_string(), typ));
//                     }
//                 }
//                 let typ = tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())
//                     .and_then(Typ::parse_tok)?;

//                 tokens
//                     .next()
//                     .ok_or_else(|| "Error, unexpected EOF".to_string())?; // \n

//                 let mut code_block = read_block(tokens).into_iter().peekable();
//                 // println!("{:?} {:?}", name, typ);

//                 let mut block = Block::new();
//                 while code_block.peek().is_some() {
//                     let statement = Statement::parse(&mut code_block)?;
//                     if !matches!(statement, Statement::If(_, _, _)) {
//                         assert_eq!(code_block.next(), Some(Token("ඞ")));
//                         assert!(matches!(code_block.next(), Some(Token("\n")) | None));
//                     }
//                     // dbg!(&statement);
//                     // dbg!(code_block.peek());
//                     block.push(statement);
//                 }

//                 Ok(Self::Func(name.0.to_string(), typ, args, block))
//             }

//             /*Some(Token("crewmate")) => { // crewmate red: int
//                 tokens
//                 .next()
//                 .ok_or_else(|| "Error, unexpected EOF".to_string())?; // crewamte

//                 let name = tokens
//                 .next()
//                 .ok_or_else(|| "Error, unexpected EOF".to_string())?; //red

//                 assert_eq!(tokens.next(), Some(Token(":")));

//                 let typ = tokens
//                 .next()
//                 .ok_or_else(|| "Error, unexpected EOF".to_string())
//                 .and_then(Typ::parse_tok)?; // int

//                 assert_eq!(tokens.next(), Some(Token("ඞ")));
//                 assert!(matches!(tokens.next(), Some(Token("\n")) | None));

//                 Ok(Self::Declare(name.0.to_string(), typ))
//             }*/
//             /*Some(Token("make")) => {
//                 todo!();
//                 tokens.next().ok_or_else(|| "Error, unexpected EOF".to_string())?; //make
//                 //OK(Self::Define("not yet", ()));
//             }*/
//             Some(x) => Err(format!("Error, unexpected token {:?}", x)),
//             None => Err("Error, unexpected EOF".to_string()),
//         }
//     }
// }

#[cfg(test)]
mod testing {
    macro_rules! span {
        // `()` indicates that the macro takes no argument.
        ($inner:pat) => {
            LocatedSpan {
                extra: ExtraData { data: $inner, .. },
                ..
            }
        };
    }

    use nom_locate::LocatedSpan;

    use super::{parse::spans::ExtraData, Ast, Block, Typ};

    fn matches_func<'a, B: FnOnce(&Block) -> bool + 'a>(
        name: &'a str,
        args: &'a [(&'a str, &'a Typ)],
        return_type: &'a Typ,
        block: B,
    ) -> impl FnOnce(&Ast) -> bool + 'a {
        move |s| {
            if let Ast::Func(span!(fname), span!(ret), args_o, span!(b)) = s {
                fname == name
                    && ret == return_type
                    && block(b)
                    && args_o
                        .iter()
                        .enumerate()
                        .all(|(i, span!((span!(n), span!(t))))| {
                            i < args.len() && {
                                let (name, typ) = args[i];
                                name == n && typ == t
                            }
                        })
            } else {
                false
            }
        }
    }

    fn matches_array<'a, T: 'a, I: Iterator<Item = Box<dyn FnOnce(&T) -> bool>>>(
        matchers: I,
    ) -> impl FnOnce(&[T]) -> bool {
        move |arr| {
            matchers
                .enumerate()
                .all(|(i, x)| i < arr.len() && x(&arr[i]))
        }
    }
}
