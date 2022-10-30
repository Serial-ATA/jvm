use std::io::Read;

use classfile::{
	Annotation, Attribute, AttributeTag, AttributeType, BootstrapMethod, Code, CodeException,
	ConstantPool, ElementValue, ElementValuePair, ElementValueTag, ElementValueType, InnerClass,
	LineNumber, LocalVariable, MethodParameter, StackMapFrame, VerificationTypeInfo,
};
use common::traits::JavaReadExt;
use common::types::u2;

pub fn read_attribute<R>(reader: &mut R, constant_pool: &ConstantPool) -> Attribute
where
	R: Read,
{
	let attribute_name_index = reader.read_u2();
	let attribute_name = constant_pool.get_constant_utf8(attribute_name_index);

	let attribute_length = reader.read_u4();

	let info = match AttributeTag::from(attribute_name) {
		AttributeTag::ConstantValue => AttributeType::ConstantValue {
			constantvalue_index: reader.read_u2(),
		},
		AttributeTag::Code => read_attribute_type_code(reader, constant_pool),
		AttributeTag::StackMapTable => read_attribute_type_stack_map_table(reader),
		AttributeTag::Exceptions => read_attribute_exceptions(reader),
		AttributeTag::InnerClasses => read_attribute_innerclasses(reader),
		AttributeTag::EnclosingMethod => AttributeType::EnclosingMethod {
			class_index: reader.read_u2(),
			method_index: reader.read_u2(),
		},
		AttributeTag::Synthetic => AttributeType::Synthetic,
		AttributeTag::Signature => AttributeType::Signature {
			signature_index: reader.read_u2(),
		},
		AttributeTag::SourceFile => AttributeType::SourceFile {
			sourcefile_index: reader.read_u2(),
		},
		AttributeTag::SourceDebugExtension => AttributeType::SourceDebugExtension {
			debug_extension: {
				let mut debug_extension = vec![0; attribute_length as usize];
				reader.read_exact(&mut debug_extension).unwrap();

				debug_extension
			},
		},
		AttributeTag::LineNumberTable => read_attribute_line_number_table(reader),
		AttributeTag::LocalVariableTable => AttributeType::LocalVariableTable {
			local_variable_table: read_attribute_local_variable_table(reader),
		},
		AttributeTag::LocalVariableTypeTable => AttributeType::LocalVariableTypeTable {
			local_variable_type_table: read_attribute_local_variable_table(reader),
		},
		AttributeTag::Deprecated => AttributeType::Deprecated,
		AttributeTag::RuntimeVisibleAnnotations => AttributeType::RuntimeVisibleAnnotations {
			annotations: read_attribute_runtime_annotations(reader, constant_pool),
		},
		AttributeTag::RuntimeInvisibleAnnotations => AttributeType::RuntimeInvisibleAnnotations {
			annotations: read_attribute_runtime_annotations(reader, constant_pool),
		},
		AttributeTag::RuntimeVisibleParameterAnnotations => {
			AttributeType::RuntimeVisibleParameterAnnotations {
				annotations: read_attribute_runtime_annotations(reader, constant_pool),
			}
		},
		AttributeTag::RuntimeInvisibleParameterAnnotations => {
			AttributeType::RuntimeInvisibleParameterAnnotations {
				annotations: read_attribute_runtime_annotations(reader, constant_pool),
			}
		},
		AttributeTag::RuntimeVisibleTypeAnnotations => {
			AttributeType::RuntimeVisibleTypeAnnotations {
				annotations: read_attribute_runtime_annotations(reader, constant_pool),
			}
		},
		AttributeTag::RuntimeInvisibleTypeAnnotations => {
			AttributeType::RuntimeInvisibleTypeAnnotations {
				annotations: read_attribute_runtime_annotations(reader, constant_pool),
			}
		},
		AttributeTag::AnnotationDefault => todo!(),
		AttributeTag::BootstrapMethods => read_attribute_bootstrap_methods(reader),
		AttributeTag::MethodParameters => read_attribute_method_parameters(reader),
	};

	Attribute {
		attribute_name_index,
		info,
	}
}

fn read_attribute_type_code<R>(reader: &mut R, constant_pool: &ConstantPool) -> AttributeType
where
	R: Read,
{
	let max_stack = reader.read_u2();
	let max_locals = reader.read_u2();

	let code_length = reader.read_u4();
	let mut code = vec![0; code_length as usize];
	reader.read_exact(&mut code).unwrap();

	let exception_table_length = reader.read_u2();
	let mut exception_table = Vec::with_capacity(exception_table_length as usize);

	for _ in 0..exception_table_length {
		exception_table.push(CodeException {
			start_pc: reader.read_u2(),
			end_pc: reader.read_u2(),
			handler_pc: reader.read_u2(),
			catch_type: reader.read_u2(),
		})
	}

	let attributes_count = reader.read_u2();
	let mut attributes = Vec::with_capacity(attributes_count as usize);

	for _ in 0..attributes_count {
		attributes.push(read_attribute(reader, constant_pool));
	}

	AttributeType::Code(Code {
		max_stack,
		max_locals,
		code,
		exception_table,
		attributes,
	})
}

fn read_attribute_type_stack_map_table<R>(reader: &mut R) -> AttributeType
where
	R: Read,
{
	let number_of_entries = reader.read_u2();
	let mut entries = Vec::with_capacity(number_of_entries as usize);

	for _ in 0..number_of_entries {
		let frame_type = reader.read_u1();

		let stack_map_frame = match frame_type {
			0..=63 => StackMapFrame::SameFrame {
				offset_delta: u2::from(frame_type),
			},
			64..=127 => StackMapFrame::SameLocals1StackItemFrame {
				offset_delta: u2::from(frame_type - 64),
				verification_type_info: [read_attribute_verification_type_info(reader)],
			},
			247 => StackMapFrame::SameLocals1StackItemFrameExtended {
				offset_delta: reader.read_u2(),
				verification_type_info: [read_attribute_verification_type_info(reader)],
			},
			248..=250 => StackMapFrame::ChopFrame {
				offset_delta: reader.read_u2(),
			},
			251 => StackMapFrame::SameFrameExtended {
				offset_delta: reader.read_u2(),
			},
			252..=254 => {
				let offset_delta = reader.read_u2();

				let locals_len = frame_type - 251;
				let mut locals = Vec::with_capacity(locals_len as usize);

				for _ in 0..locals_len {
					locals.push(read_attribute_verification_type_info(reader));
				}

				StackMapFrame::AppendFrame {
					offset_delta,
					locals,
				}
			},
			255 => {
				let offset_delta = reader.read_u2();

				let number_of_locals = reader.read_u2();
				let mut locals = Vec::with_capacity(number_of_locals as usize);

				for _ in 0..number_of_locals {
					locals.push(read_attribute_verification_type_info(reader));
				}

				let number_of_stack_items = reader.read_u2();
				let mut stack = Vec::with_capacity(number_of_stack_items as usize);

				for _ in 0..number_of_stack_items {
					stack.push(read_attribute_verification_type_info(reader));
				}

				StackMapFrame::FullFrame {
					offset_delta,
					locals,
					stack,
				}
			},
			_ => unreachable!(),
		};

		entries.push(stack_map_frame);
	}

	AttributeType::StackMapTable { entries }
}

fn read_attribute_verification_type_info<R>(reader: &mut R) -> VerificationTypeInfo
where
	R: Read,
{
	let tag = reader.read_u1();

	match tag {
		0 => VerificationTypeInfo::TopVariableInfo,
		1 => VerificationTypeInfo::IntegerVariableInfo,
		2 => VerificationTypeInfo::FloatVariableInfo,
		4 => VerificationTypeInfo::LongVariableInfo,
		3 => VerificationTypeInfo::DoubleVariableInfo,
		5 => VerificationTypeInfo::NullVariableInfo,
		6 => VerificationTypeInfo::UninitializedThisVariableInfo,
		7 => VerificationTypeInfo::ObjectVariableInfo {
			cpool_index: reader.read_u2(),
		},
		8 => VerificationTypeInfo::UninitializedVariableInfo {
			offset: reader.read_u2(),
		},
		_ => panic!("Encountered invalid verification type info tag"),
	}
}

fn read_attribute_exceptions<R>(reader: &mut R) -> AttributeType
where
	R: Read,
{
	let number_of_exceptions = reader.read_u2();
	let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);

	for _ in 0..number_of_exceptions {
		exception_index_table.push(reader.read_u2());
	}

	AttributeType::Exceptions {
		exception_index_table,
	}
}

fn read_attribute_innerclasses<R>(reader: &mut R) -> AttributeType
where
	R: Read,
{
	let number_of_classes = reader.read_u2();
	let mut classes = Vec::with_capacity(number_of_classes as usize);

	for _ in 0..number_of_classes {
		classes.push(InnerClass {
			inner_class_info_index: reader.read_u2(),
			outer_class_info_index: reader.read_u2(),
			inner_name_index: reader.read_u2(),
			inner_class_access_flags: reader.read_u2(),
		})
	}

	AttributeType::InnerClasses { classes }
}

fn read_attribute_line_number_table<R>(reader: &mut R) -> AttributeType
where
	R: Read,
{
	let line_number_table_length = reader.read_u2();
	let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);

	for _ in 0..line_number_table_length {
		line_number_table.push(LineNumber {
			start_pc: reader.read_u2(),
			line_number: reader.read_u2(),
		})
	}

	AttributeType::LineNumberTable { line_number_table }
}

fn read_attribute_local_variable_table<R>(reader: &mut R) -> Vec<LocalVariable>
where
	R: Read,
{
	let local_variable_table_length = reader.read_u2();
	let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);

	for _ in 0..local_variable_table_length {
		local_variable_table.push(LocalVariable {
			start_pc: reader.read_u2(),
			length: reader.read_u2(),
			name_index: reader.read_u2(),
			signature_index: reader.read_u2(),
			index: reader.read_u2(),
		})
	}

	local_variable_table
}

fn read_attribute_runtime_annotations<R>(
	reader: &mut R,
	constant_pool: &ConstantPool,
) -> Vec<Annotation>
where
	R: Read,
{
	let num_annotations = reader.read_u2();
	let mut annotations = Vec::with_capacity(num_annotations as usize);

	for _ in 0..num_annotations {
		annotations.push(read_annotation(reader, constant_pool));
	}

	annotations
}

fn read_annotation<R>(reader: &mut R, constant_pool: &ConstantPool) -> Annotation
where
	R: Read,
{
	let type_index = reader.read_u2();

	let num_element_value_pairs = reader.read_u2();
	let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

	for _ in 0..num_element_value_pairs {
		let element_name_index = reader.read_u2();

		let tag = ElementValueTag::from(reader.read_u1());
		let value = read_element_value_type(reader, tag, constant_pool);

		element_value_pairs.push(ElementValuePair {
			element_name_index,
			value: ElementValue { tag, ty: value },
		})
	}

	Annotation {
		type_index,
		element_value_pairs,
	}
}

#[rustfmt::skip]
fn read_element_value_type<R>(reader: &mut R, tag: ElementValueTag, constant_pool: &ConstantPool) -> ElementValueType
where
	R: Read,
{
	match tag {
		// The const_value_index item is used if the tag item is one of B, C, D, F, I, J, S, Z, or s.
		ElementValueTag::Byte    => ElementValueType::Byte    { const_value_index: reader.read_u2() },
		ElementValueTag::Char    => ElementValueType::Char    { const_value_index: reader.read_u2() },
		ElementValueTag::Double  => ElementValueType::Double  { const_value_index: reader.read_u2() },
		ElementValueTag::Float   => ElementValueType::Float   { const_value_index: reader.read_u2() },
		ElementValueTag::Int     => ElementValueType::Int     { const_value_index: reader.read_u2() },
		ElementValueTag::Long    => ElementValueType::Long    { const_value_index: reader.read_u2() },
		ElementValueTag::Short   => ElementValueType::Short   { const_value_index: reader.read_u2() },
		ElementValueTag::Boolean => ElementValueType::Boolean { const_value_index: reader.read_u2() },
		ElementValueTag::String  => ElementValueType::String  { const_value_index: reader.read_u2() },

		// The enum_const_value item is used if the tag item is e.
		ElementValueTag::Enum => ElementValueType::Enum {
			type_name_index: reader.read_u2(),
			const_value_index: reader.read_u2(),
		},

		// The class_info_index item is used if the tag item is c.
		ElementValueTag::Class => ElementValueType::Class {
			class_info_index: reader.read_u2(),
		},

		// The annotation_value item is used if the tag item is @.
		ElementValueTag::Annotation => ElementValueType::Annotation {
			annotation: read_annotation(reader, constant_pool),
		},

		// The array_value item is used if the tag item is [.
		ElementValueTag::Array => {
			let num_values = reader.read_u2();
			let mut values = Vec::with_capacity(num_values as usize);

			for _ in 0..num_values {
				values.push(read_element_value_type(reader, tag, constant_pool));
			}

			ElementValueType::Array { values }
		},
	}
}

fn read_attribute_bootstrap_methods<R>(reader: &mut R) -> AttributeType
where
	R: Read,
{
	let num_bootstrap_methods = reader.read_u2();
	let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);

	for _ in 0..num_bootstrap_methods {
		let bootstrap_method_ref = reader.read_u2();

		let num_bootstrap_arguments = reader.read_u2();
		let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);

		for _ in 0..num_bootstrap_arguments {
			bootstrap_arguments.push(reader.read_u2());
		}

		bootstrap_methods.push(BootstrapMethod {
			bootstrap_method_ref,
			bootstrap_arguments,
		})
	}

	AttributeType::BootstrapMethods { bootstrap_methods }
}

fn read_attribute_method_parameters<R>(reader: &mut R) -> AttributeType
where
	R: Read,
{
	let parameters_count = reader.read_u1();
	let mut parameters = Vec::with_capacity(parameters_count as usize);

	for _ in 0..parameters_count {
		parameters.push(MethodParameter {
			name_index: reader.read_u2(),
			access_flags: reader.read_u2(),
		})
	}

	AttributeType::MethodParameters { parameters }
}
