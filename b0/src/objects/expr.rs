use super::Literal;

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    FunctionCall(FunctionCall),
    Literal(Literal),
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Ident(String::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}
