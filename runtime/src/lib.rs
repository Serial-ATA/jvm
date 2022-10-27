pub mod classpath;
mod frame;
mod heap;
mod interpreter;
pub mod stack;

pub use frame::Frame;
pub use heap::*;
pub use interpreter::Interpreter;
