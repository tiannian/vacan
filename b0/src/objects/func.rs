use super::{Literal, Statement};

#[derive(Debug, Clone, Default)]
pub struct FunctionDecl {
    pub decorator: Option<Decorator>,
    pub header: FunctionHeader,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionHeader {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, Default)]
pub struct Decorator {
    pub name: String,
    pub args: Vec<Literal>,
}
