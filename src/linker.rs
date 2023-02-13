use std::collections::HashMap;

use crate::{
    ast::{
        parse::spans::{MapExt, Span},
        Ast, Block, Expression,
    },
    module::{Module, ModuleUsePath},
};

fn get_items_needed(module_graph: &Module) -> Vec<ModuleUsePath> {
    // FIXME update so that it only lists things used un main
    // FIXME Imports realtive to root
    let mut modules_loaded = Vec::with_capacity(1);
    let mut modules_needed = vec![module_graph];
    let mut items_needed = Vec::with_capacity(1);
    while let Some(module) = modules_needed.pop() {
        for item in &module.items {
            match &item.extra.data {
                Ast::Func(name, _, _, _) => {
                    let mut path = module.path.clone();
                    items_needed.push(
                        name.clone()
                            .map(|name| {
                                path.push(name);
                                path
                            })
                            .extra
                            .data,
                    );
                }
                Ast::Mod(_) => (),
                Ast::Import(path) => {
                    let module_path = &path.extra.data[..(path.extra.data.len() - 1)];
                    if !modules_loaded.contains(&module_path) {
                        modules_loaded.push(module_path);
                        modules_needed.push(module_graph.get_module(module_path).unwrap())
                    }
                }
            }
        }
    }
    items_needed
}

pub fn get_path<'a, 'b>(
    module: &'b Module<'a>,
    path: ModuleUsePath,
) -> Option<(ModuleUsePath, &'b Module<'a>, &'b Span<'a, Ast<'a>>)> {
    module
        .get_module(&path[..(path.len() - 1)])
        .and_then(|module| {
            module.items
            .iter()
            .find(|item|
                matches!(&item.extra.data, Ast::Func(name, _, _, _) if &name.extra.data == path.last().unwrap())
            ).map(|item| (path, module, item))
        })
}

fn mangle_name(id: usize, path: &ModuleUsePath) -> String {
    if ["ඬ"] != path.as_slice() {
        format!("{}_{id}", path.join("_"))
    } else {
        "ඬ".to_string()
    }
}

fn replace_body<'a>(items_in_scope: &HashMap<String, String>, body: Block<'a>) -> Block<'a> {
    body.into_iter()
        .map(|x| {
            x.map(|x| {
                use crate::ast::Statement::*;
                match x {
                    If(expr, a, b) => If(
                        expr.map(|expr| replace_expression(items_in_scope, expr)),
                        a.map(|body| replace_body(items_in_scope, body)),
                        b.map(|x| x.map(|body| replace_body(items_in_scope, body))),
                    ),
                    While(expr, body) => While(
                        expr.map(|expr| replace_expression(items_in_scope, expr)),
                        body.map(|body| replace_body(items_in_scope, body)),
                    ),
                    Return(expr) => Return(
                        expr.map(|expr| expr.map(|expr| replace_expression(items_in_scope, expr))),
                    ),
                    Expr(expr) => Expr(expr.map(|expr| replace_expression(items_in_scope, expr))),
                    Define(name, expr) => Define(
                        name,
                        expr.map(|expr| replace_expression(items_in_scope, expr)),
                    ),
                    x => x,
                }
            })
        })
        .collect()
}

fn replace_expression<'a>(
    items_in_scope: &HashMap<String, String>,
    expr: Expression<'a>,
) -> Expression<'a> {
    use Expression::*;
    match expr {
        Call(name, args) => Call(
            name.map(|name| replace_name(items_in_scope, name)),
            args.into_iter()
                .map(|arg| arg.map(|expr| replace_expression(items_in_scope, expr)))
                .collect(),
        ),
        Operation(op, expr_a, expr_b) => Operation(
            op,
            Box::new(expr_a.map(|expr| replace_expression(items_in_scope, expr))),
            Box::new(expr_b.map(|expr| replace_expression(items_in_scope, expr))),
        ),
        x => x,
    }
}

fn replace_name(items_in_scope: &HashMap<String, String>, name: String) -> String {
    items_in_scope.get(&name).cloned().unwrap_or(name)
}

fn load_items_in_scope<'a: 'c, 'b, 'c>(
    module: &'a Module<'b>,
    cache: &'c mut HashMap<&'a Module<'b>, HashMap<String, String>>,
    items_needed: &[ModuleUsePath],
) -> &'c HashMap<String, String> {
    cache.entry(module).or_insert_with(|| {
        let mut hm = HashMap::new();
        for item in &module.items {
            match &item.extra.data {
                Ast::Func(name, _, _, _) => {
                    let mut path = module.path.clone();
                    path.push(name.extra.data.clone());
                    if let Some(id) = items_needed
                        .iter()
                        .enumerate()
                        .find(|(_, x)| x == &&path)
                        .map(|(i, _)| i)
                    {
                        let name = mangle_name(id, &path);
                        hm.insert(path.last().unwrap().clone(), name);
                    }
                }
                Ast::Mod(_) => (),
                Ast::Import(path) => {
                    if let Some(id) = items_needed
                        .iter()
                        .enumerate()
                        .find(|(_, x)| x == &&path.extra.data)
                        .map(|(i, _)| i)
                    {
                        let name = mangle_name(id, &path.extra.data);
                        hm.insert(path.extra.data.last().unwrap().clone(), name);
                    }
                }
            }
        }
        hm
    })
}

pub fn link<'a>(module_graph: &Module<'a>) -> Vec<Span<'a, Ast<'a>>> {
    let items_needed = get_items_needed(module_graph);
    let mut cache = HashMap::new();
    items_needed
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(i, item)| get_path(module_graph, item).map(|x| (i, x)))
        .map(|(_, (_, module, item))| {
            let items_in_scope = load_items_in_scope(module, &mut cache, &items_needed);
            item.clone().map(|item| match item {
                Ast::Func(a, b, c, d) => Ast::Func(
                    a.map(|name| replace_name(items_in_scope, name)),
                    b,
                    c,
                    d.map(|body| replace_body(items_in_scope, body)),
                ),
                _ => unreachable!(),
            })
        })
        .collect()
}
