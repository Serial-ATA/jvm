#!/usr/bin/env just --justfile
# -----------------------------------------------------------------------------
# VERSIONING:
# -----------------------------------------------------------------------------

export JAVA_VERSION := "28"
export TARGET_OPENJDK_TAG := "jdk-28+17"

# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

PROJECT_ROOT := justfile_directory()
PYTHON_VENV := PROJECT_ROOT / ".venv"
GENERATED_DIR := PROJECT_ROOT / "generated"
BUILD_DIR := PROJECT_ROOT / "build"
DIST_DIR := BUILD_DIR / "dist"
OUT_DIR := BUILD_DIR / "out"
HOST_TRIPLE := `rustc -vV | awk '/host:/ { print $2 }'`

default: debug

# Clean up all build artifacts
clean:
    cargo clean
    rm -r {{ DIST_DIR }} {{ OUT_DIR }} {{ GENERATED_DIR }}

# Run `cargo test`
test *ARGS: debug
    just dist --profile debug --no-native-libs
    TEST_JAVA_HOME={{ DIST_DIR }} RUSTFLAGS="-C prefer-dynamic" cargo +nightly -Z unstable-options test {{ ARGS }}

# Run `cargo clippy`
lint *ARGS:
    cargo +nightly -Z unstable-options clippy {{ ARGS }}

# Run `cargo doc`
doc *ARGS:
    cargo +nightly -Z unstable-options doc {{ ARGS }}

# Build the entire project in debug
debug:
    cargo +nightly -Z unstable-options build --workspace

# Build the entire project in release
release:
    cargo +nightly -Z unstable-options build --release --workspace --target {{ HOST_TRIPLE }}

# Build the entire project in release with ASan
release-asan:
    RUSTFLAGS="-Z sanitizer=address -C force-frame-pointers=yes -C link-arg=-Wl,--export-dynamic-symbol=__asan_* -C link-arg=-Wl,--export-dynamic-symbol=__sanitizer_* -C link-arg=-Wl,--export-dynamic-symbol=__lsan_* -C link-arg=-Wl,--export-dynamic-symbol=__ubsan_*" cargo +nightly -Z unstable-options -Z build-std build --release --workspace --target {{ HOST_TRIPLE }}

native-debug:
    cargo +nightly -Z unstable-options build -p native-meta

native-release:
    cargo +nightly -Z unstable-options build --release -p native-meta

native: native-debug

dist *ARGS:
    PYTHONPATH={{ PROJECT_ROOT }} python3 {{ BUILD_DIR }}/entry.py {{ ARGS }}

_venv:
    python -m venv {{ PYTHON_VENV }}
    {{ PYTHON_VENV }}/bin/python -m pip install --upgrade pip
    {{ PYTHON_VENV }}/bin/python -m pip install -r {{ PROJECT_ROOT }}/scripts/requirements.txt

script name *ARGS: _venv
    {{ PYTHON_VENV }}/bin/python {{ PROJECT_ROOT }}/scripts/{{ name }}.py {{ ARGS }}

# Build and run the java binary with the provided arguments
java +ARGS: release
    just _do_java release {{ HOST_TRIPLE }} {{ ARGS }}

# Build the java binary in release mode with ASan and run with the provided arguments
java-asan +ARGS: release-asan
    just _do_java release {{ HOST_TRIPLE }} {{ ARGS }}

# Build and run the java binary in debug mode with the provided arguments
java-debug +ARGS: debug
    just _do_java debug "" {{ ARGS }}

_do_java PROFILE TARGET +ARGS:
    just dist --profile {{ PROFILE }} {{ if TARGET != "" { "--target " + TARGET } else { "" } }} --no-native-libs
    JAVA_HOME={{ DIST_DIR }} {{ DIST_DIR / "bin" / "java" }} -Djava.home={{ DIST_DIR }} {{ ARGS }}
