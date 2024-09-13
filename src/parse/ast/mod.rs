mod visitor;

use std::borrow::Cow;
use std::sync::atomic::AtomicBool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct AstNode {
	pub(crate) src: String,
	pub(crate) debug: AtomicBool,
}