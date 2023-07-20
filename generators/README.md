# JVM/generators

This directory contains projects used for code generation.

Current crates:

* [native_methods](native_methods): Parses all `*.def` files in [runtime/native](../runtime/src/native) and generates the native method lookup table
* [vm_symbols](vm_symbols): Creates all pre-interned symbols as defined in [symbols/src/lib.rs](../symbols/src/lib.rs) and `*.def` files in [runtime/native](../runtime/src/native)

Other tools:

* [asm_specs](asm_specs): Contains multiple parsers for instruction set PDFs to automatically generate
                          information used in [assembler](../assembler).