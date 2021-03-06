#![feature(core)]
#![feature(alloc)]

pub mod unsafe_rc;
pub mod unsafe_weak_set;
pub mod rc_cell;
pub mod id_rc;
pub mod id_weak_set;
pub mod weak_cell_set;

pub use unsafe_rc::{UnsafeRc, UnsafeWeak};
pub use rc_cell::{RcCell, WeakCell};
pub use id_rc::{IdRc, IdWeak};
pub use unsafe_weak_set::UnsafeWeakSet;
pub use weak_cell_set::WeakCellSet;
pub use id_weak_set::IdWeakSet;
