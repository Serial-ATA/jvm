pub mod resolved;

use crate::constant_pool::ConstantPool;
use crate::error::ClassFileParseError;

use std::io::Read;

use common::box_slice;
use common::int_types::{u1, u2};
use common::traits::JavaReadExt;

macro_rules! attribute_getter_methods {
	($($([$flag:ident])? $variant:ident),+ $(,)?) => {
		impl Attribute {
			paste::paste! {
				$(
					attribute_getter_methods!($($flag)? [<$variant:snake>]; $variant);
				)+
			}
		}
	};
	(COPY $snake_name:ident; $variant:ident) => {
		pub fn $snake_name(&self) -> Option<$variant> {
			match self.info {
				AttributeType::$variant(inner) => Some(inner),
				_ => None
			}
		}
	};
	(MARKER $snake_name:ident; $variant:ident) => {
		paste::paste! {
			pub fn [<is_ $snake_name>](&self) -> bool {
				matches!(self.info, AttributeType::$variant)
			}
		}
	};
	($snake_name:ident; $variant:ident) => {
		pub fn $snake_name(&self) -> Option<&$variant> {
			match &self.info {
				AttributeType::$variant(inner) => Some(inner),
				_ => None
			}
		}
	};
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
	/// An index into the constant pool pointing to a `CONSTANT_Utf8_info` entry representing the name of the attribute
	pub attribute_name_index: u2,
	pub info: AttributeType,
}

attribute_getter_methods! {
	[COPY  ] ConstantValue,
	Code,
	StackMapTable,
	Exceptions,
	InnerClasses,
	[COPY  ] EnclosingMethod,
	[MARKER] Synthetic,
	[COPY  ] Signature,
	[COPY  ] SourceFile,
	SourceDebugExtension,
	LineNumberTable,
	LocalVariableTable,
	LocalVariableTypeTable,
	[MARKER] Deprecated,
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
	[COPY  ] ModuleMainClass,
	[COPY  ] NestHost,
	NestMembers,
	Record,
	PermittedSubclasses,
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

impl TryFrom<&[u1]> for AttributeTag {
	type Error = ClassFileParseError;

	fn try_from(bytes: &[u1]) -> Result<Self, Self::Error> {
		match bytes {
			b"ConstantValue" => Ok(Self::ConstantValue),
			b"Code" => Ok(Self::Code),
			b"StackMapTable" => Ok(Self::StackMapTable),
			b"Exceptions" => Ok(Self::Exceptions),
			b"InnerClasses" => Ok(Self::InnerClasses),
			b"EnclosingMethod" => Ok(Self::EnclosingMethod),
			b"Synthetic" => Ok(Self::Synthetic),
			b"Signature" => Ok(Self::Signature),
			b"SourceFile" => Ok(Self::SourceFile),
			b"SourceDebugExtension" => Ok(Self::SourceDebugExtension),
			b"LineNumberTable" => Ok(Self::LineNumberTable),
			b"LocalVariableTable" => Ok(Self::LocalVariableTable),
			b"LocalVariableTypeTable" => Ok(Self::LocalVariableTypeTable),
			b"Deprecated" => Ok(Self::Deprecated),
			b"RuntimeVisibleAnnotations" => Ok(Self::RuntimeVisibleAnnotations),
			b"RuntimeInvisibleAnnotations" => Ok(Self::RuntimeInvisibleAnnotations),
			b"RuntimeVisibleParameterAnnotations" => Ok(Self::RuntimeVisibleParameterAnnotations),
			b"RuntimeInvisibleParameterAnnotations" => {
				Ok(Self::RuntimeInvisibleParameterAnnotations)
			},
			b"RuntimeVisibleTypeAnnotations" => Ok(Self::RuntimeVisibleTypeAnnotations),
			b"RuntimeInvisibleTypeAnnotations" => Ok(Self::RuntimeInvisibleTypeAnnotations),
			b"AnnotationDefault" => Ok(Self::AnnotationDefault),
			b"BootstrapMethods" => Ok(Self::BootstrapMethods),
			b"MethodParameters" => Ok(Self::MethodParameters),
			b"Module" => Ok(Self::Module),
			b"ModulePackages" => Ok(Self::ModulePackages),
			b"ModuleMainClass" => Ok(Self::ModuleMainClass),
			b"NestHost" => Ok(Self::NestHost),
			b"NestMembers" => Ok(Self::NestMembers),
			b"Record" => Ok(Self::Record),
			b"PermittedSubclasses" => Ok(Self::PermittedSubclasses),
			_ => Err(ClassFileParseError::BadAttributeTag(bytes.into())),
		}
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7-300
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeType {
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.2
	ConstantValue(ConstantValue),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.3
	Code(Code),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.4
	StackMapTable(StackMapTable),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.5
	Exceptions(Exceptions),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.6
	InnerClasses(InnerClasses),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.7
	EnclosingMethod(EnclosingMethod),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.8
	Synthetic,
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.9
	Signature(Signature),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.10
	SourceFile(SourceFile),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.11
	SourceDebugExtension(SourceDebugExtension),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.12
	LineNumberTable(LineNumberTable),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.13
	LocalVariableTable(LocalVariableTable),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.14
	LocalVariableTypeTable(LocalVariableTypeTable),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.15
	Deprecated,
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16
	RuntimeVisibleAnnotations(RuntimeVisibleAnnotations),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.17
	RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.18
	RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.19
	RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20
	RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotations),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.21
	RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotations),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.22
	AnnotationDefault(AnnotationDefault),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.23
	BootstrapMethods(BootstrapMethods),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.24
	MethodParameters(MethodParameters),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.25
	Module(Module),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.26
	ModulePackages(ModulePackages),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.27
	ModuleMainClass(ModuleMainClass),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.28
	NestHost(NestHost),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.29
	NestMembers(NestMembers),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.30
	Record(Record),
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.31
	PermittedSubclasses(PermittedSubclasses),
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ConstantValue {
	pub constantvalue_index: u2,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Code {
	/// The maximum depth of the operand stack at any point during execution
	pub max_stack: u2,
	/// The number of local variables allocated upon invocation of this method, including parameters
	pub max_locals: u2,
	/// The code that implements the method
	pub code: Box<[u1]>,
	/// A list of exception handlers in the code
	pub exception_table: Vec<CodeException>,
	/// Optional attributes associated with the code
	pub attributes: Vec<Attribute>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StackMapTable {
	pub entries: Vec<StackMapFrame>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Exceptions {
	pub exception_index_table: Vec<u2>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InnerClasses {
	pub classes: Vec<InnerClass>,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct EnclosingMethod {
	pub class_index: u2,
	pub method_index: u2,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Signature {
	pub signature_index: u2,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct SourceFile {
	pub sourcefile_index: u2,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SourceDebugExtension {
	pub debug_extension: Box<[u1]>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineNumberTable {
	pub line_number_table: Vec<LineNumber>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LocalVariableTable {
	pub local_variable_table: Vec<LocalVariable>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LocalVariableTypeTable {
	pub local_variable_type_table: Vec<LocalVariableType>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeVisibleAnnotations {
	pub annotations: Vec<Annotation>,
}

fn encode_annotations(annotations: &[Annotation]) -> Box<[u1]> {
	let num_annotations = annotations.len() as u2;
	let mut ret = Vec::with_capacity((num_annotations as usize) * size_of::<Annotation>());
	ret.extend(num_annotations.to_be_bytes());

	for annotation in annotations {
		ret.extend(annotation.as_bytes());
	}

	ret.into_boxed_slice()
}

impl RuntimeVisibleAnnotations {
	pub fn as_bytes(&self) -> Box<[u1]> {
		encode_annotations(self.annotations.as_slice())
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeInvisibleAnnotations {
	pub annotations: Vec<Annotation>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.18
pub struct ParameterAnnotations {
	pub annotations: Box<[Box<[Annotation]>]>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeVisibleParameterAnnotations {
	pub annotations: ParameterAnnotations,
}

impl RuntimeVisibleParameterAnnotations {
	pub fn as_bytes(&self) -> Box<[u1]> {
		let num_parameters = self.annotations.annotations.len() as u2;
		let num_annotations: usize = self.annotations.annotations.iter().map(|a| a.len()).sum();
		let mut ret = Vec::with_capacity(
			(num_parameters as usize) * (num_annotations * size_of::<Annotation>()),
		);
		ret.extend(num_parameters.to_be_bytes());

		for annotation in &self.annotations.annotations {
			ret.extend(encode_annotations(annotation))
		}
		ret.into_boxed_slice()
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeInvisibleParameterAnnotations {
	pub annotations: ParameterAnnotations,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeVisibleTypeAnnotations {
	pub annotations: Vec<TypeAnnotation>,
}

impl RuntimeVisibleTypeAnnotations {
	pub fn as_bytes(&self) -> Box<[u1]> {
		let num_annotations = self.annotations.len() as u2;
		let mut ret = Vec::with_capacity((num_annotations as usize) * size_of::<Annotation>());
		ret.extend(num_annotations.to_be_bytes());

		for annotation in &self.annotations {
			ret.extend(annotation.as_bytes());
		}

		ret.into_boxed_slice()
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeInvisibleTypeAnnotations {
	pub annotations: Vec<TypeAnnotation>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationDefault {
	pub default_value: ElementValue,
}

impl AnnotationDefault {
	pub fn as_bytes(&self) -> Box<[u1]> {
		self.default_value.as_bytes()
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BootstrapMethods {
	pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MethodParameters {
	pub parameters: Vec<MethodParameter>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Module {
	pub module_name_index: u2,
	pub module_flags: u2,
	pub module_version_index: u2,

	pub requires: Vec<ModuleRequire>,
	pub exports: Vec<ModuleExport>,
	pub opens: Vec<ModuleOpen>,

	pub uses_index: Vec<u2>,

	pub provides: Vec<ModuleProvide>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModulePackages {
	pub package_index: Vec<u2>,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ModuleMainClass {
	pub main_class_index: u2,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct NestHost {
	pub host_class_index: u2,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NestMembers {
	pub classes: Vec<u2>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Record {
	pub components: Vec<RecordComponentInfo>,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PermittedSubclasses {
	pub classes: Vec<u2>,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.3
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

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.4
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

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.4
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

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.6
#[derive(Debug, Clone, PartialEq)]
pub struct InnerClass {
	pub inner_class_info_index: u2,
	pub outer_class_info_index: u2,
	pub inner_name_index: u2,
	pub inner_class_access_flags: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.12
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LineNumber {
	pub start_pc: u2,
	pub line_number: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.13
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariable {
	pub start_pc: u2,
	pub length: u2,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub index: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.14
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LocalVariableType {
	pub start_pc: u2,
	pub length: u2,
	pub name_index: u2,
	pub signature_index: u2,
	pub index: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
	pub type_index: u2,
	pub element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
	pub fn parse<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
	) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		let type_index = reader.read_u2()?;

		let num_element_value_pairs = reader.read_u2()?;
		let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

		for _ in 0..num_element_value_pairs {
			element_value_pairs.push(ElementValuePair::parse(reader, constant_pool)?);
		}

		Ok(Annotation {
			type_index,
			element_value_pairs,
		})
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut ret = Vec::with_capacity(size_of::<Self>() + size_of::<u2>());
		ret.extend(self.type_index.to_be_bytes());
		ret.extend((self.element_value_pairs.len() as u2).to_be_bytes());

		for element_value_pair in &self.element_value_pairs {
			ret.extend(element_value_pair.as_bytes());
		}

		ret.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementValuePair {
	pub element_name_index: u2,
	pub value: ElementValue,
}

impl ElementValuePair {
	pub fn parse<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
	) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		let element_name_index = reader.read_u2()?;
		let value = ElementValue::parse(reader, constant_pool)?;

		Ok(ElementValuePair {
			element_name_index,
			value,
		})
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut ret = Vec::with_capacity(size_of::<Self>());
		ret.extend(self.element_name_index.to_be_bytes());
		ret.extend(self.value.as_bytes());
		ret.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementValue {
	pub tag: ElementValueTag,
	pub ty: ElementValueType,
}

impl ElementValue {
	pub fn parse<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
	) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		let tag = ElementValueTag::try_from(reader.read_u1()?)?;

		#[rustfmt::skip]
        let ty = match tag {
            // The const_value_index item is used if the tag item is one of B, C, D, F, I, J, S, Z, or s.
            ElementValueTag::Byte    => ElementValueType::Byte    { const_value_index: reader.read_u2()? },
            ElementValueTag::Char    => ElementValueType::Char    { const_value_index: reader.read_u2()? },
            ElementValueTag::Double  => ElementValueType::Double  { const_value_index: reader.read_u2()? },
            ElementValueTag::Float   => ElementValueType::Float   { const_value_index: reader.read_u2()? },
            ElementValueTag::Int     => ElementValueType::Int     { const_value_index: reader.read_u2()? },
            ElementValueTag::Long    => ElementValueType::Long    { const_value_index: reader.read_u2()? },
            ElementValueTag::Short   => ElementValueType::Short   { const_value_index: reader.read_u2()? },
            ElementValueTag::Boolean => ElementValueType::Boolean { const_value_index: reader.read_u2()? },
            ElementValueTag::String  => ElementValueType::String  { const_value_index: reader.read_u2()? },

            // The enum_const_value item is used if the tag item is e.
            ElementValueTag::Enum => ElementValueType::Enum {
                type_name_index: reader.read_u2()?,
                const_value_index: reader.read_u2()?,
            },

            // The class_info_index item is used if the tag item is c.
            ElementValueTag::Class => ElementValueType::Class {
                class_info_index: reader.read_u2()?,
            },

            // The annotation_value item is used if the tag item is @.
            ElementValueTag::Annotation => ElementValueType::Annotation {
                annotation: Annotation::parse(reader, constant_pool)?,
            },

            // The array_value item is used if the tag item is [.
            ElementValueTag::Array => {
                let num_values = reader.read_u2()?;
                let mut values = Vec::with_capacity(num_values as usize);

                for _ in 0..num_values {
                    values.push(ElementValue::parse(reader, constant_pool)?);
                }

                ElementValueType::Array { values }
            },
        };

		Ok(ElementValue { tag, ty })
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut ret =
			Vec::with_capacity(size_of::<ElementValueTag>() + size_of::<ElementValueType>());
		ret.push(self.tag as u8);
		ret.extend(self.ty.as_bytes());
		ret.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16.1
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElementValueTag {
	Byte = b'B',
	Char = b'C',
	Double = b'D',
	Float = b'F',
	Int = b'I',
	Long = b'J',
	Short = b'S',
	Boolean = b'Z',
	String = b's',
	Enum = b'e',
	Class = b'c',
	Annotation = b'@',
	Array = b'[',
}

impl TryFrom<u1> for ElementValueTag {
	type Error = ClassFileParseError;

	fn try_from(value: u1) -> Result<Self, Self::Error> {
		match value {
			b'B' => Ok(ElementValueTag::Byte),
			b'C' => Ok(ElementValueTag::Char),
			b'D' => Ok(ElementValueTag::Double),
			b'F' => Ok(ElementValueTag::Float),
			b'I' => Ok(ElementValueTag::Int),
			b'J' => Ok(ElementValueTag::Long),
			b'S' => Ok(ElementValueTag::Short),
			b'Z' => Ok(ElementValueTag::Boolean),
			b's' => Ok(ElementValueTag::String),
			b'e' => Ok(ElementValueTag::Enum),
			b'c' => Ok(ElementValueTag::Class),
			b'@' => Ok(ElementValueTag::Annotation),
			b'[' => Ok(ElementValueTag::Array),
			_ => Err(ClassFileParseError::BadElementTag(value)),
		}
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16.1
#[derive(Debug, Clone, PartialEq, Eq)]
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
        values: Vec<ElementValue>
    },
}

impl ElementValueType {
	pub fn as_bytes(&self) -> Box<[u1]> {
		match self {
			ElementValueType::Byte { const_value_index }
			| ElementValueType::Char { const_value_index }
			| ElementValueType::Double { const_value_index }
			| ElementValueType::Float { const_value_index }
			| ElementValueType::Int { const_value_index }
			| ElementValueType::Long { const_value_index }
			| ElementValueType::Short { const_value_index }
			| ElementValueType::Boolean { const_value_index }
			| ElementValueType::String { const_value_index } => {
				let [b1, b2] = const_value_index.to_be_bytes();
				box_slice![b1, b2]
			},
			ElementValueType::Enum {
				type_name_index,
				const_value_index,
			} => {
				let [b1, b2] = type_name_index.to_be_bytes();
				let [b3, b4] = const_value_index.to_be_bytes();
				box_slice![b1, b2, b3, b4]
			},
			ElementValueType::Class { class_info_index } => {
				let [b1, b2] = class_info_index.to_be_bytes();
				box_slice![b1, b2]
			},
			ElementValueType::Annotation { annotation } => annotation.as_bytes(),
			ElementValueType::Array { values } => {
				let mut ret = Vec::with_capacity(values.len() * size_of::<ElementValue>());
				ret.extend((values.len() as u2).to_be_bytes());

				for value in values {
					ret.extend(value.as_bytes());
				}

				ret.into_boxed_slice()
			},
		}
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAnnotation {
	pub target_type: u1,
	pub target_info: TypeAnnotationTargetInfo,
	pub type_path: TypePath,
	pub annotation: Annotation,
}

impl TypeAnnotation {
	pub fn parse<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
	) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		let (target_type, target_info) = TypeAnnotationTargetInfo::parse(reader)?;
		let type_path = TypePath::parse(reader)?;
		Ok(Self {
			target_type,
			target_info,
			type_path,
			annotation: Annotation::parse(reader, constant_pool)?,
		})
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut encoded = Vec::new();
		encoded.push(self.target_type);
		encoded.extend(self.target_info.as_bytes());
		encoded.extend(self.type_path.as_bytes());
		encoded.extend(self.annotation.as_bytes());
		encoded.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeAnnotationTargetInfo {
	TypeParameter {
		type_parameter_index: u1,
	},
	Supertype {
		supertype_index: u2,
	},
	TypeParameterBound {
		type_parameter_index: u1,
		bound_index: u1,
	},
	Empty,
	FormalParameter {
		formal_parameter_index: u1,
	},
	Throws {
		throws_type_index: u2,
	},
	LocalVar(Vec<LocalVarTableEntry>),
	Catch {
		exception_table_index: u2,
	},
	Offset(u2),
	TypeArgument {
		offset: u2,
		type_argument_index: u1,
	},
}

impl TypeAnnotationTargetInfo {
	pub fn parse<R>(reader: &mut R) -> Result<(u1, Self), ClassFileParseError>
	where
		R: Read,
	{
		let target_type = reader.read_u1()?;

		let target_info = match target_type {
			0x00 | 0x01 => Self::TypeParameter {
				type_parameter_index: reader.read_u1()?,
			},
			0x10 => Self::Supertype {
				supertype_index: reader.read_u2()?,
			},
			0x11 | 0x12 => Self::TypeParameterBound {
				type_parameter_index: reader.read_u1()?,
				bound_index: reader.read_u1()?,
			},
			0x13 | 0x14 | 0x15 => Self::Empty,
			0x16 => Self::FormalParameter {
				formal_parameter_index: reader.read_u1()?,
			},
			0x17 => Self::Throws {
				throws_type_index: reader.read_u2()?,
			},
			0x40 | 0x41 => {
				let table_length = reader.read_u2()?;

				let mut entries = Vec::with_capacity(table_length as usize);
				for _ in 0..table_length {
					entries.push(LocalVarTableEntry::parse(reader)?);
				}

				Self::LocalVar(entries)
			},
			0x42 => Self::Catch {
				exception_table_index: reader.read_u2()?,
			},
			0x43 | 0x44 | 0x45 | 0x46 => Self::Offset(reader.read_u2()?),
			0x47 | 0x48 | 0x49 | 0x4A | 0x4B => Self::TypeArgument {
				offset: reader.read_u2()?,
				type_argument_index: reader.read_u1()?,
			},
			_ => panic!("TODO: error"),
		};

		Ok((target_type, target_info))
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut encoded = Vec::new();
		match self {
			TypeAnnotationTargetInfo::TypeParameter {
				type_parameter_index,
			} => encoded.push(*type_parameter_index),
			TypeAnnotationTargetInfo::Supertype { supertype_index } => {
				encoded.extend(supertype_index.to_be_bytes())
			},
			TypeAnnotationTargetInfo::TypeParameterBound {
				type_parameter_index,
				bound_index,
			} => {
				encoded.push(*type_parameter_index);
				encoded.push(*bound_index);
			},
			TypeAnnotationTargetInfo::Empty => {},
			TypeAnnotationTargetInfo::FormalParameter {
				formal_parameter_index,
			} => {
				encoded.push(*formal_parameter_index);
			},
			TypeAnnotationTargetInfo::Throws { throws_type_index } => {
				encoded.extend(throws_type_index.to_be_bytes());
			},
			TypeAnnotationTargetInfo::LocalVar(entries) => {
				encoded.extend((entries.len() as u2).to_be_bytes());
				for entry in entries {
					encoded.extend(entry.as_bytes());
				}
			},
			TypeAnnotationTargetInfo::Catch {
				exception_table_index,
			} => {
				encoded.extend(exception_table_index.to_be_bytes());
			},
			TypeAnnotationTargetInfo::Offset(offset) => {
				encoded.extend(offset.to_be_bytes());
			},
			TypeAnnotationTargetInfo::TypeArgument {
				offset,
				type_argument_index,
			} => {
				encoded.extend(offset.to_be_bytes());
				encoded.push(*type_argument_index);
			},
		}

		encoded.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct LocalVarTableEntry {
	pub start_pc: u2,
	pub length: u2,
	pub index: u2,
}

impl LocalVarTableEntry {
	pub fn parse<R>(reader: &mut R) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		Ok(Self {
			start_pc: reader.read_u2()?,
			length: reader.read_u2()?,
			index: reader.read_u2()?,
		})
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut encoded = Vec::with_capacity(6);
		encoded.extend(self.start_pc.to_be_bytes());
		encoded.extend(self.length.to_be_bytes());
		encoded.extend(self.index.to_be_bytes());
		encoded.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20.2
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TypePath {
	pub path: Vec<Path>,
}

impl TypePath {
	pub fn parse<R>(reader: &mut R) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		let path_length = reader.read_u1()?;

		let mut path = Vec::with_capacity(path_length as usize);
		for _ in 0..path_length {
			path.push(Path::parse(reader)?);
		}

		Ok(Self { path })
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		let mut encoded = Vec::new();
		encoded.push(self.path.len() as u1);

		for path in &self.path {
			encoded.extend(path.as_bytes());
		}

		encoded.into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20.2
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Path {
	pub kind: TypePathKind,
	pub type_argument_index: u1,
}

impl Path {
	pub fn parse<R>(reader: &mut R) -> Result<Self, ClassFileParseError>
	where
		R: Read,
	{
		let type_path_kind = TypePathKind::try_from(reader.read_u1()?)?;
		let type_argument_index = reader.read_u1()?;

		Ok(Self {
			kind: type_path_kind,
			type_argument_index,
		})
	}

	pub fn as_bytes(&self) -> Box<[u1]> {
		vec![self.kind as u1, self.type_argument_index].into_boxed_slice()
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.20.2-220-B-A.1
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TypePathKind {
	/// Annotation is deeper in an array type
	InArrayType = 0,
	/// Annotation is deeper in a nested type
	InNestedType = 1,
	/// Annotation is on the bound of a wildcard type argument of a parameterized type
	InWildcardTypeBound = 2,
	/// Annotation is on a type argument of a parameterized type
	InTypeArgument = 3,
}

impl TryFrom<u1> for TypePathKind {
	type Error = ClassFileParseError;

	fn try_from(value: u1) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(TypePathKind::InArrayType),
			1 => Ok(TypePathKind::InNestedType),
			2 => Ok(TypePathKind::InWildcardTypeBound),
			3 => Ok(TypePathKind::InTypeArgument),
			_ => panic!("TODO: Error"),
		}
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.23
#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
	pub bootstrap_method_ref: u2,
	pub bootstrap_arguments: Vec<u2>,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.24
#[derive(Debug, Clone, PartialEq)]
pub struct MethodParameter {
	pub name_index: u2,
	pub access_flags: u2,
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.25
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

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.30
#[derive(Debug, Clone, PartialEq)]
pub struct RecordComponentInfo {
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}
