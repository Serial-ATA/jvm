//! (§4.10.1.2) Verification Type System
//!
//! The type checker enforces a type system based upon a hierarchy of verification types, illustrated below.
//!
//! ```
//! Verification type hierarchy:
//!
//!                              top
//!                  ____________/\____________
//!                 /                          \
//!                /                            \
//!             oneWord                       twoWord
//!            /   |   \                     /       \
//!           /    |    \                   /         \
//!         int  float  reference        long        double
//!                      /     \
//!                     /       \_____________
//!                    /                      \
//!                   /                        \
//!            uninitialized                    +------------------+
//!             /         \                     |  Java reference  |
//!            /           \                    |  type hierarchy  |
//! uninitializedThis  uninitialized(Offset)    +------------------+
//!                                                      |
//!                                                      |
//!                                                     null
//! ```

use crate::symbols::{sym, Symbol};

pub mod types {
	use super::VerificationType;
	use crate::symbols::Symbol;

	macro_rules! make_types {
		(
			$(
				$(#[$attr:meta])*
				$name:ident $(<$gen:ident>)? $(($($fields:ty),+))?
			),+ $(,)?
		) => {
			paste::paste! {
				$(
					$(#[$attr])*
					#[derive(Copy, Clone, Debug, PartialEq)]
					pub struct $name $(<$gen>)? $(($(pub $fields),+))?;
					pub(super) const [<$name _OFFSET>]: u8 = ${index()};
				)+
			}
		};
	}

	#[cfg_attr(rustfmt, rustfmt_skip)]
	make_types!(
		OneWord,
			// == Not real verification types, needed for array types ==
			Byte,
			Char,
			Short,
			Boolean,
			// =========================================================
			Int,
			Float,
			Reference,
				Uninitialized,
				UninitializedThis,
				UninitializedOff(usize),
				/// Class and interface types (field descriptors beginning L) correspond to verification types
				/// that use the functor `class`.
				///
				/// The verification type `class(N, L)` represents the class whose binary name is `N` as loaded by the loader `L`.
				///
				/// Note that `L` is an initiating loader (§5.3) of the class represented by `class(N, L)` and may,
				/// or may not, be the class's defining loader.
				///
				/// *For example, the class type Object would be represented as `class('java/lang/Object', BL)`, where `BL` is the bootstrap loader.*
				Class(Symbol),
				/// Array types (field descriptors beginning [) correspond to verification types that use the functor `arrayOf`.
				///
				/// Note that the primitive types `byte`, `char`, `short`, and `boolean` do not correspond to verification types,
				/// but an array type whose element type is `byte`, `char`, `short`, or `boolean` does correspond
				/// to a verification type; such verification types support the `baload`, `bastore`, `caload`, `castore`, `saload`, `sastore`, and `newarray` instructions.
				ArrayOf<T>(T),
				Null,
		TwoWord,
			Long,
			Double,
	);

	pub(super) const BITS_FOR_OFFSET: usize = 5;

	// Type erasure, convert any of the types to a `u64`.
	pub trait Erasable {
		const IDENTIFIER: u8;
		fn erase(self) -> super::VerificationType;
	}

	macro_rules! subtypes {
		($($parent:ident => [$($child:ident),+]);+ $(;)?) => {
			$($(
				paste::paste! {
					impl Erasable for $child {
						const IDENTIFIER: u8 = [<$child _OFFSET>];
						fn erase(self) -> VerificationType {
							VerificationType((Self::IDENTIFIER as u64) << (64 - BITS_FOR_OFFSET))
						}
					}
				}
			)+)+
		}
	}

	subtypes!(
		OneWord => [OneWord, Int, Float, Reference, Byte, Char, Short, Boolean];
		Reference => [Uninitialized, Null];
		Uninitialized => [UninitializedThis, UninitializedOff];
		TwoWord => [TwoWord, Long, Double];
	);

	impl Erasable for Class {
		const IDENTIFIER: u8 = Class_OFFSET;
		fn erase(self) -> VerificationType {
			VerificationType(
				((Self::IDENTIFIER as u64) << (64 - BITS_FOR_OFFSET))
					| ((self.0.as_u32() as u64) << 24),
			)
		}
	}

	impl<T: Erasable> Erasable for ArrayOf<T> {
		const IDENTIFIER: u8 = ArrayOf_OFFSET;
		fn erase(self) -> VerificationType {
			let erased_array = self.0.erase();
			VerificationType(
				((Self::IDENTIFIER as u64) << (64 - BITS_FOR_OFFSET)) | erased_array.0 << 24,
			)
		}
	}
}

#[derive(Copy, Clone)]
pub struct VerificationType(u64);

impl VerificationType {
	fn identifier(self) -> u8 {
		(self.0 >> (64 - types::BITS_FOR_OFFSET)) as u8
	}
}

/// For basic assignment checks, class assignability is implemented differently.
pub trait IsAssignable<Rhs = VerificationType> {
	fn is_assignable(self, to: Rhs) -> bool;
}

// isAssignable(oneWord, top).
// isAssignable(twoWord, top).

impl IsAssignable for types::OneWord {
	fn is_assignable(self, to: VerificationType) -> bool {
		to.identifier() == <Self as types::Erasable>::IDENTIFIER
	}
}

impl IsAssignable for types::TwoWord {
	fn is_assignable(self, to: VerificationType) -> bool {
		to.identifier() == <Self as types::Erasable>::IDENTIFIER
	}
}

// isAssignable(int, X)    :- isAssignable(oneWord, X).
// isAssignable(float, X)  :- isAssignable(oneWord, X).
// isAssignable(long, X)   :- isAssignable(twoWord, X).
// isAssignable(double, X) :- isAssignable(twoWord, X).

impl IsAssignable for types::Int {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::OneWord.is_assignable(to)
	}
}

impl IsAssignable for types::Float {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::OneWord.is_assignable(to)
	}
}

impl IsAssignable for types::Long {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::OneWord.is_assignable(to)
	}
}

impl IsAssignable for types::Double {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::OneWord.is_assignable(to)
	}
}

// isAssignable(reference, X)   :- isAssignable(oneWord, X).
// isAssignable(class(_, _), X) :- isAssignable(reference, X).
// isAssignable(arrayOf(_), X)  :- isAssignable(reference, X).

impl IsAssignable for types::Reference {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::OneWord.is_assignable(to)
	}
}

impl IsAssignable for types::Class {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::Reference.is_assignable(to)
	}
}

impl<T> IsAssignable for types::ArrayOf<T> {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::Reference.is_assignable(to)
	}
}

// isAssignable(uninitialized, X)     :- isAssignable(reference, X).
// isAssignable(uninitializedThis, X) :- isAssignable(uninitialized, X).
// isAssignable(uninitialized(_), X)  :- isAssignable(uninitialized, X).

impl IsAssignable for types::Uninitialized {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::Reference.is_assignable(to)
	}
}

impl IsAssignable for types::UninitializedThis {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::Uninitialized.is_assignable(to)
	}
}

impl IsAssignable for types::UninitializedOff {
	fn is_assignable(self, to: VerificationType) -> bool {
		types::Uninitialized.is_assignable(to)
	}
}

// isAssignable(null, class(_, _)).
// isAssignable(null, arrayOf(_)).
// isAssignable(null, X) :- isAssignable(class('java/lang/Object', BL), X),
//                          isBootstrapLoader(BL).

impl IsAssignable for types::Null {
	fn is_assignable(self, to: VerificationType) -> bool {
		to.identifier() == <types::Class as types::Erasable>::IDENTIFIER
			|| to.identifier() == <types::ArrayOf<types::Null> as types::Erasable>::IDENTIFIER
			|| (types::Class(sym!(java_lang_Object)).is_assignable(to)
				&& todo!("isBootstrapLoader"))
	}
}

// isAssignable(class(X, Lx), class(Y, Ly)) :-
//     isJavaAssignable(class(X, Lx), class(Y, Ly)).
//
// isAssignable(arrayOf(X), class(Y, L)) :-
//     isJavaAssignable(arrayOf(X), class(Y, L)).
//
// isAssignable(arrayOf(X), arrayOf(Y)) :-
//     isJavaAssignable(arrayOf(X), arrayOf(Y)).

impl IsAssignable<types::Class> for types::Class {
	fn is_assignable(self, to: types::Class) -> bool {
		self.is_java_assignable(to)
	}
}

impl<X> IsAssignable<types::Class> for types::ArrayOf<X> {
	fn is_assignable(self, to: types::Class) -> bool {
		self.is_java_assignable(to)
	}
}

impl<X, Y> IsAssignable<types::ArrayOf<Y>> for types::ArrayOf<X> {
	fn is_assignable(self, to: types::ArrayOf<Y>) -> bool {
		self.is_java_assignable(to)
	}
}

trait IsJavaAssignable<Rhs> {
	fn is_java_assignable(self, to: Rhs) -> bool;
}

// isJavaAssignable(class(_, _), class(To, L)) :-
//     loadedClass(To, L, ToClass),
//     classIsInterface(ToClass).
//
// isJavaAssignable(From, To) :-
//     isJavaSubclassOf(From, To).

impl IsJavaAssignable<types::Class> for types::Class {
	fn is_java_assignable(self, to: types::Class) -> bool {
		todo!()
	}
}

impl<X, Y> IsJavaAssignable<types::ArrayOf<Y>> for types::ArrayOf<X> {
	default fn is_java_assignable(self, _to: types::ArrayOf<Y>) -> bool {
		false
	}
}

// isJavaAssignable(arrayOf(_), class('java/lang/Object', BL)) :-
//     isBootstrapLoader(BL).
//
// isJavaAssignable(arrayOf(_), X) :-
//     isArrayInterface(X).
//
// isArrayInterface(class('java/lang/Cloneable', BL)) :-
//     isBootstrapLoader(BL).
//
// isArrayInterface(class('java/io/Serializable', BL)) :-
//     isBootstrapLoader(BL).

impl<T> IsJavaAssignable<types::Class> for types::ArrayOf<T> {
	fn is_java_assignable(self, to: types::Class) -> bool {
		(to.0 == sym!(java_lang_Object) && todo!("isBootstrapLoader")) || is_array_interface(to)
	}
}

fn is_array_interface(class: types::Class) -> bool {
	(class.0 == sym!(java_lang_Cloneable) && todo!("isBootstrapLoader"))
		|| (class.0 == sym!(java_io_Serializable) && todo!("isBootstrapLoader"))
}

// Subtyping between arrays of primitive type is the identity relation.
//
// isJavaAssignable(arrayOf(X), arrayOf(Y)) :-
//     atom(X),
//     atom(Y),
//     X = Y.

macro_rules! primitive_arrays {
	($($ty:path),+) => {
		$(
		impl IsJavaAssignable<types::ArrayOf<$ty>> for types::ArrayOf<$ty> {
			fn is_java_assignable(self, to: types::ArrayOf<$ty>) -> bool {
				true
			}
		}
		)+
	};
}

primitive_arrays!(types::Int, types::Float, types::Long, types::Double);

// Subtyping between arrays of reference type is covariant.
//
// isJavaAssignable(arrayOf(X), arrayOf(Y)) :-
//     compound(X), compound(Y), isJavaAssignable(X, Y).

impl<X, Y> IsJavaAssignable<types::ArrayOf<types::ArrayOf<Y>>>
	for types::ArrayOf<types::ArrayOf<X>>
{
	fn is_java_assignable(self, to: types::ArrayOf<types::ArrayOf<Y>>) -> bool {
		self.0.is_java_assignable(to.0)
	}
}

impl<X> IsJavaAssignable<types::ArrayOf<types::Class>> for types::ArrayOf<types::ArrayOf<X>> {
	fn is_java_assignable(self, to: types::ArrayOf<types::Class>) -> bool {
		self.0.is_java_assignable(to.0)
	}
}

impl IsJavaAssignable<types::ArrayOf<types::Class>> for types::ArrayOf<types::Class> {
	fn is_java_assignable(self, to: types::ArrayOf<types::Class>) -> bool {
		self.0.is_java_assignable(to.0)
	}
}

// Subclassing is reflexive.
//
// isJavaSubclassOf(class(SubclassName, L), class(SubclassName, L)).
//
// isJavaSubclassOf(class(SubclassName, LSub), class(SuperclassName, LSuper)) :-
//     superclassChain(SubclassName, LSub, Chain),
//     member(class(SuperclassName, L), Chain),
//     loadedClass(SuperclassName, L, Sup),
//     loadedClass(SuperclassName, LSuper, Sup).
//
// superclassChain(ClassName, L, [class(SuperclassName, Ls) | Rest]) :-
//     loadedClass(ClassName, L, Class),
//     classSuperClassName(Class, SuperclassName),
//     classDefiningLoader(Class, Ls),
//     superclassChain(SuperclassName, Ls, Rest).
//
// superclassChain('java/lang/Object', L, []) :-
//     loadedClass('java/lang/Object', L, Class),
//     classDefiningLoader(Class, BL),
//     isBootstrapLoader(BL).

impl types::Class {
	fn is_java_subclass_of(self, other: types::Class) -> bool {
		self == other || todo!()
	}
}
