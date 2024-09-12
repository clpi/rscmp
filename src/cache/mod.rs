use std::{
	collections::{HashMap, BTreeMap},
	num::NonZeroUsize,
	hash::Hash
};
use crossbeam_utils::CachePadded;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub(crate) struct Cache<'cache, K = usize, V = HashMap<String, String>>
where
	Self: 'cache + Sized,
	K: Hash + Eq + Clone + Serialize + Deserialize,
	V: Hash + Eq + Clone + Serialize + Deserialize,
{
	lru: lru::LruCache<K, V>,
	cache: CachePadded<V>,
	lasts: tokio::time::Duration,
}