#!/usr/bin/env just --justfile

# Constant VM properties

export SYSTEM_PROPS_JAVA_VERSION := "23"

export SYSTEM_PROPS_VM_SPECIFICATION_NAME := "Java Virtual Machine Specification"

export SYSTEM_PROPS_VM_NAME := "SJVM"
export SYSTEM_PROPS_VM_VENDOR := "Serial-ATA"

# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

default: debug

# Build the entire project in debug
debug: cargo build


# Build the entire project in release
release: cargo build --release

# Build and run the java binary with the provided arguments
java +ARGS: debug
    cd tools/java &&\
    cargo run -- {{ ARGS }}
