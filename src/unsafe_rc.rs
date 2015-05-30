use std::cell::UnsafeCell;
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};

pub struct UnsafeRc<T> {
	value: Rc<UnsafeCell<T>>
}

impl<T> Hash for UnsafeRc<T> {
	fn hash<H>(&self, state: &mut H) where H: Hasher {
		self.ptr().hash(state)
	}
}

impl<T> PartialEq for UnsafeRc<T> {
	fn eq(&self, other: &UnsafeRc<T>) -> bool {
		self.ptr() == other.ptr()
	}
}

impl<T> Eq for UnsafeRc<T> {}

impl<T> Clone for UnsafeRc<T> {
	fn clone(&self) -> UnsafeRc<T> {
		UnsafeRc::from_rc(self.value.clone())
	}
}

impl<T> Deref for UnsafeRc<T> {
	type Target = T;
	
	fn deref<'a>(&'a self) -> &'a T {
		unsafe {
			self.value.get().as_ref().unwrap()
		}
	}
}

impl<T> DerefMut for UnsafeRc<T> {
	fn deref_mut<'a>(&'a mut self) -> &'a mut T {
		unsafe {
			self.value.get().as_mut::<'a>().unwrap()
		}
	}
}

impl<T> UnsafeRc<T> {
	pub fn new(v: T) -> UnsafeRc<T> {
		UnsafeRc::<T> {
			value: Rc::new(UnsafeCell::new(v))
		}
	}
	
	pub fn from_rc(v: Rc<UnsafeCell<T>>) -> UnsafeRc<T> {
		UnsafeRc::<T> {
			value: v
		}
	}
  
  pub fn downgrade(&self) -> UnsafeWeak<T> {
  	UnsafeWeak::from_weak(self.value.downgrade())
	}
  
  pub fn ptr(&self) -> *const T {
  	self.value.get()
  }
}

pub struct UnsafeWeak<T> {
	value: Weak<UnsafeCell<T>>
}

impl<T> UnsafeWeak<T> {
	pub fn from_weak(r: Weak<UnsafeCell<T>>) -> UnsafeWeak<T> {
		UnsafeWeak {
			value: r
		}
	}
	
	pub fn upgrade(&self) -> Option<UnsafeRc<T>> {
		self.value.upgrade().map(UnsafeRc::from_rc)
	}
}

impl<T> Hash for UnsafeWeak<T> {
	fn hash<H>(&self, state: &mut H) where H: Hasher {
		self.upgrade().hash(state)
	}
}

impl<T> PartialEq for UnsafeWeak<T> {
	fn eq(&self, other: &UnsafeWeak<T>) -> bool {
		self.upgrade() == other.upgrade()
	}
}

impl<T> Eq for UnsafeWeak<T> {}
