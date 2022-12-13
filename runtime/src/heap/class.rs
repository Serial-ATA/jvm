use super::field::Field;
use super::method::Method;
use super::reference::{ClassRef, FieldRef};
use crate::classpath::classloader::ClassLoader;
use crate::stack::operand_stack::Operand;
use crate::reference::MethodRef;

use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use classfile::traits::PtrType;
use classfile::types::u2;
use classfile::{ClassFile, ConstantPoolRef, FieldType, MethodDescriptor};

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

#[derive(Debug)]
pub struct Class {
	pub name: Vec<u8>,
	pub access_flags: u16,
	pub constant_pool: ConstantPoolRef,
	pub super_class: Option<ClassRef>,
	pub methods: Vec<MethodRef>,
	pub fields: Vec<FieldRef>,
	pub static_field_slots: Box<[Operand]>,
	pub loader: ClassLoader,
	init_state: ClassInitializationState,
	init_lock: Arc<InitializationLock>,
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
		let static_field_slots = vec![Operand::Empty; static_field_count].into_boxed_slice();

		// We need the Class instance to create our methods and fields
		let class = Self {
			name,
			access_flags,
			constant_pool,
			super_class,
			loader,
			methods: Vec::new(),
			fields: Vec::new(),
			static_field_slots,
			init_state: ClassInitializationState::default(),
			init_lock: Arc::default(),
		};

		let classref = ClassPtr::new(class);

		classref.get_mut().methods = parsed_file
			.methods
			.iter()
			.map(|mi| Method::new(Arc::clone(&classref), mi))
			.collect();

		classref.get_mut().fields = parsed_file
			.fields
			.iter()
			.enumerate()
			.map(|(idx, fi)| {
				Field::new(
					idx,
					Arc::clone(&classref),
					fi,
					&classref.get().constant_pool,
				)
			})
			.collect();

		classref
	}

	pub fn get_main_method(&self) -> Option<MethodRef> {
		const MAIN_METHOD_NAME: &[u8] = b"main";

		if let Some(method) = self.methods.iter().find(|method| {
			method.name == MAIN_METHOD_NAME
				&& method.access_flags & 0x0001 > 0
				&& method.access_flags & 0x0008 > 0
		}) {
			let main_method_descriptor = MethodDescriptor {
				parameters: Box::new([FieldType::Array(Box::new(FieldType::Object(
					String::from("java/lang/String"),
				)))]),
				return_type: FieldType::Void,
			};

			if method.descriptor == main_method_descriptor {
				return Some(Arc::clone(method));
			}
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.2
	pub fn resolve_field(constant_pool: ConstantPoolRef, field_ref_idx: u2) -> Option<FieldRef> {
		let (class_name_index, name_and_type_index) = constant_pool.get_field_ref(field_ref_idx);

		let class_name = constant_pool.get_class_name(class_name_index);
		let classref = ClassLoader::Bootstrap.load(class_name).unwrap();

		let class = classref.get();

		let (name_index, descriptor_index) = constant_pool.get_name_and_type(name_and_type_index);

		let field_name = constant_pool.get_constant_utf8(name_index);
		let mut descriptor = constant_pool.get_constant_utf8(descriptor_index);

		let field_type = FieldType::parse(&mut descriptor);

		// When resolving a field reference, field resolution first attempts to look up
		// the referenced field in C and its superclasses:

		// 1. If C declares a field with the name and descriptor specified by the field reference,
		//    field lookup succeeds. The declared field is the result of the field lookup.
		for field in &class.fields {
			if field.name == field_name && field.descriptor == field_type {
				return Some(Arc::clone(field));
			}
		}

		// TODO:
		// 2. Otherwise, field lookup is applied recursively to the direct superinterfaces of the
		//    specified class or interface C.

		// 3. Otherwise, if C has a superclass S, field lookup is applied recursively to S.
		if let Some(super_class) = &class.super_class {
			let super_class = super_class.get();
			Class::resolve_field(super_class.constant_pool.clone(), field_ref_idx);
		}

		// 4. Otherwise, field lookup fails.
		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.5
	pub fn initialize(&mut self) {
		// 1. Synchronize on the initialization lock, LC, for C. This involves waiting until the current thread can acquire LC.
		let mut _guard = self.init_lock.lock();

		// 2. If the Class object for C indicates that initialization is in progress for C by some other thread
		if self.init_state == ClassInitializationState::InProgress {
			// then release LC and block the current thread until informed that the in-progress initialization
			// has completed, at which time repeat this procedure.
			_guard = self.init_lock.wait(_guard);
		}

		// TODO:
		// 3. If the Class object for C indicates that initialization is in progress for C by the current thread,
		//    then this must be a recursive request for initialization. Release LC and complete normally.

		// 4. If the Class object for C indicates that C has already been initialized, then no further action
		//    is required. Release LC and complete normally.
		if self.init_state == ClassInitializationState::Init {
			return;
		}

		// TODO:
		// 5. If the Class object for C is in an erroneous state, then initialization is not possible.
		//    Release LC and throw a NoClassDefFoundError.
		if self.init_state == ClassInitializationState::Failed {
			panic!("NoClassDefFoundError");
		}

		// TODO: Need to specify thread
		// 6. Otherwise, record the fact that initialization of the Class object for C is in progress
		//    by the current thread, and release LC.
		self.init_state = ClassInitializationState::InProgress;
		drop(_guard);

		//  Then, initialize each final static field of C with the constant value in its ConstantValue attribute (§4.7.2),
		//  in the order the fields appear in the ClassFile structure.
		for (idx, field) in self
			.fields
			.iter_mut()
			.filter(|field| field.is_static())
			.enumerate()
		{
			let constant_value_index = field.constant_value_index.unwrap();

			match field.descriptor {
				FieldType::Byte
				| FieldType::Char
				| FieldType::Short
				| FieldType::Boolean
				| FieldType::Int => {
					self.static_field_slots[idx] =
						Operand::Int(self.constant_pool.get_integer(constant_value_index))
				},
				FieldType::Double => {
					self.static_field_slots[idx] =
						Operand::Double(self.constant_pool.get_double(constant_value_index))
				},
				FieldType::Float => {
					self.static_field_slots[idx] =
						Operand::Float(self.constant_pool.get_float(constant_value_index))
				},
				FieldType::Long => {
					self.static_field_slots[idx] =
						Operand::Long(self.constant_pool.get_long(constant_value_index))
				},
				FieldType::Object(ref obj) if obj == "java/lang/String" => {
					unimplemented!()
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

		// TODO:
		// 9. Next, execute the class or interface initialization method of C.

		// 10. If the execution of the class or interface initialization method completes normally,
		//     then acquire LC, label the Class object for C as fully initialized, notify all waiting threads,
		//     release LC, and complete this procedure normally.

		// TODO:
		// 11. Otherwise, the class or interface initialization method must have completed abruptly by throwing some exception E.

		// 12. Acquire LC, label the Class object for C as erroneous, notify all waiting threads, release LC, and complete this
		//     procedure abruptly with reason E or its replacement as determined in the previous step.
		self.init_state = ClassInitializationState::Failed;
		self.init_lock.notify_all();
	}

	pub fn initialization_state(&self) -> ClassInitializationState {
		self.init_state
	}
}

impl Clone for Class {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			access_flags: self.access_flags,
			constant_pool: self.constant_pool.clone(),
			super_class: self.super_class.clone(),
			methods: self.methods.clone(),
			fields: self.fields.clone(),
			static_field_slots: self.static_field_slots.clone(),
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
		f.write_fmt(format_args!("{:?}", class))
	}
}
