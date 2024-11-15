use super::field::Field;
use super::method::Method;
use super::mirror::MirrorInstance;
use super::reference::{ClassRef, FieldRef};
use super::spec::class::{ClassInitializationState, InitializationLock};
use super::vtable::VTable;
use crate::classpath::classloader::ClassLoader;
use crate::reference::{MirrorInstanceRef, Reference};
use crate::JavaThread;

use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use classfile::accessflags::ClassAccessFlags;
use classfile::{ClassFile, ConstantPool, ConstantPoolRef, FieldType, MethodInfo};
use common::box_slice;
use common::int_types::{u1, u2, u4};
use common::traits::PtrType;
use instructions::Operand;
use symbols::{sym, Symbol};

struct FieldContainer {
	/// A list of all fields, including static
	fields: Vec<FieldRef>,
	/// All static field slots
	///
	/// This needs to be scaled to the `fields` field, in that index 0 of this array relates
	/// to the index of the first static field in `fields`.
	static_field_slots: Box<[Operand<Reference>]>,
	/// The number of dynamic fields in a class instance
	///
	/// This is essentially `fields.len() - static_field_slots.len()`, provided here for convenience.
	instance_field_count: u4,
}

impl FieldContainer {
	/// Used as the field container for arrays, as they have no instance fields.
	fn null() -> Self {
		Self {
			fields: Vec::new(),
			static_field_slots: box_slice![],
			instance_field_count: 0,
		}
	}
}

// TODO: Make more fields private
pub struct Class {
	pub name: Symbol,
	pub access_flags: ClassAccessFlags,
	pub loader: ClassLoader,
	pub super_class: Option<ClassRef>,
	pub interfaces: Vec<ClassRef>,
	mirror: MaybeUninit<MirrorInstanceRef>,
	field_container: FieldContainer,
	vtable: MaybeUninit<VTable<'static>>,

	pub(super) init_thread: Option<JavaThread>,
	pub(super) class_ty: ClassType,

	#[doc(hidden)]
	pub(super) init_state: ClassInitializationState,
	#[doc(hidden)]
	pub(super) init_lock: Arc<InitializationLock>,
}

impl Debug for Class {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Class")
			.field("name", &self.name.as_str())
			.field("access_flags", &self.access_flags)
			.field("loader", &self.loader)
			.field("super_class", &self.super_class)
			.field("interfaces", &self.interfaces)
			.field("vtable", &self.vtable)
			.field("instance", &self.class_ty)
			.finish()
	}
}

#[derive(Debug, Clone)]
pub enum ClassType {
	Instance(ClassDescriptor),
	Array(ArrayDescriptor),
}

#[derive(Clone)]
pub struct ClassDescriptor {
	pub source_file_index: Option<u2>,
	pub constant_pool: ConstantPoolRef,
}

impl Debug for ClassDescriptor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut debug_struct = f.debug_struct("ClassDescriptor");

		match self.source_file_index {
			Some(idx) => debug_struct.field("source_file", &unsafe {
				std::str::from_utf8_unchecked(&self.constant_pool.get_constant_utf8(idx))
			}),
			None => debug_struct.field("source_file", &"None"),
		};

		debug_struct.finish()
	}
}

#[derive(Debug, Clone)]
pub struct ArrayDescriptor {
	pub dimensions: u1,
	pub component: FieldType,
}

// Getters
impl Class {
	/// Get a reference to the constant pool for this class
	///
	/// This returns an `Option`, as array classes do not have an associated constant pool. It is
	/// guaranteed to be present otherwise.
	pub fn constant_pool(&self) -> Option<&ConstantPool> {
		match &self.class_ty {
			ClassType::Instance(instance) => Some(&instance.constant_pool),
			_ => None,
		}
	}

	/// Get the `VTable` for this class
	///
	/// This is the only way to access the class methods externally.
	#[inline]
	pub fn vtable(&self) -> &VTable<'static> {
		// SAFETY: The only way to construct a `Class` is via `Class::new()`, which ensures that the
		//         vtable is initialized.
		unsafe { self.vtable.assume_init_ref() }
	}

	/// Get the fields for this class
	///
	/// NOTE: This includes the static fields as well.
	///
	/// This is the only way to access the class fields externally.
	pub fn fields(&self) -> impl Iterator<Item = &FieldRef> {
		self.field_container.fields.iter()
	}

	/// Get the static fields for this class
	pub fn static_fields(&self) -> impl Iterator<Item = &FieldRef> {
		self.fields().filter(|field| field.is_static())
	}

	/// Get the value of the static field at `index`
	///
	/// # Panics
	///
	/// This will panic if the index is out of bounds.
	pub fn static_field_value(&self, index: usize) -> Operand<Reference> {
		self.field_container.static_field_slots[index].clone()
	}

	/// The number of non-static fields
	pub fn instance_field_count(&self) -> usize {
		self.field_container.instance_field_count as usize
	}

	/// Get the mirror for this class
	///
	/// See [`MirrorInstance`] for information on mirrors.
	pub fn mirror(&self) -> MirrorInstanceRef {
		// SAFETY: The mirror is only uninitialized for a few classes few early in VM initialization
		//         due to them loading *before* `java.lang.Class`. Afterwards, all classes are
		//         guaranteed to have mirrors.
		let mirror = unsafe { self.mirror.assume_init_ref() };
		Arc::clone(mirror)
	}
}

// Setters
impl Class {
	/// Set the value of the static field at `index`
	///
	/// # Panics
	///
	/// This will panic if the index is out of bounds.
	pub fn set_static_field(&mut self, index: usize, value: Operand<Reference>) {
		self.field_container.static_field_slots[index] = value;
	}
}

// Flags
// TODO: Cache lookups
impl Class {
	/// Whether the class is cloneable
	///
	/// NOTES:
	///
	/// * This is always true for arrays of any type
	/// * This is only true for classes that implement the `java.lang.Cloneable`
	pub fn is_cloneable(&self) -> bool {
		self.is_array() || self.implements(crate::globals::classes::java_lang_Cloneable())
	}
}

impl Class {
	/// Create a new class
	///
	/// # Safety
	///
	/// This should never be used outside of the ClassLoader. The resulting [`ClassRef`] needs to
	/// be handled properly, as some fields remain uninitialized.
	pub unsafe fn new(
		parsed_file: ClassFile,
		super_class: Option<ClassRef>,
		loader: ClassLoader,
	) -> ClassRef {
		let access_flags = parsed_file.access_flags;
		let class_name_index = parsed_file.this_class;

		let source_file_index = parsed_file.source_file_index();

		let constant_pool = parsed_file.constant_pool;

		let name_raw = constant_pool.get_class_name(class_name_index);
		let name = Symbol::intern_bytes(name_raw);

		let static_field_count = parsed_file
			.fields
			.iter()
			.filter(|field| field.access_flags.is_static())
			.count();
		let mut instance_field_count = 0;

		if let Some(ref super_class) = super_class {
			instance_field_count = super_class.field_container.instance_field_count;
		}

		let interfaces = parsed_file
			.interfaces
			.iter()
			.map(|index| {
				loader
					.load(Symbol::intern_bytes(constant_pool.get_class_name(*index)))
					.unwrap()
			})
			.collect();

		let static_field_slots = box_slice![Operand::Empty; static_field_count];

		// We need the Class instance to create our methods and fields
		let class_instance = ClassDescriptor {
			source_file_index,
			constant_pool,
		};

		let fields = FieldContainer {
			fields: Vec::new(),
			static_field_slots,
			instance_field_count,
		};

		let class = Self {
			name,
			access_flags,
			loader,
			super_class,
			interfaces,
			init_thread: None,             // Set later
			mirror: MaybeUninit::uninit(), // Set later
			field_container: fields,
			vtable: MaybeUninit::uninit(), // Set later
			class_ty: ClassType::Instance(class_instance),
			init_state: ClassInitializationState::default(),
			init_lock: Arc::default(),
		};

		let classref = ClassPtr::new(class);

		// Create our vtable...
		let vtable = new_vtable(Some(&parsed_file.methods), Arc::clone(&classref));

		let class = classref.get_mut();
		class.vtable = MaybeUninit::new(vtable);

		// Then the fields...
		let mut fields =
			Vec::with_capacity(instance_field_count as usize + parsed_file.fields.len());
		if let Some(ref super_class) = class.super_class {
			// First we have to inherit the super classes' fields
			for field in super_class.fields() {
				if !field.is_static() {
					fields.push(Arc::clone(field));
				}
			}
		}

		// Now the fields defined in our class
		let mut static_idx = 0;
		// Continue the index from our existing instance fields
		let mut instance_field_idx = core::cmp::max(0, instance_field_count) as usize;
		let constant_pool = class
			.constant_pool()
			.expect("we just set the constant pool");
		for field in parsed_file.fields {
			let field_idx = if field.access_flags.is_static() {
				&mut static_idx
			} else {
				&mut instance_field_idx
			};

			fields.push(Field::new(
				*field_idx,
				Arc::clone(&classref),
				&field,
				constant_pool,
			));

			*field_idx += 1;
		}
		class.field_container.fields = fields;

		// Update the instance field count if we encountered any new ones
		if instance_field_idx > 0 {
			if instance_field_count > 0 {
				class.field_container.instance_field_count +=
					(instance_field_idx as u4) - instance_field_count;
			} else {
				class.field_container.instance_field_count = instance_field_idx as u4;
			}
		}

		classref
	}

	/// Create a new array class of type `component`
	///
	/// # Safety
	///
	/// This should never be used outside of the ClassLoader. The resulting [`ClassRef`] needs to
	/// be handled properly, as some fields remain uninitialized.
	pub unsafe fn new_array(name: Symbol, component: FieldType, loader: ClassLoader) -> ClassRef {
		let dimensions = name
			.as_str()
			.chars()
			.take_while(|char_| *char_ == '[')
			.count() as u1;

		let array_instance = ArrayDescriptor {
			dimensions,
			component,
		};

		let class = Self {
			name,
			access_flags: ClassAccessFlags::NONE,
			loader,
			super_class: Some(crate::globals::classes::java_lang_Object()),
			// https://docs.oracle.com/javase/specs/jls/se19/html/jls-4.html#jls-4.10.3
			interfaces: vec![
				ClassLoader::Bootstrap
					.load(sym!(java_lang_Cloneable))
					.unwrap(),
				ClassLoader::Bootstrap
					.load(sym!(java_io_Serializable))
					.unwrap(),
			],
			init_thread: None,             // Set later
			mirror: MaybeUninit::uninit(), // Set later
			field_container: FieldContainer::null(),
			vtable: MaybeUninit::uninit(), // Set later
			class_ty: ClassType::Array(array_instance),
			init_state: ClassInitializationState::default(),
			init_lock: Arc::default(),
		};

		let classref = ClassPtr::new(class);

		// Create a vtable, inheriting from `java.lang.Object`
		let vtable = new_vtable(None, Arc::clone(&classref));

		let class = classref.get_mut();
		class.vtable = MaybeUninit::new(vtable);

		classref
	}

	pub fn set_initialization_state(&mut self, state: ClassInitializationState) {
		self.init_state = state;
	}

	pub fn parent_iter(&self) -> ClassParentIterator {
		ClassParentIterator {
			current_class: self.super_class.clone(),
		}
	}

	pub fn initialization_state(&self) -> ClassInitializationState {
		self.init_state
	}

	pub fn set_mirror(mirror_class: ClassRef, target: ClassRef) {
		let mirror = match target.get().class_ty {
			ClassType::Instance(_) => MirrorInstance::new(mirror_class, Arc::clone(&target)),
			ClassType::Array(_) => MirrorInstance::new_array(mirror_class, Arc::clone(&target)),
		};

		target.get_mut().mirror = MaybeUninit::new(mirror);
	}

	pub fn shares_package_with(&self, other: &Self) -> bool {
		if self.loader != other.loader {
			return false;
		}

		if self.name == other.name {
			return true;
		}

		// TODO: We can probably cache these at some point
		let Ok(other_pkg) = ClassLoader::package_name_for_class(other.name) else {
			return false;
		};

		// We should never receive an empty string from `package_name_for_class`
		if let Some(other_pkg_str) = other_pkg {
			assert!(!other_pkg_str.is_empty(), "Package name is an empty string");
		}

		let Ok(this_pkg) = ClassLoader::package_name_for_class(other.name) else {
			return false;
		};

		if this_pkg.is_none() || other_pkg.is_none() {
			// One of the two doesn't have a package, so we'll only return
			// `true` if *both* have no package.
			return this_pkg == other_pkg;
		}

		return this_pkg.unwrap() == other_pkg.unwrap();
	}

	pub fn is_subclass_of(&self, class: ClassRef) -> bool {
		let mut current_class = self;
		while let Some(super_class) = &current_class.super_class {
			if super_class == &class {
				return true;
			}

			current_class = super_class.get();
		}

		false
	}

	pub fn implements(&self, class: ClassRef) -> bool {
		for interface in &self.interfaces {
			if &class == interface || class.implements(Arc::clone(&interface)) {
				return true;
			}
		}

		for parent in self.parent_iter() {
			for interface in &parent.interfaces {
				if &class == interface || class.implements(Arc::clone(&interface)) {
					return true;
				}
			}
		}

		false
	}

	pub fn is_array(&self) -> bool {
		matches!(self.class_ty, ClassType::Array(_))
	}

	pub fn is_interface(&self) -> bool {
		self.access_flags.is_interface()
	}

	pub fn unwrap_class_instance(&self) -> &ClassDescriptor {
		match self.class_ty {
			ClassType::Instance(ref instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_class_instance_mut(&mut self) -> &mut ClassDescriptor {
		match self.class_ty {
			ClassType::Instance(ref mut instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_array_instance(&self) -> &ArrayDescriptor {
		match self.class_ty {
			ClassType::Array(ref instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_array_instance_mut(&mut self) -> &mut ArrayDescriptor {
		match self.class_ty {
			ClassType::Array(ref mut instance) => instance,
			_ => unreachable!(),
		}
	}
}

pub struct ClassParentIterator {
	current_class: Option<ClassRef>,
}

impl Iterator for ClassParentIterator {
	type Item = ClassRef;

	fn next(&mut self) -> Option<Self::Item> {
		return match &self.current_class {
			None => None,
			Some(current) => {
				let ret = self.current_class.clone();
				self.current_class = current.super_class.as_ref().map(Arc::clone);
				ret
			},
		};
	}
}

fn new_vtable(class_methods: Option<&[MethodInfo]>, classref: ClassRef) -> VTable<'static> {
	let mut vtable;
	match class_methods {
		// Initialize the vtable with the new `ClassFile`'s parsed methods
		Some(class_methods) => {
			vtable = class_methods
				.iter()
				.map(|mi| &*Method::new(Arc::clone(&classref), mi))
				.collect::<Vec<_>>();
		},
		// The vtable will only inherit from the super classes
		None => vtable = Vec::new(),
	}

	if let Some(super_class) = &classref.super_class {
		vtable.extend(super_class.vtable().iter())
	}

	VTable::from(vtable)
}

// A pointer to a Class instance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the class.
#[derive(PartialEq)]
pub struct ClassPtr(usize);

impl ClassPtr {
	pub fn unwrap_class_instance_mut(&self) -> &mut ClassDescriptor {
		match self.get_mut().class_ty {
			ClassType::Instance(ref mut instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_array_instance_mut(&self) -> &mut ArrayDescriptor {
		match self.get_mut().class_ty {
			ClassType::Array(ref mut instance) => instance,
			_ => unreachable!(),
		}
	}
}

impl PtrType<Class, ClassRef> for ClassPtr {
	fn new(val: Class) -> ClassRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ClassRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const Class {
		self.0 as *const Class
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut Class {
		self.0 as *mut Class
	}

	fn get(&self) -> &Class {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut Class {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ClassPtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut Class) };
	}
}

impl Deref for ClassPtr {
	type Target = Class;

	fn deref(&self) -> &Self::Target {
		unsafe { &(*self.as_raw()) }
	}
}

impl DerefMut for ClassPtr {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Debug for ClassPtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(class.name.as_str())
	}
}
