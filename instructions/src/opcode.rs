use common::int_types::u1;

/// Defines all the opcodes
///
/// The format of this is simple:
///
/// ```rust
/// define_opcode!(
/// 	pub enum OpCode {
/// 	    some_opcode [= byte_value] [; size = $size],
///         // ...
///     }
/// )
/// ```
///
/// Where:
///
/// * `byte_value` is the optional encoded version of the opcode, if available
/// * `$size` is the size of the full instruction (opcode + additional values), if available
///
/// For example, `OpCode::unknown` has no `byte_value` and `Opcode::tableswitch` has no `$size`.
macro_rules! define_opcodes {
    (
        $(#[$meta:meta])*
        pub enum OpCode {
            $($opcode:ident $(= $value:literal)? $(; size = $size:literal)?),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        pub enum OpCode {
            $($opcode $(= $value)?),+
        }

		impl OpCode {
			/// Convert the opcode into its corresponding byte
			///
			/// NOTE: This will only return `None` for `OpCode::unknown`
			pub fn as_u1(self) -> Option<u1> {
				match self {
                    $($(OpCode::$opcode => Some($value),)?)+
                    OpCode::unknown => None,
                }
			}

			/// Get the expected size of this opcode's instruction
			///
			/// This returns `Option<usize>`, as it is not possible to determine the size of certain
			/// instructions, such as `wide`, with merely the opcode.
			pub fn size(self) -> Option<usize> {
				match self {
                    $($(OpCode::$opcode => Some($size),)?)+
                    _ => None,
                }
			}
		}

        impl From<u1> for OpCode {
            fn from(value: u1) -> Self {
                match value {
                    $($($value => Self::$opcode,)?)+
                    _ => Self::unknown
                }
            }
        }
    }
}

define_opcodes! {
	/// A list of all possible instructions, as defined in [Chapter 6. The Java Virtual Machine Instruction Set](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5).
	#[repr(u8)]
	#[derive(Debug, Copy, Clone, PartialEq)]
	#[allow(non_camel_case_types)]
	#[rustfmt::skip]
	pub enum OpCode {
		nop                = 0x00; size = 1, // Do nothing
		aconst_null        = 0x01; size = 1, // Push null

		// --- Push the int constant <i> (-1, 0, 1, 2, 3, 4 or 5) onto the operand stack.  ---
		// Each of this family of instructions is equivalent to bipush <i> for the respective value of <i>
		iconst_m1          = 0x02; size = 1,
		iconst_0           = 0x03; size = 1,
		iconst_1           = 0x04; size = 1,
		iconst_2           = 0x05; size = 1,
		iconst_3           = 0x06; size = 1,
		iconst_4           = 0x07; size = 1,
		iconst_5           = 0x08; size = 1,
		// -----------------------------------------------------------------------------------

		// ------------ Push the long constant <l> (0 or 1) onto the operand stack -----------
		lconst_0           = 0x09; size = 1,
		lconst_1           = 0x0a; size = 1,
		// -----------------------------------------------------------------------------------

		// ------ Push the float constant <f> (0.0, 1.0, or 2.0) onto the operand stack ------
		fconst_0           = 0x0b; size = 1,
		fconst_1           = 0x0c; size = 1,
		fconst_2           = 0x0d; size = 1,
		// -----------------------------------------------------------------------------------

		// -------- Push the double constant <d> (0.0 or 1.0) onto the operand stack ---------
		dconst_0           = 0x0e; size = 1,
		dconst_1           = 0x0f; size = 1,
		// -----------------------------------------------------------------------------------

		bipush             = 0x10; size = 2, // Push byte
		sipush             = 0x11; size = 2, // Push short
		ldc                = 0x12; size = 2, // Push item from run-time constant pool
		ldc_w              = 0x13; size = 3, // Push item from run-time constant pool (wide index)
		ldc2_w             = 0x14; size = 3, // Push long or double from run-time constant pool (wide index)
		iload              = 0x15; size = 2, // Load int from local variable
		lload              = 0x16; size = 2, // Load long from local variable
		fload              = 0x17; size = 2, // Load float from local variable
		dload              = 0x18; size = 2, // Load double from local variable
		aload              = 0x19; size = 2, // Load reference from local variable

		// --- same as iload with an index of <n> ---
		iload_0            = 0x1a; size = 1,
		iload_1            = 0x1b; size = 1,
		iload_2            = 0x1c; size = 1,
		iload_3            = 0x1d; size = 1,
		// ------------------------------------------

		// --- same as lload with an index of <n> ---
		lload_0            = 0x1e; size = 1,
		lload_1            = 0x1f; size = 1,
		lload_2            = 0x20; size = 1,
		lload_3            = 0x21; size = 1,
		// ------------------------------------------

		// --- same as fload with an index of <n> ---
		fload_0            = 0x22; size = 1,
		fload_1            = 0x23; size = 1,
		fload_2            = 0x24; size = 1,
		fload_3            = 0x25; size = 1,
		// ------------------------------------------

		// --- same as dload with an index of <n> ---
		dload_0            = 0x26; size = 1,
		dload_1            = 0x27; size = 1,
		dload_2            = 0x28; size = 1,
		dload_3            = 0x29; size = 1,
		// ------------------------------------------

		// --- same as aload with an index of <n> ---
		aload_0            = 0x2a; size = 1,
		aload_1            = 0x2b; size = 1,
		aload_2            = 0x2c; size = 1,
		aload_3            = 0x2d; size = 1,
		// ------------------------------------------

		iaload             = 0x2e; size = 1,
		laload             = 0x2f; size = 1,
		faload             = 0x30; size = 1,
		daload             = 0x31; size = 1,
		aaload             = 0x32; size = 1,
		baload             = 0x33; size = 1,
		caload             = 0x34; size = 1,
		saload             = 0x35; size = 1,
		istore             = 0x36; size = 2,
		lstore             = 0x37; size = 2,
		fstore             = 0x38; size = 2,
		dstore             = 0x39; size = 2,
		astore             = 0x3a; size = 2,
		istore_0           = 0x3b; size = 1,
		istore_1           = 0x3c; size = 1,
		istore_2           = 0x3d; size = 1,
		istore_3           = 0x3e; size = 1,
		lstore_0           = 0x3f; size = 1,
		lstore_1           = 0x40; size = 1,
		lstore_2           = 0x41; size = 1,
		lstore_3           = 0x42; size = 1,
		fstore_0           = 0x43; size = 1,
		fstore_1           = 0x44; size = 1,
		fstore_2           = 0x45; size = 1,
		fstore_3           = 0x46; size = 1,
		dstore_0           = 0x47; size = 1,
		dstore_1           = 0x48; size = 1,
		dstore_2           = 0x49; size = 1,
		dstore_3           = 0x4a; size = 1,
		astore_0           = 0x4b; size = 1,
		astore_1           = 0x4c; size = 1,
		astore_2           = 0x4d; size = 1,
		astore_3           = 0x4e; size = 1,
		iastore            = 0x4f; size = 1,
		lastore            = 0x50; size = 1,
		fastore            = 0x51; size = 1,
		dastore            = 0x52; size = 1,
		aastore            = 0x53; size = 1,
		bastore            = 0x54; size = 1,
		castore            = 0x55; size = 1,
		sastore            = 0x56; size = 1,
		pop                = 0x57; size = 1,
		pop2               = 0x58; size = 1,
		dup                = 0x59; size = 1, // Duplicate the top operand stack value
		dup_x1             = 0x5a; size = 1, // Duplicate the top operand stack value and insert two values down
		dup_x2             = 0x5b; size = 1, // Duplicate the top operand stack value and insert two or three values down
		dup2               = 0x5c; size = 1, // Duplicate the top one or two operand stack values
		dup2_x1            = 0x5d; size = 1, // Duplicate the top one or two operand stack values and insert two or three values down
		dup2_x2            = 0x5e; size = 1, // Duplicate the top one or two operand stack values and insert two, three, or four values down
		swap               = 0x5f; size = 1,
		iadd               = 0x60; size = 1,
		ladd               = 0x61; size = 1,
		fadd               = 0x62; size = 1,
		dadd               = 0x63; size = 1,
		isub               = 0x64; size = 1,
		lsub               = 0x65; size = 1,
		fsub               = 0x66; size = 1,
		dsub               = 0x67; size = 1,
		imul               = 0x68; size = 1,
		lmul               = 0x69; size = 1,
		fmul               = 0x6a; size = 1,
		dmul               = 0x6b; size = 1,
		idiv               = 0x6c; size = 1,
		ldiv               = 0x6d; size = 1,
		fdiv               = 0x6e; size = 1,
		ddiv               = 0x6f; size = 1,
		irem               = 0x70; size = 1,
		lrem               = 0x71; size = 1,
		frem               = 0x72; size = 1,
		drem               = 0x73; size = 1,
		ineg               = 0x74; size = 1,
		lneg               = 0x75; size = 1,
		fneg               = 0x76; size = 1,
		dneg               = 0x77; size = 1,
		ishl               = 0x78; size = 1,
		lshl               = 0x79; size = 1,
		ishr               = 0x7a; size = 1,
		lshr               = 0x7b; size = 1,
		iushr              = 0x7c; size = 1,
		lushr              = 0x7d; size = 1,
		iand               = 0x7e; size = 1,
		land               = 0x7f; size = 1,
		ior                = 0x80; size = 1,
		lor                = 0x81; size = 1,
		ixor               = 0x82; size = 1,
		lxor               = 0x83; size = 1,
		iinc               = 0x84; size = 3, // Increment local variable by constant
		i2l                = 0x85; size = 1,
		i2f                = 0x86; size = 1,
		i2d                = 0x87; size = 1,
		l2i                = 0x88; size = 1,
		l2f                = 0x89; size = 1,
		l2d                = 0x8a; size = 1,
		f2i                = 0x8b; size = 1,
		f2l                = 0x8c; size = 1,
		f2d                = 0x8d; size = 1,
		d2i                = 0x8e; size = 1,
		d2l                = 0x8f; size = 1,
		d2f                = 0x90; size = 1,
		i2b                = 0x91; size = 1,
		i2c                = 0x92; size = 1,
		i2s                = 0x93; size = 1,
		lcmp               = 0x94; size = 1,
		fcmpl              = 0x95; size = 1,
		fcmpg              = 0x96; size = 1,
		dcmpl              = 0x97; size = 1,
		dcmpg              = 0x98; size = 1,

		// --- Branch if int comparison with zero succeeds ---
		ifeq               = 0x99; size = 3,
		ifne               = 0x9a; size = 3,
		iflt               = 0x9b; size = 3,
		ifge               = 0x9c; size = 3,
		ifgt               = 0x9d; size = 3,
		ifle               = 0x9e; size = 3,
		// ----------------------------------------------------

		// -------- Branch if int comparison succeeds ---------
		if_icmpeq           = 0x9f; size = 3,
		if_icmpne           = 0xa0; size = 3,
		if_icmplt           = 0xa1; size = 3,
		if_icmpge           = 0xa2; size = 3,
		if_icmpgt           = 0xa3; size = 3,
		if_icmple           = 0xa4; size = 3,
		// ----------------------------------------------------

		// ---- Branch if reference comparison succeeds  -----
		if_acmpeq           = 0xa5; size = 3,
		if_acmpne           = 0xa6; size = 3,
		// ---------------------------------------------------

		goto               = 0xa7; size = 3, // Branch always
		jsr                = 0xa8; size = 3, // Jump subroutine
		ret                = 0xa9; size = 2, // Return from subroutine
		tableswitch        = 0xaa, // Access jump table by index and jump
		lookupswitch       = 0xab, // Access jump table by key match and jump
		ireturn            = 0xac; size = 1, // Return int from method
		lreturn            = 0xad; size = 1, // Return long from method
		freturn            = 0xae; size = 1, // Return float from method
		dreturn            = 0xaf; size = 1, // Return double from method
		areturn            = 0xb0; size = 1, // Return reference from method
		r#return           = 0xb1; size = 1, // Return void from method
		getstatic          = 0xb2; size = 3, // Get static field from class
		putstatic          = 0xb3; size = 3, // Set static field in class
		getfield           = 0xb4; size = 3, // Fetch field from object
		putfield           = 0xb5; size = 3, // Set field in object
		invokevirtual      = 0xb6; size = 3, // Invoke instance method; dispatch based on class
		invokespecial      = 0xb7; size = 3, // Invoke instance method; direct invocation of instance initialization methods and methods of the current class and its supertypes
		invokestatic       = 0xb8; size = 3, // Invoke a class (static) method
		invokeinterface    = 0xb9; size = 5, // Invoke interface method
		invokedynamic      = 0xba; size = 5, // Invoke a dynamically-computed call site
		new                = 0xbb; size = 3, // Create new object
		newarray           = 0xbc; size = 2, // Create new array
		anewarray          = 0xbd; size = 3, // Create new array of reference
		arraylength        = 0xbe; size = 1, // Get length of array
		athrow             = 0xbf; size = 1, // Throw exception or error
		checkcast          = 0xc0; size = 3, // Check whether object is of given type
		instanceof         = 0xc1; size = 3, // Determine if object is of given type
		monitorenter       = 0xc2; size = 1, // Enter monitor for object
		monitorexit        = 0xc3; size = 1, // Exit monitor for object
		wide               = 0xc4, // Extend local variable index by additional bytes
		multianewarray     = 0xc5; size = 4, // Create new multidimensional array
		ifnull             = 0xc6; size = 3, // Branch if reference is null
		ifnonnull          = 0xc7; size = 3, // Branch if reference not null
		goto_w             = 0xc8; size = 5, // Branch always (wide index)
		jsr_w              = 0xc9; size = 5, // Jump subroutine (wide index)
		breakpoint         = 0xca, // Intended to be used by debuggers to implement breakpoints
		unknown,                   // Covers any other currently unknown opcodes

		// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.2
		impdep1            = 0xfe,
		impdep2            = 0xff,
	}
}
