use pest::error::{Error as PestError, ErrorVariant as PestErrorVariant};
use strum::{AsRefStr, AsStaticStr, Display, EnumIs, EnumIter, FromRepr, IntoStaticStr, ParseError as StrumError, VariantNames};
use std::{
    alloc::{Layout, LayoutError, System, GlobalAlloc},
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd, Reverse},
    process::{Child, Output, Termination, Stdio, ChildStdout, Command,},
    task::{Context, Waker, Poll, },
    thread::{AccessError, JoinHandle, ThreadId, Result as ThreadResult},
    future::{Ready, Future, IntoFuture, Pending, PollFn, },
    io::{Error as IoError, ErrorKind as IoErrorKind, IntoInnerError as IoIntoInnerError},
    char::{ParseCharError, CharTryFromError, TryFromCharError},
    cell::{BorrowError, BorrowMutError, Cell, LazyCell, RefCell, OnceCell},
    iter::{Filter, Inspect, IntoIterator, Iterator, Empty, Scan, Peekable},
    array::{IntoIter as ArrayIntoIter, TryFromSliceError},
    vec::{Vec, Splice, IntoIter as VecIntoIter, Drain},
    time::{Duration, Instant, SystemTime, SystemTimeError, TryFromFloatSecsError, UNIX_EPOCH},
    fs::{self, DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, ReadDir, Permissions, OpenOptions},
    slice::{EscapeAscii, SliceIndex, Chunks, Windows, },
    convert::{AsRef, Into, TryInto, AsMut, From, TryFrom, Infallible},
    clone::{Clone, self},
    error::{Error as StdError},
    boxed::{self, Box, },
    collections::{BTreeMap, BTreeSet, BinaryHeap, Bound, HashMap, HashSet, VecDeque},
    f64::{INFINITY, NEG_INFINITY},
    fmt::{self, Alignment, Binary, Debug, Display, Error as FmtError, Formatter, Pointer, Result},
    num::{IntErrorKind, NonZero, FpCategory, ParseFloatError, ParseIntError, TryFromIntError},
    ops::{Deref, DerefMut},
    str::{Utf8Error, Bytes, Chars, ParseBoolError},
    any::{Any, TypeId},
    string::{FromUtf16Error, FromUtf8Error, ParseError},
    ffi::{OsStr, OsString, IntoStringError},
    sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak, },
    char::DecodeUtf16Error,
};
use petgraph::graph::UnGraph;
use crossbeam::{
    utils::{CachePadded, Backoff},
    epoch::{LocalHandle, Collector, Owned, CompareExchangeError, Shared},
    sync::{Parker, Unparker, WaitGroup, ShardedLockReadGuard, ShardedLockWriteGuard},
    thread::{ScopedThreadBuilder, Scope, ScopedJoinHandle},
    atomic::{AtomicCell, AtomicConsume},
    deque::{Steal, Stealer, Worker},
    queue::{SegQueue, ArrayQueue},
    channel::{RecvError, SendError, SendTimeoutError, RecvTimeoutError, TryRecvError}
};
use either::Either;
use rayon::{
    range::Iter,
    array::IntoIter,
    prelude::{IndexedParallelIterator, ParallelString, ParallelSlice},
    ThreadPoolBuildError
};
use serde::{
    de::Error as SerdeDeserializeError,
    ser::Error as SerdeSerializeError,
    Deserialize, Serialize
};
use strum::EnumString;
use tracing::{Event, Subscriber, Span, Callsite, Dispatch, };
use tracing_subscriber::{EnvFilter, FmtSubscriber, Registry};

pub trait ErrorTrait: StdError + Sized {
    fn source(&self) -> Option<&(dyn StdError + 'static)>;
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&dyn StdError>;
}

#[repr(usize)]
#[derive(Display, EnumIs, AsRefStr, IntoStaticStr, VariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum AppError
{
    Pest(PestError<PestErrorVariant<String>>),
    Io(IoError),
    Fmt(FmtError),
    Time(SystemTimeError),
    Transfer(Either<SendError<String>, RecvError>),
    TimeoutTransfer(Either<SendTimeoutError<String>, RecvTimeoutError>),
    Parse(ParsingError) = 0x0007,
    Layout(LayoutError) = 0x0008,
    Other = 0x0009,
}

#[repr(usize)]
#[derive(Display, Debug, EnumIs, AsRefStr, IntoStaticStr, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum ParsingError {
    Uuid(uuid::Error),
    Bool(ParseBoolError),
    Char(ParseCharError),
    Int(ParseIntError),
    Utf8(Utf8Error),
    Str(ParseError),
    Bytes(ParseError),

    CharTryFrom(CharTryFromError),
    CharTryFromIntError(IntErrorKind),
    IntoString(IntoStringError),
    TryFromSlice(TryFromSliceError),
    TryFromInt(TryFromIntError),
    TryFromFloat(ParseFloatError),
    TryFromFloatSecs(TryFromFloatSecsError),
    TryFromChar(TryFromCharError),

    DecodeUtf16(DecodeUtf16Error),
    FromUtf16(FromUtf16Error),
    FromUtf8(FromUtf8Error),
    Other = 0x020 << 0x010,
}
impl<'e> From<ParsingError> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: ParsingError) -> Self {
        AppError::Parse(e)
    }
}
impl<'e> From<FmtError> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: FmtError) -> Self {
        AppError::Fmt(e.into())
    }
}
impl<'e> From<PestError<PestErrorVariant<String>>> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: PestError<PestErrorVariant<String>>) -> Self {
        AppError::Pest(e)
    }
}
impl<'e> From<IoError> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: IoError) -> Self {
        AppError::Io(e.into())
    }
}
impl<'e> From<SystemTimeError> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: SystemTimeError) -> Self {
        AppError::Time(e)
    }
}
impl<'e> From<LayoutError> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: LayoutError) -> Self {
        AppError::Layout(e)
    }
}
impl<'e> From<Either<SendError<String>, RecvError>> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: Either<SendError<String>, RecvError>) -> Self {
        AppError::Transfer(e)
    }
}
impl<'e> From<Either<SendTimeoutError<String>, RecvTimeoutError>> for AppError {
    #[inline(always)]
    #[must_use]
    fn from(e: Either<SendTimeoutError<String>, RecvTimeoutError>) -> Self {
        AppError::TimeoutTransfer(e)
    }
}

impl StdError for ParsingError {
    #[inline(always)]
    fn source(&self) -> Option<&'static (dyn StdError + 'static)> {
        None
    }
    #[inline(always)]
    fn description(&self) -> &str {
        ""
    }
    #[inline(always)]
    fn cause(&self) -> Option<&'static dyn StdError> {
        None
    }
}

impl<'e> StdError for AppError {
    fn source(&self) -> Option<&'static (dyn StdError + 'static)> {
        None
    }
    fn description(&self) -> &str {
        ""
    }
    fn cause(&self) -> Option<&'static dyn StdError> {
        None
    }
}

impl Default for AppError {
    fn default() -> Self {
        Self::Other
    }
}