use std::{
	collections::{
		BTreeMap, BTreeSet, BinaryHeap, VecDeque,
	}, default, ops::Deref, str::FromStr
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, EnumTable, FromRepr, IntoStaticStr, ToString, VariantArray, VariantNames};
use crate::parse::token::value::Number::Uint;

#[repr(u8)]
#[doc(hidden)]
#[derive(IntoStaticStr, AsRefStr, ToString, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Number<'ty> {
	#[default]
	#[strum(serialize = "str")]
	Opaque,
	#[strum(serialize = "str")]
	Float(f64),
	#[strum(serialize = "str")]
	Int(i64),
	#[strum(serialize = "str")]
	Uint(u128),
	#[strum(serialize = "str")]
	Usize(usize),
	#[strum(serialize = "str")]
	Isize(isize),
	#[strum(serialize = "str")]
	Complex((Number<'ty>, Number<'ty>)),
}

#[repr(usize)]
#[doc(hidden)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Compound<'ty> {
	#[default]
	#[strum(serialize = "str")]
	Box = 0x001 << 0x000,
	#[strum(serialize = "str")]
	Reference(&'ty Value<'ty>) = 0x002 << 0x000,
	#[strum(serialize = "str")]
	Set(BTreeSet<Value<'ty>>) = 0x002 << 0x001,
	#[strum(serialize = "str")]
	List(BTreeSet<Value<'ty>>) = 0x003 << 0x003,
	#[strum(serialize = "str")]
	Tuple(Vec<Value<'ty>>) = 0x004 << 0x004,
	#[strum(serialize = "str")]
	Map(BTreeMap<Value<'ty>, Value<'ty>>) = 0x006 << 0x005,
	#[strum(serialize = "str")]
	Optional(Option<Value<'ty>>) = 0x007 << 0x000,
}

#[repr(usize)]
#[doc(hidden)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Primitive<'ty> {
	#[strum(serialize = "str")]
	Str(&'ty str) = 0x000 << 0x000,
	#[strum(serialize = "str")]
	Char(char) = 0x001 << 0x000,
	#[strum(serialize = "str")]
	Num(Number<'ty>) = 0x001 << 0x001,
	#[strum(serialize = "str")]
	Bool(bool) = 0x002 << 0x002,
	#[default]
	#[strum(serialize = "str")]
	Nil = 0x003 << 0x003,
}

#[repr(usize)]
#[doc(hidden)]
#[derive(IntoStaticStr, AsRefStr, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Value<'ty> {
	#[strum(serialize = "str")]
	Bytes(Vec<u8>),
	#[strum(serialize = "str")]
	Primitive(Primitive<'ty>),
	#[strum(serialize = "str")]
	Compound(Compound<'ty>),
	#[strum(serialize = "str")]
	#[default]
	Other,
}

impl Value {

	#[inline(always)]
	pub const fn name() {
		println!("Hi")
	}

	#[inline(always)]
	pub const fn new() {

	}

	#[inline(always)]
	pub(crate) fn new_primitive(p: Primitive) -> Self {
		match p {
			Self::Compound(n) => Self::Primitive(Primitive::Num(n)),
			_ => Self::Primitive(p),
		}
	}

	#[inline(always)]
	pub(crate) fn new_num(num: Number) -> Self {
		match num {
			Number::Float(flt) => Self::Primitive(Primitive::Num(Number::Float(flt))),
			Number::Int(int) => Self::Primitive(Primitive::Num(Number::Int(int))),
			Number::Uint(uint) => Self::Primitive(Primitive::Num(Number::Uint(uint))),
			Number::Complex((real, cmpl)) => Self::Primitive(Primitive::Num(Number::Complex((real, cmpl)))),
			Number::Isize(isize) => Self::Primitive(Primitive::Num(Number::Isize(isize))),
			Number::Usize(usize) => Self::Primitive(Primitive::Num(Number::Usize(usize))),
			Number::Opaque => Self::Primitive(Primitive::Num(Number::Opaque)),
		}
	}
}


// impl FromStr for Value {
//     fn from_str(s: &str) -> Result<Self, Self::Err> {

//     }
// }
