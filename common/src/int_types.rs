//! Java specification integer types
//!
//! The Java specification uses integers specified by their size in **bytes** rather than **bits**.
//! For example [`u1`] is a [`u8`](core::primitive::u8).
//!
//! These types names are used throughout the various parsers (ex. [`JavaReadExt`](crate::traits::JavaReadExt))
//! as well as in the runtime.

#![allow(non_camel_case_types)]

pub type u1 = ::core::primitive::u8;
pub type u2 = ::core::primitive::u16;
pub type u4 = ::core::primitive::u32;
pub type u8 = ::core::primitive::u64;

pub type s1 = ::core::primitive::i8;
pub type s2 = ::core::primitive::i16;
pub type s4 = ::core::primitive::i32;
pub type s8 = ::core::primitive::i64;
