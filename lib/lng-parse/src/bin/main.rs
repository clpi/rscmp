use std::result;
use pest_derive::Parser;
use lng_parse::parse::grammar::{Grammar, Rule};
use pest::{Parser, ParseResult};
use pest::iterators::Pair;
use pest_meta::parser::parse;

fn main() -> pest::ParseResult<()> {
    println!("Starting parse");
    let pairs = Grammar::parse(
        Rule::calculation,
        r#"1 + 2"#
    )
        .unwrap();
    for pair in pairs {
        println!("{:?}", pair);
    }
    Ok(())
}
