use super::{ConstantPool, ConstantPoolValueInfo};

use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use common::int_types::{s4, s8, u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4-310
#[derive(Clone, Debug)]
pub enum LoadableConstantPoolValueInner<'a> {
	Integer(s4),
	Float(f32),
	Long(s8),
	Double(f64),
	Class(<raw::RawClassName as CpEntry<'a>>::Entry),
	String(<raw::RawString as CpEntry<'a>>::Entry),
	MethodHandle(<raw::RawMethodHandle as CpEntry<'a>>::Entry),
	MethodType(<raw::RawConstantUtf8 as CpEntry<'a>>::Entry),
	Dynamic(<raw::RawDynamic as CpEntry<'a>>::Entry),
}

impl<'a> LoadableConstantPoolValueInner<'a> {
	pub fn into_owned(self) -> LoadableConstantPoolValueInner<'static> {
		match self {
			LoadableConstantPoolValueInner::Integer(val) => {
				LoadableConstantPoolValueInner::Integer(val)
			},
			LoadableConstantPoolValueInner::Float(val) => {
				LoadableConstantPoolValueInner::Float(val)
			},
			LoadableConstantPoolValueInner::Long(val) => LoadableConstantPoolValueInner::Long(val),
			LoadableConstantPoolValueInner::Double(val) => {
				LoadableConstantPoolValueInner::Double(val)
			},
			LoadableConstantPoolValueInner::Class(val) => {
				LoadableConstantPoolValueInner::Class(val.into_owned())
			},
			LoadableConstantPoolValueInner::String(val) => {
				LoadableConstantPoolValueInner::String(Cow::Owned(val.into_owned()))
			},
			LoadableConstantPoolValueInner::MethodHandle(val) => {
				LoadableConstantPoolValueInner::MethodHandle(val.into_owned())
			},
			LoadableConstantPoolValueInner::MethodType(val) => {
				LoadableConstantPoolValueInner::MethodType(Cow::Owned(val.into_owned()))
			},
			LoadableConstantPoolValueInner::Dynamic(val) => {
				LoadableConstantPoolValueInner::Dynamic(val.into_owned())
			},
		}
	}
}

#[derive(Clone, Debug)]
pub struct LoadableConstantPoolValue<'a> {
	pub index: u2,
	pub value: LoadableConstantPoolValueInner<'a>,
}

impl<'a> LoadableConstantPoolValue<'a> {
	pub fn into_owned(self) -> LoadableConstantPoolValue<'static> {
		LoadableConstantPoolValue {
			index: self.index,
			value: self.value.into_owned(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct FieldRefEntry<'a> {
	pub class_index: u2,
	pub class: <raw::RawClassName as super::CpEntry<'a>>::Entry,
	pub name_and_type_index: u2,
	pub name_and_type: <raw::RawNameAndType as super::CpEntry<'a>>::Entry,
}

impl<'a> FieldRefEntry<'a> {
	pub fn into_owned(self) -> FieldRefEntry<'static> {
		FieldRefEntry {
			class_index: self.class_index,
			class: self.class.into_owned(),
			name_and_type_index: self.name_and_type_index,
			name_and_type: self.name_and_type.into_owned(),
		}
	}
}

/// The type of a method handle
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReferenceKind {
	/// REF_getField
	GetField = 1,
	/// REF_getStatic
	GetStatic = 2,
	/// REF_putField
	PutField = 3,
	/// REF_putStatic
	PutStatic = 4,
	/// REF_invokeVirtual
	InvokeVirtual = 5,
	/// REF_invokeStatic
	InvokeStatic = 6,
	/// REF_invokeSpecial
	InvokeSpecial = 7,
	/// REF_newInvokeSpecial
	NewInvokeSpecial = 8,
	/// REF_invokeInterface
	InvokeInterface = 9,
}

impl ReferenceKind {
	pub fn from_u8(value: u8) -> Option<ReferenceKind> {
		match value {
			1 => Some(ReferenceKind::GetField),
			2 => Some(ReferenceKind::GetStatic),
			3 => Some(ReferenceKind::PutField),
			4 => Some(ReferenceKind::PutStatic),
			5 => Some(ReferenceKind::InvokeVirtual),
			6 => Some(ReferenceKind::InvokeStatic),
			7 => Some(ReferenceKind::InvokeSpecial),
			8 => Some(ReferenceKind::NewInvokeSpecial),
			9 => Some(ReferenceKind::InvokeInterface),
			_ => None,
		}
	}

	pub fn is_field(self) -> bool {
		matches!(
			self,
			ReferenceKind::GetField
				| ReferenceKind::GetStatic
				| ReferenceKind::PutField
				| ReferenceKind::PutStatic
		)
	}

	pub fn is_method(self) -> bool {
		matches!(
			self,
			ReferenceKind::InvokeVirtual
				| ReferenceKind::InvokeStatic
				| ReferenceKind::InvokeSpecial
				| ReferenceKind::InvokeInterface
		)
	}

	pub fn is_constructor(self) -> bool {
		self == ReferenceKind::NewInvokeSpecial
	}
}

#[derive(Clone, Debug)]
pub enum ReferenceEntry<'a> {
	FieldRef(<raw::RawFieldRef as CpEntry<'a>>::Entry),
	MethodRef(<raw::RawMethodRef as CpEntry<'a>>::Entry),
}

impl<'a> ReferenceEntry<'a> {
	pub fn into_owned(self) -> ReferenceEntry<'static> {
		match self {
			ReferenceEntry::FieldRef(field) => ReferenceEntry::FieldRef(field.into_owned()),
			ReferenceEntry::MethodRef(method) => ReferenceEntry::MethodRef(method.into_owned()),
		}
	}
}

#[derive(Clone)]
pub struct ClassNameEntry<'a> {
	pub name_index: u2,
	pub name: <raw::RawConstantUtf8 as CpEntry<'a>>::Entry,
}

impl Debug for ClassNameEntry<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("ClassNameEntry")
			.field(&String::from_utf8_lossy(&self.name))
			.finish()
	}
}

impl<'a> ClassNameEntry<'a> {
	pub fn into_owned(self) -> ClassNameEntry<'static> {
		ClassNameEntry {
			name_index: self.name_index,
			name: Cow::Owned(self.name.into_owned()),
		}
	}
}

#[derive(Clone, Debug)]
pub struct MethodHandleEntry<'a> {
	pub reference_kind: ReferenceKind,
	pub reference_index: u2,
	pub reference: ReferenceEntry<'a>,
}

impl<'a> MethodHandleEntry<'a> {
	pub fn into_owned(self) -> MethodHandleEntry<'static> {
		MethodHandleEntry {
			reference_kind: self.reference_kind,
			reference_index: self.reference_index,
			reference: self.reference.into_owned(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct MethodRefEntry<'a> {
	pub is_interface: bool,
	pub class_index: u2,
	pub class: <raw::RawClassName as CpEntry<'a>>::Entry,
	pub name_and_type_index: u2,
	pub name_and_type: <raw::RawNameAndType as CpEntry<'a>>::Entry,
}

impl<'a> MethodRefEntry<'a> {
	pub fn into_owned(self) -> MethodRefEntry<'static> {
		MethodRefEntry {
			is_interface: self.is_interface,
			class_index: self.class_index,
			class: self.class.into_owned(),
			name_and_type_index: self.name_and_type_index,
			name_and_type: self.name_and_type.into_owned(),
		}
	}
}

#[derive(Clone)]
pub struct NameAndTypeEntry<'a> {
	pub name_index: u2,
	pub name: <raw::RawConstantUtf8 as CpEntry<'a>>::Entry,
	pub descriptor_index: u2,
	pub descriptor: <raw::RawConstantUtf8 as CpEntry<'a>>::Entry,
}

impl Debug for NameAndTypeEntry<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("NameAndTypeEntry")
			.field(&String::from_utf8_lossy(&self.name))
			.field(&String::from_utf8_lossy(&self.descriptor))
			.finish()
	}
}

impl<'a> NameAndTypeEntry<'a> {
	pub fn into_owned(self) -> NameAndTypeEntry<'static> {
		NameAndTypeEntry {
			name_index: self.name_index,
			name: Cow::Owned(self.name.into_owned()),
			descriptor_index: self.descriptor_index,
			descriptor: Cow::Owned(self.descriptor.into_owned()),
		}
	}
}

#[derive(Clone, Debug)]
pub struct InvokeDynamicEntry<'a> {
	pub bootstrap_method_attr_index: u2,
	pub name_and_type_index: u2,
	pub name_and_type: <raw::RawNameAndType as CpEntry<'a>>::Entry,
}

impl<'a> InvokeDynamicEntry<'a> {
	pub fn into_owned(self) -> InvokeDynamicEntry<'static> {
		InvokeDynamicEntry {
			bootstrap_method_attr_index: self.bootstrap_method_attr_index,
			name_and_type_index: self.name_and_type_index,
			name_and_type: self.name_and_type.into_owned(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct DynamicEntry<'a> {
	pub bootstrap_method_attr_index: u2,
	pub name_and_type_index: u2,
	pub name_and_type: <raw::RawNameAndType as CpEntry<'a>>::Entry,
}

impl<'a> DynamicEntry<'a> {
	pub fn into_owned(self) -> DynamicEntry<'static> {
		DynamicEntry {
			bootstrap_method_attr_index: self.bootstrap_method_attr_index,
			name_and_type_index: self.name_and_type_index,
			name_and_type: self.name_and_type.into_owned(),
		}
	}
}

pub trait CpEntry<'a> {
	type Entry;
	type HandleArgs;

	fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry;

	fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry;
}

pub mod raw {
	use super::*;
	use std::borrow::Cow;

	pub struct Integer;

	impl super::CpEntry<'_> for Integer {
		type Entry = s4;
		type HandleArgs = u4;

		fn get(cp: &ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Integer { bytes } => Self::handle(cp, *bytes),
				_ => panic!("Expected a constant value of \"Integer\""),
			}
		}

		#[inline]
		fn handle(_: &ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			args as s4
		}
	}

	pub struct Float;

	impl super::CpEntry<'_> for Float {
		type Entry = f32;
		type HandleArgs = u4;

		fn get(cp: &ConstantPool, index: u2) -> f32 {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Float { bytes } => Self::handle(cp, *bytes),
				_ => panic!("Expected a constant value of \"Float\""),
			}
		}

		#[inline]
		fn handle(_: &ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			f32::from_bits(args)
		}
	}

	pub struct Long;

	impl super::CpEntry<'_> for Long {
		type Entry = s8;
		type HandleArgs = (u4, u4);

		fn get(cp: &ConstantPool, index: u2) -> s8 {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Long {
					high_bytes,
					low_bytes,
				} => Self::handle(cp, (*high_bytes, *low_bytes)),
				_ => panic!("Expected a constant value of \"Long\""),
			}
		}

		#[inline]
		fn handle(_: &ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			(s8::from(args.0) << 32) + s8::from(args.1)
		}
	}

	pub struct Double;

	impl super::CpEntry<'_> for Double {
		type Entry = f64;
		type HandleArgs = (u4, u4);

		fn get(cp: &ConstantPool, index: u2) -> f64 {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Double {
					high_bytes,
					low_bytes,
				} => Self::handle(cp, (*high_bytes, *low_bytes)),
				_ => panic!("Expected a constant value of \"Double\""),
			}
		}

		#[inline]
		fn handle(_: &ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			let high = (args.0 as u64) << 32;
			let low = args.1;
			f64::from_bits(high | low as u64)
		}
	}

	pub struct RawClassName;

	impl<'a> super::CpEntry<'a> for RawClassName {
		type Entry = ClassNameEntry<'a>;
		type HandleArgs = u2;

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Class { name_index } => Self::handle(cp, *name_index),
				_ => panic!("Expected a constant value of \"Class\""),
			}
		}

		#[inline]
		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			ClassNameEntry {
				name_index: args,
				name: cp.get::<RawConstantUtf8>(args),
			}
		}
	}

	pub struct RawFieldRef;

	impl<'a> super::CpEntry<'a> for RawFieldRef {
		type Entry = FieldRefEntry<'a>;
		type HandleArgs = (u2, u2);

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Fieldref {
					class_index,
					name_and_type_index,
				} => Self::handle(cp, (*class_index, *name_and_type_index)),
				_ => panic!("Expected a constant value of \"Fieldref\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			FieldRefEntry {
				class_index: args.0,
				class: cp.get::<RawClassName>(args.0),
				name_and_type_index: args.1,
				name_and_type: cp.get::<RawNameAndType>(args.1),
			}
		}
	}

	pub struct RawMethodRef;

	impl<'a> super::CpEntry<'a> for RawMethodRef {
		type Entry = MethodRefEntry<'a>;
		type HandleArgs = (bool, u2, u2);

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Methodref {
					class_index,
					name_and_type_index,
				} => Self::handle(cp, (false, *class_index, *name_and_type_index)),
				ConstantPoolValueInfo::InterfaceMethodref {
					class_index,
					name_and_type_index,
				} => Self::handle(cp, (true, *class_index, *name_and_type_index)),
				_ => panic!("Expected a constant value of \"Methodref\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			MethodRefEntry {
				is_interface: args.0,
				class_index: args.1,
				class: cp.get::<RawClassName>(args.1),
				name_and_type_index: args.2,
				name_and_type: cp.get::<RawNameAndType>(args.2),
			}
		}
	}

	pub struct RawConstantUtf8;

	impl<'a> super::CpEntry<'a> for RawConstantUtf8 {
		type Entry = Cow<'a, [u1]>;
		type HandleArgs = &'a [u1];

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Utf8 { bytes, .. } => Self::handle(cp, bytes),
				_ => panic!("Expected a constant value of \"Utf8\""),
			}
		}

		#[inline]
		fn handle(_: &ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			Cow::Borrowed(args)
		}
	}

	pub struct RawString;

	impl<'a> super::CpEntry<'a> for RawString {
		type Entry = <RawConstantUtf8 as super::CpEntry<'a>>::Entry;
		type HandleArgs = u2;

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::String { string_index } => Self::handle(cp, *string_index),
				_ => panic!("Expected a constant value of \"String\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			cp.get::<RawConstantUtf8>(args)
		}
	}

	pub struct RawNameAndType;

	impl<'a> super::CpEntry<'a> for RawNameAndType {
		type Entry = NameAndTypeEntry<'a>;
		type HandleArgs = (u2, u2);

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::NameAndType {
					name_index,
					descriptor_index,
				} => Self::handle(cp, (*name_index, *descriptor_index)),
				_ => panic!("Expected a constant value of \"NameAndType\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			NameAndTypeEntry {
				name_index: args.0,
				name: cp.get::<RawConstantUtf8>(args.0),
				descriptor_index: args.1,
				descriptor: cp.get::<RawConstantUtf8>(args.1),
			}
		}
	}

	pub struct RawMethodHandle;

	impl<'a> super::CpEntry<'a> for RawMethodHandle {
		type Entry = super::MethodHandleEntry<'a>;
		type HandleArgs = (u1, u2);

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::MethodHandle {
					reference_kind: kind,
					reference_index: index,
				} => Self::handle(cp, (*kind, *index)),
				_ => panic!("Expected a constant value of \"MethodHandle\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			let reference_kind = args.0;
			let reference_index = args.1;

			let reference_kind =
				ReferenceKind::from_u8(reference_kind).expect("valid reference type");
			let reference = match reference_kind {
				ReferenceKind::GetField
				| ReferenceKind::GetStatic
				| ReferenceKind::PutField
				| ReferenceKind::PutStatic => ReferenceEntry::FieldRef(cp.get::<RawFieldRef>(reference_index)),
				ReferenceKind::InvokeVirtual
				| ReferenceKind::NewInvokeSpecial
				| ReferenceKind::InvokeStatic
				| ReferenceKind::InvokeSpecial => {
					ReferenceEntry::MethodRef(cp.get::<RawMethodRef>(reference_index))
				},
				ReferenceKind::InvokeInterface => {
					let entry = cp.get::<RawMethodRef>(reference_index);
					assert!(entry.is_interface, "method must be interface method ref");
					ReferenceEntry::MethodRef(entry)
				},
			};

			super::MethodHandleEntry {
				reference_kind,
				reference_index,
				reference,
			}
		}
	}

	pub struct RawMethodType;

	impl<'a> super::CpEntry<'a> for RawMethodType {
		type Entry = <RawConstantUtf8 as super::CpEntry<'a>>::Entry;
		type HandleArgs = u2;

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::MethodType { descriptor_index } => {
					Self::handle(cp, *descriptor_index)
				},
				_ => panic!("Expected a constant value of \"MethodType\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			cp.get::<RawConstantUtf8>(args)
		}
	}

	pub struct RawInvokeDynamic;

	impl<'a> super::CpEntry<'a> for RawInvokeDynamic {
		type Entry = InvokeDynamicEntry<'a>;
		type HandleArgs = (u2, u2);

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::InvokeDynamic {
					bootstrap_method_attr_index,
					name_and_type_index,
				} => Self::handle(cp, (*bootstrap_method_attr_index, *name_and_type_index)),
				_ => panic!("Expected a constant value of \"InvokeDynamic\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			InvokeDynamicEntry {
				bootstrap_method_attr_index: args.0,
				name_and_type_index: args.1,
				name_and_type: cp.get::<RawNameAndType>(args.1),
			}
		}
	}

	pub struct RawDynamic;

	impl<'a> super::CpEntry<'a> for RawDynamic {
		type Entry = DynamicEntry<'a>;
		type HandleArgs = (u2, u2);

		fn get(cp: &'a ConstantPool, index: u2) -> Self::Entry {
			let constant = &cp[index];

			match constant {
				ConstantPoolValueInfo::Dynamic {
					bootstrap_method_attr_index,
					name_and_type_index,
				} => Self::handle(cp, (*bootstrap_method_attr_index, *name_and_type_index)),
				_ => panic!("Expected a constant value of \"Dynamic\""),
			}
		}

		fn handle(cp: &'a ConstantPool, args: Self::HandleArgs) -> Self::Entry {
			DynamicEntry {
				bootstrap_method_attr_index: args.0,
				name_and_type_index: args.1,
				name_and_type: cp.get::<RawNameAndType>(args.1),
			}
		}
	}
}
