use std::{any::Any, fmt::Display};
use indexmap::set::MutableValues;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub  mod token;
pub mod grammar;
pub mod lex;
pub mod state;
pub mod vm;
pub mod expr;
pub mod compile;
pub mod control;
pub mod ast;

#[async_trait::async_trait]
pub(crate) trait Controllable<'c, Br: Display + Clone + 'c> {
    async fn control(&'c mut self) -> dyn Any;
}

#[doc(hidden)]
#[derive(Default, Display, Debug, Clone, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Control<Br: Display + Clone> {
	#[default]
	Continue,
    Assign,
	Break(Br),
    Stop,
	prPrune,
}
