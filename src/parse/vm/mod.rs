use std::borrow::Cow;
use pest_ast::FromPest;
use pest_vm::Vm;

pub(crate) struct VmCompiler<'c> {
	src: Cow<'c, str>,
	debug: bool,
	index: Ind
}

impl<'c> VmCompiler<'c> {
	pub(crate) fn new(src: Cow<'c, str>, debug: bool) -> Self {
		VmCompiler { src, debug }
	}

	pub(crate) fn compile(&self) -> Vm {
		Vm::new()
	}
}