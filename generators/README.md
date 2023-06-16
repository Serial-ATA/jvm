# JVM/generators

This directory contains crates used for code generation.

Current crates:

* [native_methods](native_methods): Parses all `*.def` files in [runtime/native](../runtime/src/native) and generates the native method lookup table
* [vm_symbols](vm_symbols): Creates all pre-interned symbols as defined in [symbols/src/lib.rs](../symbols/src/lib.rs) and `*.def` files in [runtime/native](../runtime/src/native)