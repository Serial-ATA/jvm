use common::int_types::{u1, u2};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
	/// An index into the constant pool pointing to a `CONSTANT_Utf8_info` entry representing the name of the attribute
	pub attribute_name_index: u2,
	pub info: AttributeType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AttributeTag {
	ConstantValue,
	Code,
	StackMapTable,
	Exceptions,
	InnerClasses,
	EnclosingMethod,
	Synthetic,
	Signature,
	SourceFile,
	SourceDebugExtension,
	LineNumberTable,
	LocalVariableTable,
	LocalVariableTypeTable,
	Deprecated,
	RuntimeVisibleAnnotations,
	RuntimeInvisibleAnnotations,
	RuntimeVisibleParameterAnnotations,
	RuntimeInvisibleParameterAnnotations,
	RuntimeVisibleTypeAnnotations,
	RuntimeInvisibleTypeAnnotations,
	AnnotationDefault,
	BootstrapMethods,
	MethodParameters,
	Module,
	ModulePackages,
	ModuleMainClass,
	NestHost,
	NestMembers,
	Record,
	PermittedSubclasses,
}

impl From<&[u1]> for AttributeTag {
	fn from(bytes: &[u1]) -> Self {
		match bytes {
			b"ConstantValue" => Self::ConstantValue,
			b"Code" => Self::Code,
			b"StackMapTable" => Self::StackMapTable,
			b"Exceptions" => Self::Exceptions,
			b"InnerClasses" => Self::InnerClasses,
			b"EnclosingMethod" => Self::EnclosingMethod,
			b"Synthetic" => Self::Synthetic,
			b"Signature" => Self::Signature,
			b"SourceFile" => Self::SourceFile,
			b"SourceDebugExtension" => Self::SourceDebugExtension,
			b"LineNumberTable" => Self::LineNumberTable,
			b"LocalVariableTable" => Self::LocalVariableTable,
			b"LocalVariableTypeTable" => Self::LocalVariableTypeTable,
			b"Deprecated" => Self::Deprecated,
			b"RuntimeVisibleAnnotations" => Self::RuntimeVisibleAnnotations,
			b"RuntimeInvisibleAnnotations" => Self::RuntimeInvisibleAnnotations,
			b"RuntimeVisibleParameterAnnotations" => Self::RuntimeVisibleParameterAnnotations,
			b"RuntimeInvisibleParameterAnnotations" => Self::RuntimeInvisibleParameterAnnotations,
			b"RuntimeVisibleTypeAnnotations" => Self::RuntimeVisibleTypeAnnotations,
			b"RuntimeInvisibleTypeAnnotations" => Self::RuntimeInvisibleTypeAnnotations,
			b"AnnotationDefault" => Self::AnnotationDefault,
			b"BootstrapMethods" => Self::BootstrapMethods,
			b"MethodParameters" => Self::MethodParameters,
			b"Module" => Self::Module,
			b"ModulePackages" => Self::ModulePackages,
			b"ModuleMainClass" => Self::ModuleMainClass,
			b"NestHost" => Self::NestHost,
			b"NestMembers" => Self::NestMembers,
			b"Record" => Self::Record,
			b"PermittedSubclasses" => Self::PermittedSubclasses,
			_ => unsafe {
				panic!(
					"Encountered unknown attribute type: {}",
					std::str::from_utf8_unchecked(bytes)
				);
			},
		}
	}
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7-300
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeType {
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.2
	ConstantValue {
		constantvalue_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.3
	Code(Code),
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.4
	StackMapTable {
		entries: Vec<StackMapFrame>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.5
	Exceptions {
		exception_index_table: Vec<u2>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.6
	InnerClasses {
		classes: Vec<InnerClass>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.7
	EnclosingMethod {
		class_index: u2,
		method_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.8
	Synthetic,
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.9
	Signature {
		signature_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.10
	SourceFile {
		sourcefile_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.11
	SourceDebugExtension {
		debug_extension: Vec<u1>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.12
	LineNumberTable {
		line_number_table: Vec<LineNumber>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.13
	LocalVariableTable {
		local_variable_table: Vec<LocalVariable>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.14
	LocalVariableTypeTable {
		local_variable_type_table: Vec<LocalVariableType>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.15
	Deprecated,
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.16
	RuntimeVisibleAnnotations {
		annotations: Vec<Annotation>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.17
	RuntimeInvisibleAnnotations {
		annotations: Vec<Annotation>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.18
	RuntimeVisibleParameterAnnotations {
		annotations: Vec<Annotation>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.19
	RuntimeInvisibleParameterAnnotations {
		annotations: Vec<Annotation>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.20
	RuntimeVisibleTypeAnnotations {
		annotations: Vec<Annotation>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.21
	RuntimeInvisibleTypeAnnotations {
		annotations: Vec<Annotation>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.22
	AnnotationDefault {
		default_value: ElementValue,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.23
	BootstrapMethods {
		bootstrap_methods: Vec<BootstrapMethod>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.24
	MethodParameters {
		parameters: Vec<MethodParameter>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.25
	Module {
		module_name_index: u2,
		module_flags: u2,
		module_version_index: u2,

		requires: Vec<ModuleRequire>,
		exports: Vec<ModuleExport>,
		opens: Vec<ModuleOpen>,

		uses_index: Vec<u2>,

		provides: Vec<ModuleProvide>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.26
	ModulePackages {
		package_index: Vec<u2>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.27
	ModuleMainClass {
		main_class_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.28
	NestHost {
		host_class_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.29
	NestMembers {
		classes: Vec<u2>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.30
	Record {
		components: Vec<RecordComponentInfo>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.31
	PermittedSubclasses {
		classes: Vec<u2>,
	},
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Code {
	/// The maximum depth of the operand stack at any point during execution
	pub max_stack: u2,
	/// The number of local variables allocated upon invocation of this method, including parameters
	pub max_locals: u2,
	/// The code that implements the method
	pub code: Vec<u1>,
	/// A list of exception handlers in the code
	pub exception_table: Vec<CodeException>,
	/// Optional attributes associated with the code
	pub attributes: Vec<Attribute>,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.3
#[derive(Debug, Clone, PartialEq)]
pub struct CodeException {
	/// The start of the range where the exception handler is active
	pub start_pc: u2,
	/// The end of the range where the exception handler is active
	pub end_pc: u2,
	/// The start of the exception handler
	pub handler_pc: u2,
	/// The constant pool index for the class of exceptions that this exception handler is designated to catch
	///
	/// If this is 0, this exception handler is called for all exceptions
	pub catch_type: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.4
#[derive(Debug, Clone, PartialEq)]
pub enum StackMapFrame {
	SameFrame {
		offset_delta: u2,
	},
	SameLocals1StackItemFrame {
		offset_delta: u2,
		verification_type_info: [VerificationTypeInfo; 1],
	},
	SameLocals1StackItemFrameExtended {
		offset_delta: u2,
		verification_type_info: [VerificationTypeInfo; 1],
	},
	ChopFrame {
		offset_delta: u2,
	},
	SameFrameExtended {
		offset_delta: u2,
	},
	AppendFrame {
		offset_delta: u2,
		locals: Vec<VerificationTypeInfo>,
	},
	FullFrame {
		offset_delta: u2,
		locals: Vec<VerificationTypeInfo>,
		stack: Vec<VerificationTypeInfo>,
	},
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.4
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationTypeInfo {
	TopVariableInfo,
	IntegerVariableInfo,
	FloatVariableInfo,
	LongVariableInfo,
	DoubleVariableInfo,
	NullVariableInfo,
	UninitializedThisVariableInfo,
	ObjectVariableInfo { cpool_index: u2 },
	UninitializedVariableInfo { offset: u2 },
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.6
#[derive(Debug, Clone, PartialEq)]
pub struct InnerClass {
	pub inner_class_info_index: u2,
	pub outer_class_info_index: u2,
	pub inner_name_index: u2,
	pub inner_class_access_flags: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.12
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LineNumber {
	pub start_pc: u2,
	pub line_number: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.13
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariable {
	pub start_pc: u2,
	pub length: u2,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub index: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.14
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableType {
	pub start_pc: u2,
	pub length: u2,
	pub name_index: u2,
	pub signature_index: u2,
	pub index: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
	pub type_index: u2,
	pub element_value_pairs: Vec<ElementValuePair>,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq)]
pub struct ElementValuePair {
	pub element_name_index: u2,
	pub value: ElementValue,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq)]
pub struct ElementValue {
	pub tag: ElementValueTag,
	pub ty: ElementValueType,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElementValueTag {
	Byte,
	Char,
	Double,
	Float,
	Int,
	Long,
	Short,
	Boolean,
	String,
	Enum,
	Class,
	Annotation,
	Array,
}

impl From<u1> for ElementValueTag {
	fn from(value: u1) -> Self {
		match value {
			b'B' => ElementValueTag::Byte,
			b'C' => ElementValueTag::Char,
			b'D' => ElementValueTag::Double,
			b'F' => ElementValueTag::Float,
			b'I' => ElementValueTag::Int,
			b'J' => ElementValueTag::Long,
			b'S' => ElementValueTag::Short,
			b'Z' => ElementValueTag::Boolean,
			b's' => ElementValueTag::String,
			b'e' => ElementValueTag::Enum,
			b'c' => ElementValueTag::Class,
			b'@' => ElementValueTag::Annotation,
			b'[' => ElementValueTag::Array,
			_ => panic!("Invalid element tag encountered: {}", value),
		}
	}
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq)]
#[rustfmt::skip]
pub enum ElementValueType {
    Byte    { const_value_index: u2 },
    Char    { const_value_index: u2 },
    Double  { const_value_index: u2 },
    Float   { const_value_index: u2 },
    Int     { const_value_index: u2 },
    Long    { const_value_index: u2 },
    Short   { const_value_index: u2 },
    Boolean { const_value_index: u2 },
    String  { const_value_index: u2 },
    Enum {
        type_name_index: u2,
        const_value_index: u2,
    },
    Class {
        class_info_index: u2,
    },
    Annotation {
        annotation: Annotation,
    },
    Array {
        values: Vec<ElementValueType>
    },
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.23
#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
	pub bootstrap_method_ref: u2,
	pub bootstrap_arguments: Vec<u2>,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.24
#[derive(Debug, Clone, PartialEq)]
pub struct MethodParameter {
	pub name_index: u2,
	pub access_flags: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.25
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleRequire {
	pub requires_index: u2,
	pub requires_flags: u2,
	pub requires_version_index: u2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleExport {
	pub exports_index: u2,
	pub exports_flags: u2,
	pub exports_to_index: Vec<u2>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleOpen {
	pub opens_index: u2,
	pub opens_flags: u2,
	pub opens_to_index: Vec<u2>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleProvide {
	pub provides_index: u2,
	pub provides_with_index: Vec<u2>,
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.30
#[derive(Debug, Clone, PartialEq)]
pub struct RecordComponentInfo {
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}
