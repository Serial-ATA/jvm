use super::{Annotation, ElementValue, ElementValuePair, ElementValueTag, ElementValueType};
use crate::attribute::BootstrapMethod;
use crate::constant_pool::types::{self, LoadableConstantPoolValue, MethodHandleEntry};
use crate::constant_pool::ConstantPool;

use common::int_types::{s4, s8, u2};

pub struct ResolvedAnnotation {
	pub name: String,
	pub element_value_pairs: Vec<ResolvedElementValuePair>,
}

impl ResolvedAnnotation {
	pub(crate) fn resolve_from(raw_annotation: &Annotation, constant_pool: &ConstantPool) -> Self {
		let name = constant_pool.get::<types::raw::RawConstantUtf8>(raw_annotation.type_index);
		let element_value_pairs = raw_annotation
			.element_value_pairs
			.iter()
			.map(|pair| ResolvedElementValuePair::resolve_from(pair, constant_pool))
			.collect();

		Self {
			name: String::from_utf8(name.to_vec()).unwrap(),
			element_value_pairs,
		}
	}
}

pub struct ResolvedElementValuePair {
	pub element_name: String,
	pub value: ResolvedElementValue,
}

impl ResolvedElementValuePair {
	fn resolve_from(raw_value_pair: &ElementValuePair, constant_pool: &ConstantPool) -> Self {
		let element_name =
			constant_pool.get::<types::raw::RawConstantUtf8>(raw_value_pair.element_name_index);
		let value = ResolvedElementValue::resolve_from(&raw_value_pair.value, constant_pool);

		Self {
			element_name: String::from_utf8(element_name.to_vec()).unwrap(), // TODO: Error handling
			value,
		}
	}
}

pub struct ResolvedElementValue {
	pub tag: ElementValueTag,
	pub value: ResolvedElementValueType,
}

impl ResolvedElementValue {
	fn resolve_from(raw_element_value: &ElementValue, constant_pool: &ConstantPool) -> Self {
		let tag = raw_element_value.tag;
		let value = match &raw_element_value.ty {
			ElementValueType::Byte { const_value_index } => ResolvedElementValueType::Byte(
				constant_pool.get::<types::raw::Integer>(*const_value_index),
			),
			ElementValueType::Char { const_value_index } => ResolvedElementValueType::Char(
				constant_pool.get::<types::raw::Integer>(*const_value_index),
			),
			ElementValueType::Double { const_value_index } => ResolvedElementValueType::Double(
				constant_pool.get::<types::raw::Double>(*const_value_index),
			),
			ElementValueType::Float { const_value_index } => ResolvedElementValueType::Float(
				constant_pool.get::<types::raw::Float>(*const_value_index),
			),
			ElementValueType::Int { const_value_index } => ResolvedElementValueType::Int(
				constant_pool.get::<types::raw::Integer>(*const_value_index),
			),
			ElementValueType::Long { const_value_index } => ResolvedElementValueType::Long(
				constant_pool.get::<types::raw::Long>(*const_value_index),
			),
			ElementValueType::Short { const_value_index } => ResolvedElementValueType::Short(
				constant_pool.get::<types::raw::Integer>(*const_value_index),
			),
			ElementValueType::Boolean { const_value_index } => ResolvedElementValueType::Boolean(
				constant_pool.get::<types::raw::Integer>(*const_value_index),
			),

			ElementValueType::String { const_value_index } => {
				let value = constant_pool.get::<types::raw::RawConstantUtf8>(*const_value_index);
				ResolvedElementValueType::String(String::from_utf8(value.to_vec()).unwrap())
			},
			ElementValueType::Enum {
				type_name_index,
				const_value_index,
			} => {
				let type_name = constant_pool.get::<types::raw::RawConstantUtf8>(*type_name_index);
				let const_value =
					constant_pool.get::<types::raw::RawConstantUtf8>(*const_value_index);
				ResolvedElementValueType::Enum {
					type_name: String::from_utf8(type_name.to_vec()).unwrap(),
					const_value: String::from_utf8(const_value.to_vec()).unwrap(),
				}
			},
			ElementValueType::Class { .. } => todo!(),
			ElementValueType::Annotation { annotation } => {
				let annotation = ResolvedAnnotation::resolve_from(annotation, constant_pool);
				ResolvedElementValueType::Annotation { annotation }
			},
			ElementValueType::Array { values } => {
				let values = values
					.iter()
					.map(|value| ResolvedElementValue::resolve_from(value, constant_pool))
					.collect();
				ResolvedElementValueType::Array { values }
			},
		};

		Self { tag, value }
	}
}

pub enum ResolvedElementValueType {
	Byte(s4),
	Char(s4),
	Double(f64),
	Float(f32),
	Int(s4),
	Long(s8),
	Short(s4),
	Boolean(s4),
	String(String),
	Enum {
		type_name: String,
		const_value: String,
	},
	Class {
		class_info_index: u2,
	},
	Annotation {
		annotation: ResolvedAnnotation,
	},
	Array {
		values: Vec<ResolvedElementValue>,
	},
}

pub struct ResolvedBootstrapMethod {
	pub method_handle_index: u2,
	pub method_handle_info: MethodHandleEntry<'static>,
	pub arguments: Vec<LoadableConstantPoolValue<'static>>,
}

impl ResolvedBootstrapMethod {
	pub(crate) fn resolve_from(raw: &BootstrapMethod, constant_pool: &ConstantPool) -> Self {
		let method_handle_info =
			constant_pool.get::<types::raw::RawMethodHandle>(raw.bootstrap_method_ref);
		let mut arguments = Vec::with_capacity(raw.bootstrap_arguments.len());
		for argument_index in &raw.bootstrap_arguments {
			arguments.push(
				constant_pool
					.get_loadable_entry(*argument_index)
					.into_owned(),
			);
		}

		Self {
			method_handle_index: raw.bootstrap_method_ref,
			method_handle_info: method_handle_info.into_owned(),
			arguments,
		}
	}
}
