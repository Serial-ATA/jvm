#!/usr/bin/env just --justfile
# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

BUILD_DIR := justfile_directory() / "build"
DIST_DIR := BUILD_DIR / "dist"

default: debug

# Build the entire project in debug
debug:
    cargo +nightly -Z unstable-options build --workspace

# Build the entire project in release
release:
    cargo +nightly -Z unstable-options build --release --workspace

native-debug:
    cargo +nightly -Z unstable-options build -p native-meta

native-release:
    cargo +nightly -Z unstable-options build --release -p native-meta

native: native-debug

dist *ARGS:
    python3 {{ BUILD_DIR }}/entry.py {{ ARGS }}

# Build and run the java binary with the provided arguments
java +ARGS: debug
    just dist
    JAVA_HOME={{ DIST_DIR }} {{ DIST_DIR / "bin" / "java" }} {{ ARGS }}
