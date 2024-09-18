use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use pest_ast::FromPest;
use pest_vm::Vm;

pub(crate) struct VmCompiler<'c> {
	src: Cow<'c, str>,
	debug: AtomicBool,
	index: AtomicUsize,
}

impl<'c> VmCompiler<'c> {
	pub(crate) fn new(src: Cow<'c, str>, debug: bool) -> Self {
		VmCompiler { src: Cow::from(src), debug: AtomicBool::new(debug), index: AtomicUsize::new(0) }
	}

	pub(crate) fn compile(&self) -> VmCompiler<'c> {
		VmCompiler::new("".into(), false)
	}
}