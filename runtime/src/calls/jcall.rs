/// Call a Java method with arguments
///
/// This will invoke `method` on `thread` with the provided arguments.
///
/// # Parameters
///
/// * `thread` - `&JavaThread`
/// * `method` - `&'static Method`
/// * `arg`(s) - `Operand`
///
/// # Returns
///
/// Will return an `Operand`, if the method returns a value.
///
/// # Examples
///
/// ```rust,ignore
/// // No arguments
/// java_call!(thread, method)
///
/// // With arguments
/// java_call!(thread, method, arg1, arg2, arg3)
/// ```
#[macro_export]
macro_rules! java_call {
	(
        $thread:ident,
        $method:ident,
		$($arg:expr),+ $(,)?
    ) => {{
		tracing::debug!(target: "java_call", "Invoking manual Java call for method `{:?}`", $method);
		let max_locals = $method.code.max_locals;
		let __ret = $thread.invoke_method_scoped($method, $crate::stack::local_stack::LocalStack::new_with_args(vec![$(Operand::from($arg)),+], max_locals as usize));
		tracing::debug!(target: "java_call", "Manual Java call finished for method `{:?}`", $method);
		__ret
	}};
	// No arguments path, still needs to allocate a LocalStack for stores
	(
        $thread:ident,
        $method:ident $(,)?
    ) => {{
		tracing::debug!(target: "java_call", "Invoking manual Java call for method `{:?}`", $method);
		let max_locals = $method.code.max_locals;
		let __ret = $thread.invoke_method_scoped($method, $crate::stack::local_stack::LocalStack::new(max_locals as usize));
		tracing::debug!(target: "java_call", "Manual Java call finished for method `{:?}`", $method);
		__ret
	}};
}
