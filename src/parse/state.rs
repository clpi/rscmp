use std::marker::PhantomData;

pub(crate) enum StateType {
	Compile,
	#[default]
	Parse,
}
pub(crate) struct State<'state, S> {
	s: &'state S,
}