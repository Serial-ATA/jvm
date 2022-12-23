use crate::class::Class;
use crate::class_instance::ClassInstance;
use crate::frame::FrameRef;
use crate::method_invoker::MethodInvoker;
use crate::reference::{FieldRef, MethodRef, Reference};
use crate::stack::operand_stack::Operand;
use crate::string_interner::StringInterner;

use std::cmp::Ordering;
use std::sync::atomic::Ordering as MemOrdering;
use std::sync::Arc;

use classfile::ConstantPoolValueInfo;
use common::int_types::{s2, s4, u2};
use common::traits::PtrType;
use instructions::{OpCode, OperandLike, StackLike};

macro_rules! trace_instruction {
	($category:literal, $instruction:ident) => {{
		#[cfg(debug_assertions)]
		{ log::trace!("[INSTRUCTION] [{}] {}", $category, stringify!($instruction)) }
	}};
	($category:literal, $instruction:ident, $fmt_string:literal, $($arg:tt)*) => {{
		#[cfg(debug_assertions)]
		{
            log::trace!(
                "[INSTRUCTION] [{}] {} - called with args: [{}]",
                $category,
                stringify!($instruction),
                format_args!($fmt_string, $($arg)*)
		    )
        }
	}};
}

// TODO: Document
macro_rules! define_instructions {
    (
        frame: $frame:ident,
        match $_ident:ident {
            $($(@GROUP { [$($member_ident:ident $(($($arg:tt),+))?),+ $(,)?] })? $($pat:pat)? => $expr:tt),+ $(,)?
        }
    ) => {
        match $_ident {
            $(
                $($(OpCode::$member_ident => $expr!($frame, $member_ident $(, $($arg),+)?)),+)?
                $($pat => $expr)?
            ),+
        }
    };
}

macro_rules! push_const {
	($frame:ident, $opcode:ident, $value:tt) => {{
		trace_instruction!("constants", $opcode);
		paste::paste! {
			{ $frame.get_operand_stack_mut().push_op(Operand:: [<Const $value>]); }
		};
	}};
}

macro_rules! local_variable_load {
	($frame:ident, $opcode:ident, $ty:ident) => {{
		let local_stack = $frame.get_local_stack();
		let index = $frame.read_byte() as usize;

		let local_variable = &local_stack[index];
		trace_instruction!(
			"loads",
			$opcode,
			"index = {}, local_variable = {:?}",
			index,
			local_variable
		);
		assert!(
			matches!(local_variable, Operand::$ty(_)),
			"Invalid operand type on local stack for `{}` instruction",
			stringify!($opcode)
		);

		paste::paste! {
			{ $frame.get_operand_stack_mut().push_op(local_variable.clone()); }
		}
	}};
	($frame:ident, $opcode:ident, $ty:ident, $index:literal) => {{
		let local_stack = $frame.get_local_stack();
		let local_variable = &local_stack[$index];
		trace_instruction!(
			"loads",
			$opcode,
			"index = {}, local_variable = {:?}",
			$index,
			local_variable
		);

		assert!(
			matches!(local_variable, Operand::$ty(_)),
			"Invalid operand type on local stack for `{}` instruction",
			stringify!($opcode)
		);

		paste::paste! {
			{ $frame.get_operand_stack_mut().push_op(local_variable.clone()); }
		}
	}};
}

macro_rules! stack_operations {
	($frame:ident, $opcode:ident) => {{
		trace_instruction!("stack", $opcode);
		$frame.get_operand_stack_mut().$opcode();
	}};
}

macro_rules! arithmetic {
	($frame:ident, $opcode:ident, $instruction:ident) => {
		paste::paste! {
			{
				let stack = $frame.get_operand_stack_mut();
				let mut val = stack.pop();
				let rhs = stack.pop();
				trace_instruction!("arithmetic", $opcode, "lhs = {:?}, rhs = {:?}", val, rhs);

				val.$instruction(rhs);
				stack.push_op(val);
			}
		}
	};
}

macro_rules! conversions {
	($frame:ident, $instruction:ident) => {{
		let stack = $frame.get_operand_stack_mut();
		let mut val = stack.pop();
		trace_instruction!("conversions", $instruction, "val = {:?}", val);

		val.$instruction();
		stack.push_op(val);
	}};
}

macro_rules! comparisons {
    ($frame:ident, $instruction:ident, $operator:tt) => {{
        let stack = $frame.get_operand_stack_mut();
        let rhs = stack.pop_int();
        let lhs = stack.pop_int();
        trace_instruction!("comparisons", $instruction, "lhs = {:?}, rhs = {:?}", lhs, rhs);

        if lhs $operator rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().get().pc.fetch_add(branch, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().get().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
    ($frame:ident, $instruction:ident, $operator:tt, $rhs:literal) => {{
        let stack = $frame.get_operand_stack_mut();
        let lhs = stack.pop_int();
        trace_instruction!("comparisons", $instruction, "lhs = {:?}, rhs = {}", lhs, $rhs);

        if lhs $operator $rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().get().pc.fetch_add(branch, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().get().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
}

macro_rules! control_return {
	($frame:ident, $instruction:ident) => {{
		trace_instruction!("control", $instruction);
		$frame.thread().get_mut().drop_to_previous_frame(None);
	}};
	($frame:ident, $instruction:ident, $return_ty:ident) => {{
		let stack = $frame.get_operand_stack_mut();
		let val = stack.pop();
		trace_instruction!("control", $instruction, "val = {:?}", val);

		assert!(
			matches!(val, Operand::$return_ty(_)),
			"Invalid return type for `{}` instruction",
			stringify!($instruction)
		);

		$frame.thread().get_mut().drop_to_previous_frame(Some(val));
	}};
}

pub struct Interpreter;

#[rustfmt::skip]
impl Interpreter {
	pub fn instruction(frame: FrameRef) {
        // The opcodes are broken into sections as defined here:
        // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-7.html

        let opcode = OpCode::from(frame.read_byte());

        define_instructions! {
            frame: frame,
            match opcode {
                // ========= Constants =========
                // TODO: ldc2_w
                OpCode::nop => { trace_instruction!("constants", nop); },
                OpCode::aconst_null => {
                    trace_instruction!("constants", aconst_null);
                    frame.get_operand_stack_mut().push_reference(Reference::Null);
                },
                @GROUP {
                    [
                        iconst_m1 (m1),
                        iconst_0 (0),
                        iconst_1 (1),
                        iconst_2 (2),
                        iconst_3 (3),
                        iconst_4 (4),
                        iconst_5 (5),
                        
                        lconst_0 (0),
                        lconst_1 (1),
                        
                        fconst_0 (0),
                        fconst_1 (1),
                        fconst_2 (2),
                        
                        dconst_0 (0),
                        dconst_1 (1),
                    ]
                } => push_const,
                OpCode::bipush => {
                    let byte = frame.read_byte_signed();
                    trace_instruction!("constants", bipush, "{}", byte);
                    
                    frame.get_operand_stack_mut().push_op(Operand::Int(s4::from(byte)));
                },
                OpCode::sipush => {
                    let short = frame.read_byte2_signed();
                    trace_instruction!("constants", sipush, "{}", short);
                    
                    frame.get_operand_stack_mut().push_op(Operand::Int(s4::from(short)));
                },
                OpCode::ldc => {
                    Interpreter::ldc(frame, false);
                },
                OpCode::ldc_w => {
                    Interpreter::ldc(frame, true);
                },
                
                // ========= Loads =========
                // TODO: iaload, laload, faload, daload, aaload, baload,
                //       caload, saload
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
                
                // ========= Stores =========
                // TODO
                
                // ========= Stack  =========
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
                } => stack_operations,
                
                // ========= Math =========
                // TODO: shl, ushr, and, or, xor, inc
                @GROUP {
                    [
                        iadd (add),
                        isub (sub),
                        imul (mul),
                        idiv (div),
                        irem (rem),
                        
                        ladd (add),
                        lsub (sub),
                        lmul (mul),
                        ldiv (div),
                        lrem (rem),
                        
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
                    let mut val = frame.get_operand_stack_mut().pop();
                    trace_instruction!("arithmetic", neg, "val = {:?}", val);
                    
                    val.neg();
                    frame.get_operand_stack_mut().push_op(val);
                },
                
                // ========= Conversions =========
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
                } => conversions,
                
                // ========= Comparisons =========
                // TODO: lcmp, dcmpl, dcmpg, if_acmpeq, if_acmpne
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
                    ]
                } => comparisons,
                OpCode::fcmpl => {
                    Interpreter::fcmp(frame, Ordering::Less);
                },
                OpCode::fcmpg => {
                    Interpreter::fcmp(frame, Ordering::Greater);
                },
                
                // ========= References =========
                // TODO: getfield, putfield,
                //       invokevirtual, invokespecial, invokestatic,
                //       invokeinterface, invokedynamic, new, newarray,
                //       anewarray, arraylength, athrow, checkcast, instanceof,
                //       monitorenter, monitorexit
                OpCode::getstatic => {
                    let field = Self::fetch_field(FrameRef::clone(&frame));
                    trace_instruction!("references", getstatic, "field = {:?}", field);
                    frame.get_operand_stack_mut().push_op(field.get_static_value());
                },
                OpCode::putstatic => {
                    let field = Self::fetch_field(FrameRef::clone(&frame));
                    let value = frame.get_operand_stack_mut().pop();
                    trace_instruction!("references", putstatic, "field = {:?}, value = {:?}", field, value);
                    
                    field.set_static_value(value);
                },
                // Static/virtual are differentiated in `MethodInvoker::invoke`
                OpCode::invokevirtual
                | OpCode::invokespecial
                | OpCode::invokestatic => {
                    let method = Self::fetch_method(FrameRef::clone(&frame));
                    trace_instruction!("references", invoke, "method = {:?}", method);
                    
                    MethodInvoker::invoke(frame, method);
                },
                OpCode::arraylength => {
                    let stack = frame.get_operand_stack_mut();
                    
                    let array_ref = stack.pop_reference();
                    trace_instruction!("references", arraylength, "array = {:?}", array_ref);
                    
                    if array_ref.is_null() {
                        panic!("NullPointerException"); // TODO
                    }
                    
                    let array_len = array_ref.extract_array().elements.element_count();
                    stack.push_int(array_len as s4);
                },
                
                // ========= Control =========
                // TODO: jsr, ret, tableswitch, lookupswitch,
                OpCode::goto => {
                    let address = frame.read_byte2_signed() as isize;
                    trace_instruction!("control", goto, "address = {}", address);
                    
                    let _ = frame.thread().get().pc.fetch_add(address, MemOrdering::Relaxed);
                },
                @GROUP {
                    [
                        ireturn (Int),
                        lreturn (Long),
                        freturn (Float),
                        dreturn (Double),
                        areturn (Reference),
                        r#return,
                    ]
                } => control_return,
                
                // ========= Extended =========
                // TODO: wide, multianewarray, jsr_w
                OpCode::ifnull => {
                    let reference = frame.get_operand_stack_mut().pop_reference();
                    trace_instruction!("extended", ifnull, "reference = {:?}", reference);
                    
                    if reference.is_null() {
                        let branch = frame.read_byte2_signed() as isize;
                        let _ = frame.thread().get().pc.fetch_add(branch, MemOrdering::Relaxed);
                    }
                },
                OpCode::ifnonnull => {
                    let reference = frame.get_operand_stack_mut().pop_reference();
                    trace_instruction!("extended", ifnonnull, "reference = {:?}", reference);
                    
                    if !reference.is_null() {
                        let branch = frame.read_byte2_signed() as isize;
                        let _ = frame.thread().get().pc.fetch_add(branch, MemOrdering::Relaxed);
                    }
                },
                OpCode::goto_w => {
                    let address = frame.read_byte4_signed() as isize;
                    trace_instruction!("extended", goto_w, "address = {}", address);
                    
                    assert!(address <= s2::MAX as isize, "goto_w offset too large!");
    
                    let _ = frame.thread().get().pc.fetch_add(address, MemOrdering::Relaxed);
                },
                
                // ========= Reserved =========
                // TODO: breakpoint
                code => {
                    unimplemented!("{:?}", code)
                }
            }
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.ldc
    fn ldc(frame: FrameRef, wide: bool) {
        let idx = if wide {
            frame.read_byte2()
        } else {
            u2::from(frame.read_byte())
        };

        let method = frame.method();
        let class_ref = &method.class;

        let constant_pool = &class_ref.unwrap_class_instance().constant_pool;
        let constant = &constant_pool[idx];
        
        trace_instruction!("constants", ldc, "wide = {}, idx = {}, constant = {:?}", wide, idx, constant);
        
        // The run-time constant pool entry at index must be loadable (§5.1),
        match constant {
            // and not any of the following:
            ConstantPoolValueInfo::Long { .. }
            | ConstantPoolValueInfo::Double { .. } => panic!("ldx called with index to long/double"),

            // If the run-time constant pool entry is a numeric constant of type int or float,
            // then the value of that numeric constant is pushed onto the operand stack as an int or float, respectively.
            ConstantPoolValueInfo::Integer { bytes } => frame.get_operand_stack_mut().push_int((*bytes) as s4),
            ConstantPoolValueInfo::Float { bytes } => frame.get_operand_stack_mut().push_float(f32::from_be_bytes(bytes.to_be_bytes())),

            // Otherwise, if the run-time constant pool entry is a string constant, that is,
            // a reference to an instance of class String, then value, a reference to that instance, is pushed onto the operand stack.
            ConstantPoolValueInfo::String { string_index } => {
                let bytes = constant_pool.get_constant_utf8(*string_index);
                let interned_string = StringInterner::get_java_string(bytes, frame.thread());

                frame.get_operand_stack_mut().push_reference(Reference::Class(interned_string));
            },

            // Otherwise, if the run-time constant pool entry is a symbolic reference to a class or interface,
            // then the named class or interface is resolved (§5.4.3.1) and value, a reference to the Class object
            // representing that class or interface, is pushed onto the operand stack.
            ConstantPoolValueInfo::Class { name_index } => {
                let class = class_ref.get();

                let class_name = constant_pool.get_class_name(*name_index);
                let classref = class.loader.load(class_name).unwrap();

                let new_class_instance = ClassInstance::new(classref);
                frame.get_operand_stack_mut().push_reference(Reference::Class(new_class_instance));
            },

            // Otherwise, the run-time constant pool entry is a symbolic reference to a method type, a method handle,
            // or a dynamically-computed constant. The symbolic reference is resolved (§5.4.3.5, §5.4.3.6) and value,
            // the result of resolution, is pushed onto the operand stack.
            ConstantPoolValueInfo::MethodHandle { .. } => unimplemented!("MethodHandle in ldc"),
            ConstantPoolValueInfo::MethodType { .. } => unimplemented!("MethodType in ldc"),
            _ => unreachable!()
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.fcmp_op
    fn fcmp(frame: FrameRef, ordering: Ordering) {
        let operand_stack = frame.get_operand_stack_mut();

        // Both value1 and value2 must be of type float.
        // The values are popped from the operand stack and a floating-point comparison is performed:
        let lhs = operand_stack.pop();
        let rhs = operand_stack.pop();

        match lhs.partial_cmp(&rhs) {
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
    
    fn fetch_field(frame: FrameRef) -> FieldRef {
        let field_ref_idx = frame.read_byte2();

        let method = frame.method();
        let class = Arc::clone(&method.class);

        let constant_pool = Arc::clone(&class.unwrap_class_instance().constant_pool);

        Class::resolve_field(frame.thread(), constant_pool, field_ref_idx)
    }
    
    fn fetch_method(frame: FrameRef) -> MethodRef {
        let method_ref_idx = frame.read_byte2();

        let method = frame.method();
        let class = Arc::clone(&method.class);

        let constant_pool = Arc::clone(&class.unwrap_class_instance().constant_pool);
        Class::resolve_method(frame.thread(), constant_pool, method_ref_idx).unwrap()
    }
}
