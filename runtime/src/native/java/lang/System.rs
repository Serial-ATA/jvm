use crate::stack::local_stack::LocalStack;

include!("def/System.registerNatives");

pub fn setIn0(_: LocalStack) {}
pub fn setOut0(_: LocalStack) {}
pub fn setErr0(_: LocalStack) {}

pub fn currentTimeMillis(_: LocalStack) {}
pub fn nanoTime(_: LocalStack) {}

pub fn arraycopy(_: LocalStack) {}

pub fn identityHashCode(_: LocalStack) {}

pub fn mapLibraryName(_: LocalStack) {}
