use crate::method::Method;

use classfile::accessflags::MethodAccessFlags;
use symbols::Symbol;

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
			&& method.descriptor == descriptor
	}

	/// Get an iterator over all the `VTable` methods
	pub fn iter<'vtable>(&'vtable self) -> impl Iterator<Item = &'a Method> + 'vtable {
		self.methods.iter().copied()
	}
}
