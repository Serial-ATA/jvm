mod spec;
pub use spec::ClassInitializationState;

use super::constant_pool::ConstantPool;
use super::field::Field;
use super::method::Method;
use super::mirror::MirrorInstance;
use super::vtable::VTable;
use crate::classpath::classloader::ClassLoader;
use crate::globals::PRIMITIVES;
use crate::objects::constant_pool::cp_types;
use crate::objects::reference::{MirrorInstanceRef, Reference};
use crate::thread::JavaThread;
use spec::InitializationLock;

use std::cell::{Cell, UnsafeCell};
use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

use classfile::accessflags::ClassAccessFlags;
use classfile::{ClassFile, FieldType, MethodInfo};
use common::box_slice;
use common::int_types::{u1, u2, u4};
use instructions::Operand;
use symbols::{sym, Symbol};

/// A cache for miscellaneous fields
#[derive(Default, Debug)]
struct MiscCache {
	array_class_name: Option<Symbol>,
	array_component_name: Option<Symbol>,
}

struct FieldContainer {
	/// A list of all fields, including static
	fields: UnsafeCell<Box<[&'static Field]>>,
	/// All static field slots
	///
	/// This needs to be scaled to the `fields` field, in that index 0 of this array relates
	/// to the index of the first static field in `fields`.
	static_field_slots: Box<[UnsafeCell<Operand<Reference>>]>,
	/// The number of dynamic fields in a class instance
	///
	/// This is essentially `fields.len() - static_field_slots.len()`, provided here for convenience.
	instance_field_count: UnsafeCell<u4>,
}

impl FieldContainer {
	/// Used as the field container for arrays, as they have no instance fields.
	fn null() -> Self {
		Self {
			fields: UnsafeCell::new(box_slice![]),
			static_field_slots: box_slice![],
			instance_field_count: UnsafeCell::new(0),
		}
	}

	fn new(static_field_slots: Box<[UnsafeCell<Operand<Reference>>]>) -> Self {
		Self {
			fields: UnsafeCell::new(box_slice![]),
			static_field_slots,
			instance_field_count: UnsafeCell::new(0),
		}
	}

	fn fields(&self) -> impl Iterator<Item = &'static Field> {
		let fields = unsafe { &*self.fields.get() };
		fields.iter().copied()
	}

	// This is only ever used in class loading
	fn set_fields(&self, new: Vec<&'static Field>) {
		let fields = self.fields.get();
		let old = core::mem::replace(unsafe { &mut *fields }, new.into_boxed_slice());
		drop(old);
	}

	/// # SAFETY
	///
	/// See [`Class::set_static_field`]
	unsafe fn set_static_field(&self, index: usize, value: Operand<Reference>) {
		let field = &self.static_field_slots[index];
		let old = core::mem::replace(unsafe { &mut *field.get() }, value);
		drop(old);
	}

	fn get_static_field(&self, index: usize) -> Operand<Reference> {
		let field = self.static_field_slots[index].get();
		unsafe { (*field).clone() }
	}

	fn get_static_field_volatile(&self, index: usize) -> Operand<Reference> {
		let field = self.static_field_slots[index].get();
		let ptr = AtomicPtr::new(field);
		unsafe { (&*ptr.load(Ordering::Acquire)).clone() }
	}

	fn instance_field_count(&self) -> u4 {
		unsafe { *self.instance_field_count.get() }
	}

	// This is only ever used in class loading
	fn set_instance_field_count(&self, value: u4) {
		unsafe {
			*self.instance_field_count.get() = value;
		}
	}
}

// TODO: Make more fields private
pub struct Class {
	pub name: Symbol,
	pub access_flags: ClassAccessFlags,
	pub loader: ClassLoader,
	pub super_class: Option<&'static Class>,
	pub interfaces: Vec<&'static Class>,
	misc_cache: UnsafeCell<MiscCache>,
	mirror: UnsafeCell<MaybeUninit<MirrorInstanceRef>>,
	field_container: FieldContainer,
	vtable: UnsafeCell<MaybeUninit<VTable<'static>>>,

	class_ty: UnsafeCell<MaybeUninit<ClassType>>,

	init_lock: Arc<InitializationLock>,

	// Used for fast path, initialization checks are needed for multiple instructions
	is_initialized: Cell<bool>,
}

// SAFETY: Any pointer writes require synchronization
unsafe impl Send for Class {}
unsafe impl Sync for Class {}

impl Debug for Class {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Class")
			.field("name", &self.name.as_str())
			.field("access_flags", &self.access_flags)
			.field("loader", &self.loader)
			.field("super_class", &self.super_class)
			.field("interfaces", &self.interfaces)
			.field("instance", &self.class_ty)
			.finish()
	}
}

impl PartialEq for Class {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

#[derive(Debug)]
pub enum ClassType {
	Instance(ClassDescriptor),
	Array(ArrayDescriptor),
}

pub struct ClassDescriptor {
	pub source_file_index: Option<u2>,
	pub constant_pool: ConstantPool,
}

impl Debug for ClassDescriptor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut debug_struct = f.debug_struct("ClassDescriptor");

		match self.source_file_index {
			Some(idx) => debug_struct.field(
				"source_file",
				&self.constant_pool.get::<cp_types::ConstantUtf8>(idx),
			),
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
	fn class_ty(&self) -> &'static ClassType {
		// SAFETY: The only way to construct a `Class` is via `Class::new()`, which ensures that the
		//         class type is initialized.
		unsafe { (&*self.class_ty.get()).assume_init_ref() }
	}

	/// Get a reference to the constant pool for this class
	///
	/// This returns an `Option`, as array classes do not have an associated constant pool. It is
	/// guaranteed to be present otherwise.
	pub fn constant_pool(&self) -> Option<&'static ConstantPool> {
		match self.class_ty() {
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
		unsafe { (&*self.vtable.get()).assume_init_ref() }
	}

	/// Get the fields for this class
	///
	/// NOTE: This includes the static fields as well.
	///
	/// This is the only way to access the class fields externally.
	pub fn fields(&self) -> impl Iterator<Item = &'static Field> {
		self.field_container.fields()
	}

	/// Get the instance fields for this class
	///
	/// This is simply [`Class::fields`] with static fields filtered out.
	pub fn instance_fields(&self) -> impl Iterator<Item = &'static Field> {
		self.fields().filter(|field| !field.is_static())
	}

	/// Get the static fields for this class
	pub fn static_fields(&self) -> impl Iterator<Item = &'static Field> {
		self.fields().filter(|field| field.is_static())
	}

	/// Get the value of the static field at `index`
	///
	/// # Panics
	///
	/// This will panic if the index is out of bounds.
	pub fn static_field_value(&self, index: usize) -> Operand<Reference> {
		self.field_container.get_static_field(index)
	}

	pub fn static_field_value_volatile(&self, index: usize) -> Operand<Reference> {
		self.field_container.get_static_field_volatile(index)
	}

	/// The number of non-static fields
	pub fn instance_field_count(&self) -> usize {
		self.field_container.instance_field_count() as usize
	}

	/// Get the mirror for this class
	///
	/// See [`MirrorInstance`] for information on mirrors.
	pub fn mirror(&self) -> MirrorInstanceRef {
		// SAFETY: The mirror is only uninitialized for a few classes few early in VM initialization
		//         due to them loading *before* `java.lang.Class`. Afterwards, all classes are
		//         guaranteed to have mirrors.
		let mirror = unsafe { (*self.mirror.get()).assume_init_ref() };
		Arc::clone(mirror)
	}

	/// Wrap the class name as an array
	///
	/// This will, for example:
	///
	/// * Convert `java/lang/Object` to `[Ljava/lang/Object;`
	/// * Convert `[Ljava/lang/Object;` to `[[Ljava/lang/Object;`
	pub fn array_class_name(&self) -> Symbol {
		if let Some(array_class_name) = unsafe { (*self.misc_cache.get()).array_class_name } {
			return array_class_name;
		};

		let ret;
		if self.is_array() {
			let name = format!("[{}", self.name.as_str());
			ret = Symbol::intern_owned(name);
		} else {
			let name = format!("[L{};", self.name.as_str());
			ret = Symbol::intern_owned(name);
		}

		unsafe {
			(*self.misc_cache.get()).array_class_name = Some(ret);
		}
		ret
	}

	/// Get the name of the component of this array class
	///
	/// # Panics
	///
	/// This will panic if called on a non-array class.
	pub fn array_component_name(&self) -> Symbol {
		if let Some(array_component_name) = unsafe { (*self.misc_cache.get()).array_component_name }
		{
			return array_component_name;
		}

		debug_assert!(
			self.is_array(),
			"This should never be called on non-array classes"
		);

		let mut class_name = &self.name.as_str()[1..];
		let ret;
		match class_name.as_bytes()[0] {
			// Multi-dimensional array
			b'[' => {
				ret = Symbol::intern(class_name);
			},
			// Some object, need to strip the leading 'L' and trailing ';'
			b'L' => {
				class_name = &class_name[1..class_name.len() - 1];
				ret = Symbol::intern(class_name);
			},
			// A primitive type
			_ => {
				debug_assert_eq!(class_name.len(), 1);
				ret = Symbol::intern(class_name);
			},
		}

		unsafe {
			(*self.misc_cache.get()).array_component_name = Some(ret);
		}
		ret
	}

	pub(self) fn initialization_lock(&self) -> Arc<InitializationLock> {
		Arc::clone(&self.init_lock)
	}
}

// Setters
impl Class {
	/// Set the value of the static field at `index`
	///
	/// NOTE: This will drop the previous value (if any).
	///
	/// # Safety
	///
	/// This method is unsafe in that it will mutate fields that other threads may be reading. However,
	/// that behavior is acceptable, as synchronization is a requirement of the Java code, not the VM.
	///
	/// # Panics
	///
	/// This will panic if the index is out of bounds.
	pub unsafe fn set_static_field(&self, index: usize, value: Operand<Reference>) {
		unsafe {
			self.field_container.set_static_field(index, value);
		}
	}

	pub fn set_static_field_volatile(&self, index: usize, value: Operand<Reference>) {
		// TODO: Actually do this right
		unsafe {
			self.field_container.set_static_field(index, value);
		}
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
		self.is_array() || self.implements(&crate::globals::classes::java_lang_Cloneable())
	}

	/// Whether the class represents an array
	pub fn is_array(&self) -> bool {
		matches!(self.class_ty(), ClassType::Array(_))
	}

	/// Whether the class is an interface
	pub fn is_interface(&self) -> bool {
		self.access_flags.is_interface()
	}

	/// Whether the class is declared abstract
	pub fn is_abstract(&self) -> bool {
		self.access_flags.is_abstract()
	}

	/// Whether this class is a subclass of `class`
	pub fn is_subclass_of(&self, class: &Class) -> bool {
		let mut current_class = self;
		while let Some(super_class) = &current_class.super_class {
			if super_class.name == class.name {
				return true;
			}

			current_class = super_class;
		}

		false
	}

	/// Whether this class can be cast into `class`
	#[allow(non_snake_case)]
	pub fn can_cast_to(&self, other: &'static Class) -> bool {
		// The following rules are used to determine whether an objectref that is not null can be cast to the resolved type
		//
		// S is the type of the object referred to by objectref, and T is the resolved class, array, or interface type

		let S_class = self;
		let T_class = other;

		// If S is a class type, then:
		//
		//     If T is a class type, then S must be the same class as T, or S must be a subclass of T;
		if !T_class.is_interface() && !T_class.is_array() {
			if S_class.name == T_class.name {
				return true;
			}

			return S_class.is_subclass_of(T_class);
		}
		//     If T is an interface type, then S must implement interface T.
		if T_class.is_interface() {
			return S_class.implements(T_class);
		}

		// If S is an array type SC[], that is, an array of components of type SC, then:
		//
		//     If T is a class type, then T must be Object.
		if !T_class.is_interface() && !T_class.is_array() {
			return T_class.name == sym!(java_lang_Object);
		}
		//     If T is an interface type, then T must be one of the interfaces implemented by arrays (JLS §4.10.3).
		if T_class.is_interface() {
			let class_name = T_class.name;
			return class_name == sym!(java_lang_Cloneable)
				|| class_name == sym!(java_io_Serializable);
		}
		//     If T is an array type TC[], that is, an array of components of type TC, then one of the following must be true:
		if T_class.is_array() {
			//         TC and SC are the same primitive type.
			let source_component = S_class.array_component_name();
			let dest_component = T_class.array_component_name();
			if PRIMITIVES.contains(&source_component) || PRIMITIVES.contains(&dest_component) {
				return source_component == dest_component;
			}

			//         TC and SC are reference types, and type SC can be cast to TC by these run-time rules.

			// It's impossible to get a reference to an unloaded class
			let S_class = ClassLoader::lookup_class(source_component).unwrap();
			let T_class = ClassLoader::lookup_class(dest_component).unwrap();
			return S_class.can_cast_to(T_class);
		}

		false
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
		super_class: Option<&'static Class>,
		loader: ClassLoader,
	) -> &'static Class {
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

		let mut super_instance_field_count = 0;
		if let Some(ref super_class) = super_class {
			super_instance_field_count = super_class.field_container.instance_field_count();
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

		let static_field_slots = box_slice![UnsafeCell::new(Operand::Empty); static_field_count];

		let class = Self {
			name,
			access_flags,
			loader,
			super_class,
			interfaces,
			misc_cache: UnsafeCell::new(MiscCache::default()),
			mirror: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			field_container: FieldContainer::new(static_field_slots),
			vtable: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			class_ty: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			init_lock: Arc::new(InitializationLock::new()),
			is_initialized: Cell::new(false),
		};

		let class: &'static mut Class = Box::leak(Box::new(class));

		// TODO: Improve?
		// CIRCULAR DEPENDENCY!
		//
		// The `ClassDescriptor` holds a `ConstantPool`, which holds a reference to the `Class`.
		let class_instance = ClassDescriptor {
			source_file_index,
			constant_pool: ConstantPool::new(class, constant_pool),
		};
		unsafe {
			*class.class_ty.get() = MaybeUninit::new(ClassType::Instance(class_instance));
		}

		// Create our vtable...
		let vtable = new_vtable(Some(&parsed_file.methods), class);
		unsafe {
			*class.vtable.get() = MaybeUninit::new(vtable);
		}

		// Then the fields...
		let mut fields =
			Vec::with_capacity(super_instance_field_count as usize + parsed_file.fields.len());
		if let Some(ref super_class) = class.super_class {
			// First we have to inherit the super classes' fields
			for field in super_class.instance_fields() {
				fields.push(field);
			}
		}

		// Now the fields defined in our class
		let mut static_idx = 0;
		// Continue the index from our existing instance fields
		let mut instance_field_idx = core::cmp::max(0, super_instance_field_count) as usize;
		let constant_pool = class
			.constant_pool()
			.expect("we just set the constant pool");
		for field in parsed_file.fields {
			let field_idx = if field.access_flags.is_static() {
				&mut static_idx
			} else {
				&mut instance_field_idx
			};

			fields.push(Field::new(*field_idx, class, &field, constant_pool));

			*field_idx += 1;
		}

		class.field_container.set_fields(fields);

		// Update the instance field count if we encountered any new ones
		if instance_field_idx > 0 {
			class
				.field_container
				.set_instance_field_count(instance_field_idx as u4);
		}

		class
	}

	/// Create a new array class of type `component`
	///
	/// # Safety
	///
	/// This should never be used outside of the ClassLoader. The resulting [`ClassRef`] needs to
	/// be handled properly, as some fields remain uninitialized.
	pub unsafe fn new_array(
		name: Symbol,
		component: FieldType,
		loader: ClassLoader,
	) -> &'static Class {
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
			misc_cache: UnsafeCell::new(MiscCache::default()),
			mirror: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			field_container: FieldContainer::null(),
			vtable: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			class_ty: UnsafeCell::new(MaybeUninit::new(ClassType::Array(array_instance))),
			init_lock: Arc::new(InitializationLock::new()),
			is_initialized: Cell::new(false),
		};

		let class: &'static mut Class = Box::leak(Box::new(class));

		// Create a vtable, inheriting from `java.lang.Object`
		let vtable = new_vtable(None, class);
		unsafe {
			*class.vtable.get() = MaybeUninit::new(vtable);
		}

		class
	}

	pub fn parent_iter(&self) -> ClassParentIterator {
		ClassParentIterator {
			current_class: self.super_class.clone(),
		}
	}

	pub fn initialization_state(&self) -> ClassInitializationState {
		let _guard = self.init_lock.lock();
		_guard.initialization_state()
	}

	/// Attempt to initialize this class
	///
	/// NOTE: If the class is being initialized by another thread, this will block until it is completed.
	pub fn initialize(&self, thread: &JavaThread) {
		if self.is_initialized.get() {
			return;
		}

		self.initialization(thread);
		self.is_initialized.set(true);
	}

	/// Set the mirror for this class
	///
	/// # Safety
	///
	/// This is only safe to call *before* the class is in use. It should never be used outside of
	/// class loading.
	pub unsafe fn set_mirror(&'static self) {
		let mirror = match self.class_ty() {
			ClassType::Instance(_) => MirrorInstance::new(self),
			ClassType::Array(_) => MirrorInstance::new_array(self),
		};

		unsafe {
			*self.mirror.get() = MaybeUninit::new(mirror);
		}
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

	pub fn implements(&self, class: &Class) -> bool {
		for interface in &self.interfaces {
			if class.name == interface.name || class.implements(&interface) {
				return true;
			}
		}

		for parent in self.parent_iter() {
			for interface in &parent.interfaces {
				if class.name == interface.name || class.implements(&interface) {
					return true;
				}
			}
		}

		false
	}

	pub fn unwrap_class_instance(&self) -> &ClassDescriptor {
		match self.class_ty() {
			ClassType::Instance(instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_array_instance(&self) -> &ArrayDescriptor {
		match self.class_ty() {
			ClassType::Array(instance) => instance,
			_ => unreachable!(),
		}
	}
}

pub struct ClassParentIterator {
	current_class: Option<&'static Class>,
}

impl Iterator for ClassParentIterator {
	type Item = &'static Class;

	fn next(&mut self) -> Option<Self::Item> {
		match &self.current_class {
			None => None,
			Some(current) => {
				let ret = self.current_class.clone();
				self.current_class = current.super_class;
				ret
			},
		}
	}
}

fn new_vtable(class_methods: Option<&[MethodInfo]>, class: &'static Class) -> VTable<'static> {
	let mut vtable;
	match class_methods {
		// Initialize the vtable with the new `ClassFile`'s parsed methods
		Some(class_methods) => {
			vtable = class_methods
				.iter()
				.map(|mi| &*Method::new(class, mi))
				.collect::<Vec<_>>();
		},
		// The vtable will only inherit from the super classes
		None => vtable = Vec::new(),
	}

	let local_methods_end = vtable.len();
	if let Some(super_class) = &class.super_class {
		vtable.extend(super_class.vtable().iter())
	}

	VTable::new(vtable, local_methods_end)
}
