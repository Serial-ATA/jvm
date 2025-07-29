use crate::classpath::loader::ClassLoader;

use std::sync::atomic::Ordering;

impl ClassLoader {
	pub fn seal(&self) {
		self.sealed.store(true, Ordering::SeqCst);
	}

	pub fn is_sealed(&self) -> bool {
		self.sealed.load(Ordering::SeqCst)
	}

	pub fn assert_not_sealed(&self) {
		if self.is_sealed() {
			panic!("Attempting to load additional classes on a sealed ClassLoader!")
		}
	}
}
