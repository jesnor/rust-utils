#![feature(core)]
#![feature(alloc)]

use std::cell::UnsafeCell;
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::hash::{Hash, Hasher};

struct RcCell<T> {
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
  fn new(v: T) -> RcCell<T> {
    RcCell::<T> {
      value: Rc::new(UnsafeCell::new(v))
    }
  }

  fn from_rc(v: Rc<UnsafeCell<T>>) -> RcCell<T> {
    RcCell::<T> {
      value: v
    }
  }

  fn weak(&self) -> WeakRcCell<T> {
    WeakRcCell::new(self.value.downgrade())
  }
}

struct WeakRcCell<T> {
  value: Weak<UnsafeCell<T>>
}

impl<T> WeakRcCell<T> {
  fn new(r: Weak<UnsafeCell<T>>) -> WeakRcCell<T> {
    WeakRcCell::<T> {
      value: r
    }
  }

  fn strong(&self) -> Option<RcCell<T>> {
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
