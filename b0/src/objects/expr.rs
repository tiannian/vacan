use super::Literal;

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    FunctionCall(FunctionCall),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}
