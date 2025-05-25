use crate::classes;
use crate::native::java::lang::String::StringInterner;
use crate::objects::array::Array;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{Symbol, sym};

use common::int_types::u2;
use common::traits::PtrType;
use instructions::OpCode;
use jni::env::JniEnv;
use jni::sys::jlong;

include_generated!("native/java/lang/def/NullPointerException.definitions.rs");

pub fn getExtendedNPEMessage(
	_env: JniEnv,
	this: Reference, // java.lang.NullPointerException
) -> Reference /* java.lang.String */ {
	let backtrace = classes::java::lang::Throwable::backtrace(this.extract_class().get());
	if backtrace.is_null() {
		// Nothing to do
		return Reference::null();
	}

	let backtrace_array_instance = backtrace.extract_primitive_array();
	let backtrace_array = backtrace_array_instance.get();
	if backtrace_array.is_empty() {
		// No backtrace, nothing to do
		return Reference::null();
	}

	// See the format of `BackTrace` in `native/java/lang/Throwable.rs`
	let backtrace_array = backtrace_array.as_slice::<jlong>();

	let method_ptr = backtrace_array[0];
	let pc = backtrace_array[1] as usize;

	let method = unsafe { &*(method_ptr as *const Method) };
	if method.is_native() {
		// No bytecode, nothing to do
		return Reference::null();
	}

	let mut target_opcode = OpCode::from(method.code.code[pc]);
	let mut operand_pos = pc + 1;

	if target_opcode == OpCode::wide {
		target_opcode = OpCode::from(method.code.code[pc + 1]);
		operand_pos += 1;
	}

	let Some(description) = description(target_opcode, method, operand_pos) else {
		// There is no extra description for this instruction, nothing to do
		return Reference::null();
	};

	Reference::class(StringInterner::intern(description.as_str()))
}

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

fn pretty_class_name(class: &'static Class) -> Symbol {
	if class.name() == sym!(java_lang_Object) {
		return sym!(Object);
	}

	if class.name() == sym!(java_lang_String) {
		return sym!(String);
	}

	class.external_name()
}
