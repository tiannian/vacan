use super::{Expr, VariableAssign, VariableDecl};

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDecl(VariableDecl),
    VariableAssign(VariableAssign),
    Expr(Expr),
    If(If),
    ForLoop(ForLoopStmt),
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub struct If {
    pub if_sub_stmt: Vec<IfSubStmt>,
}

#[derive(Debug, Clone)]
pub struct IfSubStmt {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Default)]
pub struct ForLoopStmt {
    pub init: Option<VariableDecl>,
    pub condition: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Vec<Statement>,
}
