use super::{Ident, Literal, Statement};

pub struct FunctionDecl {
    pub decorator: Option<Decorator>,
    pub header: FunctionHeader,
    pub body: Vec<Statement>,
}

pub struct FunctionHeader {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub return_type: Option<Ident>,
}

pub struct FunctionArg {
    pub name: String,
    pub ty: Ident,
}

pub struct Decorator {
    pub name: String,
    pub args: Vec<Literal>,
}
