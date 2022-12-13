use crate::class::Class;
use crate::frame::FrameRef;
use crate::heap::class::ClassInitializationState;
use crate::stack::operand_stack::Operand;
use crate::reference::Reference;

use std::cmp::Ordering;
use std::sync::atomic::Ordering as MemOrdering;
use std::sync::Arc;

use classfile::ConstantPoolValueInfo;
use classfile::traits::PtrType;
use classfile::types::{u2, u4};
use instructions::{OpCode, OperandLike, StackLike};

macro_rules! push_const {
    (STACK: $stack:expr, OPCODE: $opcode:ident, $($instruction:ident: [$($value:tt),+]),+) => {
        paste::paste! {
            match $opcode {
                $(
                    $(
                        OpCode:: [<$instruction _ $value>] => {
                            $stack.push_op(Operand:: [<Const $value>]);
                            continue;
                        }
                    ),+
                )+
                _ => {}
            }
        }
    }
}

macro_rules! stack_operations {
    (STACK: $stack:expr, OPCODE: $opcode:ident, $($instruction:ident),+) => {
        match $opcode {
            $(
                OpCode::$instruction => {
                    $stack.$instruction();
                    continue;
                }
            ),+
            _ => {}
        }
    }
}

macro_rules! arithmetic {
    (
        STACK: $stack:expr,
        OPCODE: $opcode:ident,
        $($type:ident => [$($instruction:ident),+]),+
    ) => {
        paste::paste! {
            match $opcode {
                $(
                    $(
                        OpCode:: [<$type $instruction>] => {
                            let mut val = $stack.pop();
                            let rhs = $stack.pop();
                            val.$instruction(rhs);
                            $stack.push_op(val);
                            continue;
                        }
                    ),+
                )+
                _ => {}
            }
        }
    }
}

macro_rules! conversion_instructions {
    (STACK: $stack:expr, OPCODE: $opcode:ident, $($instruction:ident),+) => {
        match $opcode {
            $(
                OpCode::$instruction => {
                    let mut val = $stack.pop();
                    val.$instruction();
                    $stack.push_op(val);
                    continue;
                }
            ),+
            _ => {}
        }
    }
}

macro_rules! comparisons {
    (
        STACK: $stack:expr,
        OPCODE: $opcode:ident,
        FRAME: $frame:expr,
        [$($instruction:ident: $operator:tt $((RHS = $rhs:literal))?),+]
    ) => {
        match $opcode {
            $(
                OpCode::$instruction => {
                    comparisons! {
                        @CREATE_BODY
                        $stack, $frame,
                        $instruction: $operator $($rhs)?
                    }
                }
            ),+
            _ => {}
        }
    };
    (
        @CREATE_BODY
        $stack:expr,
        $frame:expr,
        $instruction:ident: $operator:tt $rhs:literal
    ) => {{
        let lhs = $stack.pop_int();

        if lhs $operator $rhs {
            todo!();
        } else {
            let _ = $frame.thread().get().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }};
    (
        @CREATE_BODY
        $stack:expr,
        $frame:expr,
        $instruction:ident: $operator:tt
    ) => {{
        let rhs = $stack.pop_int();
        let lhs = $stack.pop_int();

        if lhs $operator rhs {
            todo!();
        } else {
            let _ = $frame.thread().get().pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }}
}

pub struct Interpreter {
	frame: FrameRef,
	widen: bool,
}

#[rustfmt::skip]
impl Interpreter {
	pub fn new(frame: FrameRef) -> Self {
		Self {
			frame,
			widen: false,
		}
	}

	pub fn run(&mut self) {
        loop {
            // The opcodes are broken into sections as defined here:
            // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-7.html

            let opcode = OpCode::from(self.frame.read_byte());

            // ========= Constants =========

            if opcode == OpCode::nop { continue }

            push_const! {
                STACK: self.frame.get_operand_stack_mut(),
                OPCODE: opcode,
                iconst: [m1, 0, 1, 2 , 3, 4, 5],
                lconst: [0, 1],
                fconst: [0, 1, 2],
                dconst: [0, 1]
            }

            match opcode {
                OpCode::bipush => {
                    let byte = self.frame.read_byte() as i8;
                    self.frame.get_operand_stack_mut().push_op(Operand::Int(i32::from(byte)));
                    continue;
                },
                OpCode::sipush => {
                    let short = self.frame.read_byte2() as i16;
                    self.frame.get_operand_stack_mut().push_op(Operand::Int(i32::from(short)));
                    continue;
                },
                _ => {}
            }

            // ========= Loads =========
            // TODO: iload{_[0-3]}, lload{_[0-3]}, fload{_[0-3]}, dload{_[0-3]},
            //       aload{_[0-3]}

            match opcode {
                OpCode::ldc => self.ldc(false),
                OpCode::ldc_w => self.ldc(true),
                _ => {}
            }

            // ========= Stores =========
            // TODO

            // ========= Stack =========

            stack_operations! {
                STACK: self.frame.get_operand_stack_mut(),
                OPCODE: opcode,
                pop, pop2,
                dup, dup_x1, dup_x2,
                dup2, dup2_x1, dup2_x2,
                swap
            }

            // ========= Math =========
            // TODO: shl, ushr, and, or, xor, inc

            arithmetic! {
                STACK: self.frame.get_operand_stack_mut(),
                OPCODE: opcode,
                i => [add, sub, mul, div, rem],
                l => [add, sub, mul, div, rem],
                f => [add, sub, mul, div, rem],
                d => [add, sub, mul, div, rem]
            }

            if let OpCode::ineg | OpCode::lneg | OpCode::fneg | OpCode::dneg = opcode {
                let mut val = self.frame.get_operand_stack_mut().pop();
                val.neg();
                self.frame.get_operand_stack_mut().push_op(val);

                continue;
            }

            // ========= Conversions =========

            conversion_instructions! {
                STACK: self.frame.get_operand_stack_mut(),
                OPCODE: opcode,
                i2l, i2f, i2d,
                l2i, l2f, l2d,
                f2i, f2l, f2d,
                d2i, d2l, d2f,
                i2b, i2c, i2s
            }

            // ========= Comparisons =========
            // TODO: lcmp, dcmpl, dcmpg, if_acmpeq, if_acmpne

            comparisons! {
                STACK: self.frame.get_operand_stack_mut(),
                OPCODE: opcode,
                FRAME: self.frame,
                [
                    ifeq: == (RHS = 0),
                    ifne: != (RHS = 0),
                    iflt: <  (RHS = 0),
                    ifge: >= (RHS = 0),
                    ifgt: >  (RHS = 0),
                    ifle: <= (RHS = 0),
                    if_icmpeq: ==,
                    if_icmpne: !=,
                    if_icmplt: <,
                    if_icmpge: >=,
                    if_icmpgt: >,
                    if_icmple: <=
                ]
            }

            match opcode {
                OpCode::fcmpl => self.fcmp(Ordering::Less),
                OpCode::fcmpg => self.fcmp(Ordering::Greater),
                _ => {}
            }

            // ========= References =========
            // TODO: putstatic, getfield, putfield,
            //       invokevirtual, invokespecial, invokestatic,
            //       invokeinterface, invokedynamic, new, newarray,
            //       anewarray, arraylength, athrow, checkcast, instanceof,
            //       monitorenter, monitorexit
            if opcode == OpCode::getstatic {
                let field_ref_idx = self.frame.read_byte2();

                let method = self.frame.method();
                let class = Arc::clone(&method.class);
                if class.get().initialization_state() == ClassInitializationState::Uninit {
                    Class::initialize(&class, self.frame.thread());
                }

                let field = Class::resolve_field(Arc::clone(&class.unwrap_class_instance().constant_pool), field_ref_idx).unwrap();
                self.frame.get_operand_stack_mut().push_op(field.get_static_value());
                continue;
            }

            // ========= Control =========
            // TODO: jsr, ret, tableswitch, lookupswitch,
            //       ireturn, lreturn, freturn, dreturn, areturn

            match opcode {
                OpCode::goto => {
                    let address = self.frame.read_byte2();
                    let _ = self.frame.thread().get().pc.fetch_add(address as usize, MemOrdering::Relaxed);

                    continue;
                },
                OpCode::r#return => return,
                _ => {}
            }

            // ========= Extended =========
            // TODO: multianewarray, ifnull, ifnonnull,
            //       jsr_w

            match opcode {
                OpCode::wide => {
                    self.widen = true;
                    continue;
                },
                OpCode::goto_w => {
                    let address = self.frame.read_byte4();
                    assert!(address <= u4::from(u16::MAX), "goto_w offset too large!");

                    let _ = self.frame.thread().get().pc.fetch_add(address as usize, MemOrdering::Relaxed);
                    continue;
                },
                _ => {}
            }

            // ========= Reserved =========
            // TODO: breakpoint, impdep1, impdep2

            unimplemented!("{:?}", opcode)
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.ldc
    fn ldc(&mut self, wide: bool) {
        let idx = if wide {
            self.frame.read_byte2()
        } else {
            u2::from(self.frame.read_byte())
        };

        let method = self.frame.method();
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
            ConstantPoolValueInfo::Integer { bytes } => self.frame.get_operand_stack_mut().push_int((*bytes) as i32),
            ConstantPoolValueInfo::Float { bytes } => self.frame.get_operand_stack_mut().push_float(f32::from_be_bytes(bytes.to_be_bytes())),

            // Otherwise, if the run-time constant pool entry is a string constant, that is,
            // a reference to an instance of class String, then value, a reference to that instance, is pushed onto the operand stack.
            ConstantPoolValueInfo::String { .. } => todo!("string in ldc"),

            // Otherwise, if the run-time constant pool entry is a symbolic reference to a class or interface,
            // then the named class or interface is resolved (§5.4.3.1) and value, a reference to the Class object
            // representing that class or interface, is pushed onto the operand stack.
            ConstantPoolValueInfo::Class { name_index } => {
                let class = class_ref.get();

                let class_name = constant_pool.get_class_name(*name_index);
                let classref = class.loader.load(class_name).unwrap();
                self.frame.get_operand_stack_mut().push_reference(Reference::Class(classref));
            },

            // Otherwise, the run-time constant pool entry is a symbolic reference to a method type, a method handle,
            // or a dynamically-computed constant. The symbolic reference is resolved (§5.4.3.5, §5.4.3.6) and value,
            // the result of resolution, is pushed onto the operand stack.
            ConstantPoolValueInfo::MethodHandle { .. } => {}
            ConstantPoolValueInfo::MethodType { .. } => {}
            _ => unreachable!()
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.fcmp_op
    fn fcmp(&mut self, ordering: Ordering) {
        let operand_stack = self.frame.get_operand_stack_mut();

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
}
