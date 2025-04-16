use core::panic;

use pest::iterators::Pair;

use crate::objects::{Decorator, FunctionArg, FunctionDecl, FunctionHeader};

use super::{Rule, idnet::parse_ident, literal::parse_literal};

pub fn parse_function_decl(pest_function_decl: Pair<'_, Rule>) -> Option<FunctionDecl> {
    if let Rule::FuncDecl = pest_function_decl.as_rule() {
        let mut decorator = None;
        let mut header = FunctionHeader::default();
        let mut body = vec![];

        for pest_function_decl_child in pest_function_decl.into_inner() {
            match pest_function_decl_child.as_rule() {
                Rule::Decorator => {
                    decorator = Some(parse_decorator(pest_function_decl_child));
                }
                Rule::FuncHeader => {
                    header = parse_function_header(pest_function_decl_child);
                }
                Rule::Block => {
                    // TODO: parse block
                    // body = parse_function_body(pest_function_decl_child);
                }
                _ => {
                    panic!("Unknown rule: {:?}", pest_function_decl_child.as_rule());
                }
            }
        }

        let function_decl = FunctionDecl {
            decorator,
            header,
            body,
        };

        Some(function_decl)
    } else {
        None
    }
}

fn parse_decorator(pest_decorator: Pair<'_, Rule>) -> Decorator {
    let mut decorator = Decorator::default();

    for part in pest_decorator.into_inner() {
        match part.as_rule() {
            Rule::Ident => {
                decorator.name = parse_ident(part);
            }
            Rule::Literal => {
                decorator.args.push(parse_literal(part));
            }
            _ => {
                panic!("Unknown rule: {:?} in parse decorator", part.as_rule());
            }
        }
    }

    decorator
}

fn parse_function_header(pest_function_header: Pair<'_, Rule>) -> FunctionHeader {
    let mut header = FunctionHeader::default();

    let mut pest_function_header_inner = pest_function_header.into_inner();

    let pest_function_name = pest_function_header_inner
        .next()
        .expect("Must have function name");
    header.name = parse_ident(pest_function_name);

    for part in pest_function_header_inner {
        match part.as_rule() {
            Rule::FuncArg => {
                let mut pest_inner = part.into_inner();
                let pest_arg_name = pest_inner.next().expect("Must have arg name");
                let pest_arg_type = pest_inner.next().expect("Must have arg type");

                header.args.push(FunctionArg {
                    name: parse_ident(pest_arg_name),
                    ty: parse_ident(pest_arg_type),
                });
            }
            Rule::Ident => {
                header.return_type = Some(parse_ident(part));
            }
            _ => {
                panic!("Unknown rule: {:?}", part.as_rule());
            }
        }
    }

    header
}
