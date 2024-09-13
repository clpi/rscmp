use std::borrow::Cow;
use std::cell::{RefCell, Cell};
use std::thread::Thread;
use std::task::Context;
use std::ffi::CStr;
use std::future::{Future, IntoFuture};
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use crossbeam_utils::atomic::AtomicConsume;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Compiler<'c> {
    pub(crate) src: Cow<'c, str>,
    pub(crate) debug: AtomicBool,
}
impl<'c> Deref for Compiler<'c> {
    type Target = Cow<'c, str>;

    #[must_use]
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.src
    }
}
impl<'c> Compiler<'c> {
    #[must_use]
    #[inline]
    pub(crate) const fn new(src: Cow<'c, str>, debug: bool) -> Self {
        Self {
            src,
            debug: AtomicBool::new(debug),
        }
    }

}