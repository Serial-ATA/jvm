#!/usr/bin/env just --justfile

# -----------------------------------------------------------------------------
# PROJECTS:
# -----------------------------------------------------------------------------

# TODO

GENERATED_DIR := justfile_directory() + "/generated"
GENERATORS_DIR := justfile_directory() + "/generators"
ASM_SPECS_DIR := GENERATORS_DIR + "/asm_specs"

# -----------------------------------------------------------------------------
# SUBMODULES:
# -----------------------------------------------------------------------------

INTEL_XED_PATH := ASM_SPECS_DIR + "/x86/xed"

# -----------------------------------------------------------------------------
# PYTHON VENV CONFIG:
# -----------------------------------------------------------------------------

PYTHON_VENV_DEPENDENCIES := "pip wheel requests pip-tools"
PYTHON_VENV_LOCATION := justfile_directory() + "/generators/asm_specs/.venv"
PYTHON_VENV_BIN := PYTHON_VENV_LOCATION + if os_family() == "windows" { "/Scripts" } else { "/bin" }
VENV_PYTHON_EXE := PYTHON_VENV_BIN + if os_family() == "windows" { "/python.exe" } else { "/python3" }
# Used in `clean`
VENV_UNINSTALL_LIST := PYTHON_VENV_LOCATION + "/to-uninstall.txt"

SYSTEM_PYTHON_DEFAULT := if os() == "windows" { "python" } else { "python3" }
SYSTEM_PYTHON_EXE := env_var_or_default("PYTHON", SYSTEM_PYTHON_DEFAULT)

# -----------------------------------------------------------------------------
# OTHER:
# -----------------------------------------------------------------------------

DEV_NULL := if os() == "windows" { "nul" } else { "/dev/null" }

# -----------------------------------------------------------------------------
# ASM:
# -----------------------------------------------------------------------------

ASM_GENERATED_DIR := GENERATED_DIR + "/asm_specs"

# x86/XED
X86_GENERATED_DIR := ASM_GENERATED_DIR + "/x86"
X86_SPECS_DIR := ASM_SPECS_DIR + "/x86"
INTEL_XED_MFILE_PATH := INTEL_XED_PATH + "/mfile.py"
INTEL_XED_OPTIONS := "--build-dir=" + X86_GENERATED_DIR + " --install-dir=" + DEV_NULL + " just-prep"

# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

default: debug


# Cleans any previous builds and Python venvs
clean:
    -'{{ VENV_PYTHON_EXE }}' -m pip freeze > {{ VENV_UNINSTALL_LIST }}
    -'{{ VENV_PYTHON_EXE }}' -m pip uninstall -y -r {{ VENV_UNINSTALL_LIST }}
    -rm {{ VENV_UNINSTALL_LIST }}

    # Clean Intel XED
    '{{ VENV_PYTHON_EXE }}' '{{ INTEL_XED_MFILE_PATH }}' -c {{ INTEL_XED_OPTIONS }}

    # TODO: clean other projects


# Setup the python venv
setup_python:
    if test ! -e {{ PYTHON_VENV_LOCATION }}; then {{ SYSTEM_PYTHON_EXE }} -m venv {{ PYTHON_VENV_LOCATION }}; fi
    '{{ VENV_PYTHON_EXE }}' -m pip install --upgrade {{ PYTHON_VENV_DEPENDENCIES }}


# Build Intel XED x86 decoder
build_xed: setup_python
    '{{ VENV_PYTHON_EXE }}' '{{ INTEL_XED_MFILE_PATH }}' --no-encoder --limit-strings  {{ INTEL_XED_OPTIONS }}
    cd '{{ X86_SPECS_DIR }}' && '{{ VENV_PYTHON_EXE }}' "x86/x86.py"


# Parse the various instruction sources, used by the assembler
asm: build_xed


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
    cd tools/java &&\
    cargo run -- {{ ARGS }}
