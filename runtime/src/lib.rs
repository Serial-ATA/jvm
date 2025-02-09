#![feature(c_variadic)]
#![feature(box_into_inner)]
#![feature(thread_local)]
#![feature(impl_trait_in_assoc_type)]
#![feature(macro_metavar_expr)]
#![feature(specialization)]
#![feature(sync_unsafe_cell)]
#![feature(core_intrinsics)]
#![feature(try_with_capacity)]
#![feature(array_chunks)]
#![feature(try_trait_v2)]
#![feature(associated_type_defaults)]
#![feature(hash_raw_entry)]
#![feature(ptr_as_ref_unchecked)]
extern crate core;

pub mod calls;
pub mod classpath;
pub mod error;
pub mod globals;
mod initialization;
mod interpreter;
mod method_invoker;
pub mod modules;
pub mod native;
pub mod objects;
pub mod stack;
mod symbols;
pub mod thread;
pub mod verifier;

pub use interpreter::Interpreter;
