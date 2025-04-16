use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct VacanParser;

fn main() {
    let code = fs::read_to_string("./b0/testdata/basic.va").unwrap();

    let result = VacanParser::parse(Rule::program, &code);

    let mut pairs = result.unwrap();

    let program = pairs.next().unwrap();

    println!("{:?}", program);
}
