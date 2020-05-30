use std::collections::hash_map::{DefaultHasher};
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::mem::replace;
use std::iter::{Iterator, FromIterator};
// use std::fmt::{Display, Debug};

pub struct HashSet<K> {
	items: Vec<Vec<K>>,
	len: usize,
}

impl<K> HashSet<K> {
	pub fn new() -> Self {
		Self { 
			items: Vec::new(),
			len: 0,
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self { 
			items: Vec::with_capacity(capacity),
			len: 0,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	pub fn capacity(&self) -> usize {
		self.items.len()
	}

	pub fn len(&self) -> usize {
		self.len
	}
}
const INITIAL_DEFAULT_CAPACITY: usize = 64;

impl<K> HashSet<K>
where
	K: Eq + Hash,
{

	fn get_bucket<Q>(&self, key: &Q) -> usize
	where
		K: Borrow<Q>,
		Q: Eq + Hash + ?Sized,
	{
		let mut hasher = DefaultHasher::new();
		key.hash(&mut hasher);
		(hasher.finish() % self.items.len() as u64) as usize
	}

	pub fn insert(&mut self, item: K) -> bool {
		if self.is_empty() || self.len >= 2*self.items.len() {
			self.resize();
		}

		let bucket = self.get_bucket(&item);
		
		for eitem in self.items[bucket].iter() {
			if *eitem == item {
				// println!("Found duplicate Bucket = {} item = {}", bucket, item);
				return false;
			}
		}
		self.items[bucket].push(item);
		// println!("Contents of bucket {} = {:?}", bucket, self.items[bucket]);
		self.len += 1;
		true
	}
	
	fn resize(&mut self) {
		let target_size = match self.items.len() {
			0 => INITIAL_DEFAULT_CAPACITY,
			n => 2*n,
		};
		let mut new_buckets = Vec::with_capacity(target_size);
		new_buckets.extend((0..target_size).map(|_| Vec::new()));

		for item in self.items.iter_mut().flat_map(|bucket| bucket.drain(..)) {
			let mut hasher = DefaultHasher::new();
			item.hash(&mut hasher);
			let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
			new_buckets[bucket].push(item);
		}
		replace(&mut self.items, new_buckets);
	}

	pub fn contains<Q>(&self, item: &Q) -> bool 
	where
		K: Borrow<Q>,
		Q: Eq + Hash + ?Sized
	{
		let bucket = self.get_bucket(&item);
		for eitem in self.items[bucket].iter() {
			if eitem.borrow() == item {
				return true
			}
		}
		false
	}

	pub fn remove<Q>(&mut self, item: &Q) -> bool 
	where
		K: Borrow<Q>,
		Q: Eq + Hash + ?Sized
	{
		let bucket = self.get_bucket(&item);
		for (i, eitem) in self.items[bucket].iter().enumerate() {
			if eitem.borrow() == item {
				self.items[bucket].swap_remove(i);
				return true
			}
		}
		false
	}
}
pub struct HashSetIter<'a, K> {
	set: &'a HashSet<K>,
	bucket: usize,
	at: usize,
}

impl<'a, K> HashSetIter<'a, K> {
	fn new(set: &'a HashSet<K>) -> Self {
		HashSetIter {
			set,
			bucket: 0,
			at: 0,
		}
	}
}

impl<'a, K> Iterator for HashSetIter<'a, K> {
	type Item = &'a K;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(item) = self.set.items.get(self.bucket) {
				if let Some(val) = item.get(self.at) {
					self.at += 1;
					// println!("Metrics bucket={}, at={}, val={}", self.bucket, self.at, val);
					return Some(val);
				}
				self.bucket += 1;
				self.at = 0;
				continue;
			}
			return None;
		}
	}
}

impl<'a, K> IntoIterator for &'a HashSet<K> {
	type Item = &'a K;
	type IntoIter = HashSetIter<'a, K>;

	fn into_iter(self) -> Self::IntoIter {
		HashSetIter::new(self)
	}
}

impl<K> FromIterator<K> for HashSet<K>
where
	K: Eq + Hash,
{
	fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
		let mut set = HashSet::new();
		for item in iter.into_iter() {
			set.insert(item);
		}
		set
	}	
}


