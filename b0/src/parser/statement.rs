use pest::iterators::Pair;

use crate::objects::{Expr, FunctionCall, MacrosStmt, Statement, VariableAssign, VariableDecl};

use super::{Rule, for_stmt::parse_for_stmt, if_stmt::parse_if_stmt, literal::parse_literal};

pub fn parse_statement(pest_statement: Pair<'_, Rule>) -> Statement {
    match pest_statement.as_rule() {
        Rule::FuncCallStmt => Statement::Expr(parse_expr(
            pest_statement
                .into_inner()
                .next()
                .expect("Must have func call stmt"),
        )),
        Rule::ReturnStmt => {
            let mut inner = pest_statement.into_inner();

            if inner.len() == 0 {
                Statement::Return(None)
            } else {
                Statement::Return(Some(parse_expr(inner.next().expect("Must have expr"))))
            }
        }
        Rule::VarDecl => Statement::VariableDecl(parse_var_decl(pest_statement)),
        Rule::VarAssign => Statement::VariableAssign(parse_var_assign(pest_statement)),
        Rule::ForLoopStmt => Statement::ForLoop(parse_for_stmt(pest_statement)),
        Rule::IfStmt => Statement::If(parse_if_stmt(pest_statement)),
        Rule::MacrosStmt => Statement::Macros(parse_macros_stmt(pest_statement)),
        _ => panic!("Unknown rule: {:?}", pest_statement.as_rule()),
    }
}

pub fn parse_expr(pest_expr: Pair<'_, Rule>) -> Expr {
    if let Rule::Expr = pest_expr.as_rule() {
        let inner = pest_expr.into_inner().next().expect("Must have expr");

        match inner.as_rule() {
            Rule::FuncCall => {
                let mut function_call = FunctionCall::default();
                let mut inner = inner.into_inner();

                let name = inner.next().expect("Must have name");
                function_call.name = name.as_str().to_string();

                for part in inner {
                    function_call.args.push(parse_expr(part));
                }

                Expr::FunctionCall(function_call)
            }
            Rule::Ident => Expr::Ident(inner.as_str().to_string()),
            Rule::Literal => Expr::Literal(parse_literal(inner)),
            _ => {
                panic!("Unknown rule: {:?}", inner.as_rule());
            }
        }
    } else {
        panic!("Unknown rule: {:?}", pest_expr.as_rule());
    }
}

pub fn parse_var_decl(pest_var_decl: Pair<'_, Rule>) -> VariableDecl {
    let mut var_decl = VariableDecl::default();

    let mut inner = pest_var_decl.into_inner();

    let name = inner.next().expect("Must have name");
    var_decl.name = name.as_str().to_string();

    let len = inner.len();

    if len == 1 {
        let value = inner.next().expect("Must have value");
        var_decl.value = parse_expr(value);
    } else if len == 2 {
        let ty = inner.next().expect("Must have type");
        var_decl.ty = Some(ty.as_str().to_string());

        let value = inner.next().expect("Must have value");
        var_decl.value = parse_expr(value);
    } else {
        panic!("Wrong number of children for var decl: {}", len);
    }

    var_decl
}

fn parse_var_assign(pest_var_assign: Pair<'_, Rule>) -> VariableAssign {
    let mut var_assign = VariableAssign::default();

    let mut inner = pest_var_assign.into_inner();

    let name = inner.next().expect("Must have name");
    var_assign.name = name.as_str().to_string();

    let value = inner.next().expect("Must have value");
    var_assign.value = parse_expr(value);

    var_assign
}

fn parse_macros_stmt(pest_macros_stmt: Pair<'_, Rule>) -> MacrosStmt {
    let mut macros_stmt = MacrosStmt::default();

    let mut inner = pest_macros_stmt.into_inner();

    let name = inner.next().expect("Must have name");
    macros_stmt.name = name.as_str().to_string();

    let body = inner.next().expect("Must have body");
    macros_stmt.body = body.as_str().to_string();

    macros_stmt
}
