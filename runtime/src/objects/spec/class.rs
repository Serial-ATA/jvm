use crate::class::Class;
use crate::classpath::classloader::ClassLoader;
use crate::java_call;
use crate::method::Method;
use crate::method_invoker::MethodInvoker;
use crate::reference::{ClassRef, FieldRef, Reference};
use crate::string_interner::StringInterner;
use crate::thread::JavaThread;

use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use classfile::accessflags::MethodAccessFlags;
use classfile::{ConstantPoolRef, FieldType};
use common::int_types::u2;
use common::traits::PtrType;
use instructions::Operand;
use symbols::{sym, Symbol};

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
pub(crate) struct InitializationLock(Mutex<()>, Condvar);

#[doc(hidden)]
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

impl Class {
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.2
	#[tracing::instrument(skip_all)]
	pub fn resolve_field(
		classref: ClassRef,
		constant_pool: ConstantPoolRef,
		field_ref_idx: u2,
	) -> Option<FieldRef> {
		// TODO: Double constant pool lookup
		let (_, name_and_type_index) = constant_pool.get_field_ref(field_ref_idx);

		let (name_index, descriptor_index) = constant_pool.get_name_and_type(name_and_type_index);

		let field_name = constant_pool.get_constant_utf8(name_index);
		let mut descriptor = constant_pool.get_constant_utf8(descriptor_index);

		let field_type = FieldType::parse(&mut descriptor).unwrap(); // TODO: Error handling

		// When resolving a field reference, field resolution first attempts to look up
		// the referenced field in C and its superclasses:

		// 1. If C declares a field with the name and descriptor specified by the field reference,
		//    field lookup succeeds. The declared field is the result of the field lookup.
		for field in classref.fields() {
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
				Arc::clone(&classref),
				super_class.unwrap_class_instance().constant_pool.clone(),
				field_ref_idx,
			);
		}

		// 4. Otherwise, field lookup fails.
		panic!("NoSuchFieldError") // TODO
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.3
	#[tracing::instrument(skip_all)]
	pub fn resolve_method<'a>(
		&'a self,
		thread: &mut JavaThread,
		constant_pool: ConstantPoolRef,
		method_ref_idx: u2,
	) -> Option<&'static Method> {
		let (interface_method, class_name_index, name_and_type_index) =
			constant_pool.get_method_ref(method_ref_idx);

		let (method_name_index, method_descriptor_index) =
			constant_pool.get_name_and_type(name_and_type_index);

		let method_name_raw = constant_pool.get_constant_utf8(method_name_index);
		let descriptor_raw = constant_pool.get_constant_utf8(method_descriptor_index);

		let method_name = Symbol::intern_bytes(method_name_raw);
		let descriptor = Symbol::intern_bytes(descriptor_raw);

		let class_name = constant_pool.get_class_name(class_name_index);
		let classref = self.loader.load(Symbol::intern_bytes(class_name)).unwrap();

		if interface_method {
			return Self::resolve_interface_method(thread, classref, method_name, descriptor);
		}

		// When resolving a method reference:

		//  1. If C is an interface, method resolution throws an IncompatibleClassChangeError.
		if classref.get().is_interface() {
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
		method_name: Symbol,
		descriptor: Symbol,
	) -> Option<&'static Method> {
		//    2.1. If C declares exactly one method with the name specified by the method reference, and the declaration
		//         is a signature polymorphic method (§2.9.3), then method lookup succeeds. All the class names mentioned
		//         in the descriptor are resolved (§5.4.3.1).
		let searched_method = class_ref
			.vtable()
			.iter()
			.find(|method| method.name == method_name);
		if let Some(method) = searched_method {
			if method.is_polymorphic() {
				return Some(method);
			}
		}

		// 	  2.2. Otherwise, if C declares a method with the name and descriptor specified by the method reference, method lookup succeeds.
		let searched_method = class_ref
			.vtable()
			.iter()
			.find(|method| method.name == method_name && method.descriptor == descriptor);
		if let Some(method) = searched_method {
			return Some(method);
		}

		// 	  2.3. Otherwise, if C has a superclass, step 2 of method resolution is recursively invoked on the direct superclass of C.
		if let Some(ref super_class) = &class_ref.super_class {
			if let Some(resolved_method) =
				Class::resolve_method_step_two(Arc::clone(super_class), method_name, descriptor)
			{
				return Some(&resolved_method);
			}
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.4
	fn resolve_interface_method(
		_thread: &JavaThread,
		classref: ClassRef,
		method_name: Symbol,
		descriptor: Symbol,
	) -> Option<&'static Method> {
		// When resolving an interface method reference:

		// 1. If C is not an interface, interface method resolution throws an IncompatibleClassChangeError.
		if !classref.is_interface() {
			panic!("IncompatibleClassChangeError"); // TODO
		}

		// 2. Otherwise, if C declares a method with the name and descriptor specified by the interface method reference, method lookup succeeds.
		for method in classref.vtable() {
			if method.name == method_name && method.descriptor == descriptor {
				return Some(method);
			}
		}

		// 3. Otherwise, if the class Object declares a method with the name and descriptor specified by the interface method reference, which has its ACC_PUBLIC flag
		//    set and does not have its ACC_STATIC flag set, method lookup succeeds.
		let object_class = crate::globals::classes::java_lang_Object();
		for method in object_class.vtable() {
			if method.name == method_name
				&& method.descriptor == descriptor
				&& method.is_public()
				&& !method.is_static()
			{
				return Some(method);
			}
		}

		// 4. Otherwise, if the maximally-specific superinterface methods (§5.4.3.3) of C for the name and descriptor specified by the method reference include exactly
		//    one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.
		if let Some(method) = Class::resolve_method_in_superinterfaces(
			method_name,
			descriptor,
			Arc::clone(&classref),
			true,
		) {
			return Some(method);
		}

		// 5. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that has neither its ACC_PRIVATE flag
		//    nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.
		if let Some(method) = Class::resolve_method_in_superinterfaces(
			method_name,
			descriptor,
			Arc::clone(&classref),
			false,
		) {
			return Some(method);
		}

		// 6. Otherwise, method lookup fails.
		panic!("NoSuchMethodError") // TODO
	}

	fn resolve_method_in_superinterfaces(
		method_name: Symbol,
		descriptor: Symbol,
		classref: ClassRef,
		// TODO: Deal with maximally-specific check (§5.4.3.3)
		maximally_specific: bool,
	) -> Option<&'static Method> {
		for interface in &classref.interfaces {
			if let Some(method) = Class::resolve_method_in_superinterfaces(
				method_name,
				descriptor,
				Arc::clone(interface),
				maximally_specific,
			) {
				return Some(method);
			}

			// TODO: Some way to provide negative flags
			if let Some(method) =
				interface
					.vtable()
					.find(method_name, descriptor, MethodAccessFlags::NONE)
			{
				if method.is_private() || method.is_static() {
					continue;
				}

				return Some(method);
			}
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.5
	#[expect(
		unreachable_code,
		reason = "We have no way of checking of the <clinit> executed successfully yet"
	)]
	#[tracing::instrument(skip_all)]
	pub fn initialize(class_ref: &ClassRef, thread: &mut JavaThread) {
		let class = class_ref.get();

		// 1. Synchronize on the initialization lock, LC, for C. This involves waiting until the current thread can acquire LC.
		let init = class.initialization_lock();
		let mut _guard = init.lock();

		// 2. If the Class object for C indicates that initialization is in progress for C by some other thread
		if class.initialization_state() == ClassInitializationState::InProgress
			&& !class.is_initialized_by(thread)
		{
			// then release LC and block the current thread until informed that the in-progress initialization
			// has completed, at which time repeat this procedure.
			_guard = init.wait(_guard);
		}

		// 3. If the Class object for C indicates that initialization is in progress for C by the current thread,
		//    then this must be a recursive request for initialization. Release LC and complete normally.
		if class.initialization_state() == ClassInitializationState::InProgress
			&& class.is_initialized_by(thread)
		{
			panic!();
			return;
		}

		// 4. If the Class object for C indicates that C has already been initialized, then no further action
		//    is required. Release LC and complete normally.
		if class.initialization_state() == ClassInitializationState::Init {
			return;
		}

		// TODO:
		// 5. If the Class object for C is in an erroneous state, then initialization is not possible.
		//    Release LC and throw a NoClassDefFoundError.
		if class.initialization_state() == ClassInitializationState::Failed {
			panic!("NoClassDefFoundError");
		}

		// 6. Otherwise, record the fact that initialization of the Class object for C is in progress
		//    by the current thread, and release LC.
		tracing::debug!(target: "class-init", "Starting initialization of class `{}`", class.name.as_str());
		let class = class_ref.get_mut();

		class.set_initialization_state(ClassInitializationState::InProgress);
		class.set_initializing_thread();
		drop(_guard);

		//  Then, initialize each final static field of C with the constant value in its ConstantValue attribute (§4.7.2),
		//  in the order the fields appear in the ClassFile structure.
		for field in class_ref.static_fields().filter(|field| field.is_final()) {
			let Some(constant_value_index) = field.constant_value_index else {
				continue;
			};

			let class_instance = field.class.unwrap_class_instance_mut();

			match field.descriptor {
				FieldType::Byte
				| FieldType::Char
				| FieldType::Short
				| FieldType::Boolean
				| FieldType::Int => {
					let constant_value = class_instance
						.constant_pool
						.get_integer(constant_value_index);
					class.set_static_field(field.idx, Operand::from(constant_value));
				},
				FieldType::Double => {
					let constant_value = class_instance
						.constant_pool
						.get_double(constant_value_index);
					class.set_static_field(field.idx, Operand::from(constant_value));
				},
				FieldType::Float => {
					let constant_value =
						class_instance.constant_pool.get_float(constant_value_index);
					class.set_static_field(field.idx, Operand::from(constant_value));
				},
				FieldType::Long => {
					let constant_value =
						class_instance.constant_pool.get_long(constant_value_index);
					class.set_static_field(field.idx, Operand::from(constant_value));
				},
				FieldType::Object(ref obj) if &**obj == b"java/lang/String" => {
					let raw_string = class_instance
						.constant_pool
						.get_string(constant_value_index);
					let string_instance = StringInterner::intern_bytes(raw_string);
					class.set_static_field(
						field.idx,
						Operand::Reference(Reference::class(string_instance)),
					);
				},
				_ => unreachable!(),
			}
		}

		// 7. Next, if C is a class rather than an interface, then let SC be its superclass and let SI1, ..., SIn be all
		//    superinterfaces of C [...] that declare at least one non-abstract, non-static method.
		//
		//    For each S in the list [ SC, SI1, ..., SIn ], if S has not yet been initialized, then recursively perform this
		//    entire procedure for S. If necessary, verify and prepare S first.
		if !class_ref.is_interface() {
			if let Some(super_class) = &class_ref.super_class {
				Class::initialize(super_class, thread);
			}

			for interface in &class_ref.interfaces {
				if interface
					.vtable()
					.iter()
					.any(|method| !method.is_abstract() && !method.is_static())
				{
					Class::initialize(interface, thread);
				}
			}

			// TODO:
			//    If the initialization of S completes abruptly because of a thrown exception, then acquire LC, label the Class object
			//    for C as erroneous, notify all waiting threads, release LC, and complete abruptly, throwing the same exception
			//    that resulted from initializing SC.
		}

		// TODO:
		// 8. Next, determine whether assertions are enabled for C by querying its defining loader.

		// 9. Next, execute the class or interface initialization method of C.
		Self::clinit(Arc::clone(class_ref), thread);

		// TODO: We have no way of telling if the method successfully executed
		// 10. If the execution of the class or interface initialization method completes normally,
		//     then acquire LC, label the Class object for C as fully initialized, notify all waiting threads,
		//     release LC, and complete this procedure normally.
		class.set_initialization_state(ClassInitializationState::Init);
		init.notify_all();

		tracing::debug!(target: "class-init", "Finished initialization of class `{}`", class.name.as_str());
		return;

		// TODO:
		// 11. Otherwise, the class or interface initialization method must have completed abruptly by throwing some exception E.

		// 12. Acquire LC, label the Class object for C as erroneous, notify all waiting threads, release LC, and complete this
		//     procedure abruptly with reason E or its replacement as determined in the previous step.
		class.set_initialization_state(ClassInitializationState::Failed);
		init.notify_all();
	}

	// Instance initialization method
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.1
	#[tracing::instrument(skip_all)]
	pub fn construct(
		class: ClassRef,
		thread: &mut JavaThread,
		descriptor: Symbol,
		args: Vec<Operand<Reference>>,
	) {
		// A class has zero or more instance initialization methods, each typically corresponding to a constructor written in the Java programming language.

		// A method is an instance initialization method if all of the following are true:

		//     It is defined in a class (not an interface).
		if class.get().is_interface() {
			return;
		}

		let method = class
			.vtable()
			.find(
				sym!(object_initializer_name), // It has the special name <init>.
				descriptor,                    // It is void (§4.3.3).
				MethodAccessFlags::NONE,
			)
			.unwrap();

		MethodInvoker::invoke_with_args(thread, method, args)
	}

	// Class initialization method
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.2
	#[rustfmt::skip]
	#[tracing::instrument(skip_all)]
	pub fn clinit(class: ClassRef, thread: &mut JavaThread) {
		// A class or interface has at most one class or interface initialization method and is initialized
		// by the Java Virtual Machine invoking that method (§5.5).

		// TODO: Check version number for flags and parameters
		// A method is a class or interface initialization method if all of the following are true:
		let method = class.vtable().find(
			sym!(class_initializer_name),     /* It has the special name <clinit>. */
			sym!(void_method_signature), /* It is void (§4.3.3). */
			MethodAccessFlags::ACC_STATIC    /* In a class file whose version number is 51.0 or above, the method has its ACC_STATIC flag set and takes no arguments (§4.6). */
		);

		if let Some(method) = method {
			java_call!(thread, method);
		}
	}

	/// Find an implementation of an interface method on the target class
	#[tracing::instrument(skip_all)]
	pub fn map_interface_method(class: ClassRef, method: &Method) -> &Method {
		// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.6

		// During execution of an invokeinterface or invokevirtual instruction, a method is
		// selected with respect to:
		//
		// (i) the run-time type of the object on the stack, and
		// (ii) a method that was previously resolved by the instruction.
		//
		// The rules to select a method with respect to a class or interface C and a method mR are as follows:

		// 1. If mR is marked ACC_PRIVATE, then it is the selected method.
		if method.is_private() {
			return method;
		}

		// 2. Otherwise, the selected method is determined by the following lookup procedure:

		//    If C contains a declaration of an instance method m that can override mR (§5.4.5), then m is the selected method.
		for instance_method in class.vtable() {
			if instance_method.can_override(&*method) {
				// We found an implementation
				return instance_method;
			}
		}

		//    Otherwise, if C has a superclass, a search for a declaration of an instance method that can override mR is performed,
		//    starting with the direct superclass of C and continuing with the direct superclass of that class, and so forth, until a
		//    method is found or no further superclasses exist. If a method is found, it is the selected method.
		for parent in class.parent_iter() {
			for instance_method in parent.vtable() {
				if instance_method.can_override(&*method) {
					// We found an implementation
					return instance_method;
				}
			}
		}

		//    Otherwise, the maximally-specific superinterface methods of C are determined (§5.4.3.3). If exactly one matches mR's name
		//    and descriptor and is not abstract, then it is the selected method.
		if let Some(superinterface_method) =
			Class::resolve_method_in_superinterfaces(method.name, method.descriptor, class, true)
		{
			return superinterface_method;
		}

		// No implementation found, return the original method
		method
	}
}
