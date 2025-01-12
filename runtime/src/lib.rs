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
mod string_interner;
pub mod thread;
pub mod verifier;

pub use interpreter::Interpreter;
