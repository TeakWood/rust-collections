use std::collections::hash_map::{DefaultHasher};
use std::hash::{Hash, Hasher};
use std::mem::replace;
use std::borrow::Borrow;

const INITIAL_BUCKETS: usize = 1;
pub struct HashMap<K, V> {
	buckets: Vec<Vec<(K, V)>>,
	items: usize,
}

pub struct OccupiedEntry<'a, K, V> {
	element: &'a mut (K, V),
}

pub struct VacantEntry<'a, K, V> {
	key: K,
	bucket: &'a mut Vec<(K, V)>,
}
impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
	pub fn insert(self, val: V) -> &'a mut V {
		self.bucket.push((self.key, val));
		&mut self.bucket.last_mut().unwrap().1
	}
}

pub enum Entry<'a, K, V> {
	Occupied(OccupiedEntry<'a, K, V>),
	Vacant(VacantEntry<'a, K, V>),
}
impl<'a, K, V> Entry<'a, K, V> {
	pub fn or_insert(self, value: V) -> &'a mut V {
		match self {
			Entry::Occupied(e) => &mut e.element.1,
			Entry::Vacant(e) => e.insert(value),
		}
	}

	pub fn or_insert_with<F>(self, maker: F) -> &'a mut V 
	where
		F: FnOnce() -> V
	{
		match self {
			Entry::Occupied(e) => &mut e.element.1,
			Entry::Vacant(e) => e.insert(maker()),
		}
	}
}


impl<K, V> HashMap<K, V> {
	pub fn new() -> Self {
		HashMap {
			buckets: Vec::new(),
			items: 0,
		}
	}

	pub fn len(&self) -> usize {
		self.items
	}

	pub fn is_empty(&self) -> bool {
		self.items == 0
	}
	
}

impl<K, V> HashMap<K, V>
where
	K: Eq + Hash,
{

	fn get_bucket<Q>(&self, key: &Q) -> usize 
	where
		K: Borrow<Q>,
		Q: Hash + Eq + ?Sized
	{
		let mut hasher = DefaultHasher::new();
		key.hash(&mut hasher);
		(hasher.finish() % self.buckets.len() as u64) as usize
	}

	pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V> {
		if self.buckets.is_empty() {
			self.resize();
		}

		let bucket = self.get_bucket(&key);
		for entry in &mut self.buckets[bucket] {
			if entry.0 == key {
				return Entry::Occupied(OccupiedEntry { 
					element: unsafe { &mut *(entry as *mut _)}
				})
			}
		};
		Entry::Vacant(VacantEntry { 
			key,
			bucket: &mut self.buckets[bucket],
		})
	}

	pub fn insert(&mut self, key: K, val: V) -> Option<V> {
		if self.buckets.is_empty() || self.items > 3* self.buckets.len() {
			self.resize();
		}

		let bucket = self.get_bucket(&key);
		let bucket = &mut self.buckets[bucket];

		for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
			if *ekey == key {
				use std::mem;
				return Some(mem::replace(evalue, val));
			}
		}
		bucket.push((key, val));
		self.items += 1;
		None
	}

	pub fn get<Q>(&self, key: &Q) -> Option<&V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq + ?Sized
	{
		let bucket = self.get_bucket(&key);
		self.buckets[bucket]
			.iter()
			.find(|&(ref ekey, _)| ekey.borrow() == key)
			.map(|&(_, ref evalue)| evalue)
	}

	pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
	where
		K: Borrow<Q>,
		Q: Hash + Eq + ?Sized
	{
		let bucket = self.get_bucket(key);
		let i = self.buckets[bucket].iter().position(|&(ref ekey, _)| ekey.borrow() == key)?;
		self.items -= 1;
		Some(self.buckets[bucket].swap_remove(i).1)
	}

	pub fn contains_key<Q>(&self, key: &Q) -> bool 
	where
		K: Borrow<Q>,
		Q: Hash + Eq + ?Sized
	{
		self.get(key).is_some()
	}

	fn resize(&mut self) {
		//increase size of the buckets but maintain hashing
		let target_size = match self.buckets.len() {
			0 => INITIAL_BUCKETS,
			n => 2*n,
		};
		let mut new_buckets = Vec::with_capacity(target_size);
		new_buckets.extend((0..target_size).map(|_| Vec::new()));

		for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
			let mut hasher = DefaultHasher::new();
			key.hash(&mut hasher);
			let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
			new_buckets[bucket].push((key, value));
		}
		replace(&mut self.buckets, new_buckets);
	}
}

pub struct HashMapIter<'a, K, V> {
	map: &'a HashMap<K, V>,
	bucket: usize,
	at: usize,
}

impl<'a, K, V> HashMapIter<'a, K, V> {
	fn new(map: &'a HashMap<K, V>) -> Self {
		HashMapIter {
			map,
			bucket: 0,
			at: 0, 
		}
	}
}
impl<'a, K, V> Iterator for HashMapIter<'a, K, V> {
	type Item = (&'a K, &'a V);
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.map.buckets.get(self.bucket) {
				Some(bucket) => {
					match bucket.get(self.at) {
						Some(&(ref k, ref v)) => {
							self.at += 1;
							break Some((k, v))
						}
						None => {
							self.bucket += 1;
							self.at = 0;
							continue;
						}
					}
				},
				None => break None,
			}
		}
	}
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
	type Item = (&'a K, & 'a V);
	type IntoIter = HashMapIter<'a, K, V>;

	fn into_iter(self) -> Self::IntoIter {
		HashMapIter::new(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn insert() {
		let mut map = HashMap::new();
		assert_eq!(map.len(), 0);
		assert!(map.is_empty());

		map.insert("foo", 42);
		assert_eq!(map.len(), 1);
		assert!(!map.is_empty());
		assert_eq!(map.get(&"foo"), Some(&42));
		assert_eq!(map.remove(&"foo"), Some(42));
		assert_eq!(map.get(&"foo"), None);
	}

	#[test]
	fn iter() {
		let mut map = HashMap::new();
		map.insert("foo", 42);
		map.insert("bar", 43);
		map.insert("baz", 142);
		map.insert("quox", 6);
		for (&k, &v) in &map {
			match k {
				"foo" =>  assert_eq!(v, 42),
				"bar" =>  assert_eq!(v, 43),
				"baz" =>  assert_eq!(v, 142),
				"quox" =>  assert_eq!(v, 6),
				_ => unreachable!(),
			}
		}
		assert_eq!((&map).into_iter().count(), 4);
	}
}
