use crate::method::Method;

use classfile::{FieldType, MethodDescriptor};
use common::traits::PtrType;
use symbols::sym;

impl Method {
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.3
	pub fn is_polymorphic(&self) -> bool {
		let class = self.class.get();
		let mut is_polymorphic = false;

		// A method is signature polymorphic if all of the following are true:

		//     It is declared in the java.lang.invoke.MethodHandle class or the java.lang.invoke.VarHandle class.
		is_polymorphic |= class.name == sym!(java_lang_invoke_MethodHandle)
			|| class.name == sym!(java_lang_invoke_VarHandle);

		//     It has a single formal parameter of type Object[].
		let parsed_descriptor =
			MethodDescriptor::parse(&mut &self.descriptor.as_str().as_bytes()[..]).unwrap(); // TODO: Error handling
		match &*parsed_descriptor.parameters {
			[FieldType::Array(arr_ty)] => match &**arr_ty {
				FieldType::Object(ref obj) if &**obj == b"java/lang/Object" => {},
				_ => return false,
			},
			_ => return false,
		}

		//     It has the ACC_VARARGS and ACC_NATIVE flags set.
		is_polymorphic |= self.access_flags.is_varargs() && self.is_native();

		is_polymorphic
	}

	/// Whether this method can override the provided instance method ([§5.4.3.3](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.5))
	pub fn can_override(&self, other: &Method) -> bool {
		// An instance method mC can override another instance method mA iff all of the following are true:

		// mC has the same name and descriptor as mA.
		//
		// mC is not marked ACC_PRIVATE.
		//
		// One of the following is true:
		//
		//     mA is marked ACC_PUBLIC.
		//
		//     mA is marked ACC_PROTECTED.
		//
		//     mA is marked neither ACC_PUBLIC nor ACC_PROTECTED nor ACC_PRIVATE, and either:
		//
		//         (a) the declaration of mA appears in the same run-time package as the declaration of mC, or
		//         (b) if mA is declared in a class A and mC is declared in a class C, then there exists a method mB declared in a class B
		//             such that C is a subclass of B and B is a subclass of A and mC can override mB and mB can override mA.

		(self.name == other.name && self.descriptor == other.descriptor)
			&& !self.is_private()
			&& (other.is_public()
				|| other.is_protected()
				|| (!other.is_private() && other.class.shares_package_with(self.class.get())))
	}
}
