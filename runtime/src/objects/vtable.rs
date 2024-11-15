use crate::method::Method;

use classfile::accessflags::MethodAccessFlags;
use symbols::Symbol;

#[derive(Debug)]
pub struct VTable<'a> {
	methods: Box<[&'a Method]>,
}

impl<'a> From<Vec<&'a Method>> for VTable<'a> {
	fn from(methods: Vec<&'a Method>) -> Self {
		Self {
			methods: methods.into_boxed_slice(),
		}
	}
}

impl<'a> IntoIterator for &VTable<'a> {
	type Item = &'a Method;
	type IntoIter = impl Iterator<Item = &'a Method>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> VTable<'a> {
	/// Lookup a method in the `VTable`
	///
	/// If `flags` is [`MethodAccessFlags::NONE`], this will only compare the `name` and `descriptor`.
	pub fn find(
		&self,
		name: Symbol,
		descriptor: Symbol,
		flags: MethodAccessFlags,
	) -> Option<&'a Method> {
		if let Some(method) = self.methods.iter().find(|method| {
			method.name == name
				&& (flags == MethodAccessFlags::NONE || method.access_flags & flags == flags)
				&& method.descriptor == descriptor
		}) {
			return Some(method);
		}
		None
	}

	/// Get an iterator over all the `VTable` methods
	pub fn iter<'vtable>(&'vtable self) -> impl Iterator<Item = &'a Method> + 'vtable {
		self.methods.iter().copied()
	}
}
