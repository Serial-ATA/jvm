use class_parser::JavaReadExt;
use classfile::{u1, ConstantPool};
use instructions::{DefaultStack, OpCode, Operand, StackLike};
use std::cmp::Ordering;

macro_rules! push_const {
    (STACK: $stack:expr, OPCODE: $opcode:ident, $($instruction:ident: [$($value:tt),+]),+) => {
        paste::paste! {
            match $opcode {
                $(
                    $(
                        OpCode:: [<$instruction _ $value>] => {
                            $stack.push_op(instructions::Operand:: [<Const $value>]);
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

pub struct Interpreter<'a> {
	stack: DefaultStack,
	constant_pool: &'a ConstantPool,
	code: &'a [u1],
    widen: bool,
}

impl<'a> Interpreter<'a> {
	pub fn new(stack_size: usize, pool: &'a ConstantPool, code: &'a [u1]) -> Self {
		Self {
			stack: DefaultStack::new(stack_size),
			constant_pool: pool,
			code,
            widen: false,
		}
	}

    #[rustfmt::skip]
	pub fn run(&mut self) {
        let code_reader = &mut self.code;

        loop {
            // The opcodes are broken into sections as defined here:
            // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-7.html
            
            let opcode = OpCode::from(code_reader.read_u1());

            // ========= Constants =========

            if opcode == OpCode::nop { continue }

            push_const! {
                STACK: self.stack,
                OPCODE: opcode,
                iconst: [m1, 0, 1, 2 , 3, 4, 5],
                lconst: [0, 1],
                fconst: [0, 1, 2],
                dconst: [0, 1]
            }

            match opcode {
                OpCode::bipush => {
                    self.stack.push_op(Operand::Byte(code_reader.read_u1() as i8));
                    continue;
                },
                OpCode::sipush => {
                    self.stack.push_op(Operand::Short(code_reader.read_u2() as i16));
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
                STACK: self.stack,
                OPCODE: opcode,
                pop, pop2,
                dup, dup_x1, dup_x2,
                dup2, dup2_x1, dup2_x2,
                swap
            }

            // ========= Math =========
            // TODO: shl, ushr, and, or, xor, inc

            arithmetic! {
                STACK: self.stack,
                OPCODE: opcode,
                i => [add, sub, mul, div, rem],
                l => [add, sub, mul, div, rem],
                f => [add, sub, mul, div, rem],
                d => [add, sub, mul, div, rem]
            }

            if let OpCode::ineg | OpCode::lneg | OpCode::fneg | OpCode::dneg = opcode {
                let mut val = self.stack.pop();
                val.neg();
                self.stack.push_op(val);

                continue;
            }

            // ========= Conversions =========

            conversion_instructions! {
                STACK: self.stack,
                OPCODE: opcode,
                i2l, i2f, i2d,
                l2i, l2f, l2d,
                f2i, f2l, f2d,
                d2i, d2l, d2f,
                i2b, i2c, i2s
            }

            // ========= Comparisons =========
            // TODO: lcmp, dcmpl, dcmpg, ifeq, ifne, iflt, ifgt,
            //       ifle, if_icmpeq, if_icmpne, if_icmplt, if_icmpge,
            //       if_icmpgt, if_icmple, if_acmpeq, if_acmpne
            
            match opcode {
                OpCode::fcmpl | OpCode::fcmpg => {
                    // Both value1 and value2 must be of type float.
                    // The values are popped from the operand stack and a floating-point comparison is performed:
                    let lhs = self.stack.pop();
                    let rhs = self.stack.pop();

                    match lhs.partial_cmp(&rhs) {
                        // If value1 is greater than value2, the int value 1 is pushed onto the operand stack.
                        Some(Ordering::Greater) => self.stack.push_int(1),
                        // Otherwise, if value1 is equal to value2, the int value 0 is pushed onto the operand stack.
                        Some(Ordering::Equal) => self.stack.push_int(0),
                        // Otherwise, if value1 is less than value2, the int value -1 is pushed onto the operand stack.
                        Some(Ordering::Less) => self.stack.push_int(-1),
                        // Otherwise, at least one of value1 or value2 is NaN.
                        // The fcmpg instruction pushes the int value 1 onto the operand stack and the fcmpl instruction pushes the int value -1 onto the operand stack.
                        _ => {
                            if opcode == OpCode::fcmpg {
                                self.stack.push_int(1);
                            } else {
                                self.stack.push_int(-1);
                            }
                        },
                    }
                },
                _ => {}
            }

            // ========= References =========
            // TODO: getstatic, putstatic, getfield, putfield,
            //       invokevirtual, invokespecial, invokestatic,
            //       invokeinterface, invokedynamic, new, newarray,
            //       anewarray, arraylength, athrow, checkcast, instanceof,
            //       monitorenter, monitorexit

            // ========= Control =========
            // TODO: goto, jsr, ret, tableswitch, lookupswitch,
            //       ireturn, lreturn, freturn, dreturn, areturn

            match opcode {
                OpCode::r#return => return,
                _ => {}
            }

            // ========= Extended =========
            // TODO: multianewarray, ifnull, ifnonnull,
            //       goto_w, jsr_w

            if opcode == OpCode::wide {
                self.widen = true;
            }

            // ========= Reserved =========
            // TODO: breakpoint, impdep1, impdep2

            unimplemented!("{:?}", opcode)
        }
    }
}
