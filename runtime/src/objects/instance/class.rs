use crate::objects::class::ClassPtr;
use crate::objects::instance::object::Object;
use crate::objects::instance::{CloneableInstance, Header, Instance};
use crate::thread::JavaThread;

use jni::sys::jint;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Reference to an allocated class instance
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ClassInstanceRef(*mut ClassInstance);

// SAFETY: Synchronization handled manually
unsafe impl Send for ClassInstanceRef {}

unsafe impl Sync for ClassInstanceRef {}

impl ClassInstanceRef {
	const FIELD_BASE: usize = size_of::<ClassInstance>();
}

impl Object for ClassInstanceRef {
	type Descriptor = ClassInstance;

	fn hash(&self, thread: &'static JavaThread) -> jint {
		self.header.generate_hash(thread)
	}

	fn class(&self) -> ClassPtr {
		unsafe { (&*self.0).class }
	}

	fn is_class(&self) -> bool {
		true
	}

	unsafe fn raw(&self) -> *mut () {
		self.0.cast()
	}

	unsafe fn field_base(&self) -> *mut u8 {
		let base = self.0.cast::<u8>();
		unsafe { base.add(Self::FIELD_BASE) }
	}
}

impl PartialEq for ClassInstanceRef {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Deref for ClassInstanceRef {
	type Target = ClassInstance;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

impl Debug for ClassInstanceRef {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}

#[repr(C)]
pub struct ClassInstance {
	header: Header,
	class: ClassPtr,
}

impl Debug for ClassInstance {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClassInstance")
			.field("header", &self.header)
			.field("class", &self.class)
			.finish()
	}
}

impl ClassInstance {
	pub fn new(class: ClassPtr) -> ClassInstanceRef {
		let descriptor = ClassInstance {
			header: Header::new(),
			class,
		};

		let fields_size = class.size_of_instance_fields();
		let instance_ptr = unsafe { ClassInstanceRef::allocate(descriptor, fields_size) };
		ClassInstanceRef(instance_ptr)
	}

	pub fn is_subclass_of(&self, class: ClassPtr) -> bool {
		self.class.is_subclass_of(class)
	}

	pub fn implements(&self, class: ClassPtr) -> bool {
		self.class.implements(class)
	}
}

impl CloneableInstance for ClassInstanceRef {
	type ReferenceTy = ClassInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		let cloned_instance = ClassInstance::new(self.class);
		let fields_size = self.class.size_of_instance_fields();

		// SAFETY: Every field type is `Copy`
		unsafe {
			self.field_base()
				.copy_to_nonoverlapping(cloned_instance.field_base() as _, fields_size);
		}

		cloned_instance
	}
}

impl Instance for ClassInstanceRef {}
