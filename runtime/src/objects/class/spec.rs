use crate::globals::PRIMITIVES;
use crate::java_call;
use crate::method_invoker::MethodInvoker;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::cell::UnsafeCell;
use std::sync::{Condvar, Mutex, MutexGuard};

use classfile::FieldType;
use classfile::accessflags::MethodAccessFlags;
use instructions::Operand;

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
	pub fn is_initialized_by(&self, thread: &'static JavaThread) -> bool {
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
			if S_class == T_class {
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
			return T_class == crate::globals::classes::java_lang_Object();
		}
		//     If T is an interface type, then T must be one of the interfaces implemented by arrays (JLS §4.10.3).
		if T_class.is_interface() {
			return T_class == crate::globals::classes::java_lang_Cloneable()
				|| T_class == crate::globals::classes::java_io_Serializable();
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
			let S_class = S_class.loader().lookup_class(source_component).unwrap();
			let T_class = T_class.loader().lookup_class(dest_component).unwrap();
			return S_class.can_cast_to(T_class);
		}

		false
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4
	fn link(&self) {
		// Linking a class or interface involves verifying and preparing that class or interface, its direct superclass,
		// its direct superinterfaces, and its element type (if it is an array type), if necessary.
		// Linking also involves resolution of symbolic references in the class or interface, though
		// not necessarily at the same time as the class or interface is verified and prepared.
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.2
	pub fn prepare(&self) -> Throws<()> {
		// Preparation involves creating the static fields for a class or interface and initializing such fields
		// to their default values (§2.3, §2.4). This does not require the execution of any Java Virtual Machine code;
		// explicit initializers for static fields are executed as part of initialization (§5.5), not preparation.
		for field in self.static_fields() {
			let value = Field::default_value_for_ty(&field.descriptor);
			unsafe {
				self.set_static_field(field.index(), value);
			}
		}

		// During preparation of a class or interface C, the Java Virtual Machine also imposes loading constraints (§5.3.4):

		// Let L1 be the defining loader of C. For each instance method m declared in C that can override (§5.4.5) an instance
		// method declared in a superclass or superinterface D = <N2, L2>, for each class or interface name N mentioned by the
		// descriptor of m (§4.3.3), the Java Virtual Machine imposes the loading constraint NL1 = NL2.

		// For each instance method m declared in a superinterface I = <N3, L3> of C, if C does not itself declare an instance
		// method that can override m, then a method is selected (§5.4.6) with respect to C and the method m in I.
		// Let D = <N2, L2> be the class or interface that declares the selected method. For each class or interface name N mentioned
		// by the descriptor of m, the Java Virtual Machine imposes the loading constraint NL2 = NL3.

		Throws::Ok(())
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3.2
	#[tracing::instrument(skip_all)]
	pub fn resolve_field(&self, name: Symbol, descriptor: Symbol) -> Throws<&'static Field> {
		fn inner(class: &Class, name: Symbol, descriptor: Symbol) -> Option<&'static Field> {
			// When resolving a field reference, field resolution first attempts to look up
			// the referenced field in C and its superclasses:

			// 1. If C declares a field with the name and descriptor specified by the field reference,
			//    field lookup succeeds. The declared field is the result of the field lookup.
			for field in class.fields() {
				if field.name == name && field.descriptor_sym == descriptor {
					return Some(field);
				}
			}

			// TODO:
			// 2. Otherwise, field lookup is applied recursively to the direct superinterfaces of the
			//    specified class or interface C.

			// 3. Otherwise, if C has a superclass S, field lookup is applied recursively to S.
			if let Some(super_class) = &class.super_class {
				if let Some(field) = inner(super_class, name, descriptor) {
					return Some(field);
				}
			}

			None
		}

		// 4. Otherwise, field lookup fails.
		match inner(self, name, descriptor) {
			Some(field) => Throws::Ok(field),
			None => throw!(@DEFER NoSuchFieldError),
		}
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3.3
	#[tracing::instrument(skip_all)]
	pub fn resolve_method<'a>(&self, name: Symbol, descriptor: Symbol) -> Throws<&'static Method> {
		// When resolving a method reference:

		//  1. If C is an interface, method resolution throws an IncompatibleClassChangeError.
		if self.is_interface() {
			throw!(@DEFER IncompatibleClassChangeError);
		}

		//  2. Otherwise, method resolution attempts to locate the referenced method in C and its superclasses:
		if let Some(method) = self.resolve_method_step_two(name, descriptor) {
			return Throws::Ok(method);
		}

		//  3. Otherwise, method resolution attempts to locate the referenced method in the superinterfaces of the specified class C:

		//    3.1. If the maximally-specific superinterface methods of C for the name and descriptor specified by the method reference include
		//         exactly one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(name, descriptor, true) {
			return Throws::Ok(method);
		}

		//    3.2. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that
		//         has neither its ACC_PRIVATE flag nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(name, descriptor, false) {
			return Throws::Ok(method);
		}

		//    3.3. Otherwise, method lookup fails.
		throw!(@DEFER NoSuchMethodError);
	}

	pub fn resolve_method_step_two(
		&self,
		method_name: Symbol,
		descriptor: Symbol,
	) -> Option<&'static Method> {
		//    2.1. If C declares exactly one method with the name specified by the method reference, and the declaration
		//         is a signature polymorphic method (§2.9.3), then method lookup succeeds. All the class names mentioned
		//         in the descriptor are resolved (§5.4.3.1).
		let mut searched_method = None;
		for method in self
			.vtable()
			.iter()
			.filter(|method| method.name == method_name)
		{
			match searched_method {
				Some(_) => {
					searched_method = None;
					break;
				},
				None => {
					if method.is_signature_polymorphic() {
						searched_method = Some(method)
					}
				},
			}
		}
		if let Some(method) = searched_method {
			return Some(method);
		}

		// 	  2.2. Otherwise, if C declares a method with the name and descriptor specified by the method reference, method lookup succeeds.
		let searched_method = self
			.vtable()
			.iter()
			.find(|method| method.name == method_name && method.descriptor_sym() == descriptor);
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
		name: Symbol,
		descriptor: Symbol,
	) -> Throws<&'static Method> {
		// When resolving an interface method reference:

		// 1. If C is not an interface, interface method resolution throws an IncompatibleClassChangeError.
		if !self.is_interface() {
			throw!(@DEFER IncompatibleClassChangeError);
		}

		// 2. Otherwise, if C declares a method with the name and descriptor specified by the interface method reference, method lookup succeeds.
		for method in self.vtable() {
			if method.name == name && method.descriptor_sym() == descriptor {
				return Throws::Ok(method);
			}
		}

		// 3. Otherwise, if the class Object declares a method with the name and descriptor specified by the interface method reference, which has its ACC_PUBLIC flag
		//    set and does not have its ACC_STATIC flag set, method lookup succeeds.
		let object_class = crate::globals::classes::java_lang_Object();
		for method in object_class.vtable() {
			if method.name == name
				&& method.descriptor_sym() == descriptor
				&& method.is_public()
				&& !method.is_static()
			{
				return Throws::Ok(method);
			}
		}

		// 4. Otherwise, if the maximally-specific superinterface methods (§5.4.3.3) of C for the name and descriptor specified by the method reference include exactly
		//    one method that does not have its ACC_ABSTRACT flag set, then this method is chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(name, descriptor, true) {
			return Throws::Ok(method);
		}

		// 5. Otherwise, if any superinterface of C declares a method with the name and descriptor specified by the method reference that has neither its ACC_PRIVATE flag
		//    nor its ACC_STATIC flag set, one of these is arbitrarily chosen and method lookup succeeds.
		if let Some(method) = self.resolve_method_in_superinterfaces(name, descriptor, false) {
			return Throws::Ok(method);
		}

		// 6. Otherwise, method lookup fails.
		throw!(@DEFER NoSuchMethodError);
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
	pub fn initialization(&self, thread: &'static JavaThread) -> Throws<()> {
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
			return Throws::Ok(());
		}

		// 4. If the Class object for C indicates that C has already been initialized, then no further action
		//    is required. Release LC and complete normally.
		if guard.initialization_state() == ClassInitializationState::Init {
			return Throws::Ok(());
		}

		// 5. If the Class object for C is in an erroneous state, then initialization is not possible.
		//    Release LC and throw a NoClassDefFoundError.
		if guard.initialization_state() == ClassInitializationState::Failed {
			throw!(@DEFER NoClassDefFoundError);
		}

		// 6. Otherwise, record the fact that initialization of the Class object for C is in progress
		//    by the current thread, and release LC.
		tracing::debug!(target: "class-init-start", "Starting initialization of class `{}`", self.name());

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
				| FieldType::Character
				| FieldType::Short
				| FieldType::Boolean
				| FieldType::Integer => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Integer>(constant_value_index)
						.expect("numeric constants should always resolve");
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Double => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Double>(constant_value_index)
						.expect("numeric constants should always resolve");
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Float => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Float>(constant_value_index)
						.expect("numeric constants should always resolve");
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Long => {
					let constant_value = class_instance
						.constant_pool
						.get::<cp_types::Long>(constant_value_index)
						.expect("numeric constants should always resolve");
					let value = Operand::from(constant_value);
					unsafe {
						self.set_static_field(field.index(), value);
					}
				},
				FieldType::Object(ref obj) if &**obj == b"java/lang/String" => {
					let string = class_instance
						.constant_pool
						.get::<cp_types::String>(constant_value_index)
						.expect("string constants should always resolve");
					let string_instance = StringInterner::intern(string);
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
				super_class.initialize(thread)?;
			}

			for interface in &self.interfaces {
				if interface
					.vtable()
					.iter()
					.any(|method| !method.is_abstract() && !method.is_static())
				{
					interface.initialize(thread)?;
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
		if Self::clinit(self, thread) {
			// 10. If the execution of the class or interface initialization method completes normally,
			//     then acquire LC, label the Class object for C as fully initialized, notify all waiting threads,
			//     release LC, and complete this procedure normally.
			let guard = init.lock();
			guard.set_initialization_state(ClassInitializationState::Init);
			init.notify_all();

			tracing::debug!(target: "class-init", "Finished initialization of class `{}`", self.name());
			return Throws::Ok(());
		}

		// 11. Otherwise, the class or interface initialization method must have completed abruptly by throwing some exception E.

		// 12. Acquire LC, label the Class object for C as erroneous, notify all waiting threads, release LC, and complete this
		//     procedure abruptly with reason E or its replacement as determined in the previous step.
		let guard = init.lock();
		guard.set_initialization_state(ClassInitializationState::Failed);
		init.notify_all();

		Throws::PENDING_EXCEPTION
	}

	// Instance initialization method
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.9.1
	#[tracing::instrument(skip_all)]
	pub fn construct(
		&self,
		thread: &'static JavaThread,
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
	fn clinit(&self, thread: &'static JavaThread) -> bool {
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

		!thread.has_pending_exception()
	}

	/// Finds an implementation of a method via [method selection]
	///
	/// This is used in both `invokeinterface` and `invokevirtual`
	///
	/// [method selection]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.6
	#[tracing::instrument(skip_all)]
	#[allow(non_snake_case)]
	pub fn select_method(&self, method: &'static Method) -> &'static Method {
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
			self.resolve_method_in_superinterfaces(mR.name, mR.descriptor_sym(), true)
		{
			return superinterface_method;
		}

		method
	}
}
