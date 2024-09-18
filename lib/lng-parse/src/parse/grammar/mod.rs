extern crate pest_derive;
use pest::error::Error;
use pest::{iterators::Pairs, RuleType, state, ParseResult, ParserState, Span, Position, Parser};
use pest_derive::Parser;
use std::borrow::Cow;
// use pest_vm::Vm;
// use pest_meta::parser::{parse, ParserExpr};
use crossbeam::epoch::Pointable;
// use pest::iterators::Pairs;
use strum::Display;
use tracing::error;
use tracing_subscriber::filter::LevelParseError;
use crate::parse::compile::Compiler;
use crate::parse::state;
use crate::parse::token::Tokenizer;

#[derive(Parser, Debug)]
#[grammar = "src/parse/grammar/grammar.pest"]
pub struct Grammar<'g> {
    pub src: &'g str,   
}
impl<'g> Default for Grammar<'g> {
    fn default() -> Self {
        Self {
            src: "",
        }
    }
}

impl<'g> Grammar<'g> {

    #[inline(always)]
    pub const fn new(src: &'g str) -> Self {
        Self { src }
    }

    pub(crate) fn set_src(&mut self, src: &'g str) -> &Self {
        self.src = src;
        self
    }

    pub fn parse_full(&self) -> Result<Pairs<'_, Rule>, Error<Rule>> {
        Self::parse(Rule::calculation, self.src)
    }

    pub fn parse_expr(&self) -> Result<Pairs<'_, Rule>, Error<Rule>> {
        Self::parse(Rule::expr, self.src)
    }
}
