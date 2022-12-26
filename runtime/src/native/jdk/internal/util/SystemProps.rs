pub mod Raw {
	use crate::native::NativeReturn;
	use crate::stack::local_stack::LocalStack;

	pub fn vmProperties(_: LocalStack) -> NativeReturn {
		unimplemented!("SystemProps$Raw#vmProperties")
	}

	pub fn platformProperties(_: LocalStack) -> NativeReturn {
		unimplemented!("SystemProps$Raw#platformProperties")
	}
}
