use crate::classpath::classloader::ClassLoader;
use crate::reference::{ArrayInstanceRef, ClassInstanceRef, ClassRef, Reference};

use std::fmt::{Debug, Formatter};

use common::int_types::{s1, s2, s4, s8, u1, u2};
use common::traits::PtrType;
use instructions::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
	pub class: ClassRef,
	pub fields: Box<[Operand<Reference>]>,
}

impl ClassInstance {
	pub fn new(class: ClassRef) -> ClassInstanceRef {
		let class_instance = class.unwrap_class_instance();
		let field_count = class_instance.instance_field_count;

		let fields = vec![Operand::Empty; field_count as usize].into_boxed_slice();

		ClassInstancePtr::new(Self { class, fields })
	}
}

// A pointer to a ClassInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct ClassInstancePtr(usize);

impl PtrType<ClassInstance, ClassInstanceRef> for ClassInstancePtr {
	fn new(val: ClassInstance) -> ClassInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ClassInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const ClassInstance {
		self.0 as *const ClassInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut ClassInstance {
		self.0 as *mut ClassInstance
	}

	fn get(&self) -> &ClassInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut ClassInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ClassInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut ClassInstance) };
	}
}

impl Debug for ClassInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(unsafe { std::str::from_utf8_unchecked(&class.class.get().name) })
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInstance {
	pub class: ClassRef,
	pub elements: ArrayContent,
}

impl ArrayInstance {
	pub fn new(class: ClassRef, elements: ArrayContent) -> ArrayInstanceRef {
		ArrayInstanceRef::new(Self { class, elements })
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.newarray
	pub fn new_from_type(type_code: u1, count: s4) -> ArrayInstanceRef {
		if count.is_negative() {
			panic!("NegativeArraySizeException"); // TODO
		}

		let type_character = match type_code {
			4 => b'Z',
			5 => b'C',
			6 => b'F',
			7 => b'D',
			8 => b'B',
			9 => b'S',
			10 => b'I',
			11 => b'J',
			_ => panic!("Invalid array type code: {}", type_code),
		};

		let array_class = ClassLoader::Bootstrap
			.load(&[b'[', type_character])
			.unwrap();
		let elements = ArrayContent::default_initialize(type_code, count);

		ArrayInstanceRef::new(Self {
			class: array_class,
			elements,
		})
	}

	pub fn get(&self, index: s4) -> Operand<Reference> {
		if index.is_negative() || index as usize > self.elements.element_count() {
			panic!("ArrayIndexOutOfBoundsException"); // TODO
		}

		self.elements.get(index as usize)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayContent {
	Byte(Box<[s1]>),
	Bool(Box<[s1]>),
	Short(Box<[s2]>),
	Char(Box<[u2]>),
	Int(Box<[s4]>),
	Float(Box<[f32]>),
	Double(Box<[f64]>),
	Long(Box<[s8]>),
}

impl ArrayContent {
	fn default_initialize(type_code: u1, count: s4) -> Self {
		match type_code {
			4 => Self::Bool(vec![0; count as usize].into_boxed_slice()),
			5 => Self::Char(vec![0; count as usize].into_boxed_slice()),
			6 => Self::Float(vec![0.; count as usize].into_boxed_slice()),
			7 => Self::Double(vec![0.; count as usize].into_boxed_slice()),
			8 => Self::Byte(vec![0; count as usize].into_boxed_slice()),
			9 => Self::Short(vec![0; count as usize].into_boxed_slice()),
			10 => Self::Int(vec![0; count as usize].into_boxed_slice()),
			11 => Self::Long(vec![0; count as usize].into_boxed_slice()),
			_ => panic!("Invalid array type code: {}", type_code),
		}
	}

	fn get(&self, index: usize) -> Operand<Reference> {
		match self {
			ArrayContent::Byte(content) | ArrayContent::Bool(content) => {
				Operand::Int(s4::from(content[index]))
			},
			ArrayContent::Short(content) => Operand::Int(s4::from(content[index])),
			ArrayContent::Char(content) => Operand::Int(s4::from(content[index])),
			ArrayContent::Int(content) => Operand::Int(content[index]),
			ArrayContent::Float(content) => Operand::Float(content[index]),
			ArrayContent::Double(content) => Operand::Double(content[index]),
			ArrayContent::Long(content) => Operand::Long(content[index]),
		}
	}

	pub fn element_count(&self) -> usize {
		match self {
			ArrayContent::Byte(content) | ArrayContent::Bool(content) => content.len(),
			ArrayContent::Short(content) => content.len(),
			ArrayContent::Char(content) => content.len(),
			ArrayContent::Int(content) => content.len(),
			ArrayContent::Float(content) => content.len(),
			ArrayContent::Double(content) => content.len(),
			ArrayContent::Long(content) => content.len(),
		}
	}
}
