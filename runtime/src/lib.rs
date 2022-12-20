#![feature(lint_reasons)]

pub mod classpath;
mod frame;
mod heap;
mod interpreter;
mod method_invoker;
mod native;
pub mod stack;
mod string_interner;
mod thread;

pub use frame::Frame;
pub use heap::*;
pub use interpreter::Interpreter;
pub use thread::Thread;
