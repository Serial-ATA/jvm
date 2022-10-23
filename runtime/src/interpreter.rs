use std::cmp::Ordering;
use classfile::{ConstantPool, u1};
use instructions::{DefaultStack, OpCode, StackLike};
use class_parser::JavaReadExt;

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
}

impl<'a> Interpreter<'a> {
    pub fn new(stack_size: usize, pool: &'a ConstantPool, code: &'a [u1]) -> Self {
        Self {
            stack: DefaultStack::new(stack_size),
            constant_pool: pool,
            code,
        }
    }

    pub fn run(&mut self) {
        let code_reader = &mut self.code;

        loop {
            let opcode = OpCode::from(code_reader.read_u1());
            match opcode {
                OpCode::nop => {}
                OpCode::aconst_null => {}
                OpCode::bipush => {}
                OpCode::sipush => {}
                OpCode::ldc => {}
                OpCode::ldc_w => {}
                OpCode::ldc2_w => {}
                OpCode::iload => {}
                OpCode::lload => {}
                OpCode::fload => {}
                OpCode::dload => {}
                OpCode::aload => {}
                OpCode::iload_0 => {}
                OpCode::iload_1 => {}
                OpCode::iload_2 => {}
                OpCode::iload_3 => {}
                OpCode::lload_0 => {}
                OpCode::lload_1 => {}
                OpCode::lload_2 => {}
                OpCode::lload_3 => {}
                OpCode::fload_0 => {}
                OpCode::fload_1 => {}
                OpCode::fload_2 => {}
                OpCode::fload_3 => {}
                OpCode::dload_0 => {}
                OpCode::dload_1 => {}
                OpCode::dload_2 => {}
                OpCode::dload_3 => {}
                OpCode::aload_0 => {}
                OpCode::aload_1 => {}
                OpCode::aload_2 => {}
                OpCode::aload_3 => {}
                OpCode::iaload => {}
                OpCode::laload => {}
                OpCode::faload => {}
                OpCode::daload => {}
                OpCode::aaload => {}
                OpCode::baload => {}
                OpCode::caload => {}
                OpCode::saload => {}
                OpCode::istore => {}
                OpCode::lstore => {}
                OpCode::fstore => {}
                OpCode::dstore => {}
                OpCode::astore => {}
                OpCode::istore_0 => {}
                OpCode::istore_1 => {}
                OpCode::istore_2 => {}
                OpCode::istore_3 => {}
                OpCode::lstore_0 => {}
                OpCode::lstore_1 => {}
                OpCode::lstore_2 => {}
                OpCode::lstore_3 => {}
                OpCode::fstore_0 => {}
                OpCode::fstore_1 => {}
                OpCode::fstore_2 => {}
                OpCode::fstore_3 => {}
                OpCode::dstore_0 => {}
                OpCode::dstore_1 => {}
                OpCode::dstore_2 => {}
                OpCode::dstore_3 => {}
                OpCode::astore_0 => {}
                OpCode::astore_1 => {}
                OpCode::astore_2 => {}
                OpCode::astore_3 => {}
                OpCode::iastore => {}
                OpCode::lastore => {}
                OpCode::fastore => {}
                OpCode::dastore => {}
                OpCode::aastore => {}
                OpCode::bastore => {}
                OpCode::castore => {}
                OpCode::sastore => {}
                OpCode::ineg | OpCode::lneg | OpCode::fneg | OpCode::dneg => {
                    let mut val = self.stack.pop();
                    val.neg();
                    self.stack.push_op(val);
                    continue;
                }
                OpCode::ishl => {}
                OpCode::lshl => {}
                OpCode::ishr => {}
                OpCode::lshr => {}
                OpCode::iushr => {}
                OpCode::lushr => {}
                OpCode::iand => {}
                OpCode::land => {}
                OpCode::ior => {}
                OpCode::lor => {}
                OpCode::ixor => {}
                OpCode::lxor => {}
                OpCode::iinc => {}
                OpCode::lcmp => {}
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
                }
                OpCode::dcmpl => {}
                OpCode::dcmpg => {}
                OpCode::ifeq => {}
                OpCode::ifne => {}
                OpCode::iflt => {}
                OpCode::ifge => {}
                OpCode::ifgt => {}
                OpCode::ifle => {}
                OpCode::if_icmpeq => {}
                OpCode::if_icmpne => {}
                OpCode::if_icmplt => {}
                OpCode::if_icmpge => {}
                OpCode::if_icmpgt => {}
                OpCode::if_icmple => {}
                OpCode::if_acmpeq => {}
                OpCode::if_acmpne => {}
                OpCode::goto => {}
                OpCode::jsr => {}
                OpCode::ret => {}
                OpCode::tableswitch => {}
                OpCode::lookupswitch => {}
                OpCode::ireturn => {}
                OpCode::lreturn => {}
                OpCode::freturn => {}
                OpCode::dreturn => {}
                OpCode::areturn => {}
                OpCode::r#return => {}
                OpCode::getstatic => {}
                OpCode::putstatic => {}
                OpCode::getfield => {}
                OpCode::putfield => {}
                OpCode::invokevirtual => {}
                OpCode::invokespecial => {}
                OpCode::invokestatic => {}
                OpCode::invokeinterface => {}
                OpCode::invokedynamic => {}
                OpCode::new => {}
                OpCode::newarray => {}
                OpCode::anewarray => {}
                OpCode::arraylength => {}
                OpCode::athrow => {}
                OpCode::checkcast => {}
                OpCode::instanceof => {}
                OpCode::monitorenter => {}
                OpCode::monitorexit => {}
                OpCode::wide => {}
                OpCode::multianewarray => {}
                OpCode::ifnull => {}
                OpCode::ifnonnull => {}
                OpCode::goto_w => {}
                OpCode::jsr_w => {}
                OpCode::breakpoint => {}
                OpCode::unknown => {}
                _ => {
                    push_const! {
                        STACK: self.stack,
                        OPCODE: opcode,
                        iconst: [m1, 0, 1, 2 , 3, 4, 5],
                        lconst: [0, 1],
                        fconst: [0, 1, 2],
                        dconst: [0, 1]
                    }

                    stack_operations! {
                        STACK: self.stack,
                        OPCODE: opcode,
                        pop, pop2,
                        dup, dup_x1, dup_x2,
                        dup2, dup2_x1, dup2_x2,
                        swap
                    }

                    arithmetic! {
                        STACK: self.stack,
                        OPCODE: opcode,
                        i => [add, sub, mul, div, rem],
                        l => [add, sub, mul, div, rem],
                        f => [add, sub, mul, div, rem],
                        d => [add, sub, mul, div, rem]
                    }

                    conversion_instructions! {
                        STACK: self.stack,
                        OPCODE: opcode,
                        i2l, i2f, i2d,
                        l2i, l2f, l2d,
                        f2i, f2l, f2d,
                        d2i, d2l, d2f,
                        i2b, i2c, i2s
                    }

                    unimplemented!("{:?}", opcode);
                }
            }
        }
    }
}