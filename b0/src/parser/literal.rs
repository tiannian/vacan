use pest::iterators::Pair;

use crate::objects::{FloatLiteral, HexLiteral, IntegerLiteral, Literal, StringLiteral};

use super::Rule;

pub fn parse_literal(pest_literal: Pair<'_, Rule>) -> Literal {
    let mut inner = pest_literal.into_inner();

    let next = inner.next().expect("Must have literal");

    match next.as_rule() {
        Rule::String => Literal::String(StringLiteral {
            value: next.as_str().to_string(),
        }),
        Rule::Integer => Literal::Integer(IntegerLiteral {
            value: next.as_str().to_string(),
        }),
        Rule::Float => Literal::Float(FloatLiteral {
            value: next.as_str().to_string(),
        }),
        Rule::Hex => Literal::Hex(HexLiteral {
            value: next.as_str().to_string(),
        }),
        _ => {
            panic!("Unknown rule: {:?}", next.as_rule());
        }
    }
}
