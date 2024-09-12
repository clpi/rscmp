#![allow(dead_code)]

use strum::{Display, S}
use std::fmt::Display;
use crate::parse::Token;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[repr(transparent)]
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Control<'c, ty> {
	kind: ControlKind<'c, ty>,
	token: &'c Token<'c>,
	context: std::task::Context<'c>,
}

#[derive(Debug, Clone, EnumString, Serialize, Deserialize, Default, AsRefStr,)]
#[serde(serialize_as = "snake_case")])]
pub(crate) struct ControlKind<'c, Ty: Display + Clone + 'c> {
	#[default]
	#[doc(hidden)]
	#[serde(serialize_with = "Control::serialize")]
	Continue = 0x0000 << 0x0001,
	Assign(Box<MutableValues>) = 0x0001 << 0x0001,,
	Stop = 0x0002 << 0x0001,
	Start = 0x0003 << 0x0001,
	Break(Br) = 0x0004 << 0x0001,
	Prune = 0x0005 << 0x0001,
	Spawn = 0x0006 << 0x0001,
}
pub(crate) trait Controllable<'val, Val> {
	#[default]
	Not,
}