extern crate pest_derive;
use pest::{iterators::Pairs, RuleType, state, ParseResult, ParserState, Span, Position, Parser};
use pest_derive::Parser;
// use pest_vm::Vm;
// use pest_meta::parser::{parse, ParserExpr};
use std::{
    error::Error,
};
use crossbeam::epoch::Pointable;
// use pest::iterators::Pairs;
use strum::Display;
use tracing::error;
use tracing_subscriber::filter::LevelParseError;
use crate::parse::compile::Compiler;
use crate::parse::state;
use crate::parse::token::Tokenizer;

#[derive(Parser)]
#[grammar = "src/parse/grammar/grammar.pest"]
pub struct Grammar {
}

impl Grammar {
    pub fn new() -> Self {
        let s = Self {
        };
        s
    }

    pub fn parse_rule(rule: Rule, src: &'static str) -> ParseResult<Pairs<Rule>> {
        let mut pairs = <Self as Parser<Rule>>::parse(rule, src);
        let mut ps = pairs.unwrap();
        while let Some(pair) = &mut ps.next() {
            println!("{:#?} {:?}", pair.as_rule(), pair.as_span());
        }
        Ok(ps.clone())
    }
    pub fn parse_expr(src: &str) -> ParseResult<Pairs<Rule>> {
        let mut pairs = Self::parse(Rule::expr, src);
        let mut ps = pairs.unwrap_or_else(|e| panic!("{}", e));
        while let Some(pair) = &mut ps.next() {
            println!("{:#?} {:?}", pair.as_rule(), pair.as_span());
        }
        Ok(ps.clone())
    }
}
