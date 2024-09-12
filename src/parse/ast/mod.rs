mod visitor;

use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct AstNode<'ast> {
	pub(crate) src: &'ast str,
	pub(crate) debug: bool,
}