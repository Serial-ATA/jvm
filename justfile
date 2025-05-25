#!/usr/bin/env just --justfile
# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

default: debug

# Build the entire project in debug
debug:
    cargo +nightly -Z unstable-options build

# Build the entire project in release
release:
    cargo +nightly -Z unstable-options build --release

native-debug:
    cargo +nightly -Z unstable-options build -p native-meta

native-release:
    cargo +nightly -Z unstable-options build --release -p native-meta

native: native-debug

dist *ARGS:
    python3 {{ justfile_directory() }}/build/entry.py {{ ARGS }}

# Build and run the java binary with the provided arguments
java +ARGS: debug
    cargo run --bin sj -- {{ ARGS }}
