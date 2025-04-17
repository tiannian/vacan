use super::Expr;

#[derive(Debug, Clone, Default)]
pub struct VariableDecl {
    pub name: String,
    pub ty: Option<String>,
    pub value: Expr,
}

#[derive(Debug, Clone, Default)]
pub struct VariableAssign {
    pub name: String,
    pub value: Expr,
}
