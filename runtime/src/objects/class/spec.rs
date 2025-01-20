use crate::java_call;
use crate::method_invoker::MethodInvoker;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::JavaThread;

use std::cell::UnsafeCell;
use std::sync::{Condvar, Mutex, MutexGuard};

use classfile::accessflags::MethodAccessFlags;
use classfile::FieldType;
use instructions::Operand;
use symbols::{sym, Symbol};

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.5
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

pub(super) struct InitializationLock(Mutex<InitializationGuard>, Condvar);

impl InitializationLock {
	pub fn new() -> Self {
		let mutex = Mutex::new(InitializationGuard {
			init_thread: UnsafeCell::new(None),
			init_state: UnsafeCell::new(ClassInitializationState::default()),
		});
		Self(mutex, Condvar::new())
	}
}

impl InitializationLock {
	pub fn lock(&self) -> MutexGuard<'_, InitializationGuard> {
		self.0.lock().unwrap()
	}

	fn wait<'a>(
		&self,
		guard: MutexGuard<'a, InitializationGuard>,
	) -> MutexGuard<'a, InitializationGuard> {
		self.1.wait(guard).unwrap()
	}

	fn notify_all(&self) {
		self.1.notify_all()
	}
}

pub(super) struct InitializationGuard {
	init_thread: UnsafeCell<Option<*const JavaThread>>,
	init_state: UnsafeCell<ClassInitializationState>,
}

impl InitializationGuard {
	/// Set the thread that initialized this class
	///
	/// # Panics
	///
	/// This will panic if called more than once for this class.
	fn set_initializing_thread(&self) {
		let init_thread = self.init_thread.get();
		unsafe {
			assert!(
				(&*init_thread).is_none(),
				"A class initialization thread can only be set once"
			);

			*init_thread = Some(JavaThread::current_ptr());
		}
	}

	/// Set the initialization state of this class
	fn set_initialization_state(&self, state: ClassInitializationState) {
		let init_state = self.init_state.get();
		unsafe {
			*init_state = state;
		}
	}

	/// Whether `thread` initiated the initialization of this class
	#[allow(trivial_casts)]
	pub fn is_initialized_by(&self, thread: &JavaThread) -> bool {
		// SAFETY: We hold the lock, no one can write to this, reads are safe.
		let init_thread = unsafe { *self.init_thread.get() };
		match init_thread {
			Some(init_thread) => init_thread == (thread as _),
			None => false,
		}
	}

	/// Get the initialization state of this class
	pub fn initialization_state(&self) -> ClassInitializationState {
		let init_state = unsafe { *self.init_state.get() };
		init_state
	}
}

impl Class {
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4
	fn link(&self) {
		// Linking a class or interface involves verifying and preparing that class or interface, its direct superclass,
		// its direct superinterfaces, and its element type (if it is an array type), if necessary.
		// Linking also involves resolution of symbolic references in the class or interface, though
		// not necessarily at the same time as the class or interface is verified and prepared.
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.2
	pub fn prepare(&self) {
		// Preparation involves creating the static fields for a class or interface and initializing such fields
		// to their default values (§2.3, §2.4). This does not require the execution of any Java Virtual Machine code;
		// explicit initializers for static fields are executed as part of initialization (§5.5), not preparation.
		let mut prepared_fields = Vec::new();
		for (idx, field) in self.static_fields().enumerate() {
			prepared_fields.push((field, idx));
		}

		for (field, idx) in prepared_fields {
			let value = Field::default_value_for_ty(&field.descriptor);
			unsafe {
				self.set_static_field(idx, value);
			}
		}

		// TODO:
		// During preparation of a class or interface C, the Java Virtual Machine also imposes loading constraints (§5.3.4):
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3.2
	#[tracing::instrument(skip_all)]
	pub fn resolve_field(&self, name: Symbol, descriptor: Symbol) -> Option<&'static Field> {
		let field_type = FieldType::parse(&mut descriptor.as_bytes()).unwrap(); // TODO: Error handling

		// When resolving a field reference, field resolution first attempts to look up
		// the referenced field in C and its superclasses:

		// 1. If C declares a field with the name and descriptor specified by the field reference,
		//    field lookup succeeds. The declared field is the result of the field lookup.
		for field in self.fields() {
			if field.name == name && field.descriptor == field_type {
				return Some(field);
			}
		}

		// TODO:
		// 2. Otherwise, field lookup is applied recursively to the direct superinterfaces of the
		//    specified class or interface C.

		// 3. Otherwise, if C has a superclass S, field lookup is applied recursively to S.
		if let Some(super_class) = &self.super_class {
			if let Some(field) = super_class.resolve_field(name, descriptor) {
				return Some(field);
			}
		}

		// 4. Otherwise, field lookup fails.
		panic!("NoSuchFieldError") // TODO
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3.3
	#[tracing::instrument(skip_all)]
	pub fn resolve_method<'a>(&self, name: Symbol, descriptor: Symbol) -> Option<&'static Method> {
		// When resolving a method reference:

		//  1. If C is an interface, method resolution throws an IncompatibleClassChangeError.
		if self.is_interface() {
			panic!("IncompatibleClassChangeError"); // TODO
		}

		//  2. Otherwise, method resolution attempts to locate the referenced method in C and its superclasses:
		if let ret @ Some(_) = self.resolve_method_step_two(name, descriptor) {
			return ret;
		}

		//  3. Otherwise, method resolution attempts to locate the referenced method in the superinterfaces of the specified class C:

		//    3.1. If the maximally-specific superinterface methods of C for the name and descriptor specified by the method reference include
		//         exactly one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(name, descriptor, true) {
			return Some(method);
		}

		//    3.2. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that
		//         has neither its ACC_PRIVATE flag nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(name, descriptor, false) {
			return Some(method);
		}

		//    3.3. Otherwise, method lookup fails.
		panic!("NoSuchMethodError") // TODO
	}

	pub fn resolve_method_step_two(
		&self,
		method_name: Symbol,
		descriptor: Symbol,
	) -> Option<&'static Method> {
		//    2.1. If C declares exactly one method with the name specified by the method reference, and the declaration
		//         is a signature polymorphic method (§2.9.3), then method lookup succeeds. All the class names mentioned
		//         in the descriptor are resolved (§5.4.3.1).
		let searched_method = self
			.vtable()
			.iter()
			.find(|method| method.name == method_name);
		if let Some(method) = searched_method {
			if method.is_polymorphic() {
				return Some(method);
			}
		}

		// 	  2.2. Otherwise, if C declares a method with the name and descriptor specified by the method reference, method lookup succeeds.
		let searched_method = self
			.vtable()
			.iter()
			.find(|method| method.name == method_name && method.descriptor_sym == descriptor);
		if let Some(method) = searched_method {
			return Some(method);
		}

		// 	  2.3. Otherwise, if C has a superclass, step 2 of method resolution is recursively invoked on the direct superclass of C.
		if let Some(super_class) = self.super_class {
			if let Some(resolved_method) =
				super_class.resolve_method_step_two(method_name, descriptor)
			{
				return Some(&resolved_method);
			}
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3.4
	pub fn resolve_interface_method(
		&self,
		method_name: Symbol,
		descriptor: Symbol,
	) -> Option<&'static Method> {
		// When resolving an interface method reference:

		// 1. If C is not an interface, interface method resolution throws an IncompatibleClassChangeError.
		if !self.is_interface() {
			panic!("IncompatibleClassChangeError"); // TODO
		}

		// 2. Otherwise, if C declares a method with the name and descriptor specified by the interface method reference, method lookup succeeds.
		for method in self.vtable() {
			if method.name == method_name && method.descriptor_sym == descriptor {
				return Some(method);
			}
		}

		// 3. Otherwise, if the class Object declares a method with the name and descriptor specified by the interface method reference, which has its ACC_PUBLIC flag
		//    set and does not have its ACC_STATIC flag set, method lookup succeeds.
		let object_class = crate::globals::classes::java_lang_Object();
		for method in object_class.vtable() {
			if method.name == method_name
				&& method.descriptor_sym == descriptor
				&& method.is_public()
				&& !method.is_static()
			{
				return Some(method);
			}
		}

		// 4. Otherwise, if the maximally-specific superinterface methods (§5.4.3.3) of C for the name and descriptor specified by the method reference include exactly
		//    one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(method_name, descriptor, true)
		{
			return Some(method);
		}

		// 5. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that has neither its ACC_PRIVATE flag
		//    nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(method_name, descriptor, false)
		{
			return Some(method);
		}

		// 6. Otherwise, method lookup fails.
		panic!("NoSuchMethodError") // TODO
	}

	fn resolve_method_in_superinterfaces(
		&self,
		method_name: Symbol,
		descriptor: Symbol,
		maximally_specific: bool,
	) -> Option<&'static Method> {
		// A maximally-specific superinterface method of a class or interface C for a particular method name and descriptor is any method for which all of the following are true:
		//
		//     The method is declared in a superinterface (direct or indirect) of C.
		//
		//     The method is declared with the specified name and descriptor.
		//
		//     The method has neither its ACC_PRIVATE flag nor its ACC_STATIC flag set.
		//
		//     Where the method is declared in interface I, there exists no other maximally-specific superinterface method of C with the specified name and descriptor that is declared in a subinterface of I.
		let mut maximally_specific_definition = None;

		for interface in &self.interfaces {
			if let Some(method) = interface.resolve_method_in_superinterfaces(
				method_name,
				descriptor,
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

				if maximally_specific {
					if method.is_abstract() {
						continue;
					}

					match maximally_specific_definition {
						Some(_) => maximally_specific_definition = None,
						None => maximally_specific_definition = Some(method),
					}

					continue;
				}

				return Some(method);
			}
		}

		if maximally_specific {
			return maximally_specific_definition;
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.5
	#[expect(
		unreachable_code,
		reason = "We have no way of checking of the <clinit> executed successfully yet"
	)]
	#[tracing::instrument(skip_all)]
	pub fn initialization(&self, thread: &JavaThread) {
		// 1. Synchronize on the initialization lock, LC, for C. This involves waiting until the current thread can acquire LC.
		let init = self.initialization_lock();
		let mut guard = init.lock();

		// 2. If the Class object for C indicates that initialization is in progress for C by some other thread
		if guard.initialization_state() == ClassInitializationState::InProgress
			&& !guard.is_initialized_by(thread)
		{
			// then release LC and block the current thread until informed that the in-progress initialization
			// has completed, at which time repeat this procedure.
			guard = init.wait(guard);
		}

		// 3. If the Class object for C indicates that initialization is in progress for C by the current thread,
		//    then this must be a recursive request for initialization. Release LC and complete normally.
		if guard.initialization_state() == ClassInitializationState::InProgress
			&& guard.is_initialized_by(thread)
		{
			return;
		}

		// 4. If the Class object for C indicates that C has already been initialized, then no further action
		//    is required. Release LC and complete normally.
		if guard.initialization_state() == ClassInitializationState::Init {
			return;
		}

		// TODO:
		// 5. If the Class object for C is in an erroneous state, then initialization is not possible.
		//    Release LC and throw a NoClassDefFoundError.
		if guard.initialization_state() == ClassInitializationState::Failed {
			panic!("NoClassDefFoundError");
		}

		// 6. Otherwise, record the fact that initialization of the Class object for C is in progress
		//    by the current thread, and release LC.
		tracing::debug!(target: "class-init", "Starting initialization of class `{}`", self.name.as_str());

		guard.set_initialization_state(ClassInitializationState::InProgress);
		guard.set_initializing_thread();
		drop(guard);

		//  Then, initialize each final static field of C with the constant value in its ConstantValue attribute (§4.7.2),
		//  in the order the fields appear in the ClassFile structure.
		for field in self.static_fields().filter(|field| field.is_final()) {
			let Some(constant_value_index) = field.constant_value_index else {
				continue;
			};

			let class_instance = field.class.unwrap_class_instance();

			match field.descriptor {
				FieldType::Byte
				| FieldType::Char
				| FieldType::Short
				| FieldType::Boolean
				| FieldType::Int => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Integer>(constant_value_index);
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Double => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Double>(constant_value_index);
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Float => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Float>(constant_value_index);
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Long => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Long>(constant_value_index);
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Object(ref obj) if &**obj == b"java/lang/String" => {
					let raw_string = class_instance
						.constant_pool
						.get::<cp_types::String>(constant_value_index);
					let string_instance = StringInterner::intern_symbol(raw_string);
					let value = Operand::Reference(Reference::class(string_instance));
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				_ => unreachable!(),
			}
		}

		// 7. Next, if C is a class rather than an interface, then let SC be its superclass and let SI1, ..., SIn be all
		//    superinterfaces of C [...] that declare at least one non-abstract, non-static method.
		//
		//    For each S in the list [ SC, SI1, ..., SIn ], if S has not yet been initialized, then recursively perform this
		//    entire procedure for S. If necessary, verify and prepare S first.
		if !self.is_interface() {
			if let Some(super_class) = &self.super_class {
				super_class.initialize(thread);
			}

			for interface in &self.interfaces {
				if interface
					.vtable()
					.iter()
					.any(|method| !method.is_abstract() && !method.is_static())
				{
					interface.initialize(thread);
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
		Self::clinit(self, thread);

		// TODO: We have no way of telling if the method successfully executed
		// 10. If the execution of the class or interface initialization method completes normally,
		//     then acquire LC, label the Class object for C as fully initialized, notify all waiting threads,
		//     release LC, and complete this procedure normally.
		let guard = init.lock();
		guard.set_initialization_state(ClassInitializationState::Init);
		init.notify_all();

		tracing::debug!(target: "class-init", "Finished initialization of class `{}`", self.name.as_str());
		return;

		// TODO:
		// 11. Otherwise, the class or interface initialization method must have completed abruptly by throwing some exception E.

		// 12. Acquire LC, label the Class object for C as erroneous, notify all waiting threads, release LC, and complete this
		//     procedure abruptly with reason E or its replacement as determined in the previous step.
		guard.set_initialization_state(ClassInitializationState::Failed);
		init.notify_all();
	}

	// Instance initialization method
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.9.1
	#[tracing::instrument(skip_all)]
	pub fn construct(
		&self,
		thread: &JavaThread,
		descriptor: Symbol,
		args: Vec<Operand<Reference>>,
	) {
		// A class has zero or more instance initialization methods, each typically corresponding to a constructor written in the Java programming language.

		// A method is an instance initialization method if all of the following are true:

		//     It is defined in a class (not an interface).
		if self.is_interface() {
			return;
		}

		let method = self
			.vtable()
			.find_local(
				sym!(object_initializer_name), // It has the special name <init>.
				descriptor,                    // It is void (§4.3.3).
				MethodAccessFlags::NONE,
			)
			.unwrap();

		MethodInvoker::invoke_with_args(thread, method, args)
	}

	// Class initialization method
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.9.2
    #[rustfmt::skip]
	#[tracing::instrument(skip_all)]
	fn clinit(&self, thread: &JavaThread) {
		// A class or interface has at most one class or interface initialization method and is initialized
		// by the Java Virtual Machine invoking that method (§5.5).

		// TODO: Check version number for flags and parameters
		// A method is a class or interface initialization method if all of the following are true:
		let method = self.vtable().find_local(
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
	#[allow(non_snake_case)]
	pub fn map_interface_method(&self, method: &'static Method) -> &'static Method {
		// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.6

		// During execution of an invokeinterface or invokevirtual instruction, a method is
		// selected with respect to:
		//
		// (i) the run-time type of the object on the stack, and
		// (ii) a method that was previously resolved by the instruction.
		//
		// The rules to select a method with respect to a class or interface C and a method mR are as follows:
		let mR = method;

		// 1. If mR is marked ACC_PRIVATE, then it is the selected method.
		if mR.is_private() {
			return mR;
		}

		// 2. Otherwise, the selected method is determined by the following lookup procedure:

		//    If C contains a declaration of an instance method m that can override mR (§5.4.5), then m is the selected method.
		//
		//    Otherwise, if C has a superclass, a search for a declaration of an instance method that can override mR is performed,
		//    starting with the direct superclass of C and continuing with the direct superclass of that class, and so forth, until a
		//    method is found or no further superclasses exist. If a method is found, it is the selected method.

		// (This covers both cases. A class VTable also holds the methods of all super classes)
		for instance_method in self.vtable() {
			if instance_method.can_override(mR) {
				return instance_method;
			}
		}

		//    Otherwise, the maximally-specific superinterface methods of C are determined (§5.4.3.3). If exactly one matches mR's name
		//    and descriptor and is not abstract, then it is the selected method.
		if let Some(superinterface_method) =
			self.resolve_method_in_superinterfaces(mR.name, mR.descriptor_sym, true)
		{
			return superinterface_method;
		}

		// No implementation found
		panic!("No viable methods for method selection");
	}
}
