use super::field::Field;
use super::method::Method;
use super::mirror::MirrorInstance;
use super::reference::{ClassRef, FieldRef};
use super::spec::class::{ClassInitializationState, InitializationLock};
use crate::classpath::classloader::ClassLoader;
use crate::reference::{MethodRef, MirrorInstanceRef, Reference};

use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use classfile::accessflags::{ClassAccessFlags, MethodAccessFlags};
use classfile::{ClassFile, ConstantPoolRef, FieldType};
use common::box_slice;
use common::int_types::{u1, u2, u4};
use common::traits::PtrType;
use instructions::Operand;
use symbols::{sym, Symbol};

pub struct Class {
	pub name: Symbol,
	pub access_flags: ClassAccessFlags,
	pub loader: ClassLoader,
	pub super_class: Option<ClassRef>,
	pub interfaces: Vec<ClassRef>,
	pub mirror: Option<MirrorInstanceRef>,

	pub(crate) class_ty: ClassType,

	#[doc(hidden)]
	pub(in crate::heap) init_state: ClassInitializationState,
	#[doc(hidden)]
	pub(in crate::heap) init_lock: Arc<InitializationLock>,
}

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

#[derive(Debug, Clone)]
pub enum ClassType {
	Instance(ClassDescriptor),
	Array(ArrayDescriptor),
}

#[derive(Clone)]
pub struct ClassDescriptor {
	pub source_file_index: Option<u2>,
	pub constant_pool: ConstantPoolRef,
	pub methods: Vec<MethodRef>,
	pub fields: Vec<FieldRef>,
	pub static_field_slots: Box<[Operand<Reference>]>,
	pub instance_field_count: u4,
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

		debug_struct
			.field("methods", &self.methods)
			.field("fields", &self.fields)
			.field("static_field_slots", &self.static_field_slots)
			.field("instance_field_count", &self.instance_field_count)
			.finish()
	}
}

impl ClassDescriptor {
	pub fn find_field<F>(&self, predicate: F) -> Option<FieldRef>
	where
		F: FnMut(&&FieldRef) -> bool,
	{
		self.fields.iter().find(predicate).map(Arc::clone)
	}
}

#[derive(Debug, Clone)]
pub struct ArrayDescriptor {
	pub dimensions: u1,
	pub component: FieldType,
}

impl Class {
	pub fn new(
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
			instance_field_count = super_class.unwrap_class_instance().instance_field_count;
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
			methods: Vec::new(),
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
			mirror: None, // Set later
			class_ty: ClassType::Instance(class_instance),
			init_state: ClassInitializationState::default(),
			init_lock: Arc::default(),
		};

		let classref = ClassPtr::new(class);
		let class = classref.get_mut();

		if let ClassType::Instance(ref mut class_instance) = class.class_ty {
			// Create our Methods...
			class_instance.methods = parsed_file
				.methods
				.iter()
				.map(|mi| Method::new(Arc::clone(&classref), mi))
				.collect();

			// Then the fields...
			let mut fields =
				Vec::with_capacity(instance_field_count as usize + parsed_file.fields.len());
			if let Some(ref super_class) = class.super_class {
				// First we have to inherit the super classes' fields
				for field in &super_class.unwrap_class_instance().fields {
					if !field.is_static() {
						fields.push(Arc::clone(field));
					}
				}
			}

			// Now the fields defined in our class
			let mut static_idx = 0;
			// Continue the index from our existing instance fields
			let mut instance_field_idx = core::cmp::max(0, instance_field_count) as usize;
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
					&class_instance.constant_pool,
				));

				*field_idx += 1;
			}
			class_instance.fields = fields;

			// Update the instance field count if we encountered any new ones
			if instance_field_idx > 0 {
				if instance_field_count > 0 {
					class_instance.instance_field_count +=
						(instance_field_idx as u4) - instance_field_count;
				} else {
					class_instance.instance_field_count = instance_field_idx as u4;
				}
			}
		}

		classref
	}

	pub fn new_array(name: Symbol, component: FieldType, loader: ClassLoader) -> ClassRef {
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
			super_class: Some(
				ClassLoader::lookup_class(sym!(java_lang_Object))
					.expect("java.lang.Object should be loaded"),
			),
			// https://docs.oracle.com/javase/specs/jls/se19/html/jls-4.html#jls-4.10.3
			interfaces: vec![
				ClassLoader::Bootstrap
					.load(sym!(java_lang_Cloneable))
					.unwrap(),
				ClassLoader::Bootstrap
					.load(sym!(java_io_Serializable))
					.unwrap(),
			],
			mirror: None, // Set later
			class_ty: ClassType::Array(array_instance),
			init_state: ClassInitializationState::default(),
			init_lock: Arc::default(),
		};

		ClassPtr::new(class)
	}

	pub fn set_initialization_state(&mut self, state: ClassInitializationState) {
		self.init_state = state;
	}

	pub fn get_main_method(&self) -> Option<MethodRef> {
		const MAIN_METHOD_FLAGS: MethodAccessFlags =
			MethodAccessFlags::ACC_PUBLIC.union(MethodAccessFlags::ACC_STATIC);

		self.get_method(sym!(main_name), sym!(main_signature), MAIN_METHOD_FLAGS)
	}

	pub fn get_method(
		&self,
		name: Symbol,
		descriptor: Symbol,
		flags: MethodAccessFlags,
	) -> Option<MethodRef> {
		if let ClassType::Instance(class_descriptor) = &self.class_ty {
			let search_methods = |class_descriptor: &ClassDescriptor| {
				if let Some(method) = class_descriptor.methods.iter().find(|method| {
					method.name == name
						&& (flags == MethodAccessFlags::NONE
							|| method.access_flags & flags == flags)
						&& method.descriptor == descriptor
				}) {
					return Some(Arc::clone(method));
				}
				None
			};

			if let ret @ Some(_) = search_methods(class_descriptor) {
				return ret;
			}

			for parent in self.parent_iter() {
				if let ret @ Some(_) = search_methods(parent.unwrap_class_instance()) {
					return ret;
				}
			}
		}

		None
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

		target.get_mut().mirror = Some(mirror);
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

	pub fn get_mirror(&self) -> MirrorInstanceRef {
		Arc::clone(self.mirror.as_ref().unwrap())
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
				self.current_class = current.super_class.as_ref().map(Arc::clone);
				self.current_class.clone()
			},
		};
	}
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
