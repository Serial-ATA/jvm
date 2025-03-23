mod spec;
pub use spec::ClassInitializationState;
use spec::InitializationLock;

use super::constant_pool::{ConstantPool, ResolvedEntry};
use super::field::Field;
use super::method::Method;
use super::mirror::MirrorInstance;
use super::vtable::VTable;
use crate::classpath::loader::ClassLoader;
use crate::error::RuntimeError;
use crate::globals::classes;
use crate::modules::{Module, Package};
use crate::objects::constant_pool::cp_types;
use crate::objects::reference::{MirrorInstanceRef, Reference};
use crate::symbols::Symbol;
use crate::thread::exceptions::Throws;
use crate::thread::JavaThread;

use std::cell::{Cell, UnsafeCell};
use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::{mem, ptr};

use classfile::accessflags::ClassAccessFlags;
use classfile::attribute::resolved::ResolvedBootstrapMethod;
use classfile::constant_pool::types::{raw as raw_types, ClassNameEntry};
use classfile::{ClassFile, FieldType, MethodInfo};
use common::box_slice;
use common::int_types::{u1, u2, u4};
use common::traits::PtrType;
use instructions::Operand;

/// A cache for miscellaneous fields
#[derive(Default, Debug)]
struct MiscCache {
	/// The index of the name of this class in the constant pool
	class_name_index: u2,
	nest_host: Option<&'static Class>,
	nest_host_index: Option<u2>,
	array_class_name: Option<Symbol>,
	array_component_name: Option<Symbol>,
	package_name: Option<Option<Symbol>>,
	external_name: Option<Symbol>,
	signature: Option<Symbol>,
	modifier_flags: Option<u2>,
	is_hidden: bool,
}

struct FieldContainer {
	/// A list of all fields, including static. Only ever uninit during field injection.
	fields: UnsafeCell<MaybeUninit<Box<[&'static Field]>>>,
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
			fields: UnsafeCell::new(MaybeUninit::new(box_slice![])),
			static_field_slots: box_slice![],
			instance_field_count: UnsafeCell::new(0),
		}
	}

	fn new(static_field_slots: Box<[UnsafeCell<Operand<Reference>>]>) -> Self {
		Self {
			fields: UnsafeCell::new(MaybeUninit::new(box_slice![])),
			static_field_slots,
			instance_field_count: UnsafeCell::new(0),
		}
	}

	fn fields(&self) -> impl Iterator<Item = &'static Field> {
		let fields = unsafe { (&*self.fields.get()).assume_init_ref() };
		fields.iter().copied()
	}

	// This is only ever used in class loading
	fn set_fields(&self, new: Vec<&'static Field>) {
		let fields = self.fields.get();
		let old = mem::replace(
			unsafe { &mut *fields },
			MaybeUninit::new(new.into_boxed_slice()),
		);
		drop(old);
	}

	/// # SAFETY
	///
	/// See [`Class::set_static_field`]
	unsafe fn set_static_field(&self, index: usize, value: Operand<Reference>) {
		let field = &self.static_field_slots[index];
		let old = mem::replace(unsafe { &mut *field.get() }, value);
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

	// This is only ever used in class loading and field injection
	fn set_instance_field_count(&self, value: u4) {
		unsafe {
			*self.instance_field_count.get() = value;
		}
	}
}

// TODO: Make more fields private
pub struct Class {
	// UnsafeCell, since we need to mangle the names of hidden classes *after* they are constructed.
	name: UnsafeCell<Symbol>,
	pub access_flags: ClassAccessFlags,
	loader: &'static ClassLoader,
	pub super_class: Option<&'static Class>,
	pub interfaces: Vec<&'static Class>,
	misc_cache: UnsafeCell<MiscCache>,
	mirror: UnsafeCell<MaybeUninit<MirrorInstanceRef>>,
	field_container: FieldContainer,
	vtable: UnsafeCell<MaybeUninit<VTable<'static>>>,

	nest_members: Option<Box<[Symbol]>>,
	bootstrap_methods: Option<Box<[ResolvedBootstrapMethod]>>,

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
			.field("name", &self.name().as_str())
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
		self.name() == other.name() && self.loader == other.loader
	}
}

#[derive(Debug)]
pub enum ClassType {
	Instance(ClassDescriptor),
	Array(ArrayDescriptor),
}

#[derive(Copy, Clone, Debug)]
pub struct EnclosingMethodInfo {
	/// The class of the enclosing method
	pub class: &'static Class,
	/// The enclosing method, if available
	pub method: Option<&'static Method>,
}

pub struct InnerClassInfo {
	pub inner_class: Symbol,
	pub outer_class: Option<Symbol>,
	pub inner_class_name: Option<Symbol>,
	pub inner_class_access_flags: u2,
}

pub struct ClassDescriptor {
	pub source_file_index: Option<u2>,
	pub constant_pool: ConstantPool,
	pub enclosing_method: Option<EnclosingMethodInfo>,
	inner_classes: Option<Vec<InnerClassInfo>>,
	pub is_record: bool,
}

impl ClassDescriptor {
	pub fn inner_classes(&self) -> Option<&[InnerClassInfo]> {
		self.inner_classes.as_deref()
	}
}

impl Debug for ClassDescriptor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut debug_struct = f.debug_struct("ClassDescriptor");

		match self.source_file_index {
			Some(idx) => debug_struct.field(
				"source_file",
				&self
					.constant_pool
					.get::<cp_types::ConstantUtf8>(idx)
					.expect("string constants should always be resolved"),
			),
			None => debug_struct.field("source_file", &"None"),
		};

		debug_struct
			.field("enclosing_method", &self.enclosing_method)
			.field("is_record", &self.is_record)
			.finish()
	}
}

#[derive(Debug, Clone)]
pub struct ArrayDescriptor {
	pub dimensions: u1,
	pub component: FieldType,
}

impl ArrayDescriptor {
	pub fn is_primitive(&self) -> bool {
		self.component.is_primitive()
	}
}

// Getters
impl Class {
	fn class_ty(&self) -> &'static ClassType {
		// SAFETY: The only way to construct a `Class` is via `Class::new()`, which ensures that the
		//         class type is initialized.
		unsafe { (&*self.class_ty.get()).assume_init_ref() }
	}

	/// Get the name of this class
	///
	/// NOTE: For hidden classes, the name will be mangled
	pub fn name(&self) -> Symbol {
		unsafe { *self.name.get() }
	}

	/// Get a reference to the `ClassLoader` that loaded this class
	pub fn loader(&self) -> &'static ClassLoader {
		self.loader
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

	pub fn bootstrap_methods(&self) -> Option<&[ResolvedBootstrapMethod]> {
		self.bootstrap_methods.as_deref()
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

	/// TODO: Document
	pub fn package_name(&self) -> Result<Option<Symbol>, RuntimeError> {
		if let Some(package_name) = unsafe { (*self.misc_cache.get()).package_name } {
			return Ok(package_name);
		};

		let name_str = self.name().as_str();

		if name_str.is_empty() {
			return Err(RuntimeError::BadClassName);
		}

		let Some(end) = name_str.as_bytes().iter().rposition(|c| *c == b'/') else {
			unsafe {
				(*self.misc_cache.get()).package_name = Some(None);
			}

			return Ok(None);
		};

		let mut start_index = 0;

		// Skip over '['
		if name_str.starts_with('[') {
			start_index = name_str
				.as_bytes()
				.iter()
				.take_while(|b| **b == b'[')
				.count();
			if start_index >= name_str.len() {
				return Err(RuntimeError::BadClassName);
			}
		}

		if name_str.as_bytes()[start_index] == b'L' {
			// 'L' is only valid when used in an object array component
			if !self.is_array() {
				return Err(RuntimeError::BadClassName);
			}

			// Skip over 'L'
			start_index += 1;
		}

		if start_index >= end {
			return Err(RuntimeError::BadClassName);
		}

		assert!(
			!(&name_str[start_index..end]).is_empty(),
			"Package name is an empty string"
		);

		let ret = Symbol::intern(&name_str[start_index..end].as_bytes());
		unsafe {
			(*self.misc_cache.get()).package_name = Some(Some(ret));
		}

		Ok(Some(ret))
	}

	// TODO: This is expensive, requires locking. Should just be computed when the class is created.
	pub fn package(&self) -> Option<&Package> {
		let Some(package_name) = self.package_name().unwrap() else {
			return None;
		};

		let mut package = None;
		crate::modules::with_module_lock(|guard| {
			package = self.loader().lookup_package(guard, package_name);
		});

		assert!(package.is_some(), "Package not found in loader");
		package
	}

	pub fn module(&self) -> &'static Module {
		let bootstrap_loader = ClassLoader::bootstrap();
		if !bootstrap_loader.java_base().has_obj() {
			// Assume we are early in VM initialization, where `java.base` isn't a real module yet.
			// In this case, we can assume that the intended module is `java.base`.
			return bootstrap_loader.java_base();
		}

		match self.package() {
			Some(package) => package.module(),
			None => self.loader().unnamed_module(),
		}
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
			let name = format!("[{}", self.name().as_str());
			ret = Symbol::intern(name);
		} else {
			let name = format!("[{}", self.as_signature());
			ret = Symbol::intern(name);
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

		let mut class_name = &self.name().as_str()[1..];
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

	/// Get the class name as it would appear in a method or field signature
	///
	/// This will, for example, convert `java/lang/Object` to `Ljava.lang.Object;`
	pub fn as_signature(&self) -> Symbol {
		if let Some(signature) = unsafe { (*self.misc_cache.get()).signature } {
			return signature;
		}

		let signature;
		if self.is_array() {
			signature = Symbol::intern(format!("{}", self.name().as_str()));
			unsafe {
				(*self.misc_cache.get()).signature = Some(signature);
			}
		} else {
			signature = Symbol::intern(format!("L{};", self.name().as_str()));
		}

		unsafe {
			(*self.misc_cache.get()).signature = Some(signature);
		}

		signature
	}

	/// Get the external name of this class (for user-facing messages)
	///
	/// This just takes the class name and replaces all '/' with '.'
	pub fn external_name(&self) -> Symbol {
		if let Some(external_name) = unsafe { (*self.misc_cache.get()).external_name } {
			return external_name;
		}

		let signature = Symbol::intern(self.name().as_str().replace('/', "."));
		unsafe {
			(*self.misc_cache.get()).signature = Some(signature);
		}

		signature
	}

	pub fn external_kind(&self) -> &'static str {
		if self.is_interface() {
			"interface"
		} else if self.is_abstract() {
			"abstract class"
		} else {
			"class"
		}
	}

	/// Get the external name of this class (for user-facing messages)
	///
	/// This just takes the class name and replaces all '/' with '.'
	pub fn nest_host(&'static self, thread: &JavaThread) -> Throws<&'static Class> {
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

	/// A standard help string for errors including the class, loader, and module
	///
	/// Format:
	/// ```
	///   <fully-qualified-external-class-name> is in module <module-name>[@<version>]
	///                                         of loader <loader-name_and_id>[, parent loader <parent-loader-name_and_id>]
	/// ```
	pub fn in_module_of_loader(&self, use_are: bool, include_parent_loader: bool) -> String {
		let are_or_is = if use_are { "are" } else { "is" };

		// 1. fully-qualified external name of the class
		let external_name = self.external_name();

		let module_name_phrase;
		let module_name;

		let mut module_version_separator = "";
		let mut module_version = "";

		'module_info: {
			let mut target_class = self;
			if self.is_array() {
				let array_descriptor = self.unwrap_array_instance();

				// Simplest case, primitive arrays are in java.base
				if array_descriptor.is_primitive() {
					module_name_phrase = "module ";
					module_name = "java.base";
					break 'module_info;
				}

				let FieldType::Object(class_name) = &array_descriptor.component else {
					unreachable!()
				};

				// TODO: There shouldn't be a case where the component isn't already loaded, but just incase
				//       this should bubble up an exception, not unwrap.
				let class_name = Symbol::intern(class_name);
				target_class = target_class.loader().load(class_name).unwrap();
			}

			match target_class.module().name() {
				Some(name) => {
					module_name_phrase = "module ";
					module_name = name.as_str();
					if target_class.module().should_show_version() {
						module_version_separator = "@";
						module_version = target_class
							.module()
							.version()
							.expect("version should exist")
							.as_str();
					}
				},
				None => {
					module_name_phrase = "";
					module_name = "unnamed module";
				},
			}
		}

		let loader_name_and_id = self.loader().name_and_id();

		// TODO: set these
		let mut parent_loader_phrase = "";
		let mut parent_loader_name_and_id = "";

		format!(
			"{external_name} {are_or_is} in \
			 {module_name_phrase}{module_name}{module_version_separator}{module_version} of \
			 loader {loader_name_and_id}{parent_loader_phrase}{parent_loader_name_and_id}",
		)
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

	/// Inject a set of fields into this class
	///
	/// This can only ever be called once, and is **NEVER** to be used outside of initialization.
	///
	/// This allows us to store extra information in objects as necessary, such as a [`Module`] pointer
	/// in a `java.lang.Module` object.
	pub unsafe fn inject_fields(
		&self,
		fields: impl IntoIterator<Item = &'static Field>,
		field_count: usize,
	) {
		assert!(field_count > 0, "field injection requires at least 1 field");

		let old_fields_ptr = self.field_container.fields.get();
		let old_fields = unsafe {
			let old_fields = ptr::replace(old_fields_ptr, MaybeUninit::uninit());
			old_fields.assume_init()
		};

		let max_instance_index =
			old_fields
				.iter()
				.fold(0, |a, b| if b.is_static() { a } else { a.max(b.index()) });

		let expected_len = old_fields.len() + field_count;
		let mut new_fields = Vec::with_capacity(expected_len);
		new_fields.extend(old_fields);

		for (idx, field) in fields.into_iter().enumerate() {
			assert!(!field.is_static());
			field.set_index(max_instance_index + idx + 1);
			new_fields.push(field);
		}

		assert_eq!(new_fields.len(), expected_len);

		let new_fields = MaybeUninit::new(new_fields.into_boxed_slice());
		unsafe {
			ptr::write(old_fields_ptr, new_fields);
		}

		// + 1 as `max_instance_index` is the index of the *last* instance field, not of the first
		// field we inject.
		self.field_container
			.set_instance_field_count((max_instance_index + 1 + field_count) as u32);
	}

	/// Set the nest host for this class
	///
	/// This should only be used in `java.lang.ClassLoader#defineClass0`. Setting an incorrect
	/// nest host can result in permission issues.
	pub unsafe fn set_nest_host(&self, nest_host: &'static Self) {
		(*self.misc_cache.get()).nest_host = Some(nest_host);
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

	/// Get the [access flags] for this class
	///
	/// [access flags]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.1-200-E.1
	#[inline]
	pub fn access_flags(&self) -> ClassAccessFlags {
		self.access_flags
	}

	/// Get the modifier flags, as returned by `java.lang.Class#getModifiers()`
	///
	/// This is different from the [`access_flags()`](Self::access_flags), as this checks the
	/// `InnerClasses` attribute, in the case that this class is a member class.
	pub fn modifier_flags(&self) -> u2 {
		if let Some(modifier_flags) = unsafe { (*self.misc_cache.get()).modifier_flags } {
			return modifier_flags;
		};

		let mut access_flags = self.access_flags.as_u2();
		if let Some(inner_classes) = self.unwrap_class_instance().inner_classes() {
			for inner_class in inner_classes {
				if inner_class.inner_class_name == Some(self.name()) {
					access_flags = inner_class.inner_class_access_flags;
				}
			}
		};

		// Make sure `ACC_SUPER` isn't set
		access_flags &= (!ClassAccessFlags::ACC_SUPER).as_u2();
		unsafe {
			(*self.misc_cache.get()).modifier_flags = Some(access_flags);
		}

		access_flags
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

	/// Whether the class is a record
	pub fn is_record(&self) -> bool {
		match self.class_ty() {
			ClassType::Instance(ref instance) => instance.is_record,
			_ => false,
		}
	}

	/// Whether the class is hidden
	///
	/// A hidden class is simply a class created by `java.lang.invoke.MethodHandles.Lookup#defineHiddenClass()`
	pub fn is_hidden(&self) -> bool {
		unsafe { &*self.misc_cache.get() }.is_hidden
	}

	/// Whether this class is a subclass of `class`
	pub fn is_subclass_of(&self, class: &Class) -> bool {
		if self == class {
			return true;
		}

		let mut current_class = self;
		while let Some(super_class) = &current_class.super_class {
			if *super_class == class {
				return true;
			}

			current_class = super_class;
		}

		false
	}

	pub fn has_nest_member(&self, other: &Class) -> bool {
		let Some(nest_members) = &self.nest_members else {
			return false;
		};

		nest_members.iter().any(|name| *name == other.name())
	}

	/// Whether `self` is a nestmate of `other`, meaning they are under the same nest host
	///
	/// Resolving the nest host of either class may throw.
	pub fn is_nestmate_of(
		&'static self,
		other: &'static Class,
		thread: &JavaThread,
	) -> Throws<bool> {
		let self_host = self.nest_host(thread)?;
		let other_host = other.nest_host(thread)?;
		Throws::Ok(self_host == other_host)
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
		loader: &'static ClassLoader,
		is_hidden: bool,
	) -> Throws<&'static Class> {
		let access_flags = parsed_file.access_flags;
		let class_name_index = parsed_file.this_class;

		let source_file_index = parsed_file.source_file_index();

		// Check the NestMembers attribute
		let nest_members = match parsed_file.nest_members() {
			Some(nest_members) => Some(
				nest_members
					.map(|entry| Symbol::intern(entry.name))
					.collect::<Box<[Symbol]>>(),
			),
			None => None,
		};

		let nest_host_index = parsed_file.nest_host_index();

		// Check the BootstrapMethods attribute
		let bootstrap_methods = match parsed_file.bootstrap_methods() {
			Some(bootstrap_methods) => {
				Some(bootstrap_methods.collect::<Box<[ResolvedBootstrapMethod]>>())
			},
			None => None,
		};

		let enclosing_method = match parsed_file.enclosing_method() {
			Some(enclosing_method_info) => {
				let enclosing_class =
					loader.load(Symbol::intern(&*enclosing_method_info.class.name))?;

				let enclosing_method;
				match enclosing_method_info.method {
					None => enclosing_method = None,
					Some(name_and_type) => {
						let method_name = Symbol::intern(&*name_and_type.name);
						let method_descriptor = Symbol::intern(&*name_and_type.descriptor);
						if enclosing_class.is_interface() {
							enclosing_method = Some(
								enclosing_class
									.resolve_interface_method(method_name, method_descriptor)?,
							);
						} else {
							enclosing_method = Some(
								enclosing_class.resolve_method(method_name, method_descriptor)?,
							);
						}
					},
				}

				Some(EnclosingMethodInfo {
					class: enclosing_class,
					method: enclosing_method,
				})
			},
			None => None,
		};

		let inner_classes = match parsed_file.inner_classes() {
			Some(inner_classes) => {
				let mut classes = Vec::with_capacity(inner_classes.len());
				for class in inner_classes {
					classes.push(InnerClassInfo {
						inner_class: Symbol::intern(&*class.inner_class.name),
						outer_class: class.outer_class.map(|oc| Symbol::intern(&*oc.name)),
						inner_class_name: class
							.inner_name
							.map(|inner_name| Symbol::intern(&*inner_name)),
						inner_class_access_flags: class.access_flags,
					})
				}

				Some(classes)
			},
			None => None,
		};

		// TODO: Actually retain the information from the record attribute
		let is_record = parsed_file
			.attributes
			.iter()
			.any(|attr| attr.record().is_some());

		let constant_pool = parsed_file.constant_pool;

		let name_raw = constant_pool.get::<raw_types::RawClassName>(class_name_index);
		let name = Symbol::intern(&*name_raw.name);

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
				let interface_class_name =
					constant_pool.get::<raw_types::RawClassName>(*index).name;
				loader.load(Symbol::intern(&*interface_class_name))
			})
			.collect::<Throws<Vec<_>>>()?;

		let static_field_slots = box_slice![UnsafeCell::new(Operand::Empty); static_field_count];

		let class = Self {
			name: UnsafeCell::new(name),
			access_flags,
			loader,
			super_class,
			interfaces,
			misc_cache: UnsafeCell::new(MiscCache {
				class_name_index,
				nest_host_index,
				is_hidden,
				..MiscCache::default()
			}),
			mirror: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			field_container: FieldContainer::new(static_field_slots),
			vtable: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			nest_members,
			bootstrap_methods,
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
			inner_classes,
			enclosing_method,
			is_record,
		};
		unsafe {
			*class.class_ty.get() = MaybeUninit::new(ClassType::Instance(class_instance));
		}

		// Hidden class names can collide, need to mangle. Relies on the `ClassDescriptor`
		if class.is_hidden() {
			class.mangle_name();
		}

		// Create our vtable...
		let vtable = new_vtable(Some(parsed_file.methods), class);
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

		Throws::Ok(class)
	}

	fn mangle_name(&'static self) {
		let ptr = self as *const Class as usize;
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

	/// Create a new array class of type `component`
	///
	/// # Safety
	///
	/// This should never be used outside of the ClassLoader. The resulting [`ClassRef`] needs to
	/// be handled properly, as some fields remain uninitialized.
	pub unsafe fn new_array(
		name: Symbol,
		component: FieldType,
		loader: &'static ClassLoader,
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
			name: UnsafeCell::new(name),
			access_flags: ClassAccessFlags::NONE,
			loader,
			super_class: Some(crate::globals::classes::java_lang_Object()),
			// https://docs.oracle.com/javase/specs/jls/se19/html/jls-4.html#jls-4.10.3
			interfaces: vec![
				crate::globals::classes::java_lang_Cloneable(),
				crate::globals::classes::java_io_Serializable(),
			],
			misc_cache: UnsafeCell::new(MiscCache::default()),
			mirror: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			field_container: FieldContainer::null(),
			vtable: UnsafeCell::new(MaybeUninit::uninit()), // Set later
			nest_members: None,
			bootstrap_methods: None,
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
	pub fn initialize(&self, thread: &JavaThread) -> Throws<()> {
		if self.is_initialized.get() {
			return Throws::Ok(());
		}

		self.initialization(thread)?;
		self.is_initialized.set(true);

		Throws::Ok(())
	}

	/// Set the mirror for this class
	///
	/// This optionally takes an existing mirror, otherwise it will create a new one.
	///
	/// # Safety
	///
	/// This is only safe to call *before* the class is in use. It should never be used outside of
	/// class loading.
	pub unsafe fn set_mirror(&'static self, mirror: Option<MirrorInstanceRef>) {
		let final_mirror = match mirror {
			Some(mirror) => mirror,
			None => match self.class_ty() {
				ClassType::Instance(_) => {
					let mirror = MirrorInstance::new(self);
					mirror.get().set_module(self.module().obj());
					mirror
				},
				ClassType::Array(_) => {
					let mirror = MirrorInstance::new_array(self);
					let bootstrap_loader = ClassLoader::bootstrap();
					mirror.get().set_module(bootstrap_loader.java_base().obj());
					mirror
				},
			},
		};

		unsafe {
			*self.mirror.get() = MaybeUninit::new(final_mirror);
		}
	}

	pub fn shares_package_with(&self, other: &Self) -> bool {
		if self.loader != other.loader {
			return false;
		}

		if self.name() == other.name() {
			return true;
		}

		let Ok(other_pkg) = other.package_name() else {
			return false;
		};

		let Ok(this_pkg) = self.package_name() else {
			return false;
		};

		if this_pkg.is_none() || other_pkg.is_none() {
			// One of the two doesn't have a package, so we'll only return
			// `true` if *both* have no package.
			return this_pkg == other_pkg;
		}

		this_pkg.unwrap() == other_pkg.unwrap()
	}

	pub fn implements(&self, target_interface: &Class) -> bool {
		if !target_interface.is_interface() {
			// TODO: Assertion maybe?
			return false;
		}

		if self.is_interface() && self == target_interface {
			return true;
		}

		for super_interface in &self.interfaces {
			if target_interface == *super_interface || super_interface.implements(&target_interface)
			{
				return true;
			}
		}

		for parent in self.parent_iter() {
			for super_interface in &parent.interfaces {
				if target_interface == *super_interface
					|| super_interface.implements(&target_interface)
				{
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

fn new_vtable(class_methods: Option<Vec<MethodInfo>>, class: &'static Class) -> VTable<'static> {
	let mut vtable;
	match class_methods {
		// Initialize the vtable with the new `ClassFile`'s parsed methods
		Some(class_methods) => {
			vtable = class_methods
				.into_iter()
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
