use pest::iterators::Pair;

use super::Rule;

pub fn parse_ident(pest_ident: Pair<'_, Rule>) -> String {
    if let Rule::Ident = pest_ident.as_rule() {
        pest_ident.as_str().to_string()
    } else {
        panic!("Unknown rule: {:?}", pest_ident.as_rule());
    }
}
