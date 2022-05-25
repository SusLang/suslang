// Replace with Typ if functions is added as a type

use std::{collections::HashMap, fmt::Debug};

use crate::ast::{Ast, Expression, Operator, Statement, Typ};

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

trait Scope<'a, 'b>
where
    'a: 'b,
{
    fn add(&mut self, s: &'a str, t: Type);

    fn push<'c>(&'b mut self) -> InnerScope<'a, 'c>
    where
        'b: 'c;

    fn get(&self, s: &str) -> Option<&Type>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct GlobalScope<'a> {
    top_level: HashMap<&'a str, Type>,
    data: Vec<HashMap<&'a str, Type>>,
}

impl<'a> GlobalScope<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn pop(&mut self) {
        self.data.pop();
    }
}

impl<'a, 'b> Scope<'a, 'b> for GlobalScope<'a>
where
    'a: 'b,
{
    fn add(&mut self, s: &'a str, t: Type) {
        self.data
            .last_mut()
            .unwrap_or(&mut self.top_level)
            .insert(s, t);
    }

    fn push<'c>(&'b mut self) -> InnerScope<'a, 'c>
    where
        'b: 'c,
    {
        self.data.push(Default::default());
        InnerScope { parent: self }
    }

    fn get(&self, s: &str) -> Option<&Type> {
        self.data
            .iter()
            .rev()
            .chain(std::iter::once(&self.top_level))
            .map(|x| x.get(s))
            .find(|x| x.is_some())
            .flatten()
    }
}

struct InnerScope<'a, 'b>
where
    'a: 'b,
{
    parent: &'b mut GlobalScope<'a>,
}

impl<'a, 'b> Drop for InnerScope<'a, 'b> {
    fn drop(&mut self) {
        self.parent.pop();
    }
}

impl<'a, 'b> Debug for InnerScope<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.parent)
    }
}

impl<'a, 'b> Scope<'a, 'b> for InnerScope<'a, 'b> {
    fn push<'c>(&'c mut self) -> InnerScope<'a, 'c>
    where
        'b: 'c,
    {
        self.parent.push()
    }

    fn add(&mut self, s: &'a str, t: Type) {
        self.parent.add(s, t)
    }

    fn get(&self, s: &str) -> Option<&Type> {
        self.parent.get(s)
    }
}

pub fn typecheck(a: &[Ast]) {
    let mut scopes = GlobalScope::new();
    for a in a {
        match a {
            Ast::Func(name, ret, args, _) => scopes.add(
                name,
                Type::Function(
                    args.iter().map(|(_, t)| Type::from(*t)).collect(),
                    Box::new(Type::from(*ret)),
                ),
            ),
        }
    }

    dbg!(&scopes);

    for a in a {
        match a {
            Ast::Func(name, ret, args, body) => {
                let ret: Type = (*ret).into();
                let f_name = name;
                let mut scope = scopes.push();
                for (name, t) in args {
                    scope.add(name, (*t).into());
                }
                
                for line in body {
                    match line {
                        Statement::If(cond, _body, _else_body) => {
                            // typecheck condition
                            let e_type = typecheck_expr(&mut scope, f_name, cond);
                            if e_type != Type::Bool {
                                panic!("Error on if condition in function {f_name}: Expected Bool but found {e_type:?}")
                            }
                            // TODO Check body and else
                        }
                        Statement::Return(x) => {
                            // Check expression type is the same as ret
                            let e_type = x
                                .as_ref()
                                .map(|x| typecheck_expr(&mut scope, f_name, x))
                                .unwrap_or(Type::Void);
                            if e_type != ret {
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
                // drop scope
            }
        }
    }
}


fn typecheck_expr<'a, 'b, S>(scope: &mut S, f_name: &str, e: &Expression) -> Type
where
    'a: 'b,
    S: Scope<'a, 'b>,
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
                Some(e) => panic!("On function {f_name}: Expected string literal on report, found {e:?}"),
                None => unreachable!(),
            }
            .chars().collect::<Vec<_>>().windows(2)
            {
                if c[0] == '%' {
                    match c[1] {
                        'd' => expected_args.push(Type::Number),
                        's' => expected_args.push(Type::String),
                        '%' => (), // Ignore
                        x => panic!("On function {f_name}: Unexpected char {x:?} after %")
                    }
                }
            }

            let mut will_crash = false;
            if expected_args.len() != args.len()-1 {
                panic!("On function {f_name}: Unexpected number of format arguments on report, expected {}, but found {}", expected_args.len(), args.len()-1);
            }
            for (found, expected) in args.iter().skip(1).map(|x| typecheck_expr(scope, f_name, x)).zip(expected_args) {
                if expected != found {
                    will_crash = true;
                    eprintln!("On function {f_name}: In arguments for call to report expected {expected:?}, but found {found:?}")
                }
            }
            if will_crash {
                panic!("On function {f_name}: Error on report arguments")
            }

            Type::Void
        }
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
        Expression::StringLit(_) => Type::String,
        Expression::NumLit(_) => Type::Number,
        Expression::Variable(name) => scope
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("On function {f_name}: Variable {name} not declared")),
    }
}
