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
        $method:expr,
		$($arg:expr),* $(,)?
    ) => {{
        let stack = $thread.stack();
        $(stack.push_op(::instructions::Operand::from($arg));)*
        java_call!($thread, $method)
    }};
	// No arguments path
	(
        $thread:expr,
        $method:expr $(,)?
    ) => {{
        $thread.invoke_method_scoped($method)
	}};
}
