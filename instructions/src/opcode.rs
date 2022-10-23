// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[rustfmt::skip]
pub enum OpCode {
    nop                = 0x00, // Do nothing
    aconst_null        = 0x01, // Push null

    // --- Push the int constant <i> (-1, 0, 1, 2, 3, 4 or 5) onto the operand stack.  ---
    // Each of this family of instructions is equivalent to bipush <i> for the respective value of <i>
    iconst_m1          = 0x02,
    iconst_0           = 0x03,
    iconst_1           = 0x04,
    iconst_2           = 0x05,
    iconst_3           = 0x06,
    iconst_4           = 0x07,
    iconst_5           = 0x08,
    // -----------------------------------------------------------------------------------

    // ------------ Push the long constant <l> (0 or 1) onto the operand stack -----------
    lconst_0           = 0x09,
    lconst_1           = 0x0a,
    // -----------------------------------------------------------------------------------

    // ------ Push the float constant <f> (0.0, 1.0, or 2.0) onto the operand stack ------
    fconst_0           = 0x0b,
    fconst_1           = 0x0c,
    fconst_2           = 0x0d,
    // -----------------------------------------------------------------------------------

    // -------- Push the double constant <d> (0.0 or 1.0) onto the operand stack ---------
    dconst_0           = 0x0e,
    dconst_1           = 0x0f,
    // -----------------------------------------------------------------------------------

    bipush             = 0x10, // Push byte
    sipush             = 0x11, // Push short
    ldc                = 0x12, // Push item from run-time constant pool
    ldc_w              = 0x13, // Push item from run-time constant pool (wide index)
    ldc2_w             = 0x14, // Push long or double from run-time constant pool (wide index)
    iload              = 0x15, // Load int from local variable
    lload              = 0x16, // Load long from local variable
    fload              = 0x17, // Load float from local variable
    dload              = 0x18, // Load double from local variable
    aload              = 0x19, // Load reference from local variable

    // --- same as iload with an index of <n> ---
    iload_0            = 0x1a,
    iload_1            = 0x1b,
    iload_2            = 0x1c,
    iload_3            = 0x1d,
    // ------------------------------------------

    // --- same as lload with an index of <n> ---
    lload_0            = 0x1e,
    lload_1            = 0x1f,
    lload_2            = 0x20,
    lload_3            = 0x21,
    // ------------------------------------------

    // --- same as fload with an index of <n> ---
    fload_0            = 0x22,
    fload_1            = 0x23,
    fload_2            = 0x24,
    fload_3            = 0x25,
    // ------------------------------------------

    // --- same as dload with an index of <n> ---
    dload_0            = 0x26,
    dload_1            = 0x27,
    dload_2            = 0x28,
    dload_3            = 0x29,
    // ------------------------------------------

    // --- same as aload with an index of <n> ---
    aload_0            = 0x2a,
    aload_1            = 0x2b,
    aload_2            = 0x2c,
    aload_3            = 0x2d,
    // ------------------------------------------

    iaload             = 0x2e,
    laload             = 0x2f,
    faload             = 0x30,
    daload             = 0x31,
    aaload             = 0x32,
    baload             = 0x33,
    caload             = 0x34,
    saload             = 0x35,
    istore             = 0x36,
    lstore             = 0x37,
    fstore             = 0x38,
    dstore             = 0x39,
    astore             = 0x3a,
    istore_0           = 0x3b,
    istore_1           = 0x3c,
    istore_2           = 0x3d,
    istore_3           = 0x3e,
    lstore_0           = 0x3f,
    lstore_1           = 0x40,
    lstore_2           = 0x41,
    lstore_3           = 0x42,
    fstore_0           = 0x43,
    fstore_1           = 0x44,
    fstore_2           = 0x45,
    fstore_3           = 0x46,
    dstore_0           = 0x47,
    dstore_1           = 0x48,
    dstore_2           = 0x49,
    dstore_3           = 0x4a,
    astore_0           = 0x4b,
    astore_1           = 0x4c,
    astore_2           = 0x4d,
    astore_3           = 0x4e,
    iastore            = 0x4f,
    lastore            = 0x50,
    fastore            = 0x51,
    dastore            = 0x52,
    aastore            = 0x53,
    bastore            = 0x54,
    castore            = 0x55,
    sastore            = 0x56,
    pop                = 0x57,
    pop2               = 0x58,
    dup                = 0x59,
    dup_x1             = 0x5a,
    dup_x2             = 0x5b,
    dup2               = 0x5c,
    dup2_x1            = 0x5d,
    dup2_x2            = 0x5e,
    swap               = 0x5f,
    iadd               = 0x60,
    ladd               = 0x61,
    fadd               = 0x62,
    dadd               = 0x63,
    isub               = 0x64,
    lsub               = 0x65,
    fsub               = 0x66,
    dsub               = 0x67,
    imul               = 0x68,
    lmul               = 0x69,
    fmul               = 0x6a,
    dmul               = 0x6b,
    idiv               = 0x6c,
    ldiv               = 0x6d,
    fdiv               = 0x6e,
    ddiv               = 0x6f,
    irem               = 0x70,
    lrem               = 0x71,
    frem               = 0x72,
    drem               = 0x73,
    ineg               = 0x74,
    lneg               = 0x75,
    fneg               = 0x76,
    dneg               = 0x77,
    ishl               = 0x78,
    lshl               = 0x79,
    ishr               = 0x7a,
    lshr               = 0x7b,
    iushr              = 0x7c,
    lushr              = 0x7d,
    iand               = 0x7e,
    land               = 0x7f,
    ior                = 0x80,
    lor                = 0x81,
    ixor               = 0x82,
    lxor               = 0x83,
    iinc               = 0x84, // Increment local variable by constant
    i2l                = 0x85,
    i2f                = 0x86,
    i2d                = 0x87,
    l2i                = 0x88,
    l2f                = 0x89,
    l2d                = 0x8a,
    f2i                = 0x8b,
    f2l                = 0x8c,
    f2d                = 0x8d,
    d2i                = 0x8e,
    d2l                = 0x8f,
    d2f                = 0x90,
    i2b                = 0x91,
    i2c                = 0x92,
    i2s                = 0x93,
    lcmp               = 0x94,
    fcmpl              = 0x95,
    fcmpg              = 0x96,
    dcmpl              = 0x97,
    dcmpg              = 0x98,

    // --- Branch if int comparison with zero succeeds ---
    ifeq               = 0x99,
    ifne               = 0x9a,
    iflt               = 0x9b,
    ifge               = 0x9c,
    ifgt               = 0x9d,
    ifle               = 0x9e,
    // ----------------------------------------------------

    // -------- Branch if int comparison succeeds ---------
    if_icmpeq           = 0x9f,
    if_icmpne           = 0xa0,
    if_icmplt           = 0xa1,
    if_icmpge           = 0xa2,
    if_icmpgt           = 0xa3,
    if_icmple           = 0xa4,
    // ----------------------------------------------------

    // ---- Branch if reference comparison succeeds  -----
    if_acmpeq           = 0xa5,
    if_acmpne           = 0xa6,
    // ---------------------------------------------------

    goto               = 0xa7, // Branch always
    jsr                = 0xa8, // Jump subroutine
    ret                = 0xa9, // Return from subroutine
    tableswitch        = 0xaa, // Access jump table by index and jump
    lookupswitch       = 0xab, // Access jump table by key match and jump
    ireturn            = 0xac, // Return int from method
    lreturn            = 0xad, // Return long from method
    freturn            = 0xae, // Return float from method
    dreturn            = 0xaf, // Return double from method
    areturn            = 0xb0, // Return reference from method
    r#return           = 0xb1, // Return void from method
    getstatic          = 0xb2, // Get static field from class
    putstatic          = 0xb3, // Set static field in class
    getfield           = 0xb4, // Fetch field from object
    putfield           = 0xb5, // Set field in object
    invokevirtual      = 0xb6, // Invoke instance method; dispatch based on class
    invokespecial      = 0xb7, // Invoke instance method; direct invocation of instance initialization methods and methods of the current class and its supertypes
    invokestatic       = 0xb8, // Invoke a class (static) method
    invokeinterface    = 0xb9, // Invoke interface method
    invokedynamic      = 0xba, // Invoke a dynamically-computed call site
    new                = 0xbb, // Create new object
    newarray           = 0xbc, // Create new array
    anewarray          = 0xbd, // Create new array of reference
    arraylength        = 0xbe, // Get length of array
    athrow             = 0xbf, // Throw exception or error
    checkcast          = 0xc0, // Check whether object is of given type
    instanceof         = 0xc1, // Determine if object is of given type
    monitorenter       = 0xc2, // Enter monitor for object
    monitorexit        = 0xc3, // Exit monitor for object
    wide               = 0xc4, // Extend local variable index by additional bytes
    multianewarray     = 0xc5, // Create new multidimensional array
    ifnull             = 0xc6, // Branch if reference is null
    ifnonnull          = 0xc7, // Branch if reference not null
    goto_w             = 0xc8, // Branch always (wide index)
    jsr_w              = 0xc9, // Jump subroutine (wide index)
    breakpoint         = 0xca, // Intended to be used by debuggers to implement breakpoints
    unknown                    // Covers any other currently unknown opcodes

    // TODO: impdep1, impdep2; https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.2
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::nop,
            1 => OpCode::aconst_null,
            2 => OpCode::iconst_m1,
            3 => OpCode::iconst_0,
            4 => OpCode::iconst_1,
            5 => OpCode::iconst_2,
            6 => OpCode::iconst_3,
            7 => OpCode::iconst_4,
            8 => OpCode::iconst_5,
            9 => OpCode::lconst_0,
            10 => OpCode::lconst_1,
            11 => OpCode::fconst_0,
            12 => OpCode::fconst_1,
            13 => OpCode::fconst_2,
            14 => OpCode::dconst_0,
            15 => OpCode::dconst_1,
            16 => OpCode::bipush,
            17 => OpCode::sipush,
            18 => OpCode::ldc,
            19 => OpCode::ldc_w,
            20 => OpCode::ldc2_w,
            21 => OpCode::iload,
            22 => OpCode::lload,
            23 => OpCode::fload,
            24 => OpCode::dload,
            25 => OpCode::aload,
            26 => OpCode::iload_0,
            27 => OpCode::iload_1,
            28 => OpCode::iload_2,
            29 => OpCode::iload_3,
            30 => OpCode::lload_0,
            31 => OpCode::lload_1,
            32 => OpCode::lload_2,
            33 => OpCode::lload_3,
            34 => OpCode::fload_0,
            35 => OpCode::fload_1,
            36 => OpCode::fload_2,
            37 => OpCode::fload_3,
            38 => OpCode::dload_0,
            39 => OpCode::dload_1,
            40 => OpCode::dload_2,
            41 => OpCode::dload_3,
            42 => OpCode::aload_0,
            43 => OpCode::aload_1,
            44 => OpCode::aload_2,
            45 => OpCode::aload_3,
            46 => OpCode::iaload,
            47 => OpCode::laload,
            48 => OpCode::faload,
            49 => OpCode::daload,
            50 => OpCode::aaload,
            51 => OpCode::baload,
            52 => OpCode::caload,
            53 => OpCode::saload,
            54 => OpCode::istore,
            55 => OpCode::lstore,
            56 => OpCode::fstore,
            57 => OpCode::dstore,
            58 => OpCode::astore,
            59 => OpCode::istore_0,
            60 => OpCode::istore_1,
            61 => OpCode::istore_2,
            62 => OpCode::istore_3,
            63 => OpCode::lstore_0,
            64 => OpCode::lstore_1,
            65 => OpCode::lstore_2,
            66 => OpCode::lstore_3,
            67 => OpCode::fstore_0,
            68 => OpCode::fstore_1,
            69 => OpCode::fstore_2,
            70 => OpCode::fstore_3,
            71 => OpCode::dstore_0,
            72 => OpCode::dstore_1,
            73 => OpCode::dstore_2,
            74 => OpCode::dstore_3,
            75 => OpCode::astore_0,
            76 => OpCode::astore_1,
            77 => OpCode::astore_2,
            78 => OpCode::astore_3,
            79 => OpCode::iastore,
            80 => OpCode::lastore,
            81 => OpCode::fastore,
            82 => OpCode::dastore,
            83 => OpCode::aastore,
            84 => OpCode::bastore,
            85 => OpCode::castore,
            86 => OpCode::sastore,
            87 => OpCode::pop,
            88 => OpCode::pop2,
            89 => OpCode::dup,
            90 => OpCode::dup_x1,
            91 => OpCode::dup_x2,
            92 => OpCode::dup2,
            93 => OpCode::dup2_x1,
            94 => OpCode::dup2_x2,
            95 => OpCode::swap,
            96 => OpCode::iadd,
            97 => OpCode::ladd,
            98 => OpCode::fadd,
            99 => OpCode::dadd,
            100 => OpCode::isub,
            101 => OpCode::lsub,
            102 => OpCode::fsub,
            103 => OpCode::dsub,
            104 => OpCode::imul,
            105 => OpCode::lmul,
            106 => OpCode::fmul,
            107 => OpCode::dmul,
            108 => OpCode::idiv,
            109 => OpCode::ldiv,
            110 => OpCode::fdiv,
            111 => OpCode::ddiv,
            112 => OpCode::irem,
            113 => OpCode::lrem,
            114 => OpCode::frem,
            115 => OpCode::drem,
            116 => OpCode::ineg,
            117 => OpCode::lneg,
            118 => OpCode::fneg,
            119 => OpCode::dneg,
            120 => OpCode::ishl,
            121 => OpCode::lshl,
            122 => OpCode::ishr,
            123 => OpCode::lshr,
            124 => OpCode::iushr,
            125 => OpCode::lushr,
            126 => OpCode::iand,
            127 => OpCode::land,
            128 => OpCode::ior,
            129 => OpCode::lor,
            130 => OpCode::ixor,
            131 => OpCode::lxor,
            132 => OpCode::iinc,
            133 => OpCode::i2l,
            134 => OpCode::i2f,
            135 => OpCode::i2d,
            136 => OpCode::l2i,
            137 => OpCode::l2f,
            138 => OpCode::l2d,
            139 => OpCode::f2i,
            140 => OpCode::f2l,
            141 => OpCode::f2d,
            142 => OpCode::d2i,
            143 => OpCode::d2l,
            144 => OpCode::d2f,
            145 => OpCode::i2b,
            146 => OpCode::i2c,
            147 => OpCode::i2s,
            148 => OpCode::lcmp,
            149 => OpCode::fcmpl,
            150 => OpCode::fcmpg,
            151 => OpCode::dcmpl,
            152 => OpCode::dcmpg,
            153 => OpCode::ifeq,
            154 => OpCode::ifne,
            155 => OpCode::iflt,
            156 => OpCode::ifge,
            157 => OpCode::ifgt,
            158 => OpCode::ifle,
            159 => OpCode::if_icmpeq,
            160 => OpCode::if_icmpne,
            161 => OpCode::if_icmplt,
            162 => OpCode::if_icmpge,
            163 => OpCode::if_icmpgt,
            164 => OpCode::if_icmple,
            165 => OpCode::if_acmpeq,
            166 => OpCode::if_acmpne,
            167 => OpCode::goto,
            168 => OpCode::jsr,
            169 => OpCode::ret,
            170 => OpCode::tableswitch,
            171 => OpCode::lookupswitch,
            172 => OpCode::ireturn,
            173 => OpCode::lreturn,
            174 => OpCode::freturn,
            175 => OpCode::dreturn,
            176 => OpCode::areturn,
            177 => OpCode::r#return,
            178 => OpCode::getstatic,
            179 => OpCode::putstatic,
            180 => OpCode::getfield,
            181 => OpCode::putfield,
            182 => OpCode::invokevirtual,
            183 => OpCode::invokespecial,
            184 => OpCode::invokestatic,
            185 => OpCode::invokeinterface,
            186 => OpCode::invokedynamic,
            187 => OpCode::new,
            188 => OpCode::newarray,
            189 => OpCode::anewarray,
            190 => OpCode::arraylength,
            191 => OpCode::athrow,
            192 => OpCode::checkcast,
            193 => OpCode::instanceof,
            194 => OpCode::monitorenter,
            195 => OpCode::monitorexit,
            196 => OpCode::wide,
            197 => OpCode::multianewarray,
            198 => OpCode::ifnull,
            199 => OpCode::ifnonnull,
            200 => OpCode::goto_w,
            201 => OpCode::jsr_w,
            202 => OpCode::breakpoint,
            _ => OpCode::unknown,
        }
    }
}