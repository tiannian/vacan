use super::Literal;

pub enum Expr {
    Ident(Ident),
    FunctionCall(FunctionCall),
    Literal(Literal),
}

pub struct Ident {
    pub name: String,
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}
