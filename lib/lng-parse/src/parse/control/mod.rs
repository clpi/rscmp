use strum::{AsRefStr, AsStaticStr, Display, EnumIs, EnumIter};
use std::fmt::Display;
use std::ops::Not;
use indexmap::set::MutableValues;
use crate::parse::token::Token;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug)]
pub(crate) struct Control<'c> {
	kind: ControlKind,
	token: Token<'c>,
	context: std::task::Context<'c>,
}

#[derive(Debug, Display, EnumIs, AsRefStr, EnumString, Serialize, Deserialize, EnumIter)]
#[strum(serialize_all = "snake_case", prefix = "control_")]
pub(crate) enum ControlKind {
	Continue,
	Assign,
	Stop,
	Start,
	Break,
	Prune,
	Spawn,
}
pub(crate) trait Controllable{
	type Control;
}