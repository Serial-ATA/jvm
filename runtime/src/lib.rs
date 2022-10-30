pub mod classpath;
mod frame;
mod heap;
mod interpreter;
mod native;
pub mod stack;

pub use frame::Frame;
pub use heap::*;
pub use interpreter::Interpreter;
