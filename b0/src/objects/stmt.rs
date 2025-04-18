use super::{Expr, VariableAssign, VariableDecl};

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDecl(VariableDecl),
    VariableAssign(VariableAssign),
    Expr(Expr),
    If(Vec<IfSubStmt>),
    ForLoop(ForLoopStmt),
    Return(Option<Expr>),
    Macros(MacrosStmt),
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct MacrosStmt {
    pub name: String,
    pub body: String,
}
