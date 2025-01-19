#![allow(unused_imports)] // Intellij-Rust doesn't like this file much, the imports used in macros are not recognized

use crate::method_invoker::MethodInvoker;
use crate::objects::array::ArrayInstance;
use crate::objects::class::{Class, ClassInitializationState};
use crate::objects::class_instance::ClassInstance;
use crate::objects::constant_pool::cp_types::{self, Entry};
use crate::objects::field::Field;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::string_interner::StringInterner;
use crate::thread::frame::Frame;
use crate::thread::JavaThread;

use std::cmp::Ordering;
use std::sync::atomic::Ordering as MemOrdering;
use std::sync::Arc;

use crate::thread::exceptions::handle_exception;
use classfile::ConstantPoolValueInfo;
use common::int_types::{s2, s4, s8, u2};
use common::traits::PtrType;
use instructions::{ConstOperandType, OpCode, Operand, StackLike};
use symbols::{sym, Symbol};

macro_rules! trace_instruction {
    (@START $instruction:tt, $category:ident) => {{
		#[cfg(debug_assertions)]
		{ tracing::trace!(target: "instruction", "[{}] {} START", stringify!($category), stringify!($instruction)) }
	}};
    (@END $instruction:tt, $category:ident) => {{
		#[cfg(debug_assertions)]
		{ tracing::trace!(target: "instruction", "[{}] {} SUCCEEDED", stringify!($category), stringify!($instruction)) }
	}};
    (@BLOCK $category:ident, $instruction:tt, $expr:expr) => {{
        #[allow(unreachable_code)]
        {
            trace_instruction!(@START $instruction, $category);
            { $expr };
            trace_instruction!(@END $instruction, $category);
        }
    }};
}

// TODO: Document
macro_rules! define_instructions {
    (
        frame: $frame:ident,
        match $_ident:ident {
            $(
                CATEGORY: $category:ident
                $(
                    $(@GROUP { [$($member_ident:ident $(($($arg:tt),+))?),+ $(,)?] })?
                    $($pat:pat)? => $expr:tt
                ),+
            );*
            $(;)?
        }
    ) => {
        match $_ident {
            $(
                $(
                    $($(OpCode::$member_ident => trace_instruction!(@BLOCK $category, $member_ident, $expr!($frame, $member_ident $(, $($arg),+)?))),+)?
                    $($pat => trace_instruction!(@BLOCK $category, $pat, $expr))?
                ),+
            ),+
        }
    };
}

macro_rules! push_const {
	($frame:ident, $opcode:ident, $value:tt $(, $const_value:ident)?) => {{
		paste::paste! {
			{ $frame.stack_mut().push_op(Operand:: [<Const $value>] $((ConstOperandType:: $const_value))?); }
		};
        push_const!($frame, $($const_value)?)
	}};
    // Add `Empty` slots for long/double
    ($frame:ident, Long) => {{
		paste::paste! {
			{ $frame.stack_mut().push_op(Operand::Empty); }
		};
	}};
    ($frame:ident, Double) => {{
		paste::paste! {
			{ $frame.stack_mut().push_op(Operand::Empty); }
		};
	}};
    ($_frame:ident, $($_const_value:ident)?) => {{}};
}

macro_rules! local_variable_load {
	($frame:ident, $opcode:ident, $ty:ident) => {{
		let index = $frame.read_byte() as usize;
		local_variable_load!($frame, $opcode, $ty, index)
	}};
	($frame:ident, $opcode:ident, $ty:ident, $index:expr) => {{
		let local_stack = $frame.local_stack_mut();

		let local_variable = &local_stack[$index];
		paste::paste! {
			assert!(
				local_variable.[<is_ $ty:lower>](),
				"Invalid operand type on local stack for `{}` instruction: {:?}",
				stringify!($opcode),
				local_variable
			);
		}

		let local_variable = local_variable.clone();
		paste::paste! {
			{ $frame.stack_mut().push_op(local_variable); }
		}
	}};
}

macro_rules! load_from_array {
	($frame:ident, $opcode:ident) => {{
		let stack = $frame.stack_mut();
		let index = stack.pop_int();

		let object_ref = stack.pop_reference();
		let array_ref = object_ref.extract_array();

		// TODO: Validate the type, right now the output is just trusted
		//       to be correct
		stack.push_op(array_ref.get().get(index));
	}};
}

macro_rules! local_variable_store {
	($frame:ident, $opcode:ident, $ty:ident) => {{
		let index = $frame.read_byte() as usize;
		local_variable_store!($frame, $opcode, $ty, index)
	}};
	($frame:ident, $opcode:ident, $ty:ident, $index:expr) => {{
		let stack = $frame.stack_mut();
		let value = stack.pop();
		paste::paste! {
			assert!(
				value.[<is_ $ty:lower>](),
				"Invalid type on operand stack for `{}` instruction: {:?}",
				stringify!($opcode),
				value
			);
		}

		let local_stack = $frame.local_stack_mut();
		local_stack[$index] = value;
	}};
}

macro_rules! store_into_array {
	($frame:ident, $opcode:ident) => {{
		let stack = $frame.stack_mut();
		let value = stack.pop();
		let index = stack.pop_int();

		let object_ref = stack.pop_reference();
		let array_ref = object_ref.extract_array();

		array_ref.get_mut().store(index, value);
	}};
}

macro_rules! stack_operations {
	($frame:ident, $opcode:ident) => {{
		$frame.stack_mut().$opcode();
	}};
}

macro_rules! arithmetic {
	($frame:ident, $opcode:ident, $instruction:ident) => {{
		paste::paste! {
			{
				let stack = $frame.stack_mut();
				let rhs = stack.pop();
				let mut val = stack.pop();

				val.$instruction(rhs);
				stack.push_op(val);
			}
		}
	}};
}

macro_rules! conversions {
	($frame:ident, $instruction:ident) => {{
		let stack = $frame.stack_mut();
		let mut val = stack.pop();

		val.$instruction();
		stack.push_op(val);
	}};
}

/// The way branching is implemented requires that we add the `branch` to the `pc`
/// of the instruction. Since the `read_byte*` implementations seek `pc`, we need to subtract
/// the *2* branch bytes and *1* opcode byte from the branch before jumping.
const COMPARISON_SEEK_BACK: isize = -3;
macro_rules! comparisons {
    ($frame:ident, $instruction:ident, $operator:tt) => {{
        let stack = $frame.stack_mut();
        let rhs = stack.pop_int();
        let lhs = stack.pop_int();

        if lhs $operator rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().pc.fetch_add(branch + COMPARISON_SEEK_BACK, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
    ($frame:ident, $instruction:ident, $operator:tt, $rhs:literal) => {{
        let stack = $frame.stack_mut();
        let lhs = stack.pop_int();

        if lhs $operator $rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().pc.fetch_add(branch + COMPARISON_SEEK_BACK, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
    ($frame:ident, $instruction:ident, $operator:tt, $ty:ident) => {{
        let stack = $frame.stack_mut();
        paste::paste! {
            let rhs = stack.[<pop_ $ty>]();
            let lhs = stack.[<pop_ $ty>]();
        }

        if lhs $operator rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().pc.fetch_add(branch + COMPARISON_SEEK_BACK, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
}

macro_rules! control_return {
	($frame:ident, $instruction:ident) => {{
		let thread = $frame.thread();
		thread.drop_to_previous_frame(None);
	}};
	($frame:ident, $instruction:ident, $return_ty:ident) => {{
		let stack = $frame.stack_mut();
		let value = stack.pop();

		paste::paste! {
			assert!(
				value.[<is_ $return_ty>](),
				"Invalid type on operand stack for `{}` instruction: {:?}",
				stringify!($instruction),
				value
			);
		}

		let thread = $frame.thread();
		thread.drop_to_previous_frame(Some(value));
	}};
}

pub struct Interpreter;

#[rustfmt::skip]
#[allow(unused_braces)]
impl Interpreter {
	pub fn instruction(frame: &mut Frame) {
        // The opcodes are broken into sections as defined here:
        // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-7.html

        let opcode = OpCode::from(frame.read_byte());

        define_instructions! {
            frame: frame,
            match opcode {
                // ========= Constants =========
                CATEGORY: constants
                OpCode::nop => {},
                OpCode::aconst_null => {
                    frame.stack_mut().push_reference(Reference::null());
                },
                @GROUP {
                    [
                        iconst_m1 (m1),
                        iconst_0 (0, Int),
                        iconst_1 (1, Int),
                        iconst_2 (2, Int),
                        iconst_3 (3),
                        iconst_4 (4),
                        iconst_5 (5),
                        
                        lconst_0 (0, Long),
                        lconst_1 (1, Long),
                        
                        fconst_0 (0, Float),
                        fconst_1 (1, Float),
                        fconst_2 (2, Float),
                        
                        dconst_0 (0, Double),
                        dconst_1 (1, Double),
                    ]
                } => push_const,
                OpCode::bipush => {
                    let byte = frame.read_byte_signed();
                    frame.stack_mut().push_op(Operand::Int(s4::from(byte)));
                },
                OpCode::sipush => {
                    let short = frame.read_byte2_signed();
                    frame.stack_mut().push_op(Operand::Int(s4::from(short)));
                },
                OpCode::ldc => {
                    Interpreter::ldc(frame, false);
                },
                OpCode::ldc_w => {
                    Interpreter::ldc(frame, true);
                },
                OpCode::ldc2_w => {
                    Interpreter::ldc2_w(frame);
                };
                
                // ========= Loads =========
                CATEGORY: loads
                @GROUP {
                    [
                        iload   (Int),
                        iload_0 (Int, 0),
                        iload_1 (Int, 1),
                        iload_2 (Int, 2),
                        iload_3 (Int, 3),
                        
                        lload   (Long),
                        lload_0 (Long, 0),
                        lload_1 (Long, 1),
                        lload_2 (Long, 2),
                        lload_3 (Long, 3),
                        
                        fload   (Float),
                        fload_0 (Float, 0),
                        fload_1 (Float, 1),
                        fload_2 (Float, 2),
                        fload_3 (Float, 3),
                        
                        dload   (Double),
                        dload_0 (Double, 0),
                        dload_1 (Double, 1),
                        dload_2 (Double, 2),
                        dload_3 (Double, 3),
                        
                        aload   (Reference),
                        aload_0 (Reference, 0),
                        aload_1 (Reference, 1),
                        aload_2 (Reference, 2),
                        aload_3 (Reference, 3),
                    ]
                } => local_variable_load,
                @GROUP {
                    [
                        iaload,
                        laload,
                        faload,
                        daload,
                        aaload,
                        baload,
                        caload,
                        saload,
                    ]
                } => load_from_array;
                
                // ========= Stores =========
                CATEGORY: stores
                @GROUP {
                    [
                        istore   (int),
                        istore_0 (int, 0),
                        istore_1 (int, 1),
                        istore_2 (int, 2),
                        istore_3 (int, 3),
                        
                        lstore   (long),
                        lstore_0 (long, 0),
                        lstore_1 (long, 1),
                        lstore_2 (long, 2),
                        lstore_3 (long, 3),
                        
                        fstore   (float),
                        fstore_0 (float, 0),
                        fstore_1 (float, 1),
                        fstore_2 (float, 2),
                        fstore_3 (float, 3),
                        
                        dstore   (double),
                        dstore_0 (double, 0),
                        dstore_1 (double, 1),
                        dstore_2 (double, 2),
                        dstore_3 (double, 3),
                        
                        astore   (reference),
                        astore_0 (reference, 0),
                        astore_1 (reference, 1),
                        astore_2 (reference, 2),
                        astore_3 (reference, 3),
                    ]
                } => local_variable_store,
                @GROUP {
                    [
                        iastore,
                        lastore,
                        fastore,
                        dastore,
                        aastore,
                        bastore,
                        castore,
                        sastore,
                    ]
                } => store_into_array;
                
                // ========= Stack  =========
                CATEGORY: stack
                @GROUP {
                    [
                        pop,
                        pop2,
                        dup,
                        dup_x1,
                        dup_x2,
                        dup2,
                        dup2_x1,
                        dup2_x2,
                        swap,
                    ]
                } => stack_operations;
                
                // ========= Math =========
                // TODO: ushr
                CATEGORY: math
                @GROUP {
                    [
                        iadd (add),
                        isub (sub),
                        imul (mul),
                        idiv (div),
                        irem (rem),
                        ishl (shl),
                        ishr (shr),
                        iand (and),
                        ior  (or),
                        ixor (xor),
                        iushr (ushr),
                        
                        ladd (add),
                        lsub (sub),
                        lmul (mul),
                        ldiv (div),
                        lrem (rem),
                        lshl (shl),
                        lshr (shr),
                        land (and),
                        lor  (or),
                        lxor (xor),
                        lushr (ushr),
                        
                        fadd (add),
                        fsub (sub),
                        fmul (mul),
                        fdiv (div),
                        frem (rem),
                        
                        dadd (add),
                        dsub (sub),
                        dmul (mul),
                        ddiv (div),
                        drem (rem),
                    ]
                } => arithmetic,
                OpCode::ineg
                | OpCode::lneg
                | OpCode::fneg
                | OpCode::dneg => {
                    let mut val = frame.stack_mut().pop();
                    
                    val.neg();
                    frame.stack_mut().push_op(val);
                },
                OpCode::iinc => {
                    let index = frame.read_byte();
                    let const_ = frame.read_byte_signed();
                    
                    frame.local_stack_mut()[index as usize].add(Operand::Int(s4::from(const_)));
                };
                
                // ========= Conversions =========
                CATEGORY: conversions
                @GROUP {
                    [
                        i2l,
                        i2f,
                        i2d,
                        
                        l2i,
                        l2f,
                        l2d,
                        
                        f2i,
                        f2l,
                        f2d,
                        
                        d2i,
                        d2l,
                        d2f,
                        
                        i2b,
                        i2c,
                        i2s
                    ]
                } => conversions;
                
                // ========= Comparisons =========
                CATEGORY: comparisons
                OpCode::lcmp => {
                    let stack = frame.stack_mut();
                    let value2 = stack.pop_long();
                    let value1 = stack.pop_long();
                    
                    match value1.cmp(&value2) {
                        Ordering::Greater => stack.push_int(1),
                        Ordering::Equal => stack.push_int(0),
                        Ordering::Less => stack.push_int(-1)
                    }
                },
                OpCode::fcmpl => {
                    Interpreter::fcmp(frame, Ordering::Less);
                },
                OpCode::fcmpg => {
                    Interpreter::fcmp(frame, Ordering::Greater);
                },
                OpCode::dcmpl => {
                    Interpreter::dcmp(frame, Ordering::Less);
                },
                OpCode::dcmpg => {
                    Interpreter::dcmp(frame, Ordering::Greater);
                },
                @GROUP {
                    [
                        ifeq       (==, 0),
                        ifne       (!=, 0),
                        iflt       (< , 0),
                        ifge       (>=, 0),
                        ifgt       (> , 0),
                        ifle       (<=, 0),
                        if_icmpeq  (==),
                        if_icmpne  (!=),
                        if_icmplt  (<),
                        if_icmpge  (>=),
                        if_icmpgt  (>),
                        if_icmple  (<=),
                        if_acmpeq  (==, reference),
                        if_acmpne  (!=, reference),
                    ]
                } => comparisons;
                
                // ========= References =========
                // TODO: 
                //       invokedynamic
                CATEGORY: references
                OpCode::getstatic => {
                    let field = Self::fetch_field(frame, true);
                    frame.stack_mut().push_op(field.get_static_value());
                },
                OpCode::putstatic => {
                    let field = Self::fetch_field(frame, true);
                    let value = frame.stack_mut().pop();

                    field.set_static_value(value);
                },
                OpCode::getfield => {
                    let field = Self::fetch_field(frame, false);
                    if field.is_static() {
                        panic!("IncompatibleClassChangeError"); // TODO
                    }

                    let stack = frame.stack_mut();

                    let object_ref = stack.pop_reference();

                    let field_value = object_ref.get_field_value(field);
                    stack.push_op(field_value);
                },
                OpCode::putfield => {
                    let field = Self::fetch_field(frame, false);
                    if field.is_static() {
                        panic!("IncompatibleClassChangeError"); // TODO
                    }

                    // TODO: if the resolved field is final, it must be declared in the current class,
                    //       and the instruction must occur in an instance initialization method of the current class.
                    //       Otherwise, an IllegalAccessError is thrown.

                    let stack = frame.stack_mut();

                    let value = stack.pop();
                    let mut object_ref = stack.pop_reference();

                    object_ref.put_field_value(field, value);
                },
                OpCode::invokevirtual => {
                    let method = Self::fetch_method(frame, false);
                    MethodInvoker::invoke_virtual(frame, method);
                },
                OpCode::invokespecial => {
                    let method = Self::fetch_method(frame, false);
                    MethodInvoker::invoke(frame, method);
                },
                OpCode::invokestatic => {
                    let method = Self::fetch_method(frame, true);
                    MethodInvoker::invoke(frame, method);
                },
                OpCode::invokeinterface => {
                    let method = Self::fetch_method(frame, false);
                    // The count operand is an unsigned byte that must not be zero.
                    let count = frame.read_byte();
                    assert!(count > 0);

                    // The value of the fourth operand byte must always be zero.
                    assert_eq!(frame.read_byte(), 0);

                    MethodInvoker::invoke_interface(frame, method);
                },
                OpCode::new => {
                    let new_class_instance = Self::new(frame);
                    frame.stack_mut().push_reference(Reference::class(new_class_instance));
                },
                OpCode::newarray => {
                    let type_code = frame.read_byte();

                    let stack = frame.stack_mut();
                    let count = stack.pop_int();
                    
                    let array_ref = ArrayInstance::new_from_type(type_code, count);
                    stack.push_reference(Reference::array(array_ref));
                },
                OpCode::anewarray => {
                    let index = frame.read_byte2();

                    let constant_pool = frame.constant_pool();
                    let array_class = constant_pool.get::<cp_types::Class>(index);

                    let stack = frame.stack_mut();
                    let count = stack.pop_int();

                    let array_ref = ArrayInstance::new_reference(count, array_class);
                    stack.push_reference(Reference::array(array_ref));
                },
                OpCode::arraylength => {
                    let stack = frame.stack_mut();
                    let object_ref = stack.pop_reference();
                    let array_ref = object_ref.extract_array();
                    
                    let array_len = array_ref.get().elements.element_count();
                    stack.push_int(array_len as s4);
                },
                OpCode::athrow => {
                    let object_ref = frame.stack_mut().pop_reference();
                    let thread = frame.thread();
                    thread.throw_exception(object_ref);
                },
                OpCode::instanceof => { Self::instanceof_checkcast(frame, opcode) },
                OpCode::checkcast => { Self::instanceof_checkcast(frame, opcode) },
                OpCode::monitorenter => {
                    let object_ref = frame.stack_mut().pop_reference();
                    object_ref.monitor_enter(JavaThread::current())
                },
                OpCode::monitorexit => {
                    let object_ref = frame.stack_mut().pop_reference();
                    object_ref.monitor_exit(JavaThread::current())
                };

                // ========= Control =========
                // TODO: jsr, ret,
                CATEGORY: control
                OpCode::goto => {
                    let address = frame.read_byte2_signed() as isize;
                    let _ = frame.thread().pc.fetch_add(address + COMPARISON_SEEK_BACK, MemOrdering::Relaxed);
                },
                OpCode::tableswitch => {
                    Self::tableswitch(frame)
                },
                OpCode::lookupswitch => {
                    Self::lookupswitch(frame)
                },
                @GROUP {
                    [
                        ireturn (int),
                        lreturn (long),
                        freturn (float),
                        dreturn (double),
                        areturn (reference),
                        r#return,
                    ]
                } => control_return;
                
                // ========= Extended =========
                // TODO: wide, multianewarray, jsr_w
                CATEGORY: extended
                OpCode::multianewarray => {
                    Self::multianewarray(frame);
                },
                OpCode::ifnull => {
                    let reference = frame.stack_mut().pop_reference();
                    
                    if reference.is_null() {
                        let branch = frame.read_byte2_signed() as isize;
                        let _ = frame.thread().pc.fetch_add(branch + COMPARISON_SEEK_BACK, MemOrdering::Relaxed);
                    } else {
                        let _ = frame.thread().pc.fetch_add(2, MemOrdering::Relaxed);
                    }
                },
                OpCode::ifnonnull => {
                    let reference = frame.stack_mut().pop_reference();
                    
                    if reference.is_null() {
                        let _ = frame.thread().pc.fetch_add(2, MemOrdering::Relaxed);
                    } else {
                        let branch = frame.read_byte2_signed() as isize;
                        let _ = frame.thread().pc.fetch_add(branch + COMPARISON_SEEK_BACK, MemOrdering::Relaxed);
                    }
                },
                OpCode::goto_w => {
                    let address = frame.read_byte4_signed() as isize;
                    
                    assert!(address <= s2::MAX as isize, "goto_w offset too large!");
    
                    // See doc comment on `COMPARISON_SEEK_BACK` above for explanation of this subtraction
                    let _ = frame.thread().pc.fetch_add(address - 4, MemOrdering::Relaxed);
                };
                
                // ========= Reserved =========
                // TODO: breakpoint
                CATEGORY: reserved
                unknown_code => {
                    unimplemented!("{:?}", unknown_code)
                };
            }
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ldc
    fn ldc(frame: &mut Frame, wide: bool) {
        let idx = if wide {
            frame.read_byte2()
        } else {
            u2::from(frame.read_byte())
        };

        let constant_pool = frame.constant_pool();
        let constant = constant_pool.get_any(idx);

        // The run-time constant pool entry at index must be loadable (§5.1),
        match constant {
            // and not any of the following:
            Entry::Long { .. }
            | Entry::Double { .. } => panic!("ldc called with index to long/double"),

            // If the run-time constant pool entry is a numeric constant of type int or float,
            // then the value of that numeric constant is pushed onto the operand stack as an int or float, respectively.
            Entry::Integer(int) => frame.stack_mut().push_int(int),
            Entry::Float(float) => frame.stack_mut().push_float(float),

            // Otherwise, if the run-time constant pool entry is a string constant, that is,
            // a reference to an instance of class String, then value, a reference to that instance, is pushed onto the operand stack.
            Entry::String(string) => {
                let interned_string = StringInterner::intern_symbol(string);
                frame.stack_mut().push_reference(Reference::class(interned_string));
            },

            // Otherwise, if the run-time constant pool entry is a symbolic reference to a class or interface,
            // then the named class or interface is resolved (§5.4.3.1) and value, a reference to the Class object
            // representing that class or interface, is pushed onto the operand stack.
            Entry::Class(class) => {
                frame.stack_mut().push_reference(Reference::mirror(class.mirror()));
            },

            // Otherwise, the run-time constant pool entry is a symbolic reference to a method type, a method handle,
            // or a dynamically-computed constant. The symbolic reference is resolved (§5.4.3.5, §5.4.3.6) and value,
            // the result of resolution, is pushed onto the operand stack.
            Entry::MethodHandle { .. } => unimplemented!("MethodHandle in ldc"),
            Entry::MethodType { .. } => unimplemented!("MethodType in ldc"),
            _ => unreachable!()
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ldc2_w
    fn ldc2_w(frame: &mut Frame) {
        let idx = frame.read_byte2();

        let constant_pool = frame.constant_pool();
        let constant = constant_pool.get_any(idx);

        // The run-time constant pool entry at index must be loadable (§5.1),
        match constant {
            // and not any of the following:
            Entry::Long(long) => {
                frame.stack_mut().push_long(long)
            },
            Entry::Double(double) => {
                frame.stack_mut().push_double(double)
            },

            _ => panic!("ldc2_w called with index to non long/double constant")
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fcmp_op
    fn fcmp(frame: &mut Frame, ordering: Ordering) {
        let operand_stack = frame.stack_mut();

        // Both value1 and value2 must be of type float.
        // The values are popped from the operand stack and a floating-point comparison is performed:
        let value2 = operand_stack.pop_float();
        let value1 = operand_stack.pop_float();

        match value1.partial_cmp(&value2) {
            // If value1 is greater than value2, the int value 1 is pushed onto the operand stack.
            Some(Ordering::Greater) => operand_stack.push_int(1),
            // Otherwise, if value1 is equal to value2, the int value 0 is pushed onto the operand stack.
            Some(Ordering::Equal) => operand_stack.push_int(0),
            // Otherwise, if value1 is less than value2, the int value -1 is pushed onto the operand stack.
            Some(Ordering::Less) => operand_stack.push_int(-1),
            // Otherwise, at least one of value1 or value2 is NaN.
            // The fcmpg instruction pushes the int value 1 onto the operand stack and the fcmpl instruction pushes the int value -1 onto the operand stack.
            _ => {
                match ordering {
                    Ordering::Greater => operand_stack.push_int(1),
                    Ordering::Less => operand_stack.push_int(-1),
                    _ => unreachable!()
                }
            },
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.dcmp_op
    fn dcmp(frame: &mut Frame, ordering: Ordering) {
        let operand_stack = frame.stack_mut();

        // Both value1 and value2 must be of type double.
        // The values are popped from the operand stack and a floating-point comparison is performed:
        let value2 = operand_stack.pop_double();
        let value1 = operand_stack.pop_double();

        match value1.partial_cmp(&value2) {
            // If value1 is greater than value2, the int value 1 is pushed onto the operand stack.
            Some(Ordering::Greater) => operand_stack.push_int(1),
            // Otherwise, if value1 is equal to value2, the int value 0 is pushed onto the operand stack.
            Some(Ordering::Equal) => operand_stack.push_int(0),
            // Otherwise, if value1 is less than value2, the int value -1 is pushed onto the operand stack.
            Some(Ordering::Less) => operand_stack.push_int(-1),
            // Otherwise, at least one of value1 or value2 is NaN.
            // The dcmpg instruction pushes the int value 1 onto the operand stack and the dcmpl instruction pushes the int value -1 onto the operand stack.
            _ => {
                match ordering {
                    Ordering::Greater => operand_stack.push_int(1),
                    Ordering::Less => operand_stack.push_int(-1),
                    _ => unreachable!()
                }
            },
        }
    }
    
    fn instanceof_checkcast(frame: &mut Frame, opcode: OpCode) {
        let index = frame.read_byte2();

        let objectref;
        {
            let stack = frame.stack_mut();
            objectref = stack.pop_reference();

            if objectref.is_null() {
                match opcode {
                    // If objectref is null, the instanceof instruction pushes an int result of 0 as an int onto the operand stack.
                    OpCode::instanceof => stack.push_int(0),
                    // If objectref is null, then the operand stack is unchanged.
                    OpCode::checkcast => stack.push_reference(objectref),
                    _ => unreachable!()
                }

                return;
            }
        }

        let constant_pool = frame.constant_pool();
        let class = constant_pool.get::<cp_types::Class>(index);

        let stack = frame.stack_mut();
        if objectref.is_instance_of(class) {
            match opcode {
                // If objectref is an instance of the resolved class or array type, or implements the resolved interface,
                // the instanceof instruction pushes an int result of 1 as an int onto the operand stack
                OpCode::instanceof => stack.push_int(1),
                // If objectref can be cast to the resolved class, array, or interface type, the operand stack is unchanged
                OpCode::checkcast => stack.push_reference(objectref),
                _ => unreachable!()
            }

            return;
        }

        match opcode {
            OpCode::instanceof => stack.push_int(0),
            OpCode::checkcast => panic!("ClassCastException"), // TODO
            _ => unreachable!()
        }
    }

    fn fetch_field(frame: &mut Frame, is_static: bool) -> &'static Field {
        let field_ref_idx = frame.read_byte2();

        let constant_pool = frame.constant_pool();

        let ret = constant_pool.get::<cp_types::FieldRef>(field_ref_idx);
        if is_static {
            ret.class.initialize(frame.thread());
        }

        ret
    }
    
    fn fetch_method(frame: &mut Frame, is_static: bool) -> &'static Method {
        let method_ref_idx = frame.read_byte2();

        let constant_pool = frame.constant_pool();

        let ret = constant_pool.get::<cp_types::MethodRef>(method_ref_idx);
        if is_static {
            // On successful resolution of the method, the class or interface that declared the resolved method is initialized if that class or interface has not already been initialized
            ret.class().initialize(frame.thread());
        }

        ret
    }

    fn new(frame: &mut Frame) -> ClassInstanceRef {
        let index = frame.read_byte2();

        let constant_pool = frame.constant_pool();

        let class = constant_pool.get::<cp_types::Class>(index);
        if class.is_interface() || class.is_abstract() {
            panic!("InstantiationError") // TODO
        }

        // On successful resolution of the class, it is initialized if it has not already been initialized
        class.initialize(frame.thread());

        ClassInstance::new(class)
    }

    fn tableswitch(frame: &mut Frame) {
        // Subtract 1, since we already read the opcode
        let opcode_address = frame.thread().pc.load(MemOrdering::Relaxed) - 1;
        frame.skip_padding();
        
        let default = frame.read_byte4_signed() as isize;
        let low = frame.read_byte4_signed() as isize;
        let high = frame.read_byte4_signed() as isize;
        assert!(low <= high);
        
        let jump_offsets_size = (high - low + 1) as usize;
        let mut jump_offsets = vec![0; jump_offsets_size];
        for i in &mut jump_offsets {
            *i =  frame.read_byte4_signed();
        }
        
        let offset;
        let index = frame.stack_mut().pop_int() as isize;
        if index < low || index > high {
            offset = default;
        } else {
            offset = jump_offsets[(index - low) as usize] as isize;
        }
        
        frame.thread().pc.store(opcode_address + offset, MemOrdering::Relaxed);
    }

    fn lookupswitch(frame: &mut Frame) {
        // Subtract 1, since we already read the opcode
        let opcode_address = frame.thread().pc.load(MemOrdering::Relaxed) - 1;
        frame.skip_padding();
        
        let default = frame.read_byte4_signed() as isize;
        let npairs = frame.read_byte4_signed();
        assert!(npairs >= 0);
        
        let mut match_offset_pairs = vec![(0i32, 0i32); npairs as usize];
        for (match_, offset) in &mut match_offset_pairs {
            *match_ = frame.read_byte4_signed();
            *offset = frame.read_byte4_signed();
        }

        let key = frame.stack_mut().pop_int();
        let offset;
        
        if let Some((_, matched_offset)) = match_offset_pairs.iter().find(|(match_, _)| *match_ == key) {
            offset = *matched_offset as isize;
        } else {
            offset = default;
        }

        frame.thread().pc.store(opcode_address + offset, MemOrdering::Relaxed);
    }
	
	fn multianewarray(frame: &mut Frame) {
		let index = frame.read_byte2();
		let dimensions = frame.read_byte();
		
		assert!(dimensions >= 1);
		
        let constant_pool = frame.constant_pool();
        let class = constant_pool.get::<cp_types::Class>(index);
		
		class.initialize(frame.thread());
		
		let counts = frame.stack_mut().popn(dimensions as usize);
		
        let array_ref = handle_exception!(frame.thread(), ArrayInstance::new_multidimensional(counts.into_iter().map(|op| op.expect_int()), class));
        frame.stack_mut().push_reference(Reference::array(array_ref));
	}
}
