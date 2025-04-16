use anyhow::Result;
use pest_derive::Parser;

use crate::objects::{FunctionDecl, FunctionHeader};

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct VacanPestParser;

pub struct VacanParser;

impl VacanParser {
    pub fn parse(&self, code: &str) -> Result<Vec<FunctionDecl>> {
        use pest::Parser;
        let mut pairs = VacanPestParser::parse(Rule::Program, code)?;

        let function_decls = pairs
            .next()
            .ok_or(anyhow::anyhow!("No function declaration found"))?;

        let function_decls = function_decls.into_inner();

        let mut functions = vec![];

        for pest_function_decl in function_decls {
            println!("{:#?}", pest_function_decl);

            let decl = FunctionDecl {
                decorator: None,
                header: FunctionHeader {
                    name: "".into(),
                    args: vec![],
                    return_type: None,
                },
                body: vec![],
            };

            functions.push(decl);
        }

        Ok(functions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let code = include_str!("../../testdata/simple.va");
        let parser = VacanParser;
        let functions = parser.parse(code).unwrap();

        println!("{:#?}", functions);
        // assert_eq!(functions.len(), 1);
    }
}
