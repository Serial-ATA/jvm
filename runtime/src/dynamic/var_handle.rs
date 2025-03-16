use crate::objects::constant_pool::cp_types::MethodEntry;
use crate::thread::exceptions::Throws;
use crate::thread::frame::Frame;

pub fn resolve_invoke_virtual(frame: &Frame, entry: &MethodEntry) -> Throws<()> {
	// If the resolved method is signature polymorphic and declared in the
	// java.lang.invoke.VarHandle class, then the invokevirtual instruction proceeds as follows,
	// where N and D are the name and descriptor of the method symbolically referenced by the instruction.
	let N = entry.method.name;
	let D = entry.method.descriptor_sym();

	// First, a reference to an instance of java.lang.invoke.VarHandle.AccessMode is obtained
	// as if by invocation of the valueFromMethodName method of java.lang.invoke.VarHandle.AccessMode
	// with a String argument denoting N.
	todo!();

	// Second, a reference to an instance of java.lang.invoke.MethodType is obtained as if by
	// invocation of the accessModeType method of java.lang.invoke.VarHandle on the instance
	// objectref, with the instance of java.lang.invoke.VarHandle.AccessMode as the argument.
	todo!();

	// Third, a reference to an instance of java.lang.invoke.MethodHandle is obtained as if by
	// invocation of the varHandleExactInvoker method of java.lang.invoke.MethodHandles with
	// the instance of java.lang.invoke.VarHandle.AccessMode as the first argument and the
	// instance of java.lang.invoke.MethodType as the second argument. The resulting instance
	// is called the invoker method handle.
	todo!();

	// Finally, the nargs argument values and objectref are popped from the operand stack, and
	// the invoker method handle is invoked. The invocation occurs as if by execution of an
	// invokevirtual instruction that indicates a run-time constant pool index to a symbolic
	// reference R where:
	//
	//     * R is a symbolic reference to a method of a class;
	//
	//     * for the symbolic reference to the class in which the method is to be found, R specifies java.lang.invoke.MethodHandle;
	//
	//     * for the name of the method, R specifies invoke;
	//
	//     * for the descriptor of the method, R specifies a return type indicated by the return
	//       descriptor of D, and specifies a first parameter type of java.lang.invoke.VarHandle
	//       followed by the parameter types indicated by the parameter descriptors of D (if any) in order.
	//
	// and where it is as if the following items were pushed, in order, onto the operand stack:
	//
	//     * a reference to the instance of java.lang.invoke.MethodHandle (the invoker method handle);
	//
	//     * objectref;
	//
	//     * the nargs argument values, where the number, type, and order of the values must be consistent with the type descriptor of the invoker method handle.
}
