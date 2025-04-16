use super::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub ty: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct VariableAssign {
    pub name: String,
    pub value: Expr,
}
