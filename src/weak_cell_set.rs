use std::collections::HashSet;
use std::collections::hash_set::Iter;
use rc_cell::{RcCell, WeakCell};

pub struct WeakCellSet<T> {
	set: HashSet<WeakCell<T>>
}

impl<T> WeakCellSet<T> {
	pub fn new() -> WeakCellSet<T> {
		WeakCellSet {
			set: HashSet::new()
		}
	}
	
	pub fn insert(&mut self, value: &RcCell<T>) -> bool {
		self.set.insert(value.downgrade())
	}
	
	pub fn remove(&mut self, value: &RcCell<T>) -> bool {
		self.set.remove(&value.downgrade())
	}
	
	pub fn iter(&self) -> WeakCellSetIterator<T> {
		WeakCellSetIterator {
			iter: self.set.iter()
		}
	}
}

pub struct WeakCellSetIterator<'a, T: 'a> {
	iter: Iter<'a, WeakCell<T>>
}

impl<'a, T> IntoIterator for &'a WeakCellSet<T> {
	type Item = RcCell<T>;
	type IntoIter = WeakCellSetIterator<'a, T>;
	
	fn into_iter(self) -> WeakCellSetIterator<'a, T> {
		self.iter()
	}
}

impl<'a, T> Iterator for WeakCellSetIterator<'a, T> {
	type Item = RcCell<T>;
	
	fn next(&mut self) -> Option<RcCell<T>> {
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
