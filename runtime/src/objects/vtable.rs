use crate::objects::method::Method;
use crate::symbols::Symbol;

use std::ops::Index;

use classfile::accessflags::MethodAccessFlags;

#[derive(Debug)]
pub struct VTable<'a> {
	methods: Box<[&'a Method]>,
	local_methods_end: usize,
}

impl<'a> IntoIterator for &VTable<'a> {
	type Item = &'a Method;
	type IntoIter = impl Iterator<Item = &'a Method>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> VTable<'a> {
	pub fn new(methods: Vec<&'a Method>, local_methods_end: usize) -> Self {
		Self {
			methods: methods.into_boxed_slice(),
			local_methods_end,
		}
	}

	/// Lookup a method in the `VTable`
	///
	/// If `flags` is [`MethodAccessFlags::NONE`], this will only compare the `name` and `descriptor`.
	///
	/// See also: [`VTable::find_local`]
	pub fn find(
		&self,
		name: Symbol,
		descriptor: Symbol,
		flags: MethodAccessFlags,
	) -> Option<&'a Method> {
		self.methods
			.iter()
			.find(|method| Self::suitable_method(method, name, descriptor, flags))
			.copied()
	}

	/// Lookup a local method in the `VTable`
	///
	/// This **will not** search super classes or interfaces.
	///
	/// If `flags` is [`MethodAccessFlags::NONE`], this will only compare the `name` and `descriptor`.
	///
	/// See also: [`VTable::find`]
	pub fn find_local(
		&self,
		name: Symbol,
		descriptor: Symbol,
		flags: MethodAccessFlags,
	) -> Option<&'a Method> {
		self.methods
			.iter()
			.take(self.local_methods_end)
			.find(|method| Self::suitable_method(method, name, descriptor, flags))
			.copied()
	}

	fn suitable_method(
		method: &'a Method,
		name: Symbol,
		descriptor: Symbol,
		flags: MethodAccessFlags,
	) -> bool {
		method.name == name
			&& (flags == MethodAccessFlags::NONE || method.access_flags & flags == flags)
			&& method.descriptor_sym() == descriptor
	}

	/// Get an iterator over all the `VTable` methods
	///
	/// NOTE: This will include all method from super classes and interfaces. See [`VTable::iter_local`]
	///       to iterate over methods *only* defined in this class.
	pub fn iter<'vtable>(&'vtable self) -> impl Iterator<Item = &'a Method> + 'vtable {
		self.methods.iter().copied()
	}

	/// Get an iterator over the local `VTable` methods
	///
	/// NOTE: This will not include methods from super classes or interfaces. See [`VTable::iter`]
	///       to iterate over all methods.
	pub fn iter_local<'vtable>(&'vtable self) -> impl Iterator<Item = &'a Method> + 'vtable {
		self.iter().take(self.local_methods_end)
	}
}

impl<'a> Index<usize> for VTable<'a> {
	type Output = Method;

	fn index(&self, index: usize) -> &Self::Output {
		self.methods[index]
	}
}
