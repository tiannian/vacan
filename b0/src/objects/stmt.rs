use super::{Expr, FunctionCall, VariableAssign, VariableDecl};

pub enum Statement {
    VariableDecl(VariableDecl),
    VariableAssign(VariableAssign),
    FunctionCall(FunctionCall),
    If(If),
    ForLoop(ForLoopStmt),
    Return(Option<Expr>),
}

pub struct If {
    pub if_sub_stmt: Vec<IfSubStmt>,
}

pub struct IfSubStmt {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

pub struct ForLoopStmt {
    pub init: Option<VariableDecl>,
    pub condition: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Vec<Statement>,
}
