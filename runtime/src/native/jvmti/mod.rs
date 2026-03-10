//! # JVMTI Functions
//!
//! This module contains the definitions for the JVMTI functions, divided into modules as is [the specification](https://docs.oracle.com/javase/8/docs/platform/jvmti/jvmti.html).

#![allow(non_snake_case)]

mod memory_management;
mod thread;
mod thread_group;
