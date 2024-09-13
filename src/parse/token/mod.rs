use std::borrow::Cow;
use std::ops::{ControlFlow, Range, Rem, Index, IndexMut};
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
use sourcemap::Token as SourceToken;
use crate::parse::token::keyword::{Keyword};
use crate::parse::token::op::Op;
use crate::parse::token::span::{Span, SpanIndex};
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicPtr, AtomicUsize};
use async_trait::async_trait;
use itertools::Itertools;
use pest_meta::parser::Rule::line_comment;
// use crate::parse::grammar::Rule::value;

pub  mod expr;
pub  mod span;
pub  mod op;
pub  mod keyword;
pub (crate) mod types;
pub(crate) mod control;

#[repr(usize)]
#[derive(FromRepr, VariantNames, EnumTryAs, AsRefStr, IntoStaticStr,  EnumIs, EnumIter, Serialize, Deserialize, EnumString, Default, Display, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
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
#[derive(VariantNames, VariantArray, FromRepr, EnumTable, EnumTryAs, AsRefStr, IntoStaticStr, EnumIs, EnumIter, Serialize,EnumString, Default, Display, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "String")]
pub(crate) enum Direction {
    #[default]
    Left,
    Right,
}

#[repr(usize)]
#[derive(VariantNames,  FromRepr, EnumTryAs, AsRefStr, IntoStaticStr, EnumIs, EnumIter, Serialize, EnumString, Default, Display, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
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
        TokenKind::Op(value.into())
    }
}
impl From<String> for TokenKind {
    fn from(value: String) -> Self {
        TokenKind::Ident(value.into())
    }
}
// impl From<TokenKind> for Token {
//     fn from(value: TokenKind) -> Self {
//         Token { text: "".into(), span: Span::default(), kind: value.into() }
//     }
// }
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
    }
}
#[derive(Default, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
pub(crate) struct Token<'token> {
    pub text: Cow<'token, str>,
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash, Default, Eq)]
pub(crate) struct Tokenizer<'token> {
    pub index: SpanIndex,
    pub src: Cow<'token, str>,
    pub tokens: Vec<&'token Token<'token>>,
}
// impl<'token> TryFrom<&'token str> for Tokenizer<'token> {
//     type Error = ParseError;
//     fn try_from(value: &'token str) -> Result<Tokenizer<'token>, ParseError>{
//         Ok(Tokenizer::from(value))
//     }
// }
// impl<'token> TryFrom<Cow<'token, str>> for Token<'token> {
//     type Error = ParseError;
//     fn try_from(value: Cow<'token, str>) -> Result<Token<'token>, ParseError>{
//         Token::from_str(value.to_string().as_str())
//     }
// }

// impl<'token> Into<&'token str> for Token<'token> {
//     fn into(&'token self) -> &'token str {
//         self.text.as_ref()
//     }
// }
// impl<'token> AsRef<&'token str> for Token<'token> {
//     fn as_ref(&self) -> &str {
//         self.text.as_ref()
//     }
// }
// impl<'t> Into<&'t [&'t Token<'t>]> for Tokenizer<'t> {
//     fn into(self) -> &'t [&'t Token<'t>] {
//         let t = Vec::from(self.tokens.clone());
//         t.as_slice()
//     }
// }
impl<'t> Into<Vec<&'t Token<'t>>> for Tokenizer<'t> {
    fn into(self) -> Vec<&'t Token<'t>> {
        self.tokens.to_vec()
    }
}
// impl<'t> AsRef<&'t [Token<'t>]> for Tokenizer<'t> {
//     fn as_ref(&self) -> &'t [Token<'t>] {
//         self.tokens.to_vec().as_ref()
//     }
// }

// impl<'t> AsRef<Vec<Token<'t>>> for Tokenizer<'t> {
//     fn as_ref(&self) -> &Vec<Token<'t>> {
//         self.tokens
//     }
// }
// impl<'token> Deref for Tokenizer<'token> {
//     type Target = Vec<&'token Token<'token>>;
//
//     #[inline(always)]
//     fn deref(&self) -> &Self::Target {
//         let s = self.clone().tokens;
//     }
// }
// impl<'t> DerefMut for Tokenizer<'t> {
//     #[inline(always)]
//     fn deref_mut(&'t mut self) -> &'t mut Self::Target {
//         &mut self.tokens
//     }
// }
// impl<'token> Deref<'token> for Token<'token> {
//     type Target = String;
//
//     #[inline(always)]
//     fn deref(&self) -> &'token Self::Target {
//         self.clone().text.to_string()
//     }
// }
// impl<'token> DerefMut<'token> for Token<'token> {
//     #[inline(always)]
//     fn deref_mut(&'token mut self) -> &'token mut Self::Target {
//         self.text.to_mut()
//     }
// }
// impl<'token> Into<String> for Token<'token> {
//     #[inline(always)]
//     fn into(&'token self) -> String {
//         self.text.to_string()
//     }
// }
// impl<'token> AsRef<&'token str> for Token<'token> {
//     #[inline(always)]
//     fn as_ref(&'token self) -> &'token str {
//         self.text.as_ref()
//     }
// }
// impl<'token> AsRef<&'token [u8]> for Token<'token> {
//     #[inline(always)]
//     fn as_ref(&self) -> &'token [u8] {
//         &'token self.text.as_bytes()
//     }
// }
impl<'token> AsRef<Cow<'token, str>> for Token<'token> {
    fn as_ref(&self) -> &Cow<'token, str> {
        &self.text
    }
}

// impl<'token> FromStr for Token<'token> {
//     type Err = ParseError;
//     fn from_str(s: &'token str) -> Result<Self, Self::Err> {
//         Self::try_from(s)
//     }
// }
// impl<'token> ParallelString<'token> for Tokenizer<'token> {
//     fn as_parallel_string(&'token self) -> &'token str {
//         self.src.as_ref()
//     }
// }
// impl<'token> ParallelString<'token> for Token<'token> {
//     fn as_parallel_string(&'token self) -> &'token str {
//         self.text.as_ref()
//     }
// }
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
            tokens: Vec::new()
        }
    }
    // pub fn next(&'token mut self) -> Option<&'token &'token Token> {
    //     let token = self.tokens.first();
    //     if token.is_none() {
    //         let token = Token::from_str(self.src.as_ref()).unwrap().to_owned());
    //         self.tokens.append(token.clone());
    //     }
    //     self.tokens.first()
    // }
    pub fn peek(&'token self) -> Option<&'token &Token> {
        self.tokens.last()
    }
    pub fn push(&'token mut self, token: &'token Token) {
        self.tokens.push(token);
    }
    pub fn pop(&'token mut self) -> Option<&'token Token> {
        self.tokens.pop()
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
    // pub fn iter(&'token self) -> impl Iterator<Item = &'token Token> {
    //     self.tokens.iter()
    // }
    // pub fn iter_mut(&'token mut self) -> impl Iterator<Item = &'token &'token mut Token> {
    //     self.tokens.iter_mut()
    // }
    // pub fn into_iter(self) -> impl Iterator<Item = Token<'token>> {
    //     self.tokens.into_iter()
    // }
    pub fn as_slice(&'token self) -> &'token [&Token] {
        self.tokens.as_slice()
    }
    pub fn as_mut_slice(&'token mut self) -> &'token mut [&Token] {
        self.tokens.as_mut_slice()
    }
    // pub fn as_vec(&'token self) -> Vec<Token<'token>> {
    //     self.tokens.clone().into_vec()
    // }
    pub fn as_mut_vec(&'token mut self) -> Vec<&'token mut &Token> {
        self.tokens.iter_mut().collect_vec()
    }
}
// impl<'token> IntoIterator for Tokenizer<'token> {
//     type Item = Token<'token>;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
    // fn into_iter(self) -> Self::IntoIter {
    //     self.tokens.to_vec().into_iter().into()
    // }
// }
// impl<'token> AsRef<&'token [Token]> for Tokenizer<'token> {
//     fn as_ref(self) -> &'token [Token] {
//         self.tokens.into()
//     }
// }
// impl<'token> IntoIterator for Tokenizer<'token> {
//     type Item = Token<'token>;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.tokens.iter()
//     }
// }
// impl<'token> AsRef<Vec<Token<'token>>> for Tokenizer<'token> {
//     fn as_ref(&self) -> &Vec<Token<'token>> {
//         &self.clone().tokens.into()
//     }
// }
// impl FromStr for TokenKind {
//     type Err = ParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         TokenKind::try_from(s)
//     }
// }

#[doc(hidden)]
#[async_trait::async_trait]
pub(crate) trait HasOp<'op>: Sized + 'op
where
    Self: 'op + Sized,
{
    type Text: Into<String>;
    #[inline(always)]
    fn is_op(&'op self) -> AtomicBool;
    // #[inline(always)]
    // fn name(&'op self) -> Cow<'op, str> {
    //     Cow::Borrowed(self.into())
    // }
}
// impl<'op> Debug for OpLike<op> {
//     fn fmt(&'op self, f: &'op mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("OpLike")
//             .field("is_op", &self.is_op())
//             .finish()
//     }
// }
