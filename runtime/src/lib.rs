#![feature(lint_reasons)]

pub mod classpath;
mod error;
mod frame;
pub mod globals;
mod heap;
mod initialization;
mod interpreter;
mod method_invoker;
pub mod native;
pub mod stack;
mod string_interner;
mod thread;

pub use frame::Frame;
pub use heap::*;
pub use interpreter::Interpreter;
pub use thread::{JVMOptions, Thread};
