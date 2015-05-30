use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub struct RcCell<T> {
	value: Rc<RefCell<T>>
}

impl<T> RcCell<T> {
	pub fn new(v: T) -> RcCell<T> {
		RcCell { value: Rc::new(RefCell::new(v)) }
	}
	
	pub fn from_rc(r: Rc<RefCell<T>>) -> RcCell<T> {
		RcCell { value: r }
	}
	
	pub fn downgrade(&self) -> WeakCell<T> {
		WeakCell::from_weak(self.value.downgrade())
	}
	
	fn ptr(&self) -> *const RefCell<T> {
		self.value.deref()
	}
	
	pub fn get<'a>(&'a self) -> RefMut<'a, T> {
		self.value.borrow_mut()
	}
}

impl<T> Clone for RcCell<T> {
	fn clone(&self) -> RcCell<T> {
		RcCell::from_rc(self.value.clone())
	}
}

impl<T> Deref for RcCell<T> {
	type Target = RefCell<T>;
	
	fn deref(&self) -> &RefCell<T> {
  	self.value.deref()
	}
}

impl<T> Hash for RcCell<T> {
	fn hash<H>(&self, state: &mut H) where H: Hasher {
		self.ptr().hash(state)
	}
}

impl<T> PartialEq for RcCell<T> {
	fn eq(&self, other: &RcCell<T>) -> bool {
		self.ptr() == other.ptr()
	}
}

impl<T> Eq for RcCell<T> {}

pub struct WeakCell<T> {
	value: Weak<RefCell<T>>
}

impl<T> WeakCell<T> {
	pub fn from_weak(r: Weak<RefCell<T>>) -> WeakCell<T> {
		WeakCell { value: r }
	}
	
	pub fn upgrade(&self) -> Option<RcCell<T>> {
		self.value.upgrade().map(RcCell::from_rc)
	}
}

impl<T> Hash for WeakCell<T> {
	fn hash<H>(&self, state: &mut H) where H: Hasher {
		self.upgrade().hash(state)
	}
}

impl<T> PartialEq for WeakCell<T> {
	fn eq(&self, other: &WeakCell<T>) -> bool {
		self.upgrade() == other.upgrade()
	}
}

impl<T> Eq for WeakCell<T> {}
