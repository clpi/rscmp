use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use petgraph::algo::BoundedMeasure;
use petgraph::visit::ControlFlow;
use rayon::str::ParallelString;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, EnumTable, FromRepr, IntoStaticStr, ParseError, VariantArray, VariantNames};
use crate::parse::token::op::{Op, OpMath};

#[allow(clippy::perf)]
#[derive(IntoStaticStr, AsRefStr, VariantArray, VariantNames, EnumTable, Serialize, Deserialize, Display, EnumString, EnumIter, Default, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Keyword {
	#[default]
	Def ,
	Does ,
	Else ,
	Then ,
	When ,
	Where ,
	Let ,
	Set ,
	Get ,
	Put ,
	In ,
	If ,
	As ,
	At ,
	Of ,
	On ,
	Or ,
	Nil ,
	To ,
	Do ,
	Can ,
	Not ,
	And ,
	Try ,
	For ,
	Is ,
	With ,
	From ,
	Type ,
	Init ,
	Exit ,
	Loop ,
	Next ,
	New ,
	Old ,
	So ,
	This ,
	Take ,
	Has ,
	Was ,
	Until ,
	Out ,
	Pub ,
	Private ,
	Into ,
}

// impl Deref for Keyword {
// 	type Target = String;
//
// 	#[inline(always)]
// 	fn deref(self) -> &Self::Target {
// 		&self.into()
// 	}
// }
// impl DerefMut for Keyword {
// 	#[inline(always)]
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		self
// 	}
// }
// impl Into<String> for Keyword {
// 	#[inline]
// 	fn into(self) -> String {
// 		String::from("As")
// 	}
// }
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
