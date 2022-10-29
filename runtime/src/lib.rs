pub mod classpath;
mod frame;
mod heap;
mod interpreter;
pub mod stack;
mod native;

pub use frame::Frame;
pub use heap::*;
pub use interpreter::Interpreter;
