use crate::classpath::loader::ClassLoader;
use crate::native::jni::JniEnvExt;
use crate::native::jni::references::JObjectExt;
use crate::native::jvm::strings::StringInterner;
use crate::objects::class::ClassPtr;
use crate::objects::constant_pool::cp_types;
use crate::objects::instance::array::{Array, PrimitiveArrayInstance};
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};
use crate::thread::exceptions::throw;
use crate::thread::frame::stack::VisibleStackFrame;
use crate::{classes, globals};

use std::slice;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::objects::{JObject, JObjectArray, JString, JThrowable};
use ::jni::sys::{jint, jlong};
use common::int_types::{s8, u2};
use instructions::OpCode;
use native_macros::jni_call;

/// A wrapper for backtrace (`java.lang.Throwable#backtrace`) creation
///
/// The field in Java is defined as:
///
/// ```java
/// private transient Object backtrace;
/// ```
///
/// So we're given free rein to define the format of our backtrace.
///
/// The format is the following struct:
///
/// ```text
/// struct BackTrace {
///     method: &Method as jlong,
///     pc: jlong,
/// }
/// ```
///
/// Flattened into an `long[]`:
///
/// ```text
/// ["java/lang/Foo#foo", 2, "java/lang/Foo#bar", 5]
/// ```
pub struct BackTrace {
	inner: Vec<jlong>,
}

impl BackTrace {
	const NUMBER_OF_FIELDS: usize = 2;

	pub fn from_encoded(backtrace: &[jlong]) -> BackTraceIter<'_> {
		BackTraceIter {
			inner: backtrace.iter(),
		}
	}

	/// Create a new `BackTrace`
	///
	/// `depth` is the number of methods in the backtrace
	fn new(mut depth: usize) -> Self {
		depth *= Self::NUMBER_OF_FIELDS;

		BackTrace {
			inner: Vec::with_capacity(depth),
		}
	}

	#[allow(trivial_casts)]
	fn push(&mut self, frame: VisibleStackFrame<'_>) {
		let method = frame.method();
		let pc = match frame {
			VisibleStackFrame::Regular(frame) => frame.stashed_pc(),
			_ => -1,
		};

		self.inner.push(std::ptr::from_ref(method) as jlong);
		self.inner.push(pc as jlong);
	}

	fn into_obj(self) -> Reference {
		let array = PrimitiveArrayInstance::new::<jlong>(&self.inner);
		Reference::array(array)
	}
}

pub struct BackTraceElement {
	pub method: &'static Method,
	pub pc: jlong,
}

pub struct BackTraceIter<'a> {
	inner: slice::Iter<'a, jlong>,
}

impl Iterator for BackTraceIter<'_> {
	type Item = BackTraceElement;

	fn next(&mut self) -> Option<Self::Item> {
		match self.inner.next_chunk::<2>() {
			Ok([method, pc]) => Some(BackTraceElement {
				method: unsafe { &*(*method as *const Method) },
				pc: *pc,
			}),
			Err(_) => None,
		}
	}
}

// Initialize the java.lang.Throwable field offsets
unsafe fn initialize() {
	static ONCE: Once = Once::new();
	ONCE.call_once(|| unsafe {
		classes::java::lang::Throwable::init_offsets();
	});
}

#[jni_call]
pub extern "C" fn JVM_FillInStackTrace(env: JniEnv, receiver: JObject) {
	let thread = unsafe { env.thread() };

	unsafe { initialize() };

	let Some(this) = (unsafe { receiver.to_reference() }) else {
		throw!(thread, NullPointerException);
	};

	// Reset the current fields
	classes::java::lang::Throwable::set_backtrace(this, Reference::null());
	classes::java::lang::Throwable::set_stackTrace(this, Reference::null());

	let stack_depth = thread.frame_stack().visible_depth();

	let mut frames_to_skip = 0;
	for frame in thread.frame_stack().iter() {
		if frame
			.method()
			.class()
			.is_subclass_of(globals::classes::java_lang_Throwable())
		{
			if frame.method().name == sym!(fillInStackTrace)
				|| frame.method().name == sym!(object_initializer_name)
			{
				frames_to_skip += 1;
			}

			continue;
		}

		break;
	}

	assert!(frames_to_skip < stack_depth);

	// Create the backtrace
	let backtrace_depth = stack_depth - frames_to_skip;
	let mut backtrace = BackTrace::new(backtrace_depth);
	for frame in thread
		.frame_stack()
		.iter()
		.skip(frames_to_skip)
		.take(backtrace_depth)
	{
		backtrace.push(frame);
	}

	classes::java::lang::Throwable::set_backtrace(this, backtrace.into_obj());
	classes::java::lang::Throwable::set_depth(this, backtrace_depth as jint);
}

#[jni_call]
pub extern "C" fn JVM_GetExtendedNPEMessage(env: JniEnv, throwable: JThrowable) -> JString {
	fn description(opcode: OpCode, method: &'static Method, operand_pos: usize) -> Option<String> {
		match opcode {
			OpCode::iaload => Some(String::from("Cannot load from int array")),
			OpCode::faload => Some(String::from("Cannot load from float array")),
			OpCode::aaload => Some(String::from("Cannot load from object array")),
			OpCode::baload => Some(String::from("Cannot load from byte/boolean array")),
			OpCode::caload => Some(String::from("Cannot load from char array")),
			OpCode::saload => Some(String::from("Cannot load from short array")),
			OpCode::laload => Some(String::from("Cannot load from long array")),
			OpCode::daload => Some(String::from("Cannot load from double array")),

			OpCode::iastore => Some(String::from("Cannot store to int array")),
			OpCode::fastore => Some(String::from("Cannot store to float array")),
			OpCode::aastore => Some(String::from("Cannot store to object array")),
			OpCode::bastore => Some(String::from("Cannot store to byte/boolean array")),
			OpCode::castore => Some(String::from("Cannot store to char array")),
			OpCode::sastore => Some(String::from("Cannot store to short array")),
			OpCode::lastore => Some(String::from("Cannot store to long array")),
			OpCode::dastore => Some(String::from("Cannot store to double array")),

			OpCode::arraylength => Some(String::from("Cannot read the array length")),

			OpCode::athrow => Some(String::from("Cannot throw exception")),

			OpCode::monitorenter => Some(String::from("Cannot enter synchronized block")),
			OpCode::monitorexit => Some(String::from("Cannot exit synchronized block")),

			OpCode::getfield => todo!(),
			OpCode::putfield => todo!(),

			OpCode::invokevirtual | OpCode::invokespecial | OpCode::invokeinterface => {
				let cp_index = u2::from_be_bytes([
					method.code.code[operand_pos],
					method.code.code[operand_pos + 1],
				]);
				let methodref = method
					.class()
					.constant_pool()
					.unwrap()
					.get::<cp_types::MethodRef>(cp_index)
					.expect("method should be resolved at this point");

				Some(format!(
					"Cannot invoke \"{}.{}({})\"",
					pretty_class_name(methodref.method.class()),
					methodref.method.name,
					methodref.method.external_signature(true)
				))
			},

			// Nothing to do for other instructions
			_ => None,
		}
	}

	fn pretty_class_name(class: ClassPtr) -> Symbol {
		if class.name() == sym!(java_lang_Object) {
			return sym!(Object);
		}

		if class.name() == sym!(java_lang_String) {
			return sym!(String);
		}

		class.external_name()
	}

	let thread = unsafe { env.thread() };

	let Some(throwable) = (unsafe { throwable.to_reference() }) else {
		return JString::null(); // TODO: Exception?
	};

	let backtrace = classes::java::lang::Throwable::backtrace(throwable.extract_class());
	if backtrace.is_null() {
		// Nothing to do
		return JString::null();
	}

	let backtrace_array = backtrace.extract_primitive_array();
	if backtrace_array.is_empty() {
		// No backtrace, nothing to do
		return JString::null();
	}

	// See the format of `BackTrace` in `native/java/lang/Throwable.rs`
	let backtrace_array = backtrace_array.as_slice::<jlong>();

	let method_ptr = backtrace_array[0];
	let pc = backtrace_array[1] as usize;

	let method = unsafe { &*(method_ptr as *const Method) };
	if method.is_native() {
		// No bytecode, nothing to do
		return JString::null();
	}

	let mut target_opcode = OpCode::from(method.code.code[pc]);
	let mut operand_pos = pc + 1;

	if target_opcode == OpCode::wide {
		target_opcode = OpCode::from(method.code.code[pc + 1]);
		operand_pos += 1;
	}

	let Some(description) = description(target_opcode, method, operand_pos) else {
		// There is no extra description for this instruction, nothing to do
		return JString::null();
	};

	let ptr = thread
		.jni_refs()
		.allocate(Reference::class(StringInterner::intern(
			description.as_str(),
		)));
	unsafe { JString::from_raw(ptr) }
}

#[jni_call]
pub extern "C" fn JVM_InitStackTraceElementArray(
	env: JniEnv,
	elements: JObjectArray,
	backtrace: JObject,
	depth: jint,
) {
	let thread = unsafe { env.thread() };

	unsafe {
		initialize_stack_trace_element();
	}

	let (Some(backtrace), Some(elements)) =
		(unsafe { (backtrace.to_reference(), elements.to_reference()) })
	else {
		throw!(thread, NullPointerException);
	};

	let stacktrace_elements = elements.extract_object_array();
	let stacktrace_elements = stacktrace_elements.as_slice();

	if stacktrace_elements.len() != depth as usize {
		throw!(thread, IndexOutOfBoundsException);
	}

	// See the `BackTrace` struct in `java/lang/Throwable.rs` for the format.
	let backtrace_array = backtrace.extract_primitive_array();
	let backtrace_array_contents = backtrace_array.as_slice::<jlong>();

	if !backtrace_array_contents.len().is_multiple_of(2) {
		throw!(
			thread,
			InternalError,
			"backtrace array is not an even length"
		);
	}

	for ([method, pc], stacktrace_element) in backtrace_array_contents
		.as_chunks::<2>()
		.0
		.iter()
		.copied()
		.zip(stacktrace_elements.iter())
	{
		let method = unsafe { &*(method as *const Method) };
		fill_in_stack_trace(stacktrace_element.extract_class(), method, pc);
	}
}

#[jni_call]
pub extern "C" fn JVM_InitStackTraceElement(
	_env: JniEnv,
	_element: JObject,
	_stack_frame_info: JObject,
) {
	unsafe {
		initialize_stack_trace_element();
	}

	unimplemented!("java.lang.StackTraceElement#initStackTraceElement");
}

unsafe fn initialize_stack_trace_element() {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// Already initialized
		return;
	}

	let Some(class) = ClassLoader::bootstrap().lookup_class(sym!(java_lang_StackTraceElement))
	else {
		unreachable!("StackTraceElement methods can't be called if the class isn't loaded already");
	};

	unsafe {
		crate::globals::classes::set_java_lang_StackTraceElement(class);
		classes::java::lang::StackTraceElement::init_offsets();
	}
}

fn fill_in_stack_trace(stacktrace_element: ClassInstanceRef, method: &Method, pc: s8) {
	let declaring_class_object = method.class().mirror();
	classes::java::lang::StackTraceElement::set_declaringClassObject(
		stacktrace_element,
		Reference::mirror(declaring_class_object),
	);

	// TODO: classLoaderName
	// TODO: moduleName
	// TODO: moduleVersion
	let declaring_class = StringInterner::intern(method.class().name());
	classes::java::lang::StackTraceElement::set_declaringClass(
		stacktrace_element,
		Reference::class(declaring_class),
	);

	let method_name = StringInterner::intern(method.name);
	classes::java::lang::StackTraceElement::set_methodName(
		stacktrace_element,
		Reference::class(method_name),
	);

	match method.class().source_file_name() {
		Some(name) => {
			let file_name = StringInterner::intern(name);
			classes::java::lang::StackTraceElement::set_fileName(
				stacktrace_element,
				Reference::class(file_name),
			);
		},
		None => {
			classes::java::lang::StackTraceElement::set_fileName(
				stacktrace_element,
				Reference::null(),
			);
		},
	}

	let line_number = method.line_number(pc as isize);
	classes::java::lang::StackTraceElement::set_lineNumber(stacktrace_element, line_number);
}
