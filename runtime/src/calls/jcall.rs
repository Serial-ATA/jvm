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
        $thread:expr,
        $method:ident,
		$($arg:expr),+ $(,)?
    ) => {{
		let max_locals = $method.code.max_locals;
		let local_stack = $crate::stack::local_stack::LocalStack::new_with_args(vec![$(::instructions::Operand::from($arg)),+], max_locals as usize);
		java_call!(@WITH_ARGS_LIST $thread, $method, local_stack)
	}};
	// No arguments path, still needs to allocate a LocalStack for stores
	(
        $thread:expr,
        $method:ident $(,)?
    ) => {{
		let max_locals = $method.code.max_locals;
		let local_stack = $crate::stack::local_stack::LocalStack::new(max_locals as usize);
		java_call!(@WITH_ARGS_LIST $thread, $method, local_stack)
	}};
	(
		@WITH_ARGS_LIST
        $thread:expr,
        $method:ident,
		$args_list:ident $(,)?
    ) => {{
		tracing::debug!(target: "java_call", "Invoking manual Java call for method `{:?}`", $method);
		let __thread = $thread;
		let __ret = __thread.invoke_method_scoped($method, $args_list);
		tracing::debug!(target: "java_call", "Manual Java call finished for method `{:?}`", $method);
		__ret
	}};
}
