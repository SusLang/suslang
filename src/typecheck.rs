// Replace with Typ if functions is added as a type

use std::fmt::Debug;

use crate::{
    ast::{Ast, Expression, Operator, Statement, Typ},
    scope::{GlobalScope, Scope},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Function(Vec<Self>, Box<Self>),
    Void,
    Number,
    Bool,
    String,
}

impl From<Typ> for Type {
    fn from(t: Typ) -> Self {
        match t {
            Typ::Num => Self::Number,
            Typ::Str => Self::String,
            Typ::Bool => Self::Bool,
            Typ::Void => Self::Void,
        }
    }
}

pub fn typecheck(a: &[Ast]) {
    let mut scopes = GlobalScope::new();
    for a in a {
        match a {
            Ast::Func(name, ret, args, _) => scopes.add(
                name,
                Type::Function(
                    args.iter()
                        .map(|s| Type::from(s.extra.data.1.extra.data))
                        .collect(),
                    Box::new(Type::from(ret.extra.data)),
                ),
            ),
        }
    }

    for a in a {
        match a {
            Ast::Func(name, ret, args, body) => {
                let ret: Type = ret.extra.data.into();
                let f_name = name;
                let mut scope = scopes.push();
                for arg in args {
                    scope.add(name, (*t).into());
                }

                typecheck_body(scope, f_name, &ret, body);
            }
        }
    }
}

fn typecheck_body<'a, S>(mut scope: S, f_name: &str, ret: &Type, body: &'a [Statement])
where
    S: Scope<'a, Type>,
{
    for line in body {
        match line {
            Statement::If(cond, body, else_body) => {
                // typecheck condition
                let e_type = typecheck_expr(&mut scope, f_name, cond);
                if e_type != Type::Bool {
                    panic!("Error on if condition in function {f_name}: Expected Bool but found {e_type:?}")
                }
                typecheck_body(scope.push(), f_name, ret, body);
                if let Some(else_body) = else_body.as_ref() {
                    typecheck_body(scope.push(), f_name, ret, else_body);
                }
            }
            Statement::While(cond, body) => {
                // typecheck condition
                let e_type = typecheck_expr(&mut scope, f_name, cond);
                if e_type != Type::Bool {
                    panic!("Error on if condition in function {f_name}: Expected Bool but found {e_type:?}")
                }
                typecheck_body(scope.push(), f_name, ret, body);
            }
            Statement::Return(x) => {
                // Check expression type is the same as ret
                let e_type = x
                    .as_ref()
                    .map(|x| typecheck_expr(&mut scope, f_name, x))
                    .unwrap_or(Type::Void);
                if &e_type != ret {
                    panic!("Error on return type for function {f_name}: expected {ret:?} but found {e_type:?}")
                }
                break;
            }
            Statement::Expr(e) => {
                // Typecheck expression
                typecheck_expr(&mut scope, f_name, e);
            }
            Statement::Declare(name, t) => {
                scope.add(name, (*t).into());
            }
            Statement::Define(name, e) => {
                let t = scope.get(name).cloned();
                t.map_or_else(|| {
                    panic!("{} is not defined", name);
                }, |t| {
                    let e_type = typecheck_expr(&mut scope, f_name, e);
                    if t != e_type {
                        panic!("Error on variable assigment in function {f_name}: Expected {t:?}, but found {e_type:?}");
                    }
                })
            }
        }
    }
}

fn typecheck_expr<'a, S>(scope: &mut S, f_name: &str, e: &Expression) -> Type
where
    S: Scope<'a, Type> + ?Sized,
{
    match e {
        Expression::Call(name, args) if name == "report" => {
            // COMPILER MAGIIIC
            if args.is_empty() {
                panic!("On function {f_name}: Nothing to report");
            }
            let mut expected_args = Vec::new();
            for c in match args.first() {
                Some(Expression::StringLit(s)) => s,
                Some(e) => {
                    panic!("On function {f_name}: Expected string literal on report, found {e:?}")
                }
                None => unreachable!(),
            }
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            {
                if c[0] == '%' {
                    match c[1] {
                        'd' => expected_args.push(vec![Type::Number, Type::Bool]),
                        's' => expected_args.push(vec![Type::String]),
                        '%' => (), // Ignore
                        x => panic!("On function {f_name}: Unexpected char {x:?} after %"),
                    }
                }
            }

            let mut will_crash = false;
            if expected_args.len() != args.len() - 1 {
                panic!("On function {f_name}: Unexpected number of format arguments on report, expected {}, but found {}", expected_args.len(), args.len()-1);
            }
            for (found, expected) in args
                .iter()
                .skip(1)
                .map(|x| typecheck_expr(scope, f_name, x))
                .zip(expected_args)
            {
                if !expected.contains(&found) {
                    will_crash = true;
                    eprintln!("On function {f_name}: In arguments for call to report expected {expected:?}, but found {found:?}")
                }
            }
            if will_crash {
                panic!("On function {f_name}: Error on report arguments")
            }

            Type::Void
        }

        Expression::Call(name, args) if name == "len" => {
            if args.len() != 1 {
                panic!(
                    "On function {f_name}: Expected 1 argument for len, found {}",
                    args.len()
                );
            }
            let t = typecheck_expr(scope, f_name, args.first().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            Type::Number
        }

        Expression::Call(name, args) if name == "getelement" => {
            if args.len() != 2 {
                panic!(
                    "On function {f_name}: Expected 2 arguments for getelement, found {}",
                    args.len()
                );
            }
            let t = typecheck_expr(scope, f_name, args.first().unwrap());
            if t != Type::String {
                // for now only strings
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            let t = typecheck_expr(scope, f_name, args.last().unwrap());
            if t != Type::Number {
                panic!("On function {f_name}: Expected Number, found {t:?}");
            }
            Type::String
        }

        Expression::Call(name, args) if name == "setelelment" => {
            if args.len() != 3 {
                panic!(
                    "On function {f_name}: Expected 3 arguments for setelement, found {}",
                    args.len()
                );
            }
            let t = typecheck_expr(scope, f_name, args.first().unwrap());
            if t != Type::String {
                // for now only strings
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            let t = typecheck_expr(scope, f_name, args.get(1).unwrap());
            if t != Type::Number {
                panic!("On function {f_name}: Expected Number, found {t:?}");
            }
            let t = typecheck_expr(scope, f_name, args.last().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            Type::String // for now only strings
        }
        //TODO: move this to string library, doing this here is a hack
        Expression::Call(name, args) if name == "replace" => {
            if args.len() != 3 {
                panic!(
                    "On function {f_name}: Expected 3 arguments for replace, found {}",
                    args.len()
                );
            }
            let t = typecheck_expr(scope, f_name, args.first().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            let t = typecheck_expr(scope, f_name, args.get(1).unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            let t = typecheck_expr(scope, f_name, args.last().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            Type::String
        }

        Expression::Call(name, args) if name == "split" => {
            if args.len() != 2 {
                panic!(
                    "On function {f_name}: Expected 2 arguments for split, found {}",
                    args.len()
                );
            }
            let t = typecheck_expr(scope, f_name, args.first().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            let t = typecheck_expr(scope, f_name, args.last().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            Type::String // technically it's a list of strings, but we don't have that yet
        }

        Expression::Call(name, args) if name == "openfile" => {
            if args.len() != 1 {
                panic!(
                    "On function {f_name}: Expected 1 argument for openfile, found {}",
                    args.len()
                );
            }
            let t = typecheck_expr(scope, f_name, args.first().unwrap());
            if t != Type::String {
                panic!("On function {f_name}: Expected String, found {t:?}");
            }
            Type::String // technically it's a list of strings, but we don't have that yet
        }
        //end of string library
        Expression::Call(name, args) => {
            match scope.get(name).cloned() {
                Some(Type::Function(args_t, ret)) => {
                    if args.len() != args_t.len() {
                        panic!(
                            "On function {f_name}: Expected {} arguments, but found {}",
                            args_t.len(),
                            args.len()
                        )
                    }
                    // let args_t = args_t.clone();
                    let mut will_crash = false;
                    for (expected, found) in args_t
                        .into_iter()
                        .zip(args.iter().map(|x| typecheck_expr(scope, f_name, x)))
                    {
                        if expected != found {
                            will_crash = true;
                            eprintln!("On function {f_name}: In arguments for call to {name} expected {expected:?}, but found {found:?}")
                        }
                    }
                    if will_crash {
                        panic!("On function {f_name}: Error on function arguments")
                    }
                    *ret
                }
                Some(_) => panic!("{name} is not callable"),
                //None => Type::Void,
                None => panic!("On function {f_name}: function {name} is not defined"),
            }
        }
        Expression::Operation(Operator::Lt, a, b) => {
            let a_type = typecheck_expr(scope, f_name, a.as_ref());
            let b_type = typecheck_expr(scope, f_name, b.as_ref());
            if a_type != Type::Number {
                panic!("On function {f_name}: LHS of < is not a number, it is a {a_type:?}");
            }

            if b_type != Type::Number {
                panic!("On function {f_name}: RHS of < is not a number, it is a {a_type:?}");
            }
            Type::Bool
        }
        Expression::Operation(Operator::Add, a, b) => {
            let a_type = typecheck_expr(scope, f_name, a.as_ref());
            let b_type = typecheck_expr(scope, f_name, b.as_ref());
            if a_type != Type::Number {
                panic!("On function {f_name}: LHS of + is not a number, it is a {a_type:?}");
            }

            if b_type != Type::Number {
                panic!("On function {f_name}: RHS of + is not a number, it is a {a_type:?}");
            }
            Type::Number
        }
        Expression::Operation(Operator::Sub, a, b) => {
            let a_type = typecheck_expr(scope, f_name, a.as_ref());
            let b_type = typecheck_expr(scope, f_name, b.as_ref());
            if a_type != Type::Number {
                panic!("On function {f_name}: LHS of - is not a number, it is a {a_type:?}");
            }

            if b_type != Type::Number {
                panic!("On function {f_name}: RHS of - is not a number, it is a {a_type:?}");
            }
            Type::Number
        }
        Expression::Operation(Operator::Eq, a, b) => {
            let a_type = typecheck_expr(scope, f_name, a.as_ref());
            let b_type = typecheck_expr(scope, f_name, b.as_ref());
            if a_type != b_type {
                panic!("On function {f_name}: LHS of == is not the same type as RHS, it is a {a_type:?} and {b_type:?}");
            }
            Type::Bool
        }
        Expression::Operation(Operator::GEt, a, b) => {
            let a_type = typecheck_expr(scope, f_name, a.as_ref());
            let b_type = typecheck_expr(scope, f_name, b.as_ref());
            if a_type != Type::Number {
                panic!("On function {f_name}: LHS of >= is not a number, it is a {a_type:?}");
            }

            if b_type != Type::Number {
                panic!("On function {f_name}: RHS of >= is not a number, it is a {a_type:?}");
            }
            Type::Bool
        }
        Expression::Operation(Operator::Mod, a, b) => {
            let a_type = typecheck_expr(scope, f_name, a.as_ref());
            let b_type = typecheck_expr(scope, f_name, b.as_ref());
            if a_type != Type::Number {
                panic!("On function {f_name}: LHS of % is not a number, it is a {a_type:?}");
            }

            if b_type != Type::Number {
                panic!("On function {f_name}: RHS of % is not a number, it is a {a_type:?}");
            }
            Type::Number
        }
        Expression::StringLit(_) => Type::String,
        Expression::NumLit(_) => Type::Number,
        Expression::Variable(name) => scope
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("On function {f_name}: Variable {name} not declared")),
        Expression::BoolLit(_) => Type::Bool,
    }
}
