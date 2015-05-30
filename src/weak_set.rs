struct WeakSet<T> {
  set: HashSet<WeakRcCell<T>>
}

impl<T> WeakSet<T> {
  fn new() -> WeakSet<T> {
    WeakSet {
      set: HashSet::new()
    }
  }

  fn insert(&mut self, value: &RcCell<T>) -> bool {
    self.set.insert(value.weak())
  }

  fn remove(&mut self, value: &RcCell<T>) -> bool {
    self.set.remove(&value.weak())
  }

  fn iter(&self) -> WeakSetIterator<T> {
    WeakSetIterator {
      iter: self.set.iter()
    }
  }
}

struct WeakSetIterator<'a, T: 'a> {
  iter: Iter<'a, WeakRcCell<T>>
}

impl<'a, T> IntoIterator for &'a WeakSet<T> {
  type Item = RcCell<T>;
  type IntoIter = WeakSetIterator<'a, T>;

  fn into_iter(self) -> WeakSetIterator<'a, T> {
    self.iter()
  }
}

impl<'a, T> Iterator for WeakSetIterator<'a, T> {
  type Item = RcCell<T>;

  fn next(&mut self) -> Option<RcCell<T>> {
    loop {
      if let Some(r) = self.iter.next() {
        if let Some(sr) = r.strong() {
          return Some(sr)
        }
      }
      else {
        return None
      }
    }
  }
}

