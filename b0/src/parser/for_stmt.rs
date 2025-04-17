use pest::iterators::Pair;

use crate::objects::ForLoopStmt;

use super::Rule;

pub fn parse_for_stmt(pest_for_stmt: Pair<'_, Rule>) -> ForLoopStmt {
    println!("for stmt: {:#?}", pest_for_stmt);

    let mut for_stmt = ForLoopStmt::default();

    for_stmt
}
