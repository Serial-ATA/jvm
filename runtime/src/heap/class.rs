use super::field::Field;
use super::method::Method;
use super::reference::{ClassRef, FieldRef};
use crate::class_instance::MirrorInstance;
use crate::classpath::classloader::ClassLoader;
use crate::method_invoker::MethodInvoker;
use crate::reference::{MethodRef, MirrorInstanceRef, Reference};
use crate::string_interner::StringInterner;
use crate::thread::ThreadRef;

use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use classfile::{ClassFile, ConstantPoolRef, FieldType};
use common::int_types::{u1, u2, u4};
use common::traits::PtrType;
use instructions::Operand;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.5
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassInitializationState {
	/// This Class object is verified and prepared but not initialized.
	#[default]
	Uninit,
	/// This Class object is being initialized by some particular thread.
	InProgress,
	/// This Class object is fully initialized and ready for use.
	Init,
	/// This Class object is in an erroneous state, perhaps because initialization was attempted and failed.
	Failed,
}

#[derive(Default, Debug)]
struct InitializationLock(Mutex<()>, Condvar);

impl InitializationLock {
	fn lock(&self) -> MutexGuard<'_, ()> {
		self.0.lock().unwrap()
	}

	fn wait<'a>(&self, guard: MutexGuard<'a, ()>) -> MutexGuard<'a, ()> {
		self.1.wait(guard).unwrap()
	}

	fn notify_all(&self) {
		self.1.notify_all()
	}
}

pub struct Class {
	pub name: Vec<u1>,
	pub access_flags: u2,
	pub loader: ClassLoader,
	pub super_class: Option<ClassRef>,

	pub(crate) class_ty: ClassType,

	init_state: ClassInitializationState,
	init_lock: Arc<InitializationLock>,
}

impl Debug for Class {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Class")
			.field("name", &unsafe {
				std::str::from_utf8_unchecked(&self.name)
			})
			.field("access_flags", &self.access_flags)
			.field("loader", &self.loader)
			.field("super_class", &self.super_class)
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
	pub constant_pool: ConstantPoolRef,
	pub methods: Vec<MethodRef>,
	pub fields: Vec<FieldRef>,
	pub static_field_slots: Box<[Operand<Reference>]>,
	pub instance_field_count: u4,
}

impl Debug for ClassDescriptor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClassDescriptor")
			.field("methods", &self.methods)
			.field("fields", &self.fields)
			.field("static_field_slots", &self.static_field_slots)
			.field("instance_field_count", &self.instance_field_count)
			.finish()
	}
}

#[derive(Debug, Clone)]
pub struct ArrayDescriptor {
	pub dimensions: u1,
	pub component: FieldType,
}

#[rustfmt::skip]
impl Class {
	// Access flags
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.1-200-E.1

	pub const ACC_PUBLIC    : u2 = 0x0001; /* Declared public; may be accessed from outside its package. */
	pub const ACC_FINAL     : u2 = 0x0010; /* Declared final; no subclasses allowed. */
	pub const ACC_SUPER     : u2 = 0x0020; /* Treat superclass methods specially when invoked by the invokespecial instruction. */
	pub const ACC_INTERFACE : u2 = 0x0200; /* Is an interface, not a class. */
	pub const ACC_ABSTRACT  : u2 = 0x0400; /* Declared abstract; must not be instantiated. */
	pub const ACC_SYNTHETIC : u2 = 0x1000; /* Declared synthetic; not present in the source code. */
	pub const ACC_ANNOTATION: u2 = 0x2000; /* Declared as an annotation interface. */
	pub const ACC_ENUM      : u2 = 0x4000; /* Declared as an enum class. */
	pub const ACC_MODULE    : u2 = 0x8000; /* Is a module, not a class or interface. */

	pub const ANY_FLAGS     : u2 = 0x0000; /* NOT PART OF SPEC, used internally when access flags do not matter */
}

impl Class {
	pub fn new(
		parsed_file: ClassFile,
		super_class: Option<ClassRef>,
		loader: ClassLoader,
	) -> ClassRef {
		let access_flags = parsed_file.access_flags;
		let class_name_index = parsed_file.this_class;
		let name = parsed_file
			.constant_pool
			.get_class_name(class_name_index)
			.to_vec();

		let constant_pool = parsed_file.constant_pool;

		let static_field_count = parsed_file
			.fields
			.iter()
			.filter(|field| field.is_static())
			.count();
		let mut instance_field_count = 0;

		if let Some(ref super_class) = super_class {
			instance_field_count = super_class.unwrap_class_instance().instance_field_count;
		}

		let static_field_slots = vec![Operand::Empty; static_field_count].into_boxed_slice();

		// We need the Class instance to create our methods and fields
		let class_instance = ClassDescriptor {
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
						instance_field_count += 1;
					}
				}
			}

			// Now the fields defined in our class
			let mut static_idx = 0;
			let mut instance_field_idx = 0;
			for field in parsed_file.fields {
				let field_idx = if field.is_static() {
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
				class_instance.instance_field_count += instance_field_idx as u4;
			}
		}

		classref
	}

	pub fn new_array(name: &[u1], component: FieldType, loader: ClassLoader) -> ClassRef {
		let dimensions = name.iter().take_while(|char_| **char_ == b'[').count() as u1;

		let array_instance = ArrayDescriptor {
			dimensions,
			component,
		};

		let class = Self {
			name: name.to_vec(),
			access_flags: 0,
			loader,
			super_class: Some(
				ClassLoader::lookup_class(b"java/lang/Object")
					.expect("java.lang.Object should be loaded"),
			),
			class_ty: ClassType::Array(array_instance),
			init_state: ClassInitializationState::default(),
			init_lock: Arc::default(),
		};

		ClassPtr::new(class)
	}

	pub fn get_main_method(&self) -> Option<MethodRef> {
		const MAIN_METHOD_NAME: &[u1] = b"main";
		const MAIN_METHOD_DESCRIPTOR: &[u1] = b"([Ljava/lang/String;)V";
		const MAIN_METHOD_FLAGS: u2 = Method::ACC_PUBLIC | Method::ACC_STATIC;

		self.get_method(MAIN_METHOD_NAME, MAIN_METHOD_DESCRIPTOR, MAIN_METHOD_FLAGS)
	}

	pub fn get_method(&self, name: &[u1], descriptor: &[u1], flags: u2) -> Option<MethodRef> {
		if let ClassType::Instance(class_instance) = &self.class_ty {
			return class_instance
				.methods
				.iter()
				.find(|method| {
					method.name == name
						&& (flags == 0 || method.access_flags & flags == flags)
						&& method.descriptor == descriptor
				})
				.map(Arc::clone);
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.2
	pub fn resolve_field(
		thread: ThreadRef,
		constant_pool: ConstantPoolRef,
		field_ref_idx: u2,
	) -> Option<FieldRef> {
		let (class_name_index, name_and_type_index) = constant_pool.get_field_ref(field_ref_idx);

		let class_name = constant_pool.get_class_name(class_name_index);
		let classref = ClassLoader::Bootstrap.load(class_name).unwrap();

		if classref.get().initialization_state() != ClassInitializationState::Init {
			// TODO: This is a hack, really need a better way to signal to the caller that a class is
			//       initializing and handle the seeking elsewhere
			let _ = thread
				.get_mut()
				.pc
				.fetch_sub(3, std::sync::atomic::Ordering::Relaxed);

			Class::initialize(&classref, Arc::clone(&thread));
			return None;
		}

		let (name_index, descriptor_index) = constant_pool.get_name_and_type(name_and_type_index);

		let field_name = constant_pool.get_constant_utf8(name_index);
		let mut descriptor = constant_pool.get_constant_utf8(descriptor_index);

		let field_type = FieldType::parse(&mut descriptor);

		// When resolving a field reference, field resolution first attempts to look up
		// the referenced field in C and its superclasses:

		// 1. If C declares a field with the name and descriptor specified by the field reference,
		//    field lookup succeeds. The declared field is the result of the field lookup.
		let class_instance = classref.unwrap_class_instance();
		for field in &class_instance.fields {
			if field.name == field_name && field.descriptor == field_type {
				return Some(Arc::clone(field));
			}
		}

		// TODO:
		// 2. Otherwise, field lookup is applied recursively to the direct superinterfaces of the
		//    specified class or interface C.

		// 3. Otherwise, if C has a superclass S, field lookup is applied recursively to S.
		if let Some(super_class) = &classref.get().super_class {
			Class::resolve_field(
				thread,
				super_class.unwrap_class_instance().constant_pool.clone(),
				field_ref_idx,
			);
		}

		// 4. Otherwise, field lookup fails.
		panic!("NoSuchFieldError") // TODO
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.3
	pub fn resolve_method(
		thread: ThreadRef,
		constant_pool: ConstantPoolRef,
		method_ref_idx: u2,
	) -> Option<MethodRef> {
		let (interface_method, class_name_index, name_and_type_index) =
			constant_pool.get_method_ref(method_ref_idx);

		let class_name = constant_pool.get_class_name(class_name_index);
		let classref = ClassLoader::Bootstrap.load(class_name).unwrap();

		if classref.get().initialization_state() != ClassInitializationState::Init {
			// TODO: This is a hack, really need a better way to signal to the caller that a class is
			//       initializing and handle the seeking elsewhere
			let _ = thread
				.get_mut()
				.pc
				.fetch_sub(3, std::sync::atomic::Ordering::Relaxed);

			Class::initialize(&classref, thread);
			return None;
		}

		let (method_name_index, method_descriptor_index) =
			constant_pool.get_name_and_type(name_and_type_index);

		let method_name = constant_pool.get_constant_utf8(method_name_index);
		let descriptor = constant_pool.get_constant_utf8(method_descriptor_index);

		if interface_method {
			return Self::resolve_interface_method(thread, classref, method_name, descriptor);
		}

		// When resolving a method reference:

		//  1. If C is an interface, method resolution throws an IncompatibleClassChangeError.
		if classref.get().access_flags & Class::ACC_INTERFACE != 0 {
			panic!("IncompatibleClassChangeError"); // TODO
		}

		//  2. Otherwise, method resolution attempts to locate the referenced method in C and its superclasses:
		if let ret @ Some(_) = Class::resolve_method_step_two(classref, method_name, descriptor) {
			return ret;
		}

		// TODO: Method resolution in superinterfaces
		//  3. Otherwise, method resolution attempts to locate the referenced method in the superinterfaces of the specified class C:

		//    3.1. If the maximally-specific superinterface methods of C for the name and descriptor specified by the method reference include
		//         exactly one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.

		//    3.2. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that
		//         has neither its ACC_PRIVATE flag nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.

		//    3.3. Otherwise, method lookup fails.
		panic!("NoSuchMethodError") // TODO
	}

	pub fn resolve_method_step_two(
		class_ref: ClassRef,
		method_name: &[u1],
		descriptor: &[u1],
	) -> Option<MethodRef> {
		//    2.1. If C declares exactly one method with the name specified by the method reference, and the declaration
		//         is a signature polymorphic method (§2.9.3), then method lookup succeeds. All the class names mentioned
		//         in the descriptor are resolved (§5.4.3.1).
		let class_instance = class_ref.unwrap_class_instance();
		let searched_method = class_instance
			.methods
			.iter()
			.find(|method| method.name == method_name);
		if let Some(method) = searched_method {
			if method.is_polymorphic() {
				return Some(Arc::clone(method));
			}
		}

		// 	  2.2. Otherwise, if C declares a method with the name and descriptor specified by the method reference, method lookup succeeds.
		let searched_method = class_instance
			.methods
			.iter()
			.find(|method| method.name == method_name && method.descriptor == descriptor);
		if let Some(method) = searched_method {
			return Some(Arc::clone(method));
		}

		// 	  2.3. Otherwise, if C has a superclass, step 2 of method resolution is recursively invoked on the direct superclass of C.
		if let Some(ref super_class) = &class_ref.get().super_class {
			if let Some(resolved_method) =
				Class::resolve_method_step_two(Arc::clone(super_class), method_name, descriptor)
			{
				return Some(Arc::clone(&resolved_method));
			}
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.4
	fn resolve_interface_method(
		_thread: ThreadRef,
		classref: ClassRef,
		method_name: &[u1],
		descriptor: &[u1],
	) -> Option<MethodRef> {
		// When resolving an interface method reference:

		// 1. If C is not an interface, interface method resolution throws an IncompatibleClassChangeError.
		if classref.get().access_flags & Class::ACC_INTERFACE != Class::ACC_INTERFACE {
			panic!("IncompatibleClassChangeError"); // TODO
		}

		// 2. Otherwise, if C declares a method with the name and descriptor specified by the interface method reference, method lookup succeeds.
		let class_instance = classref.unwrap_class_instance();
		for method in &class_instance.methods {
			if method.name == method_name && method.descriptor == descriptor {
				return Some(Arc::clone(method));
			}
		}

		// 3. Otherwise, if the class Object declares a method with the name and descriptor specified by the interface method reference, which has its ACC_PUBLIC flag
		//    set and does not have its ACC_STATIC flag set, method lookup succeeds.
		let object_class = ClassLoader::Bootstrap
			.load(b"java/lang/Object")
			.expect("there's no way we can make it here without loading Object");
		for method in &object_class.unwrap_class_instance().methods {
			if method.name == method_name
				&& method.descriptor == descriptor
				&& method.is_public()
				&& !method.is_static()
			{
				return Some(Arc::clone(method));
			}
		}

		// TODO: Method resolution in superinterfaces
		// 4. Otherwise, if the maximally-specific superinterface methods (§5.4.3.3) of C for the name and descriptor specified by the method reference include exactly
		//    one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.

		// 5. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that has neither its ACC_PRIVATE flag
		//    nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.

		// 6. Otherwise, method lookup fails.
		panic!("NoSuchMethodError") // TODO
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.5
	#[expect(
		unreachable_code,
		reason = "We have no way of checking of the <clinit> executed successfully yet"
	)]
	pub fn initialize(class_ref: &ClassRef, thread: ThreadRef) {
		let class = class_ref.get_mut();

		// 1. Synchronize on the initialization lock, LC, for C. This involves waiting until the current thread can acquire LC.
		let mut _guard = class.init_lock.lock();

		// 2. If the Class object for C indicates that initialization is in progress for C by some other thread
		if class.init_state == ClassInitializationState::InProgress {
			// then release LC and block the current thread until informed that the in-progress initialization
			// has completed, at which time repeat this procedure.
			_guard = class.init_lock.wait(_guard);
		}

		// TODO:
		// 3. If the Class object for C indicates that initialization is in progress for C by the current thread,
		//    then this must be a recursive request for initialization. Release LC and complete normally.

		// 4. If the Class object for C indicates that C has already been initialized, then no further action
		//    is required. Release LC and complete normally.
		if class.init_state == ClassInitializationState::Init {
			return;
		}

		// TODO:
		// 5. If the Class object for C is in an erroneous state, then initialization is not possible.
		//    Release LC and throw a NoClassDefFoundError.
		if class.init_state == ClassInitializationState::Failed {
			panic!("NoClassDefFoundError");
		}

		// TODO: Need to specify thread
		// 6. Otherwise, record the fact that initialization of the Class object for C is in progress
		//    by the current thread, and release LC.
		class.init_state = ClassInitializationState::InProgress;
		drop(_guard);

		let class_instance = class_ref.unwrap_class_instance_mut();

		//  Then, initialize each final static field of C with the constant value in its ConstantValue attribute (§4.7.2),
		//  in the order the fields appear in the ClassFile structure.
		for field in class_instance
			.fields
			.iter_mut()
			.filter(|field| field.is_static() && field.is_final())
		{
			let Some(constant_value_index) = field.constant_value_index else {
				continue
			};

			let class_instance = field.class.unwrap_class_instance_mut();

			match field.descriptor {
				FieldType::Byte
				| FieldType::Char
				| FieldType::Short
				| FieldType::Boolean
				| FieldType::Int => {
					class_instance.static_field_slots[field.idx] = Operand::Int(
						class_instance
							.constant_pool
							.get_integer(constant_value_index),
					)
				},
				FieldType::Double => {
					class_instance.static_field_slots[field.idx] = Operand::Double(
						class_instance
							.constant_pool
							.get_double(constant_value_index),
					)
				},
				FieldType::Float => {
					class_instance.static_field_slots[field.idx] =
						Operand::Float(class_instance.constant_pool.get_float(constant_value_index))
				},
				FieldType::Long => {
					class_instance.static_field_slots[field.idx] =
						Operand::Long(class_instance.constant_pool.get_long(constant_value_index))
				},
				FieldType::Object(ref obj) if obj == "java/lang/String" => {
					let raw_string = class_instance
						.constant_pool
						.get_string(constant_value_index);
					let string_instance =
						StringInterner::intern_string(raw_string, Arc::clone(&thread));
					class_instance.static_field_slots[field.idx] =
						Operand::Reference(Reference::Class(string_instance));
				},
				_ => unreachable!(),
			}
		}

		// TODO:
		// 7. Next, if C is a class rather than an interface, then let SC be its superclass and let SI1, ..., SIn be all
		//    superinterfaces of C [...] that declare at least one non-abstract, non-static method.
		//
		//    For each S in the list [ SC, SI1, ..., SIn ], if S has not yet been initialized, then recursively perform this
		//    entire procedure for S. If necessary, verify and prepare S first.

		//    If the initialization of S completes abruptly because of a thrown exception, then acquire LC, label the Class object
		//    for C as erroneous, notify all waiting threads, release LC, and complete abruptly, throwing the same exception
		//    that resulted from initializing SC.

		// TODO:
		// 8. Next, determine whether assertions are enabled for C by querying its defining loader.

		// 9. Next, execute the class or interface initialization method of C.
		Self::clinit(Arc::clone(class_ref), thread);

		// TODO: We have no way of telling if the method successfully executed
		// 10. If the execution of the class or interface initialization method completes normally,
		//     then acquire LC, label the Class object for C as fully initialized, notify all waiting threads,
		//     release LC, and complete this procedure normally.
		class.init_state = ClassInitializationState::Init;
		class.init_lock.notify_all();
		return;

		// TODO:
		// 11. Otherwise, the class or interface initialization method must have completed abruptly by throwing some exception E.

		// 12. Acquire LC, label the Class object for C as erroneous, notify all waiting threads, release LC, and complete this
		//     procedure abruptly with reason E or its replacement as determined in the previous step.
		class.init_state = ClassInitializationState::Failed;
		class.init_lock.notify_all();
	}

	// Instance initialization method
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.1
	pub fn construct(
		class: ClassRef,
		thread: ThreadRef,
		descriptor: &[u1],
		args: Vec<Operand<Reference>>,
	) {
		const CONSTRUCTOR_METHOD_NAME: &[u1] = b"<init>";

		// A class has zero or more instance initialization methods, each typically corresponding to a constructor written in the Java programming language.

		// A method is an instance initialization method if all of the following are true:

		//     It is defined in a class (not an interface).
		if class.get().access_flags & Class::ACC_INTERFACE > 0 {
			return;
		}

		let method = class
			.get()
			.get_method(
				CONSTRUCTOR_METHOD_NAME, // It has the special name <init>.
				descriptor,              // It is void (§4.3.3).
				Class::ANY_FLAGS,
			)
			.unwrap();

		MethodInvoker::invoke_with_args(thread, method, args)
	}

	// Class initialization method
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.2
	#[rustfmt::skip]
	pub fn clinit(class: ClassRef, thread: ThreadRef) {
		// A class or interface has at most one class or interface initialization method and is initialized
		// by the Java Virtual Machine invoking that method (§5.5).

		// TODO: Check version number for flags and parameters
		// A method is a class or interface initialization method if all of the following are true:
		let method = class.get().get_method(
			b"<clinit>",        /* It has the special name <clinit>. */
			b"()V",        /* It is void (§4.3.3). */
			Method::ACC_STATIC /* In a class file whose version number is 51.0 or above, the method has its ACC_STATIC flag set and takes no arguments (§4.6). */
		);

		if let Some(method) = method {
			MethodInvoker::invoke_with_args(thread, method, Vec::new());
		}
	}

	pub fn initialization_state(&self) -> ClassInitializationState {
		self.init_state
	}

	pub fn create_mirrored(class: ClassRef) -> MirrorInstanceRef {
		let mirror_class = ClassLoader::lookup_class(b"java/lang/Class")
			.expect("java.lang.Class should be loaded at this point");

		MirrorInstance::new(mirror_class, class)
	}
}

impl Clone for Class {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			access_flags: self.access_flags,
			super_class: self.super_class.clone(),
			class_ty: self.class_ty.clone(),
			loader: self.loader,
			init_state: self.init_state,
			init_lock: Arc::new(InitializationLock::default()),
		}
	}
}

// A pointer to a Class instance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the class.
#[derive(PartialEq)]
pub struct ClassPtr(usize);

impl ClassPtr {
	pub fn unwrap_class_instance(&self) -> &ClassDescriptor {
		match self.get().class_ty {
			ClassType::Instance(ref instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_class_instance_mut(&self) -> &mut ClassDescriptor {
		match self.get_mut().class_ty {
			ClassType::Instance(ref mut instance) => instance,
			_ => unreachable!(),
		}
	}

	pub fn unwrap_array_instance(&self) -> &ArrayDescriptor {
		match self.get().class_ty {
			ClassType::Array(ref instance) => instance,
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

impl Debug for ClassPtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(unsafe { std::str::from_utf8_unchecked(&class.name) })
	}
}
