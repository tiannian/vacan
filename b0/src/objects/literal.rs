#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct HexLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(StringLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Hex(HexLiteral),
}
