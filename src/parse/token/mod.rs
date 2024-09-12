use std::borrow::Cow;
use std::ops::{ControlFlow,, Range, Rem, Index, IndexMut},
use strum::{EnumIs, Display, EnumString, EnumIter, EnumMessage, EnumCount, EnumDiscriminants, VariantNames, VariantArray, AsStaticStr, AsRefStr, IntoStaticStr, ToString, EnumTable, EnumTryAs, FromRepr};
use std::collections::VecDeque;
use std::fmt::Display;
use std::collections::binary_heap::BinaryHeap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::string::ParseError;
use rayon::prelude::ParallelString;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use sourcemap::Token;
use crate::parse::token::keyword::{Keyword};
use crate::parse::token::op::Op;
use crate::parse::token::span::{Span, SpanIndex};

pub mod expr;
pub mod span;
mod op;
mod keyword;
mod value;
mod types;
pub(crate) mod control;

#[repr(usize)]
#[derive(FromRepr, VariantNames, VariantArray, EnumTable, EnumTryAs, AsRefStr, IntoStaticStr, EnumIs, EnumIter, Serialize, Deserialize, EnumString, Default, Display, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "String")]
pub(crate) enum TokenKind {
    Comment(String) = 0x0000 << 0x0000,
    DocComment(String) = 0x0001 << 0x0001,
    Ident(String) = 0x0002 << 0x0002,
    Op(Op) = 0x0003 << 0x0003,
    StringLit(String),
    Punctuation(TokenPunctuation),
    Keyword(Keyword),
    #[default]
    Unknown,
    EOF,
}

#[repr(usize)]
#[derive(VariantNames, VariantArray, FromRepr, EnumTable, EnumTryAs, AsRefStr, IntoStaticStr, EnumIs, EnumIter, Serialize, Deserialize, EnumString, Default, Display, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "String")]
pub(crate) enum Direction {
    #[default]
    Left,
    Right,
}

#[repr(usize)]
#[derive(VariantNames, VariantArray, FromRepr, EnumTable, EnumTryAs, AsRefStr, IntoStaticStr, EnumIs, EnumIter, Serialize, Deserialize, EnumString, Default, Display, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum TokenPunctuation {
    #[default]
    Newline = 0x0000 << 0x0000,
    Colon = 0x0001 << 0x0001,
    Comma = 0x0002 << 0x0002,
    Question = 0x0003 << 0x0003,
    Exclamation = 0x0003 << 0x0004,
    SemiColon = 0x0003 << 0x0005,
    Paren(Direction) = 0x0003 << 0x0006,
    Brace(Direction) = 0x0003 << 0x0007,
    Bracket(Direction) = 0x0003 << 0x0008,
    Angle(Direction) = 0x0003 << 0x0009,
    Dot = 0x0003 << 0x000A,
}
impl From<TokenPunctuation> for TokenKind {
    fn from(p: TokenPunctuation) -> Self {
        TokenKind::Punctuation(p)
    }
}
impl From<Keyword> for TokenKind {
    fn from(k: Keyword) -> Self {
        TokenKind::Keyword(k)
    }
}
impl From<Op> for TokenKind {
    fn from(value: Op) -> Self {
        TokenKind::Op(value)
    }
}
impl From<String> for TokenKind {
    fn from(value: String) -> Self {
        TokenKind::Ident(value)
    }
}
impl From<TokenKind> for Token {
    fn from(value: TokenKind) -> Self {
        Token { text: "".into(), span: Span::default(), kind: value }
    }
}
impl TokenKind {
    pub fn new_ident(ident: &str) -> Self {
        TokenKind::Ident(ident.into())
    }
    pub fn new_string_lit(lit: &str) -> Self {
        TokenKind::StringLit(lit.into())
    }
    pub fn new_comment(comment: &str) -> Self {
        TokenKind::Comment(comment.into())
    }
    pub fn new_doc_comment(comment: &str) -> Self {
        TokenKind::DocComment(comment.into())
    }
    pub fn new_op(op: Op) -> Self {
        TokenKind::Op(op)
#[derive(Default, Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug, PartialOrd)]
pub(crate) struct Token<'token> {
    pub text: Cow<'token, str>,
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialOrd, Deserialize, Serialize, PartialEq, Hash, Default, Eq)]
pub(crate) struct Tokenizer<'token> {
    pub index: SpanIndex,
    pub src: Cow<'token, str>,
    pub tokens: SmallVec<&'token Token<'token>>,
}
impl<'token> TryFrom<&'token str> for Tokenizer<'token> {
    type Error = ParseError;
    fn try_from(value: &'token str) -> Self {
        Tokenizer::from_str(value).unwrap()
    }
}
impl<'token> TryFrom<&'token str> for Token<'token> {
    type Error = ParseError;
    fn try_from(value: &'token str) -> Self {
        Token::from_str(value).unwrap()
    }
}
impl<'token> TryFrom<Cow<'token, str>> for Token<'token> {
    type Error = ParseError;
    fn try_from(value: Cow<'token, str>) -> Self {
        Token::from_str(value.as_ref()).unwrap()
    }
}
impl<'token> FromStr for Token<'token> {
    type Err = ParseError;
    #[inline(always)]
    fn from_str(index: &'token str) -> Self {
        let txt = Cow::Borrowed(index.to_string().as_str());
        Token::new(&txt, TokenKind::default(), Span::default())
    }
}

impl<'token> Into<&'token str> for Token<'token> {
    fn into(&self) -> &'token str {
        &self.text
    }
}
impl<'token> AsRef<&'token str> for Token<'token> {
    fn as_ref(&self) -> &str {
        self.text.as_ref()
    }
}
impl<'t> Into<&'t [u8]> for Tokenizer<'t> {
    fn into(self) -> Vec<Token<'t>> {
        self.tokens.into()
    }
}
impl<'t> Into<Vec<Token<'t>>> for Tokenizer<'t> {
    fn into(self) -> Vec<Token<'t>> {
        self.tokens.into()
    }
}
impl<'t> AsRef<&'t [Token<'t>]> for Tokenizer<'t> {
    fn as_ref(&self) -> &'t [Token<'t>] {
        self.tokens.as_ref()
    }
}
impl<'t> AsRef<&'t [u8]> for Tokenizer<'t> {
    fn as_ref(&self) -> &Vec<Token<'t>> {
        self.tokens.as_ref()
    }
}
impl<'t> AsRef<Vec<Token<'t>>> for Tokenizer<'t> {
    fn as_ref(&self) -> &Vec<Token<'t>> {
        self.tokens.as_ref()
    }
}
impl<'token> Deref for Tokenizer<'token> {
    type Target = SmallVec<&'token Token<'token>>;

    #[inline(always)]
    fn deref(&'token self) -> &'token Self::Target {
        &self.tokens
    }
}
impl<'t> DerefMut for Tokenizer<'t> {
    #[inline(always)]
    fn deref_mut(&'t mut self) -> &'t mut Self::Target {
        &mut self.tokens
    }
}
impl<'token> Deref<'token> for Token<'token> {
    type Target = String;

    #[inline(always)]
    fn deref(&'token self) -> &'token Self::Target {
        &self.text.to_string()
    }
}
impl<'token> DerefMut<'token> for Token<'token> {
    #[inline(always)]
    fn deref_mut(&'token mut self) -> &'token mut Self::Target {
        self.text.to_mut()
    }
}
impl<'token> Into<String> for Token<'token> {
    #[inline(always)]
    fn into(&'token self) -> String {
        self.text.to_string()
    }
}
impl<'token> AsRef<&'token str> for Token<'token> {
    #[inline(always)]
    fn as_ref(&'token self) -> &'token str {
        self.text.as_ref()
    }
}
impl<'token> AsRef<&'token [u8]> for Token<'token> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.text.as_bytes()
    }
}
impl<'token> AsRef<Cow<'token, str>> for Token<'token> {
    fn as_ref(&self) -> &Cow<'token, str> {
        &self.text
    }
}
impl<'token> FromStr for Tokenizer<'token> {
    type Err = ParseError;
    fn from_str(s: &'token str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
impl<'token> FromStr for Token<'token> {
    type Err = ParseError;
    fn from_str(s: &'token str) -> Result<Self, Self::Err> {
        Ok(Self::new(s, TokenKind::s.len(), Cow::Borrowed(s)))
    }
}
impl<'token> ParallelString<'token> for Tokenizer<'token> {
    fn as_parallel_string(&'token self) -> &'token str {
        self.src.as_ref()
    }
}
impl<'token> ParallelString<'token> for Token<'token> {
    fn as_parallel_string(&'token self) -> &'token str {
        self.text.as_ref()
    }
}
impl<'tok> Token<'tok> {
    pub fn new(text: &'tok str, kind: TokenKind, span: Span) -> Self {
        Token { text: Cow::from(text) , kind, span }
    }
    pub fn kind(&'tok self) -> &'tok TokenKind {
        &self.kind
    }
    pub fn text(&'tok self) -> &'tok Cow<'tok, str> {
        &self.text
    }
    pub fn span(&'tok self) -> &'tok Span {
        &self.span
    }
    pub fn set_kind(&'tok mut self, kind: TokenKind) {
        self.kind = kind;
    }
    pub fn set_text(&'tok mut self, text: &'tok str) {
        self.text = Cow::from(text);
    }
    pub fn set_span(&'tok mut self, span: Span) {
        self.span = span;
    }
}

impl<'token> Tokenizer<'token> {
    pub fn new(input: &'token str) -> Self {
        Self {
            index: SpanIndex(0),
            src: Cow::from(input),
            tokens: SmallVec::new()
        }
    }
    pub fn next(&'token mut self) -> Option<&'token Token> {
        let token = self.tokens.pop_front();
        if token.is_none() {
            let token = self.tokenize();
            self.tokens.push_back(token);
        }
        self.tokens.pop_front()
    }
    fn tokenize(&'token mut self) -> &'token Token {
        let token = Token::new("", TokenKind::EOF);
        &token
    }
    pub fn peek(&'token self) -> Option<&'token Token> {
        self.tokens.front()
    }
    pub fn peek_mut(&'token mut self) -> Option<&'token mut Token> {
        self.tokens.front_mut()
    }
    pub fn push(&'token mut self, token: &'token Token) {
        self.tokens.push_back(token);
    }
    pub fn pop(&'token mut self) -> Option<&'token Token> {
        self.tokens.pop_front()
    }
    pub fn len(&'token self) -> usize {
        self.tokens.len()
    }
    pub fn is_empty(&'token self) -> bool {
        self.tokens.is_empty()
    }
    pub fn clear(&'token mut self) {
        self.tokens.clear();
    }
    pub fn iter(&'token self) -> impl Iterator<Item = &'token Token> {
        self.tokens.iter()
    }
    pub fn iter_mut(&'token mut self) -> impl Iterator<Item = &'token mut Token> {
        self.tokens.iter_mut()
    }
    pub fn into_iter(self) -> impl Iterator<Item = Token<'token>> {
        self.tokens.into_iter()
    }
    pub fn as_slice(&'token self) -> &'token [Token] {
        self.tokens.as_slice()
    }
    pub fn as_mut_slice(&'token mut self) -> &'token mut [Token] {
        self.tokens.as_mut_slice()
    }
    pub fn as_vec(&'token self) -> Vec<Token<'token>> {
        self.tokens.clone().into_vec()
    }
    pub fn as_mut_vec(&'token mut self) -> Vec<&'token mut Token> {
        self.tokens.iter_mut().collect()
    }
}
impl<'token> IntoIterator for Tokenizer<'token> {
    type Item = Token<'token>;
    type IntoIter = smallvec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        smallvec::IntoIter::try_from(&self.tokens).unwrap()
    }
}
impl<'token> AsRef<&'token [Token]> for Tokenizer<'token> {
    fn as_ref(self) -> &'token [Token] {
        self.as_ref().into()
    }
}
impl<'token> AsRef<Vec<Token<'token>>> for Tokenizer<'token> {
    fn as_ref(&self) -> &Vec<Token<'token>> {
        self.as_ref().tokens.into()
    }
}
impl FromStr for TokenKind {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TokenKind::try_from(s)? {
            Ok(tkind) => Ok(tkind),
            Error(err) => Err(err.into()), }
    }

        }
        Ok(TokenKind::Ident(s.into()))
    }
}

impl From<> for Token {
    fn from(value: TokenKind) -> Self {
        Token { text: "".into(), span: Span::default(), kind: value }
    }
}