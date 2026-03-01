#!/usr/bin/env just --justfile
# -----------------------------------------------------------------------------
# VERSIONING:
# -----------------------------------------------------------------------------

export JAVA_VERSION := "27"
export TARGET_OPENJDK_TAG := "jdk-27+0"

# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

PROJECT_ROOT := justfile_directory()
PYTHON_VENV := PROJECT_ROOT / ".venv"
BUILD_DIR := PROJECT_ROOT / "build"
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
    PYTHONPATH={{ PROJECT_ROOT }} python3 {{ BUILD_DIR }}/entry.py {{ ARGS }}

venv:
    python -m venv {{ PYTHON_VENV }}
    {{ PYTHON_VENV }}/bin/python -m pip install --upgrade pip
    {{ PYTHON_VENV }}/bin/python -m pip install -r {{ PROJECT_ROOT }}/scripts/requirements.txt

script name *ARGS: venv
    {{ PYTHON_VENV }}/bin/python {{ PROJECT_ROOT }}/scripts/{{ name }}.py {{ ARGS }}

# Build and run the java binary with the provided arguments
java +ARGS: debug
    just dist --profile debug
    {{ DIST_DIR / "bin" / "java" }} -Djava.home={{ DIST_DIR }} {{ ARGS }}
