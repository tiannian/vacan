use super::Literal;

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    FunctionCall(FunctionCall),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}
