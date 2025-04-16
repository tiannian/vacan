use super::{Expr, Ident};

pub struct VariableDecl {
    pub name: String,
    pub ty: Ident,
    pub value: Expr,
}

pub struct VariableAssign {
    pub name: String,
    pub value: Expr,
}
