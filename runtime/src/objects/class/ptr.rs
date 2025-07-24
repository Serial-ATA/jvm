use super::{Class, ClassType};
use crate::classpath::loader::ClassLoader;
use crate::globals::classes;
use crate::objects::constant_pool::{ResolvedEntry, cp_types};
use crate::objects::instance::mirror::{MirrorInstance, MirrorInstanceRef};
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;

use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::Deref;

/// A pointer to a heap-allocated [`Class`]
///
/// For all intents and purposes, this can be considered identical to `&'static Class`.
///
/// Since every [`Class`] is 256-byte aligned, the low 8 bits of every pointer can be used for
/// [metadata].
///
/// [metadata]: ClassPtr::metadata()
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClassPtr(*mut Class);

// SAFETY: Synchronization handled manually
unsafe impl Send for ClassPtr {}
unsafe impl Sync for ClassPtr {}

impl ClassPtr {
	const METADATA_MASK: usize = 0xFF;
	const POINTER_MASK: usize = !Self::METADATA_MASK;

	pub(super) fn new(class: Class) -> Self {
		let class: &'static mut Class = Box::leak(Box::new(class));
		Self(class as *mut Class)
	}

	/// Raw access to the [`Class`], to be used sparingly
	pub(super) fn raw(self) -> *mut Class {
		self.0
	}

	pub(super) fn as_static_ref(self) -> &'static Class {
		debug_assert!(!self.0.is_null());

		// SAFETY: `ClassPtr` can only ever be constructed with a valid `Class`, never null
		unsafe { self.0.as_ref_unchecked::<'static>() }
	}

	pub fn metadata(self) -> u8 {
		((self.0 as usize) & Self::METADATA_MASK) as u8
	}

	pub fn set_metadata(self, metadata: u8) -> Self {
		let new = self.0 as usize | metadata as usize;
		Self(new as *mut Class)
	}
}

// Dangerous mutation methods
impl ClassPtr {
	/// Set the mirror for this class
	///
	/// This optionally takes an existing mirror, otherwise it will create a new one.
	///
	/// # Safety
	///
	/// This is only safe to call *before* the class is in use. It should never be used outside of
	/// class loading.
	pub unsafe fn set_mirror(self, mirror: Option<MirrorInstanceRef>) {
		let final_mirror = match mirror {
			Some(mirror) => mirror,
			None => match self.class_ty() {
				ClassType::Instance(_) => {
					let mirror = MirrorInstance::new(self);
					mirror.set_module(self.module().obj());
					mirror
				},
				ClassType::Array(_) => {
					let mirror = MirrorInstance::new_array(self);
					let bootstrap_loader = ClassLoader::bootstrap();
					mirror.set_module(bootstrap_loader.java_base().obj());
					mirror
				},
			},
		};

		unsafe {
			*self.mirror.get() = MaybeUninit::new(final_mirror);
		}
	}

	/// Used for hidden classes
	pub(super) fn mangle_name(self) {
		let ptr = self.raw() as usize;
		let new_name_str = format!("{}+{ptr}", self.name());
		let new_name = Symbol::intern(&new_name_str);
		unsafe {
			*self.name.get() = new_name;
		}

		let cp = self
			.constant_pool()
			.expect("only used for non-array classes");

		// SAFETY: The `class_name_index` is known to be correct, since the original name was derived
		//         from it in `Class::new()`.
		unsafe {
			let class_name_index = (*self.misc_cache.get()).class_name_index;
			cp.overwrite::<cp_types::Class>(class_name_index, ResolvedEntry { class: self });
		}
	}
}

// Nest methods, since they require setting a `ClassPtr` stored in the `Class` itself
impl ClassPtr {
	/// Fetch/resolve the nest host of this class
	///
	/// # Exceptions
	///
	/// * Unable to resolve the nest host from the constant pool
	/// * The nest host is invalid
	///   * Not in the same package as `self`
	///   * `self` isn't a member of the host
	pub fn nest_host(self, thread: &'static JavaThread) -> Throws<ClassPtr> {
		if let Some(nest_host) = unsafe { (*self.misc_cache.get()).nest_host } {
			return Throws::Ok(nest_host);
		}

		let index;
		unsafe {
			// In the case that the class has no nest host, we can just set it to itself
			let Some(nest_host_index) = (*self.misc_cache.get()).nest_host_index else {
				(*self.misc_cache.get()).nest_host = Some(self);
				return Throws::Ok(self);
			};

			index = nest_host_index;
		}

		let nest_host_class = self
			.constant_pool()
			.expect("not called on array classes")
			.get::<cp_types::Class>(index);

		match nest_host_class {
			Throws::Ok(class) => {
				let mut error = None;
				if !self.shares_package_with(class) {
					error = Some("types are in different packages");
					todo!();
				}

				if !class.has_nest_member(self) {
					error = Some("current type is not listed as nest member");
					todo!();
				}

				// Nest host resolved
				if error.is_none() {
					unsafe {
						(*self.misc_cache.get()).nest_host = Some(class);
					}
					return Throws::Ok(class);
				}
			},
			Throws::Exception(e) => {
				if e.kind().class() == classes::java_lang_VirtualMachineError() {
					return Throws::PENDING_EXCEPTION;
				}

				todo!("print nest host error")
			},
		}

		// If all else fails, set to self
		unsafe {
			(*self.misc_cache.get()).nest_host = Some(self);
		}
		Throws::Ok(self)
	}

	/// If `self` is a nest host, check if `other` is a member of the nest
	pub fn has_nest_member(self, other: Self) -> bool {
		let Some(nest_members) = &self.nest_members else {
			return false;
		};

		nest_members.iter().any(|name| *name == other.name())
	}

	/// Whether `self` is a nestmate of `other`, meaning they are under the same nest host
	///
	/// # Exceptions
	///
	/// See [`Self::nest_host()`]
	pub fn is_nestmate_of(&self, other: ClassPtr, thread: &'static JavaThread) -> Throws<bool> {
		let self_host = self.nest_host(thread)?;
		let other_host = other.nest_host(thread)?;
		Throws::Ok(self_host == other_host)
	}
}

impl Deref for ClassPtr {
	type Target = Class;

	fn deref(&self) -> &Self::Target {
		debug_assert!(!self.0.is_null());

		let addr = self.0 as usize & Self::POINTER_MASK;
		unsafe { &*(addr as *const Class) }
	}
}

impl Debug for ClassPtr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}

impl PartialEq for ClassPtr {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl PartialEq<Class> for ClassPtr {
	fn eq(&self, other: &Class) -> bool {
		self.deref() == other
	}
}

impl PartialEq<&Class> for ClassPtr {
	fn eq(&self, other: &&Class) -> bool {
		self.deref() == *other
	}
}

impl PartialEq<ClassPtr> for Class {
	fn eq(&self, other: &ClassPtr) -> bool {
		self == other.deref()
	}
}

impl PartialEq<ClassPtr> for &'_ Class {
	fn eq(&self, other: &ClassPtr) -> bool {
		*self == other.deref()
	}
}
