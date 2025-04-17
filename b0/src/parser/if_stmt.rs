use pest::iterators::Pair;

use crate::{objects::IfSubStmt, parser::statement::parse_statement};

use super::{Rule, statement::parse_expr};

pub fn parse_if_stmt(pest_if_stmt: Pair<'_, Rule>) -> Vec<IfSubStmt> {
    let mut if_sub_stmts = Vec::new();

    if let Rule::IfStmt = pest_if_stmt.as_rule() {
        let inner = pest_if_stmt.into_inner();

        for part in inner {
            if let Rule::IfSubStmt = part.as_rule() {
                let mut inner = part.into_inner();

                let mut sub_stmt = IfSubStmt::default();

                let condition = inner.next().expect("Must have condition");
                sub_stmt.condition = parse_expr(condition);

                let block = inner.next().expect("Must have block");
                for part in block.into_inner() {
                    if let Rule::Statement = part.as_rule() {
                        let stmt = part.into_inner().next().expect("Must have stmt");
                        sub_stmt.body.push(parse_statement(stmt));
                    } else {
                        panic!("Unknown rule: {:?}", part.as_rule());
                    }
                }

                if_sub_stmts.push(sub_stmt);
            } else {
                panic!("Unknown rule: {:?}", part.as_rule());
            }
        }
    } else {
        panic!("Unknown rule: {:?}", pest_if_stmt.as_rule());
    }

    if_sub_stmts
}
