use core::num::bignum::FullOps;
use std::fmt::Debug;
use std::str::pattern::DoubleEndedSearcher;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicPtr, AtomicUsize};
use async_trait::async_trait;

#[doc(hidden)]
#[repr(transparent)]
#[async_trait::async_trait]
pub(crate) trait HasOp<'op>: Sized + 'op
where
	Self: 'op + Sized,
 {
	 type Boolean = AtomicBool;
	 type Isize = AtomicIsize;
	 type Usize = AtomicUsize;
	 type Ptr<P> = AtomicPer<P>;

	 #[inline(always)]
	fn is_op(&'op self) -> AtomicBool;
	 #[inline(always)]
	fn name(&'op self) -> Cow<'op, str> {
	}
}
impl<'op> Debug for impl OpLike<op> {
	fn fmt(&'op self, f: &'op mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("OpLike")
			.field("is_op", &self.is_op())
			.finish()
	}
}

impl<'op> OpLike<'op> for FullOps