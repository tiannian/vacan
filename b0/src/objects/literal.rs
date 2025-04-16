pub struct StringLiteral {
    pub value: String,
}

pub struct IntegerLiteral {
    pub value: String,
}

pub struct FloatLiteral {
    pub value: String,
}

pub struct HexLiteral {
    pub value: String,
}

pub enum Literal {
    String(StringLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Hex(HexLiteral),
}
