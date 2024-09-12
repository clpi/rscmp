extern crate pest;
use pest::{state, ParseResult, Parser, ParserState,  Span};
use pest_derive::Parser;
use pest_vm::Vm;
use pest_meta::parser::ParserExpr;
use std::{
    error::Error,
};
use strum::Display;
use crate::parse::compile::Compiler;
use crate::parse::token::Tokenizer;

#[derive(Parser)]
#[grammar = "parse/grammar/grammar.pest"]
pub(crate) struct Grammar<'grammar> {
    pub(crate) state: pest::Position<'grammar>,
    pub(crate) rules: Vec<pest::Token<'grammar, String>>,
}

impl<'grammar> Grammar<'grammar> {

}
