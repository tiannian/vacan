use pest::iterators::Pair;

use crate::{
    objects::ForLoopStmt,
    parser::statement::{parse_expr, parse_var_decl},
};

use super::{Rule, statement::parse_statement};

pub fn parse_for_stmt(pest_for_stmt: Pair<'_, Rule>) -> ForLoopStmt {
    let mut for_stmt = ForLoopStmt::default();

    let mut inner = pest_for_stmt.into_inner();

    let mut init = inner.next().expect("Must have init").into_inner();
    if init.len() == 1 {
        for_stmt.init = Some(parse_var_decl(init.next().expect("Must have var decl")));
    }

    let mut cond = inner.next().expect("Must have cond").into_inner();
    if cond.len() == 1 {
        for_stmt.condition = Some(parse_expr(cond.next().expect("Must have expr")));
    }

    let mut step = inner.next().expect("Must have step").into_inner();
    if step.len() == 1 {
        for_stmt.update = Some(parse_expr(step.next().expect("Must have expr")));
    }

    let body = inner.next().expect("Must have body").into_inner();
    for part in body {
        if part.as_rule() == Rule::Block {
            let inner = part.into_inner();
            for part in inner {
                if part.as_rule() == Rule::Statement {
                    let stmt = part.into_inner().next().expect("Must have stmt");
                    let statement = parse_statement(stmt);
                    for_stmt.body.push(statement);
                }
            }
        } else {
            panic!("Unknown rule: {:?}", part.as_rule());
        }
    }

    for_stmt
}
