use super::reference::ClassRef;
use crate::reference::MethodRef;

use classfile::traits::PtrType;
use classfile::types::{u1, u2};
use classfile::{Code, FieldType, MethodDescriptor, MethodInfo};

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
	pub class: ClassRef,
	pub access_flags: u2,
	pub name: Vec<u1>,
	pub descriptor: MethodDescriptor,
	pub code: Code,
}

#[rustfmt::skip]
impl Method {
	// Access flags
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.6-200-A.1

	pub const ACC_PUBLIC      : u2 = 0x0001; /* Declared public; may be accessed from outside its package. */
	pub const ACC_PRIVATE     : u2 = 0x0002; /* Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4). */
	pub const ACC_PROTECTED   : u2 = 0x0004; /* Declared protected; may be accessed within subclasses. */
	pub const ACC_STATIC      : u2 = 0x0008; /* Declared static. */
	pub const ACC_FINAL       : u2 = 0x0010; /* Declared final; must not be overridden (§5.4.5). */
	pub const ACC_SYNCHRONIZED: u2 = 0x0020; /* Declared synchronized; invocation is wrapped by a monitor use. */ // TODO: This is not respected anywhere
	pub const ACC_BRIDGE      : u2 = 0x0040; /* A bridge method, generated by the compiler. */
	pub const ACC_VARARGS     : u2 = 0x0080; /* Declared with variable number of arguments. */
	pub const ACC_NATIVE      : u2 = 0x0100; /* Declared native; implemented in a language other than the Java programming language. */
	pub const ACC_ABSTRACT    : u2 = 0x0400; /* Declared abstract; no implementation is provided. */
	pub const ACC_STRICT      : u2 = 0x0800; /* In a class file whose major version number is at least 46 and at most 60: Declared strictfp. */
	pub const ACC_SYNTHETIC   : u2 = 0x1000; /* Declared synthetic; not present in the source code. */
}

impl Method {
	pub fn new(class: ClassRef, method_info: &MethodInfo) -> MethodRef {
		let constant_pool = &class.unwrap_class_instance().constant_pool;

		let access_flags = method_info.access_flags;

		let name_index = method_info.name_index;
		let name = constant_pool.get_constant_utf8(name_index).to_vec();

		let descriptor_index = method_info.descriptor_index;
		let mut descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

		let descriptor = MethodDescriptor::parse(&mut descriptor_bytes);

		let code = method_info.get_code_attribute().unwrap_or_default();

		let method = Self {
			class,
			access_flags,
			name,
			descriptor,
			code,
		};

		MethodRef::new(method)
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.3
	pub fn is_polymorphic(&self) -> bool {
		const METHODHANDLE_CLASS_NAME: &[u1] = b"java/lang/invoke/MethodHandle";
		const VARHANDLE_CLASS_NAME: &[u1] = b"java/lang/invoke/VarHandle";

		let class = self.class.get();
		let mut is_polymorphic = false;

		// A method is signature polymorphic if all of the following are true:

		//     It is declared in the java.lang.invoke.MethodHandle class or the java.lang.invoke.VarHandle class.
		is_polymorphic |=
			class.name == METHODHANDLE_CLASS_NAME || class.name == VARHANDLE_CLASS_NAME;

		//     It has a single formal parameter of type Object[].
		match &*self.descriptor.parameters {
			[FieldType::Array(arr_ty)] => match &**arr_ty {
				FieldType::Object(ref obj) if obj == "java/lang/Object" => {},
				_ => return false,
			},
			_ => return false,
		}

		//     It has the ACC_VARARGS and ACC_NATIVE flags set.
		is_polymorphic |= self.access_flags & Method::ACC_VARARGS != 0
			&& self.access_flags & Method::ACC_NATIVE != 0;

		is_polymorphic
	}
}
