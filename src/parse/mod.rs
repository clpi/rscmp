use std::{any::Any, fmt::Display};
use indexmap::set::MutableValues;
use serde::{Deserialize, Serialize};

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
    async fn control(&mut self) -> Any;
}

#[doc(hidden)]
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(serialize_all = "snake_case")]
pub(crate) enum Control<'c, Br: Display + Clone + 'c> {
	###[default]
    #[must_use = "Progress must flow"]
	CoContinue,
    Assign(Box<MutableValues>),
	BrBreak(Br),
    #[default = "Stops"]
    Stop,
	prPrune,
}
