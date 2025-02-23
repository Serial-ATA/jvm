use crate::objects::method::Method;

impl Method {
	/// Whether this method is [signature polymorphic]
	///
	/// [signature polymorphic]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.9.3
	pub fn is_signature_polymorphic(&self) -> bool {
		// A method is signature polymorphic if all of the following are true:
		//
		//     It is declared in the java.lang.invoke.MethodHandle class or the java.lang.invoke.VarHandle class.
		(self.class == crate::globals::classes::java_lang_invoke_MethodHandle()
			|| self.class == crate::globals::classes::java_lang_invoke_VarHandle()) &&

			//     It has a single formal parameter of type Object[].
			(self.descriptor.parameters.len() == 1 && self.descriptor.parameters[0].is_array_of_class(b"java/lang/Object")) &&

			//     It has the ACC_VARARGS and ACC_NATIVE flags set.
			(self.is_var_args() && self.is_native())
	}

	/// Whether this method can override the provided instance method ([§5.4.3.3](https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.5))
	#[allow(non_snake_case)]
	pub fn can_override(&self, other: &Method) -> bool {
		// An instance method mC can override another instance method mA iff all of the following are true:

		let mC = self;
		let mA = other;

		// mC has the same name and descriptor as mA.
		if mC.name != mA.name || mC.descriptor_sym() != mA.descriptor_sym() {
			return false;
		}

		// mC is not marked ACC_PRIVATE.
		if mC.is_private() {
			return false;
		}

		// One of the following is true:

		//     mA is marked ACC_PUBLIC.
		//     mA is marked ACC_PROTECTED.
		if mA.is_public() || mA.is_protected() {
			return true;
		}

		//     mA is marked neither ACC_PUBLIC nor ACC_PROTECTED nor ACC_PRIVATE, and either:
		if !mA.is_private() {
			//         (a) the declaration of mA appears in the same run-time package as the declaration of mC, or
			if mA.class.shares_package_with(mC.class) {
				return true;
			}

			//         (b) if mA is declared in a class A and mC is declared in a class C, then there exists a method mB declared in a class B
			//             such that C is a subclass of B and B is a subclass of A and mC can override mB and mB can override mA.
			let mA_class = mA.class;
			let mC_class = mC.class;

			if let Some(applicable_super) = mC_class
				.parent_iter()
				.find(|parent| parent.super_class == Some(mA_class))
			{
				for mB in applicable_super.vtable() {
					if mC.can_override(mB) && mB.can_override(mA) {
						return true;
					}
				}
			}
		}

		false
	}
}
