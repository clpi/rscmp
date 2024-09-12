use std::fmt::{Display, Formatter};
use std::str::FromStr;
use rayon::str::ParallelString;
use pest_vm::Vm;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, FromRepr, EnumIter, EnumProperty, EnumString, EnumTable, IntoStaticStr, ToString, VariantArray, VariantNames, ParseError, EnumTryAs};
use crate::parse::token::keyword::{Keyword};

#[repr(usize)]
#[non_exhaustive]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, EnumTryAs, EnumProperty, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Op {
	#[strum(serialize = "keyword")]
	Keyword(Keyword) = 0x000000000,
	Assign(OpAssign) = 0x000000001,
	Prefix(OpPrefix) = 0x000000008,
	Logic(OpLogic) = 0x000000002,
	Compare(OpCompare) = 0x000000003,
	Math(OpMath) = 0x000000004,
	Block(OpBlock) = 0x000000005,
	#[default]
	Other,
}
#[repr(usize)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[allow(clippy::default_trait_access)]
pub(crate) enum OpPrefix {
	#[default]
	#[strum(serialize = "keyword")]
	Pipe,
	#[strum(serialize = "&")]
	Ref = 0x00000000D << 13,
	#[strum(serialize = "*")]
	Ptr = 0x00000000D << 13,
	#[strum(serialize = "not")]
	Not = 0x00000000D << 13,
}
#[repr(usize)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum OpAssign {
	#[default]
	#[strum(serialize = "=")]
	Assign = 0x00000000F << 15,
	#[strum(serialize = "=")]
	AddAssign = 0x000000010 << 16,
	#[strum(serialize = "=")]
	SubAssign = 0x000000011 << 17,
	#[strum(serialize = "=")]
	DivAssign = 0x000000012 << 18,
	#[strum(serialize = "=")]
	MulAssign = 0x000000013 << 19,
	#[strum(serialize = "=")]
	ModAssign = 0x000000014 << 20,
	#[strum(serialize = "=")]
	AndAssign = 0x000000015 << 21,
	#[strum(serialize = "=")]
	OrAssign = 0x000000016 << 22,
	#[strum(serialize = "=")]
	EqAssign = 0x000000017 << 23,
	#[strum(serialize = "=")]
	NeqAssign = 0x000000018 << 24,
}

#[repr(usize)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum OpBlock {
	#[default]
	#[strum(serialize = "if")]
	If = 0x000000001 << 0,
	#[strum(serialize = "else if")]
	ElseIf = 0x000000002 << 1,
	#[strum(serialize = "else")]
	Else = 0x000000003 << 2,
	#[strum(serialize = "else")]
	For = 0x000000004 << 3,
	#[strum(serialize = "else")]
	While = 0x000000005 << 4,
	#[strum(serialize = "else")]
	Do = 0x000000006 << 5,
	#[strum(serialize = "else")]
	With = 0x000000007 << 6,
	#[strum(serialize = "else")]
	As = 0x000000008 << 7,
}

#[repr(usize)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum OpLogic {
	#[default]
	#[strum(serialize = "and")]
	And = 0x000000005 << 5,
	#[strum(serialize = "or")]
	Or = 0x000000006 << 6,
	#[strum(serialize = "xor")]
	Xor = 0x000000006 << 8,
	#[strum(serialize = "eq")]
	Eq = 0x000000007 << 7,
}
#[repr(usize)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum OpCompare {
	#[default]
	#[strum(serialize = "==")]
	Eq = 0x000000008 << 8,
	#[strum(serialize = "!=")]
	Neq = 0x000000008 << 8,
	#[strum(serialize = ">")]
	Gt = 0x000000009 << 9,
	#[strum(serialize = "<")]
	Lt = 0x00000000A << 10,
	#[strum(serialize = ">=")]
	Gte = 0x00000000B << 11,
	#[strum(serialize = "<=")]
	Lte = 0x00000000C << 12,
	#[strum(serialize = "===")]
	Is = 0x000000004 << 14,
	#[strum(serialize = "!")]
	Not = 0x000000004 << 15,
}
#[repr(usize)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum OpMath {
	#[default]
	#[strum(serialize = "+")]
	Add = 0x000000001 << 0,
	#[strum(serialize = "-")]
	Sub = 0x000000002 << 1,
	#[strum(serialize = "/")]
	Div = 0x000000003 << 2,
	#[strum(serialize = "*")]
	Mul = 0x000000004 << 3,
	#[strum(serialize = "%")]
	Mod = 0x000000004 << 4,
	#[strum(serialize = "floor")]
	Floor = 0x000000004 << 5,
	#[strum(serialize = "floor")]
	Ceil = 0x000000004 << 6,
	#[strum(serialize = "floor")]
	Round = 0x000000004 << 7,
}
impl FromStr for Op {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Op::Math(OpMath::Add)),
			"-" => Ok(Op::Math(OpMath::Sub)),
			"/" => Ok(Op::Math(OpMath::Div)),
			"*" => Ok(Op::Math(OpMath::Mul)),
			"%" => Ok(Op::Math(OpMath::Mod)),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}
impl FromStr for OpPrefix {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"|" => Ok(OpPrefix::Pipe),
			"&" => Ok(OpPrefix::Ref),
			"*" => Ok(OpPrefix::Ptr),
			"not" => Ok(OpPrefix::Not),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}
impl FromStr for OpAssign {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"=" => Ok(OpAssign::Assign),
			"+=" => Ok(OpAssign::AddAssign),
			"-=" => Ok(OpAssign::SubAssign),
			"/=" => Ok(OpAssign::DivAssign),
			"*=" => Ok(OpAssign::MulAssign),
			"%" => Ok(OpAssign::ModAssign),
			"&=" => Ok(OpAssign::AndAssign),
			"|=" => Ok(OpAssign::OrAssign),
			"==" => Ok(OpAssign::EqAssign),
			"!=" => Ok(OpAssign::NeqAssign),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}
impl FromStr for OpLogic {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"&&" => Ok(OpLogic::And),
			"||" => Ok(OpLogic::Or),
			"^" => Ok(OpLogic::Xor),
			"==" => Ok(OpLogic::Eq),
			"!" => Ok(OpLogic::Not),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}
impl FromStr for OpCompare {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"==" => Ok(OpCompare::Eq),
			"!=" => Ok(OpCompare::Neq),
			">" => Ok(OpCompare::Gt),
			"<" => Ok(OpCompare::Lt),
			">=" => Ok(OpCompare::Gte),
			"<=" => Ok(OpCompare::Lte),
			"is" => Ok(OpCompare::Is),
			"not" => Ok(OpCompare::Not),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}
impl FromStr for OpMath {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(OpMath::Add),
			"-" => Ok(OpMath::Sub),
			"/" => Ok(OpMath::Div),
			"*" => Ok(OpMath::Mul),
			"%" => Ok(OpMath::Mod),
			"floor" => Ok(OpMath::Floor),
			"ceil" => Ok(OpMath::Ceil),
			"round" => Ok(OpMath::Round),
			_ => Err(ParseError::VariantNotFound),
		}
	}
}

impl Op {
	fn new() -> Self {
		Self::Other
	}
}
impl From<OpAssign> for Op {
	fn from(op: OpAssign) -> Self {
		Op::Assign(op)
	}
}
impl From<OpLogic> for Op {
	fn from(op: OpLogic) -> Self {
		Op::Logic(op)
	}
}
impl From<OpCompare> for Op {
	fn from(op: OpCompare) -> Self {
		Op::Compare(op)
	}
}
impl From<OpPrefix> for Op {
	fn from(op: OpPrefix) -> Self {
		Op::Prefix(op)
	}
}
impl From<OpAssign> for Op {
	fn from(op: OpAssign) -> Self {
		Op::Assign(op)
	}
}
impl From<OpMath> for Op {
	fn from(op: OpMath) -> Self {
		Op::Math(op)
	}
}
impl From<OpBlock> for Op {
	fn from(op: OpBlock) -> Self {
		Op::Block(op)
	}
}
impl From<Keyword> for Op {
	fn from(op: Keyword) -> Self {
		Op::Keyword(op)
	}
}

impl From<Keyword> for Op {
	fn from(op: Keyword) -> Self {
		Op::Keyword(op.into())
	}
}

impl ParallelString for OpLogic{
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl ParallelString for OpBlock{
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl ParallelString for OpMath {
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl ParallelString for OpCompare {
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl ParallelString for OpPrefix {
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl ParallelString for Op {
	fn as_parallel_string(&self) -> &str {
		self.into()
	}
}
impl Op {
	fn df() -> Self {
		Self::from_repr(0).into()
	}

}