#![feature(core)]
#![feature(alloc)]

use std::cell::UnsafeCell;
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::hash::{Hash, Hasher};

pub struct RcCell<T> {
  value: Rc<UnsafeCell<T>>
}

impl<T> Hash for RcCell<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    self.value.get().hash(state)
  }
}

impl<T> PartialEq for RcCell<T> {
  fn eq(&self, other: &RcCell<T>) -> bool {
    self.value.get() == other.value.get()
  }
}

impl<T> Eq for RcCell<T> {}

impl<T> Clone for RcCell<T> {
  fn clone(&self) -> RcCell<T> {
    RcCell::from_rc(self.value.clone())
  }
}

impl<T> Deref for RcCell<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe {
            self.value.get().as_ref::<'a>().unwrap()
        }
    }
}

impl<T> DerefMut for RcCell<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe {
            self.value.get().as_mut::<'a>().unwrap()
        }
    }
}

impl<T> RcCell<T> {
  pub fn new(v: T) -> RcCell<T> {
    RcCell::<T> {
      value: Rc::new(UnsafeCell::new(v))
    }
  }
  
  pub fn from_rc(v: Rc<UnsafeCell<T>>) -> RcCell<T> {
    RcCell::<T> {
      value: v
    }
  }
  
  pub fn weak(&self) -> WeakRcCell<T> {
    WeakRcCell::from_weak(self.value.downgrade())
  }
}

pub struct WeakRcCell<T> {
  value: Weak<UnsafeCell<T>>
}

impl<T> WeakRcCell<T> {
  pub fn from_weak(r: Weak<UnsafeCell<T>>) -> WeakRcCell<T> {
    WeakRcCell::<T> {
      value: r
    }
  }

  pub fn strong(&self) -> Option<RcCell<T>> {
    self.value.upgrade().map(|r| RcCell::from_rc(r))
  }
}

impl<T> Hash for WeakRcCell<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    self.strong().hash(state) 
  }
}

impl<T> PartialEq for WeakRcCell<T> {
  fn eq(&self, other: &WeakRcCell<T>) -> bool {
    self.strong() == other.strong()
  }
}

impl<T> Eq for WeakRcCell<T> {}

pub struct WeakSet<T> {
  set: HashSet<WeakRcCell<T>>
}

impl<T> WeakSet<T> {
  pub fn new() -> WeakSet<T> {
    WeakSet {
      set: HashSet::new()
    }
  }
  
  pub fn insert(&mut self, value: &RcCell<T>) -> bool {
    self.set.insert(value.weak())
  }

  pub fn remove(&mut self, value: &RcCell<T>) -> bool {
    self.set.remove(&value.weak())
  }
  
  pub fn iter(&self) -> WeakSetIterator<T> {
    WeakSetIterator {
      iter: self.set.iter()
    }
  }
}

pub struct WeakSetIterator<'a, T: 'a> {
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
