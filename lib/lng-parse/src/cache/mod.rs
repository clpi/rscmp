use std::{
	collections::{HashMap, BTreeMap},
	num::NonZeroUsize,
	hash::Hash
};
use std::collections::HashSet;
use crossbeam_utils::CachePadded;
use serde::{Deserialize, Serialize};

// #[derive(Debug)]
// pub(crate) struct Cache<'cache, K = usize, V = HashSet<String>>
// // where
// 	// Self: Sized,
// 	// K: Eq + Clone + Serialize + Deserialize<'cache> + Hash + 'cache,
// 	// V: Eq + Clone + Serialize + Deserialize<'cache> + 'cache,
// {
// 	lru: lru::LruCache<K, V>,
// 	cache: CachePadded<V>,
// 	lasts: tokio::time::Duration,
// }