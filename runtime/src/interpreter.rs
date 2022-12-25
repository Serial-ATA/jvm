use crate::class::Class;
use crate::class_instance::ArrayInstance;
use crate::classpath::classloader::ClassLoader;
use crate::frame::FrameRef;
use crate::method_invoker::MethodInvoker;
use crate::reference::{FieldRef, MethodRef, Reference};
use crate::string_interner::StringInterner;

use std::cmp::Ordering;
use std::sync::atomic::Ordering as MemOrdering;
use std::sync::Arc;

use classfile::ConstantPoolValueInfo;
use common::int_types::{s2, s4, u2};
use common::traits::PtrType;
use instructions::{ConstOperandType, OpCode, Operand, StackLike};

macro_rules! trace_instruction {
    (@START $instruction:tt, $category:ident) => {{
		#[cfg(debug_assertions)]
		{ log::trace!("[INSTRUCTION] [{}] {} START", stringify!($category), stringify!($instruction)) }
	}};
    (@END $instruction:tt, $category:ident) => {{
		#[cfg(debug_assertions)]
		{ log::trace!("[INSTRUCTION] [{}] {} SUCCEEDED", stringify!($category), stringify!($instruction)) }
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
			{ $frame.get_operand_stack_mut().push_op(Operand:: [<Const $value>] $((ConstOperandType:: $const_value))?); }
		};
	}};
}

macro_rules! local_variable_load {
	($frame:ident, $opcode:ident, $ty:ident) => {{
		let local_stack = $frame.get_local_stack_mut();
		let index = $frame.read_byte() as usize;

		let local_variable = &local_stack[index];
		paste::paste! {
			assert!(
				local_variable.[<is_ $ty:lower>](),
				"Invalid operand type on local stack for `{}` instruction",
				stringify!($opcode)
			);
		}

		paste::paste! {
			{ $frame.get_operand_stack_mut().push_op(local_variable.clone()); }
		}
	}};
	($frame:ident, $opcode:ident, $ty:ident, $index:literal) => {{
		let local_stack = $frame.get_local_stack_mut();
		let local_variable = &local_stack[$index];

		paste::paste! {
			assert!(
				local_variable.[<is_ $ty:lower>](),
				"Invalid operand type on local stack for `{}` instruction",
				stringify!($opcode)
			);
		}

		paste::paste! {
			{ $frame.get_operand_stack_mut().push_op(local_variable.clone()); }
		}
	}};
}

macro_rules! load_from_array {
	($frame:ident, $opcode:ident) => {{
		let stack = $frame.get_operand_stack_mut();
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
		let local_stack = $frame.get_local_stack_mut();
		let index = $frame.read_byte() as usize;

		let stack = $frame.get_operand_stack_mut();
		let value = stack.pop();
		paste::paste! {
			assert!(
				value.[<is_ $ty:lower>](),
				"Invalid type on operand stack for `{}` instruction",
				stringify!($opcode)
			);
		}

		local_stack[index] = value;
	}};
	($frame:ident, $opcode:ident, $ty:ident, $index:literal) => {{
		let local_stack = $frame.get_local_stack_mut();

		let stack = $frame.get_operand_stack_mut();
		let value = stack.pop();
		paste::paste! {
			assert!(
				value.[<is_ $ty:lower>](),
				"Invalid type on operand stack for `{}` instruction",
				stringify!($opcode)
			);
		}

		local_stack[$index] = value;
	}};
}

macro_rules! store_into_array {
	($frame:ident, $opcode:ident) => {{
		let stack = $frame.get_operand_stack_mut();
		let value = stack.pop();
		let index = stack.pop_int();

		let object_ref = stack.pop_reference();
		let array_ref = object_ref.extract_array();

		array_ref.get_mut().store(index, value);
	}};
}

macro_rules! stack_operations {
	($frame:ident, $opcode:ident) => {{
		$frame.get_operand_stack_mut().$opcode();
	}};
}

macro_rules! arithmetic {
	($frame:ident, $opcode:ident, $instruction:ident) => {{
		paste::paste! {
			{
				let stack = $frame.get_operand_stack_mut();
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
		let stack = $frame.get_operand_stack_mut();
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
        let stack = $frame.get_operand_stack_mut();
        let rhs = stack.pop_int();
        let lhs = stack.pop_int();

        if lhs $operator rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().get().pc.fetch_add(branch + COMPARISON_SEEK_BACK, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().get().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
    ($frame:ident, $instruction:ident, $operator:tt, $rhs:literal) => {{
        let stack = $frame.get_operand_stack_mut();
        let lhs = stack.pop_int();

        if lhs $operator $rhs {
            let branch = $frame.read_byte2_signed() as isize;
            let _ = $frame.thread().get().pc.fetch_add(branch + COMPARISON_SEEK_BACK, std::sync::atomic::Ordering::Relaxed);
        } else {
            let _ = $frame.thread().get().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
}

macro_rules! control_return {
	($frame:ident, $instruction:ident) => {{
		$frame.thread().get_mut().drop_to_previous_frame(None);
	}};
	($frame:ident, $instruction:ident, $return_ty:ident) => {{
		let stack = $frame.get_operand_stack_mut();
		let val = stack.pop();

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
                CATEGORY: constants
                // TODO: ldc2_w
                OpCode::nop => {},
                OpCode::aconst_null => {
                    frame.get_operand_stack_mut().push_reference(Reference::Null);
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
                    frame.get_operand_stack_mut().push_op(Operand::Int(s4::from(byte)));
                },
                OpCode::sipush => {
                    let short = frame.read_byte2_signed();
                    frame.get_operand_stack_mut().push_op(Operand::Int(s4::from(short)));
                },
                OpCode::ldc => {
                    Interpreter::ldc(frame, false);
                },
                OpCode::ldc_w => {
                    Interpreter::ldc(frame, true);
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
                        istore   (Int),
                        istore_0 (Int, 0),
                        istore_1 (Int, 1),
                        istore_2 (Int, 2),
                        istore_3 (Int, 3),
                        
                        lstore   (Long),
                        lstore_0 (Long, 0),
                        lstore_1 (Long, 1),
                        lstore_2 (Long, 2),
                        lstore_3 (Long, 3),
                        
                        fstore   (Float),
                        fstore_0 (Float, 0),
                        fstore_1 (Float, 1),
                        fstore_2 (Float, 2),
                        fstore_3 (Float, 3),
                        
                        dstore   (Double),
                        dstore_0 (Double, 0),
                        dstore_1 (Double, 1),
                        dstore_2 (Double, 2),
                        dstore_3 (Double, 3),
                        
                        astore   (Reference),
                        astore_0 (Reference, 0),
                        astore_1 (Reference, 1),
                        astore_2 (Reference, 2),
                        astore_3 (Reference, 3),
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
                // TODO: ushr, and, or, xor, inc
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
                        
                        ladd (add),
                        lsub (sub),
                        lmul (mul),
                        ldiv (div),
                        lrem (rem),
                        lshl (shl),
                        lshr (shr),
                        
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
                    
                    val.neg();
                    frame.get_operand_stack_mut().push_op(val);
                },
                OpCode::iinc => {
                    let index = frame.read_byte();
                    let const_ = frame.read_byte_signed();
                    
                    frame.get_local_stack_mut()[index as usize].add(Operand::Int(s4::from(const_)));
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
                // TODO: lcmp, dcmpl, dcmpg, if_acmpeq, if_acmpne
                CATEGORY: comparisons
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
                };
                
                // ========= References =========
                // TODO: 
                //       invokeinterface, invokedynamic, new,
                //       athrow, checkcast, instanceof,
                //       monitorenter, monitorexit
                CATEGORY: references
                OpCode::getstatic => {
                    let field = Self::fetch_field(FrameRef::clone(&frame));
                    frame.get_operand_stack_mut().push_op(field.get_static_value());
                },
                OpCode::putstatic => {
                    let field = Self::fetch_field(FrameRef::clone(&frame));
                    let value = frame.get_operand_stack_mut().pop();
                    
                    field.set_static_value(value);
                },
                OpCode::getfield => {
                    let field = Self::fetch_field(FrameRef::clone(&frame));
                    if field.is_static() {
                        panic!("IncompatibleClassChangeError"); // TODO
                    }

                    let stack = frame.get_operand_stack_mut();
                    
                    let object_ref = stack.pop_reference();
                    let mirror = object_ref.extract_mirror();
                    
                    let field_value = mirror.get().get_field_value(field.idx);
                    stack.push_op(field_value);
                },
                OpCode::putfield => {
                    let field = Self::fetch_field(FrameRef::clone(&frame));
                    if field.is_static() {
                        panic!("IncompatibleClassChangeError"); // TODO
                    }
                    
                    // TODO: if the resolved field is final, it must be declared in the current class,
                    //       and the instruction must occur in an instance initialization method of the current class.
                    //       Otherwise, an IllegalAccessError is thrown. 
                    
                    let stack = frame.get_operand_stack_mut();
                    
                    let value = stack.pop();
                    let object_ref = stack.pop_reference();
                    let mirror = object_ref.extract_mirror();
                    
                    mirror.get_mut().put_field_value(field.idx, value);
                },
                // Static/virtual are differentiated in `MethodInvoker::invoke`
                OpCode::invokevirtual
                | OpCode::invokespecial
                | OpCode::invokestatic => {
                    let method = Self::fetch_method(FrameRef::clone(&frame));
                    MethodInvoker::invoke(frame, method);
                },
                OpCode::arraylength => {
                    let stack = frame.get_operand_stack_mut();
                    let object_ref = stack.pop_reference();
                    let array_ref = object_ref.extract_array();
                    
                    let array_len = array_ref.get().elements.element_count();
                    stack.push_int(array_len as s4);
                },
                OpCode::newarray => {
                    let stack = frame.get_operand_stack_mut();
                    
                    let type_code = frame.read_byte();
                    let count = stack.pop_int();
                    
                    let array_ref = ArrayInstance::new_from_type(type_code, count);
                    stack.push_reference(Reference::Array(array_ref));
                },
                OpCode::anewarray => {
                    let stack = frame.get_operand_stack_mut();
                    
                    let index = frame.read_byte2();
                    let count = stack.pop_int();
                    
                    let method = frame.method();
                    let class_ref = &method.class;
            
                    let constant_pool = &class_ref.unwrap_class_instance().constant_pool;
                    let array_class_name = constant_pool.get_class_name(index);
                    
                    let array_class = ClassLoader::Bootstrap.load(array_class_name).unwrap();
                    let array_ref = ArrayInstance::new_reference(count, array_class);
                    stack.push_reference(Reference::Array(array_ref));
                };
                
                // ========= Control =========
                // TODO: jsr, ret, tableswitch, lookupswitch,
                CATEGORY: control
                OpCode::goto => {
                    let address = frame.read_byte2_signed() as isize;
                    let _ = frame.thread().get().pc.fetch_add(address + COMPARISON_SEEK_BACK, MemOrdering::Relaxed);
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
                } => control_return;
                
                // ========= Extended =========
                // TODO: wide, multianewarray, jsr_w
                CATEGORY: extended
                OpCode::ifnull => {
                    let reference = frame.get_operand_stack_mut().pop_reference();
                    
                    if reference.is_null() {
                        let branch = frame.read_byte2_signed() as isize;
                        let _ = frame.thread().get().pc.fetch_add(branch + COMPARISON_SEEK_BACK, MemOrdering::Relaxed);
                    } else {
                        let _ = frame.thread().get().pc.fetch_add(2, MemOrdering::Relaxed);
                    }
                },
                OpCode::ifnonnull => {
                    let reference = frame.get_operand_stack_mut().pop_reference();
                    
                    if reference.is_null() {
                        let _ = frame.thread().get().pc.fetch_add(2, MemOrdering::Relaxed);
                    } else {
                        let branch = frame.read_byte2_signed() as isize;
                        let _ = frame.thread().get().pc.fetch_add(branch + COMPARISON_SEEK_BACK, MemOrdering::Relaxed);
                    }
                },
                OpCode::goto_w => {
                    let address = frame.read_byte4_signed() as isize;
                    
                    assert!(address <= s2::MAX as isize, "goto_w offset too large!");
    
                    // See doc comment on `COMPARISON_SEEK_BACK` above for explanation of this subtraction
                    let _ = frame.thread().get().pc.fetch_add(address - 4, MemOrdering::Relaxed);
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

                frame.get_operand_stack_mut().push_reference(Reference::Mirror(interned_string));
            },

            // Otherwise, if the run-time constant pool entry is a symbolic reference to a class or interface,
            // then the named class or interface is resolved (§5.4.3.1) and value, a reference to the Class object
            // representing that class or interface, is pushed onto the operand stack.
            ConstantPoolValueInfo::Class { name_index } => {
                let class = class_ref.get();

                let class_name = constant_pool.get_constant_utf8(*name_index);
                let classref = class.loader.load(class_name).unwrap();

                let new_mirror_instance = Class::create_mirrored(classref);
                frame.get_operand_stack_mut().push_reference(Reference::Mirror(new_mirror_instance));
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
