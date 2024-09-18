use std::result;
use pest_derive::Parser;
use lng::prelude::parse::grammar::{Grammar, Rule};
use pest::{Parser, ParseResult};
use pest::iterators::Pair;
use pest_meta::parser::parse;

fn main() -> pest::ParseResult<()> {
    println!("Hello, world!");
    let pairs = Grammar::parse_expr(r#"1 + 2 * (9 / 3)"#)
        .unwrap();
    for pair in pairs {
        println!("{:?}", pair);
    }
    Ok(())
}
