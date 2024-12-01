#![feature(c_variadic)]
#![feature(box_into_inner)]
#![feature(thread_local)]
#![feature(impl_trait_in_assoc_type)]
#![feature(macro_metavar_expr)]
#![feature(specialization)]
#![feature(sync_unsafe_cell)]

pub mod calls;
pub mod classpath;
mod error;
mod frame;
pub mod globals;
mod initialization;
mod interpreter;
mod method_invoker;
pub mod native;
mod objects;
pub mod stack;
mod string_interner;
mod thread;
pub mod verifier;

pub use frame::Frame;
pub use interpreter::Interpreter;
pub use objects::*;
pub use thread::{JVMOptions, JavaThread};
