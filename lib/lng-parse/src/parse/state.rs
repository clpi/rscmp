use std::marker::PhantomData;

pub(crate) enum StateType {
	Compile,
	Parse,
}
pub(crate) struct State<'state, S> {
	s: &'state S,
}