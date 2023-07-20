#!/usr/bin/env just --justfile

# -----------------------------------------------------------------------------
# PYTHON VENV CONFIG:
# -----------------------------------------------------------------------------
PYTHON_VENV_DEPENDENCIES := "pip wheel pip-tools py-pdf-parser"
PYTHON_VENV_LOCATION := "./generators/asm_specs/.venv"
PYTHON_VENV_BIN := PYTHON_VENV_LOCATION + if os_family() == "windows" { "/Scripts" } else { "/bin" }
VENV_PYTHON_EXE := PYTHON_VENV_BIN + if os_family() == "windows" { "/python.exe" } else { "/python3" }

SYSTEM_PYTHON_DEFAULT := if os() == "windows" { "python" } else { "python3" }
SYSTEM_PYTHON_EXE := env_var_or_default("PYTHON", SYSTEM_PYTHON_DEFAULT)

# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

default: debug

# Setup the python venv
setup_python:
    if test ! -e {{ PYTHON_VENV_LOCATION }}; then {{ SYSTEM_PYTHON_EXE }} -m venv {{ PYTHON_VENV_LOCATION }}; fi
    {{ VENV_PYTHON_EXE }} -m pip install --upgrade {{ PYTHON_VENV_DEPENDENCIES }}

# Download and parse the various asm instruction set PDFs, used by the assembler
asm: setup_python

# Build the assembler project
assembler: asm
    cargo build

# Build the entire project in debug
debug: asm
    cargo build

# Build the entire project in release
release: asm
    cargo build --release

# Build and run the java binary with the provided arguments
java +ARGS: debug
    cd tools/java
    cargo run -- {{ ARGS }}
