#![feature(c_variadic)]
#![feature(box_into_inner)]
#![feature(thread_local)]
#![feature(impl_trait_in_assoc_type)]
#![feature(macro_metavar_expr)]
#![feature(specialization)]
#![feature(sync_unsafe_cell)]
#![feature(try_with_capacity)]
#![feature(array_chunks)]
#![feature(try_trait_v2)]
#![feature(associated_type_defaults)]
#![feature(ptr_as_ref_unchecked)]
#![feature(iter_next_chunk)]
#![feature(if_let_guard)]
#![feature(reentrant_lock)]
#![feature(std_internals)]

pub mod calls;
pub mod classes;
pub mod classpath;
mod dynamic;
pub mod error;
pub mod globals;
mod initialization;
mod interpreter;
mod method_invoker;
pub mod modules;
pub mod native;
pub mod objects;
pub mod options;
pub mod stack;
mod symbols;
pub mod thread;
pub mod verifier;

pub use interpreter::Interpreter;
