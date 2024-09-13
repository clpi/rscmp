use std::borrow::{Borrow, Cow};
use std::fmt::{Formatter,Display};
use std::ops::{Deref, DerefMut, Index, Sub};
use std::str::FromStr;
use std::string::ParseError;
use std::sync::atomic::{AtomicUsize, Ordering};
use pest::LinesSpan;
use pyo3::ToPyObject;
use rayon::str::ParallelString;
use serde::{Deserialize, Serialize};
use crate::parse::compile::Compiler;
use crate::parse::grammar::Rule::value;
use crate::parse::token::{Token, TokenKind, Tokenizer};

#[derive(Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub(crate) struct SpanIndex(pub(crate) usize);

impl Deref for SpanIndex {
	type Target = usize;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AsRef<usize> for SpanIndex {
	#[inline(always)]
	fn as_ref(&self) -> &usize {
		&self.0
	}
}
impl DerefMut for SpanIndex {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl From<usize> for SpanIndex {
	#[inline(always)]
	fn from(index: usize) -> Self {
		SpanIndex(index)
	}
}

#[derive(Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq, Debug, PartialOrd)]
pub(crate) struct Span
{
	pub start: SpanIndex,
	pub end: SpanIndex,
}
impl Display for SpanIndex {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.0)
	}

}
impl<'span> Display for Span {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", &self.start, &self.end)
	}
}

// impl<'span> Index<SpanIndex> for Tokenizer<'span> {
// 	type Output = Span;
//
// 	fn index(&self, index: SpanIndex) -> &Self::Output {
// 		self[index.borrow().load(Ordering::Relaxed)]
// 	}
// }
// impl<'span> Index<SpanIndex> for Cow<'span, str> {
// 	type Output = Span;
//
// 	fn index(&self, index: SpanIndex) -> &Self::Output {
// 		self[&index.borrow().load(Ordering::Relaxed)]
// 	}
// }

// impl<'span> Index<SpanIndex> for Compiler<'span> {
// 	type Output = Span;
//
	// fn index(&self, index: SpanIndex) -> &Self::Output {
	// 	self[index.borrow().load(Ordering::Relaxed)]
	// }
// }

// impl Borrow<AtomicUsize> for SpanIndex {
// 	fn borrow(&self) -> &AtomicUsize {
// 		 let a: AtomicUsize = self.0.into();
// 		a
// 	}
// }
impl Borrow<SpanIndex> for Span {
	fn borrow(&self) -> &SpanIndex {
		&self.start
	}
}
impl From<SpanIndex> for usize {
	#[inline(always)]
	fn from(index: SpanIndex) -> Self {
		index.0
	}
}
impl From<SpanIndex> for Span {
	#[inline(always)]
	fn from(index: SpanIndex) -> Self {
		Span { start: index.clone(), end: index.into() }
	}
}
impl Span {

	// #[inline(always)]
	// pub(crate) const fn length(&self) -> usize {
	// 	let a = self.start.deref();
	// 	self.end.kksub(*a)
	// }
	//
	pub(crate) fn new(start: usize, end: usize) -> Self {
		Span { start: start.into(), end: end.into() }
	}

	pub fn set_end(&mut self, end: usize) -> () {
		self.end = end.into();
	}
	pub fn set_start(&mut self, start: usize) -> () {
		self.start = start.into();
	}
	//
	// #[inline(always)]
	// pub(crate) const fn pos(self) -> (usize, usize) {
	// 	(self.start, self.end.into())
	// }

}