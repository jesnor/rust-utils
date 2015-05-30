use std::collections::HashSet;
use std::collections::hash_set::Iter;
use unsafe_rc::{UnsafeWeak, UnsafeRc};

pub struct UnsafeWeakSet<T> {
	set: HashSet<UnsafeWeak<T>>
}

impl<T> UnsafeWeakSet<T> {
	pub fn new() -> UnsafeWeakSet<T> {
		UnsafeWeakSet {
			set: HashSet::new()
		}
	}
	
	pub fn insert(&mut self, value: &UnsafeRc<T>) -> bool {
		self.set.insert(value.downgrade())
	}
	
	pub fn remove(&mut self, value: &UnsafeRc<T>) -> bool {
		self.set.remove(&value.downgrade())
	}
	
	pub fn iter(&self) -> UnsafeWeakSetIterator<T> {
		UnsafeWeakSetIterator {
			iter: self.set.iter()
		}
	}
}

pub struct UnsafeWeakSetIterator<'a, T: 'a> {
	iter: Iter<'a, UnsafeWeak<T>>
}

impl<'a, T> IntoIterator for &'a UnsafeWeakSet<T> {
	type Item = UnsafeRc<T>;
	type IntoIter = UnsafeWeakSetIterator<'a, T>;
	
	fn into_iter(self) -> UnsafeWeakSetIterator<'a, T> {
		self.iter()
	}
}

impl<'a, T> Iterator for UnsafeWeakSetIterator<'a, T> {
	type Item = UnsafeRc<T>;
	
	fn next(&mut self) -> Option<UnsafeRc<T>> {
		loop {
			if let Some(r) = self.iter.next() {
				if let Some(sr) = r.upgrade() {
					return Some(sr)
				}
			}
			else {
				return None
			}
		}
	}
}
