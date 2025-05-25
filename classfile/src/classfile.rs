use crate::accessflags::ClassAccessFlags;
use crate::attribute::resolved::{
	ResolvedBootstrapMethod, ResolvedEnclosingMethod, ResolvedInnerClass,
};
use crate::attribute::{Attribute, AttributeType, SourceFile};
use crate::constant_pool::types::{ClassNameEntry, raw as raw_types};
use crate::constant_pool::{self, ConstantPool};
use crate::fieldinfo::FieldInfo;
use crate::methodinfo::MethodInfo;
use crate::parse::error::Result;

use std::borrow::Cow;
use std::io::Read;

use common::int_types::{u1, u2};

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.1
#[derive(Debug, Clone, PartialEq)]
pub struct ClassFile {
	pub minor_version: u2,
	pub major_version: u2,
	pub constant_pool: ConstantPool,
	pub access_flags: ClassAccessFlags,
	pub this_class: u2,
	pub super_class: u2,
	pub interfaces: Vec<u2>,
	pub fields: Vec<FieldInfo>,
	pub methods: Vec<MethodInfo>,
	pub attributes: Vec<Attribute>,
}

impl ClassFile {
	pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
		crate::parse::parse_class(reader)
	}

	pub fn get_super_class(&self) -> Option<Cow<'_, [u1]>> {
		// For a class, the value of the super_class item either must be zero or must be a valid
		// index into the constant_pool table.
		let super_class_index = self.super_class;

		let mut super_class = None;

		// If the value of the super_class item is zero, then this class file must represent
		// the class Object, the only class or interface without a direct superclass.
		if super_class_index == 0 {
			let this_class_name = self
				.constant_pool
				.get::<constant_pool::types::raw::RawClassName>(self.this_class)
				.name;
			assert_eq!(
				*this_class_name, *b"java/lang/Object",
				"Only java/lang/Object can have no superclass!"
			);
		} else {
			super_class = Some(
				self.constant_pool
					.get::<constant_pool::types::raw::RawClassName>(super_class_index),
			);
		}

		super_class.map(|class| class.name)
	}

	pub fn get_super_interfaces(&self) -> impl Iterator<Item = Cow<'_, [u1]>> {
		self.interfaces
			.iter()
			.map(|index| self.constant_pool.get::<raw_types::RawClassName>(*index))
			.map(|class| class.name)
	}

	pub fn source_file_index(&self) -> Option<u2> {
		for attr in &self.attributes {
			if let AttributeType::SourceFile(SourceFile { sourcefile_index }) = attr.info {
				return Some(sourcefile_index);
			}
		}

		None
	}

	pub fn enclosing_method(&self) -> Option<ResolvedEnclosingMethod<'_>> {
		for attr in &self.attributes {
			let Some(enclosing_method) = attr.enclosing_method() else {
				continue;
			};

			return Some(ResolvedEnclosingMethod::resolve_from(
				enclosing_method,
				&self.constant_pool,
			));
		}

		None
	}

	pub fn inner_classes(
		&self,
	) -> Option<impl ExactSizeIterator<Item = ResolvedInnerClass<'_>> + use<'_>> {
		for attr in &self.attributes {
			let Some(inner_classes) = attr.inner_classes() else {
				continue;
			};

			let iter = inner_classes.classes.iter().map(move |inner_class| {
				ResolvedInnerClass::resolve_from(inner_class, &self.constant_pool)
			});

			return Some(iter);
		}

		None
	}

	pub fn nest_host_index(&self) -> Option<u2> {
		for attr in &self.attributes {
			let Some(nest_host) = attr.nest_host() else {
				continue;
			};

			return Some(nest_host.host_class_index);
		}

		None
	}

	pub fn nest_members(&self) -> Option<impl Iterator<Item = ClassNameEntry<'_>> + use<'_>> {
		for attr in &self.attributes {
			let Some(nest_members) = attr.nest_members() else {
				continue;
			};

			let iter = nest_members
				.classes
				.iter()
				.map(|index| self.constant_pool.get::<raw_types::RawClassName>(*index));

			return Some(iter);
		}

		None
	}

	pub fn bootstrap_methods(
		&self,
	) -> Option<impl Iterator<Item = ResolvedBootstrapMethod> + use<'_>> {
		for attr in &self.attributes {
			let Some(bootstrap_methods) = attr.bootstrap_methods() else {
				continue;
			};

			let iter = bootstrap_methods
				.bootstrap_methods
				.iter()
				.map(move |bsm| ResolvedBootstrapMethod::resolve_from(bsm, &self.constant_pool));

			return Some(iter);
		}

		None
	}
}
