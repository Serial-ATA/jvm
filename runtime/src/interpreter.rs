use crate::frame::Frame;
use crate::heap::class::ClassInitializationState;
use crate::stack::operand_stack::Operand;

use std::cmp::Ordering;
use std::sync::atomic::Ordering as MemOrdering;

use common::traits::PtrType;
use common::types::u4;
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
            let _ = $frame.pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
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
            let _ = $frame.pc.fetch_add(2, std::sync::atomic::Ordering::Relaxed);
        }
    }}
}

pub struct Interpreter<'a> {
	frame: Frame<'a>,
	widen: bool,
}

#[rustfmt::skip]
impl<'a> Interpreter<'a> {
	pub fn new(frame: Frame<'a>) -> Self {
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
                STACK: self.frame.stack,
                OPCODE: opcode,
                iconst: [m1, 0, 1, 2 , 3, 4, 5],
                lconst: [0, 1],
                fconst: [0, 1, 2],
                dconst: [0, 1]
            }

            match opcode {
                OpCode::bipush => {
                    let byte = self.frame.read_byte() as i8;
                    self.frame.stack.push_op(Operand::Int(i32::from(byte)));
                    continue;
                },
                OpCode::sipush => {
                    let short = self.frame.read_byte2() as i16;
                    self.frame.stack.push_op(Operand::Int(i32::from(short)));
                    continue;
                },
                _ => {}
            }

            // ========= Loads =========
            // TODO

            // ========= Stores =========
            // TODO

            // ========= Stack =========

            stack_operations! {
                STACK: self.frame.stack,
                OPCODE: opcode,
                pop, pop2,
                dup, dup_x1, dup_x2,
                dup2, dup2_x1, dup2_x2,
                swap
            }

            // ========= Math =========
            // TODO: shl, ushr, and, or, xor, inc

            arithmetic! {
                STACK: self.frame.stack,
                OPCODE: opcode,
                i => [add, sub, mul, div, rem],
                l => [add, sub, mul, div, rem],
                f => [add, sub, mul, div, rem],
                d => [add, sub, mul, div, rem]
            }

            if let OpCode::ineg | OpCode::lneg | OpCode::fneg | OpCode::dneg = opcode {
                let mut val = self.frame.stack.pop();
                val.neg();
                self.frame.stack.push_op(val);

                continue;
            }

            // ========= Conversions =========

            conversion_instructions! {
                STACK: self.frame.stack,
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
                STACK: self.frame.stack,
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
                OpCode::fcmpl | OpCode::fcmpg => {
                    // Both value1 and value2 must be of type float.
                    // The values are popped from the operand stack and a floating-point comparison is performed:
                    let lhs = self.frame.stack.pop();
                    let rhs = self.frame.stack.pop();

                    match lhs.partial_cmp(&rhs) {
                        // If value1 is greater than value2, the int value 1 is pushed onto the operand stack.
                        Some(Ordering::Greater) => self.frame.stack.push_int(1),
                        // Otherwise, if value1 is equal to value2, the int value 0 is pushed onto the operand stack.
                        Some(Ordering::Equal) => self.frame.stack.push_int(0),
                        // Otherwise, if value1 is less than value2, the int value -1 is pushed onto the operand stack.
                        Some(Ordering::Less) => self.frame.stack.push_int(-1),
                        // Otherwise, at least one of value1 or value2 is NaN.
                        // The fcmpg instruction pushes the int value 1 onto the operand stack and the fcmpl instruction pushes the int value -1 onto the operand stack.
                        _ => {
                            if opcode == OpCode::fcmpg {
                                self.frame.stack.push_int(1);
                            } else {
                                self.frame.stack.push_int(-1);
                            }
                        },
                    }
                },
                _ => {}
            }

            // ========= References =========
            // TODO: putstatic, getfield, putfield,
            //       invokevirtual, invokespecial, invokestatic,
            //       invokeinterface, invokedynamic, new, newarray,
            //       anewarray, arraylength, athrow, checkcast, instanceof,
            //       monitorenter, monitorexit
            if opcode == OpCode::getstatic {
                let idx = self.frame.read_byte2();
                let (class_name_index, name_and_type_index) = self.frame.constant_pool.get_field_ref(idx);

                let class = self.frame.method.class.get_mut();
                if class.initialization_state() == ClassInitializationState::Uninit {
                    class.initialize();
                }

                let field = self.frame.method.class.get().resolve_field(name_and_type_index).unwrap();
                self.frame.stack.push_op(field.get_static_value());
                continue;
            }

            // ========= Control =========
            // TODO: jsr, ret, tableswitch, lookupswitch,
            //       ireturn, lreturn, freturn, dreturn, areturn

            match opcode {
                OpCode::goto => {
                    let address = self.frame.read_byte2();
                    let _ = self.frame.pc.fetch_add(address as usize, MemOrdering::Relaxed);

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

                    let _ = self.frame.pc.fetch_add(address as usize, MemOrdering::Relaxed);
                    continue;
                },
                _ => {}
            }

            // ========= Reserved =========
            // TODO: breakpoint, impdep1, impdep2

            unimplemented!("{:?}", opcode)
        }
    }
}
