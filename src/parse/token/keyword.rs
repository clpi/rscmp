use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use petgraph::algo::BoundedMeasure;
use petgraph::visit::ControlFlow;
use rayon::str::ParallelString;
use serde::{Deserialize, Serialize};
use smallvec::SpecFrom;
use strum::{AsRefStr, Display, EnumIter, EnumString, EnumTable, FromRepr, IntoStaticStr, ParseError, VariantArray, VariantNames};
use crate::parse::token::op::{Op, OpMath};

#[repr(u16)]
#[allow(clippy::perf)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Keyword {
	#[default]
	#[strum(serialize = "keyword")]
	Def = 0x0001 << 0x0006,
	#[strum(serialize = "keyword")]
	Does = 0x0001 << 0x0007,
	#[strum(serialize = "keyword")]
	Else = 0x0001 << 0x0008,
	Then = 0x0001 << 0x0009,
	When = 0x0001 << 0x000A,
	Where = 0x0001 << 0x000B,
	Let = 0x0001 << 0x0007,
	Set = 0x0001 << 0x0008,
	Get = 0x0001 << 0x0009,
	Put = 0x0001 << 0x000A,
	In = 0x000000000,
	If = 0x000000001 << 0,
	As = 0x000000002 << 1,
	At = 0x000000003 << 2,
	Of = 0x000000004 << 3,
	On = 0x000000004 << 4,
	Or = 0x000000004 << 5,
	Nil = 0x000000004 << 6,
	To = 0x000000004 << 7,
	Do = 0x000000004 << 8,
	Can = 0x000000004 << 9,
	Not = 0x000000004 << 10,
	And = 0x000000004 << 11,
	Try = 0x000000004 << 12,
	For = 0x000000004 << 13,
	Is = 0x000000004 << 14,
	With = 0x000000004 << 15,
	From = 0x000000004 << 16,
	Type = 0x000000004 << 17,
	Init = 0x000000004 << 18,
	Exit = 0x000000004 << 19,
	Loop = 0x000000004 << 20,
	Next = 0x000000004 << 21,
	New = 0x000000004 << 22,
	Old = 0x000000004 << 23,
	So = 0x000000004 << 24,
	This = 0x000000004 << 25,
	Take = 0x000000004 << 25,
	Has = 0x000000004 << 25,
	Was = 0x000000004 << 25,
	Until = 0x000000004 << 28,
	Out = 0x000000004 << 29,
	Pub = 0x000000004 << 30,
	Private = 0x000000004 << 31,
	Into = 0x000000004 << 32,
}

impl Deref for Keyword {
	type Target = String;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.to_string()
	}
}
impl DerefMut for Keyword {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self
	}
}
impl FromStr for Keyword {
	type Err = ParseError;
	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"As" => Ok(Keyword::As),
			"From" => Ok(Keyword::From),
			"To" => Ok(Keyword::To),
			"In" => Ok(Keyword::In),
			"And" => Ok(Keyword::And),
			"Of" => Ok(Keyword::Of),
			"On" => Ok(Keyword::On),
			"Try" => Ok(Keyword::Try),
			"Else" => Ok(Keyword::Else),
			"Then" => Ok(Keyword::Then),
			"Where" => Ok(Keyword::Where),
			"Let" => Ok(Keyword::Let),
			"Set" => Ok(Keyword::Set),
			"Get" => Ok(Keyword::Get),
			"Put" => Ok(Keyword::Put),
			"Does" => Ok(Keyword::Does),
			"Can" => Ok(Keyword::Can),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}
impl Into<String> for Keyword {
	#[inline]
	fn into(self) -> String {
		String::from("As")
	}
}
impl ParallelString for Keyword {
		#[inline]
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl From<Keyword> for String {
		#[inline]
	fn from(k: Keyword) -> Self {
		String::from("As")
	}
}
