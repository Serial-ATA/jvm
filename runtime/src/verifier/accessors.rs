//! (§4.10.1.1) Accessors for Java Virtual Machine Artifacts

use super::type_system::VerificationType;
use crate::class::Class;
use crate::method::Method;
use crate::vtable::VTable;

use classfile::accessflags::MethodAccessFlags;
use classfile::{Attribute, MethodDescriptor};
use symbols::{sym, Symbol};

pub(super) trait ClassAccessorExt {
	/// Extracts the name, `ClassName`, of the class `Class`.
	fn class_name(&self) -> Symbol;
	/// True iff the class, `Class`, is an interface.
	fn is_interface(&self) -> bool;
	/// True iff the class, `Class`, is not a final class.
	fn is_not_final(&self) -> bool;
	/// Extracts the name, `SuperClassName`, of the superclass of class `Class`.
	fn super_class_name(&self) -> Symbol;
	/// Extracts a list, `Interfaces`, of the direct superinterfaces of the class `Class`.
	fn interfaces(&self) -> &[&'static Class];
	/// Extracts a list, `Methods`, of the methods declared in the class `Class`.
	fn methods(&self) -> &VTable<'_>;
	/// Extracts a list, `Attributes`, of the attributes of the class `Class`.
	fn attributes(&self) -> &[Attribute];
	/// Extracts the defining class loader, `Loader`, of the class `Class`.
	fn defining_loader(&self) -> &'static Class;
	/// True iff the class loader `Loader` is the bootstrap class loader.
	fn is_bootstrap_loader(&self) -> bool;
	/// True iff there exists a class named `Name` whose representation (in accordance with this specification) when loaded by the class loader `InitiatingLoader` is `ClassDefinition`.
	fn loaded_class(&self) -> bool;
	/// True iff the package names of `self` and `other` are the same.
	fn same_package_name(&self, other: &Class) -> bool;
	/// True iff the package names of `self` and `other` are different.
	fn different_package_name(&self, other: &Class) -> bool;
}

impl ClassAccessorExt for Class {
	#[inline]
	fn class_name(&self) -> Symbol {
		self.name
	}

	#[inline]
	fn is_interface(&self) -> bool {
		self.is_interface()
	}

	#[inline]
	fn is_not_final(&self) -> bool {
		!self.access_flags.is_final()
	}

	fn super_class_name(&self) -> Symbol {
		assert!(
			self.super_class.is_some(),
			"This should never be called on java.lang.Object"
		);
		self.super_class.as_ref().unwrap().name
	}

	fn interfaces(&self) -> &[&'static Class] {
		todo!()
	}

	fn methods(&self) -> &VTable<'_> {
		self.vtable()
	}

	fn attributes(&self) -> &[Attribute] {
		todo!()
	}

	fn defining_loader(&self) -> &'static Class {
		todo!()
	}

	fn is_bootstrap_loader(&self) -> bool {
		todo!()
	}

	fn loaded_class(&self) -> bool {
		todo!()
	}

	#[inline]
	fn same_package_name(&self, other: &Class) -> bool {
		self.shares_package_with(other)
	}

	#[inline]
	fn different_package_name(&self, other: &Class) -> bool {
		!self.same_package_name(other)
	}
}

pub(super) trait MethodAccessorExt {
	/// Extracts the name, `Name`, of the method `Method`.
	fn name(&self) -> Symbol;
	/// Extracts the access flags, `AccessFlags`, of the method `Method`.
	fn access_flags(&self) -> MethodAccessFlags;
	/// Extracts the descriptor, `Descriptor`, of the method `Method`.
	fn descriptor(&self) -> &MethodDescriptor;
	/// Extracts a list, `Attributes`, of the attributes of the method `Method`.
	fn attributes(&self) -> &[Attribute];
	/// True iff `Method` (regardless of class) is `<init>`.
	fn is_init(&self) -> bool;
	/// True iff `Method` (regardless of class) is not `<init>`.
	fn is_not_init(&self) -> bool;
	/// True iff Method in class `Class` is not final.
	fn is_not_final(&self, class: &'static Class) -> bool;
	/// True iff Method in class `Class` is static.
	fn is_static(&self, class: &'static Class) -> bool;
	/// True iff Method in class `Class` is not static.
	fn is_not_static(&self, class: &'static Class) -> bool;
	/// True iff Method in class `Class` is private.
	fn is_private(&self, class: &'static Class) -> bool;
	/// True iff Method in class `Class` is not private.
	fn is_not_private(&self, class: &'static Class) -> bool;
	/// True iff there is a member named `MemberName` with descriptor `MemberDescriptor` in the class `MemberClass` and it is protected.
	fn is_protected(
		&self,
		member_class: &'static Class,
		member_name: Symbol,
		member_descriptor: Symbol,
	) -> bool;
	/// True iff there is a member named `MemberName` with descriptor `MemberDescriptor` in the class `MemberClass` and it is not protected.
	fn is_not_protected(
		&self,
		member_class: &'static Class,
		member_name: Symbol,
		member_descriptor: Symbol,
	) -> bool;
}

impl MethodAccessorExt for Method {
	#[inline]
	fn name(&self) -> Symbol {
		self.name
	}

	#[inline]
	fn access_flags(&self) -> MethodAccessFlags {
		self.access_flags
	}

	fn descriptor(&self) -> &MethodDescriptor {
		todo!()
	}

	#[inline]
	fn attributes(&self) -> &[Attribute] {
		self.attributes()
	}

	#[inline]
	fn is_init(&self) -> bool {
		self.name == sym!(object_initializer_name)
	}

	#[inline]
	fn is_not_init(&self) -> bool {
		!self.is_init()
	}

	fn is_not_final(&self, class: &'static Class) -> bool {
		todo!()
	}

	fn is_static(&self, class: &'static Class) -> bool {
		todo!()
	}

	fn is_not_static(&self, class: &'static Class) -> bool {
		todo!()
	}

	fn is_private(&self, class: &'static Class) -> bool {
		todo!()
	}

	fn is_not_private(&self, class: &'static Class) -> bool {
		todo!()
	}

	fn is_protected(
		&self,
		member_class: &'static Class,
		member_name: Symbol,
		member_descriptor: Symbol,
	) -> bool {
		todo!()
	}

	fn is_not_protected(
		&self,
		member_class: &'static Class,
		member_name: Symbol,
		member_descriptor: Symbol,
	) -> bool {
		todo!()
	}
}

pub(super) fn parse_field_descriptor(descriptor: Symbol) -> VerificationType {
	todo!("parse_field_descriptor")
}

pub(super) fn parse_method_descriptor(
	descriptor: Symbol,
	arg_type_list: &[VerificationType],
) -> VerificationType {
	todo!("parse_method_descriptor")
}