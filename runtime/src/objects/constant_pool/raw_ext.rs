use classfile::constant_pool::types::LoadableConstantPoolValue;

/// Used to convert a [`LoadableConstantPoolValue`] into its boxed counterpart.
///
/// This means that a constant of [`LoadableConstantPoolValue::Float`], for example, will become a
/// `java.lang.Float`.
///
/// This conversion is necessary for certain operations, such as linking dynamic call sites, where
/// lists of `Object`s must be provided.
pub trait LoadableCpValueIntoBoxed {
	fn into_boxed(self) -> Self;
}

impl LoadableCpValueIntoBoxed for LoadableConstantPoolValue<'_> {
	fn into_boxed(self) -> Self {
		todo!("Value boxing")
	}
}
