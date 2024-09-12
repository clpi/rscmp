use std::fmt::Binary;
use std::ops::Index;
use itertools::Group;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, EnumVariantNames};
use tracing::field::Field;

#[repr(u16)]
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Default, PartialEq, Eq, Hash, EnumIter)]
pub(crate) enum Expr<'expr> {
	#[default]
	Literal = 0x0001 << 0x0001,
	Ident = 0x0002 << 0x0002,
	Unary = 0x0003 << 0x0003,
	Binary,
	Call,
	Index,
	Field,
	Group,
	Assign,
	Block,
	With,
	If,
	As,
	While,
	For,
	Return,
	Break,
	Continue,
	Fn,
	Method,
	Class,
	Use,
	Export,
	Error,
	Test,
}
