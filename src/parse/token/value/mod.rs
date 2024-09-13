use std::{
	collections::{
		BTreeMap, BTreeSet, BinaryHeap, VecDeque,
	}, default, ops::Deref, str::FromStr
};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo};
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, EnumTable, FromRepr, IntoStaticStr, ToString, VariantArray, VariantNames};

#[repr(u8)]
#[doc(hidden)]
#[derive(Ord, AsRefStr, ToString, FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, , Debug, PartialOrd, Partial  Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Number {
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
	Complex((usize, usize)),
}

#[repr(usize)]
#[doc(hidden)]
#[derive(AsRefStr, FromRepr, VariantArray,  PartialOrd, Ord, VariantNames, EnumTable, Serialize, Deserialize, Display, , Debug, Partial  Serialize, Deserialize, )]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Compound {
	#[default]
	#[strum(serialize = "str")]
	Box ,
	#[strum(serialize = "str")]
	Reference(Value) ,
	#[strum(serialize = "str")]
	Set(BTreeSet<Value>) ,
	#[strum(serialize = "str")]
	List(BTreeSet<Value>) ,
	#[strum(serialize = "str")]
	Tuple(Vec<Value>) ,
	#[strum(serialize = "str")]
	Map(Vec<(Value, Value)>),
	#[strum(serialize = "str")]
	Optional(Option<Value>),
}
#[repr(usize)]
#[doc(hidden)]
#[derive(AsRefStr, VariantArray, VariantNames, EnumTable,Display, Debug, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub (crate) enum RangeKind<N> {
	#[strum(serialize = "str")]
	Range(Range<N>),
	#[strum(serialize = "str")]
	RangeInclusive(RangeInclusive<N>),
	#[strum(serialize = "str")]
	RangeFull(RangeFull),
	#[strum(serialize = "str")]
	RangeTo(RangeTo<N>),
	#[strum(serialize = "str")]
	RangeFrom(RangeFrom<N>),
	#[strum(serialize = "str")]
	Other,
}

#[repr(usize)]
#[derive(AsRefStr,  FromRepr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, , Debug, PartialOrd, Partial Serialize, Deserialize, Ord)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Primitive {
	#[strum(serialize = "str")]
	Str(String) ,
	#[strum(serialize = "str")]
	Char(char) ,
	#[strum(serialize = "str")]
	Number(Number) ,
	#[strum(serialize = "str")]
	Bool(bool) ,
	#[strum(serialize = "str")]
	Opaque ,
	#[strum(serialize = "str")]
	Nil ,
}

#[repr(usize)]
#[doc(hidden)]
#[derive(AsRefStr, FromRepr,  PartialOrd, Eq, Ord, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, , Debug, PartialEq, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Value {
	#[strum(serialize = "str")]
	Bytes(Vec<u8>),
	#[strum(serialize = "str")]
	Primitive,
	#[strum(serialize = "str")]
	Compound(Compound),
	// #[strum(serialize = "str")]
	// Range(RangeKind<usize>),
	#[strum(serialize = "str")]
	#[default]
	Other,
}

impl Value {

	#[inline(always)]
	pub fn name() {
		println!("Hi")
	}

	#[inline(always)]
	pub const fn new() {

	}

	// #[inline(always)]
	// pub(crate) fn new_primitive(p: Number) -> Self {
	// 	match p {
	// 		Self::Compound(n) ,
	// 		_ ,
	// 	}
	// }

	// #[inline(always)]
	// pub(crate) fn new_num(num: Number) -> Self {
	// 	match num {
	// 		Number::Float(flt) ,
	// 		Number::Int(int) ,
	// 		// Number::Uint(uint) ,
	// 		Number::Complex((real, cmpl)) ,
	// 		Number::Isize(isize) ,
	// 		Number::Usize(usize) ,
	// 		_ => Self::Primitive(Primitive::Num(Number::Opaque))
	// 	}
	// }
}


// impl FromStr for Value {
//     fn from_str(s: &str) -> Result<Self, Self::Err> {

//     }
// }
