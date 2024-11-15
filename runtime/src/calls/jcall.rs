use crate::method::Method;
use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;
use crate::JavaThread;
use instructions::Operand;
// macro_rules! static_jcall {
//     (
//         ENV: $env:ident,
// 		THREAD: $thread:ident,
//         METHOD: $method:ident,
//         ARGS: $($arg:expr),*
//     ) => {
// 		debug_assert!($method.is_static());
// 		if $method.is_native() {
// 			// Try to lookup and set the method prior to calling
// 			let _ = $crate::native::lookup::lookup_native_method($method, $thread);
// 			$method.native_invoker()(
// 				$method,
// 				$env,
// 				(jclass_from_classref(Arc::clone(&$method.class)), $($arg),*),
// 			)
// 		}
//
// 		unimplemented!("Non-native static method jcall");
// 	};
// }
//

/// Call a Java method with arguments
///
/// This will invoke `method` on `thread` with the provided arguments.
///
/// # Parameters
///
/// * `thread` - `&mut JavaThread`
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
		$crate::calls::jcall::java_call_inner($thread, $method, $crate::stack::local_stack::LocalStack::new_with_args(vec![$(Operand::from($arg)),+], max_locals as usize))
	}};
	// No arguments path, still needs to allocate a LocalStack for stores
	(
        $thread:ident,
        $method:ident $(,)?
    ) => {{
		tracing::debug!(target: "java_call", "Invoking manual Java call for method `{:?}`", $method);
		let max_locals = $method.code.max_locals;
		$crate::calls::jcall::java_call_inner($thread, $method, $crate::stack::local_stack::LocalStack::new(max_locals as usize))
	}};
}

#[doc(hidden)]
pub fn java_call_inner(
	thread: &mut JavaThread,
	method: &'static Method,
	args: LocalStack,
) -> Option<Operand<Reference>> {
	thread.invoke_method_with_local_stack(method, args);
	thread.run();
	thread.pull_remaining_operand()
}
