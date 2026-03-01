# Fetches the jvm.h (and related headers) from the given tag, and generates a list of function
# signatures from it.
#
# The generated signatures are used in `../runtime/build.rs`.
#
# Do not run this directly, use `just script jvm_h <GIT TAG>`

import dataclasses
import subprocess
import sys
import tempfile
from enum import StrEnum
from typing import Union
from urllib.parse import quote

import requests
from pathlib import Path
import os
import clang.cindex

HEADERS = [
    "https://raw.githubusercontent.com/openjdk/jdk/refs/tags/{tag}/src/java.base/share/native/include/jni.h",
    "https://raw.githubusercontent.com/openjdk/jdk/refs/tags/{tag}/src/java.base/unix/native/include/jni_md.h",
    "https://raw.githubusercontent.com/openjdk/jdk/refs/tags/{tag}/src/hotspot/os/posix/include/jvm_md.h",
    "https://raw.githubusercontent.com/openjdk/jdk/refs/tags/{tag}/src/hotspot/share/include/jvm_io.h",
    "https://raw.githubusercontent.com/openjdk/jdk/refs/tags/{tag}/src/hotspot/share/include/jvm_constants.h",
    "https://raw.githubusercontent.com/openjdk/jdk/refs/tags/{tag}/src/hotspot/share/include/jvm.h",
]
OUTPUT_PATH = Path(__file__).parent.parent / "runtime" / "vm_functions.txt"

VM_FUNCTIONS_TXT_HEADER = """# DO NOT EDIT THIS MANUALLY!
# Generated with ../scripts/jvm_h.py
#
# Format: <name><TAB>[PARAM_TYPE1,PARAM_TYPE2][<TAB>-> RETURN_TYPE]
"""

CLASSFILE_CONSTANTS_H = """
enum {
    JVM_ACC_PUBLIC        = 0x0001,
    JVM_ACC_PRIVATE       = 0x0002,
    JVM_ACC_PROTECTED     = 0x0004,
    JVM_ACC_STATIC        = 0x0008,
    JVM_ACC_FINAL         = 0x0010,
    JVM_ACC_SYNCHRONIZED  = 0x0020,
    JVM_ACC_SUPER         = 0x0020,
    JVM_ACC_VOLATILE      = 0x0040,
    JVM_ACC_BRIDGE        = 0x0040,
    JVM_ACC_TRANSIENT     = 0x0080,
    JVM_ACC_VARARGS       = 0x0080,
    JVM_ACC_NATIVE        = 0x0100,
    JVM_ACC_INTERFACE     = 0x0200,
    JVM_ACC_ABSTRACT      = 0x0400,
    JVM_ACC_STRICT        = 0x0800,
    JVM_ACC_SYNTHETIC     = 0x1000,
    JVM_ACC_ANNOTATION    = 0x2000,
    JVM_ACC_ENUM          = 0x4000,
    JVM_ACC_MODULE        = 0x8000
};
"""


def fetch(out_dir, tag):
    for header in HEADERS:
        header = header.replace("{tag}", quote(tag))
        response = requests.get(header)
        response.raise_for_status()

        with open(out_dir / header.rsplit("/")[-1], "w") as f:
            f.write(response.text)

    # Need to handle classfile_constants.h separately, since it's a generated file.
    # Nothing in jvm.h needs the generated portions though, so we just copy in what we actually use.
    with open(out_dir / "classfile_constants.h", "w") as f:
        f.write(CLASSFILE_CONSTANTS_H)


class CType(StrEnum):
    VOID = "void"
    CHAR = "char"
    JBYTE = "jbyte"
    INT = "int"
    UNSIGNEDCHAR = "unsignedchar"
    UNSIGNEDSHORT = "unsignedshort"

    def rust_mapping(self) -> str:
        match self:
            case CType.VOID:
                return "c_void"
            case CType.CHAR:
                return "c_char"
            case CType.JBYTE:
                return "jbyte"
            case CType.INT:
                return "c_int"
            case CType.UNSIGNEDCHAR:
                return "c_char"
            case CType.UNSIGNEDSHORT:
                return "c_short"


class JniType(StrEnum):
    JNIENV = "JNIEnv"
    JBOOLEAN = "jboolean"
    JBYTE = "jbyte"
    JCHAR = "jchar"
    JSHORT = "jshort"
    JINT = "jint"
    JLONG = "jlong"
    JFLOAT = "jfloat"
    JDOUBLE = "jdouble"

    JOBJECT = "jobject"
    JCLASS = "jclass"
    JTHROWABLE = "jthrowable"
    JSTRING = "jstring"
    JWEAK = "jweak"

    JARRAY = "jarray"
    JBOOLEANARRAY = "jbooleanArray"
    JBYTEARRAY = "jbyteArray"
    JCHARARRAY = "jcharArray"
    JSHORTARRAY = "jshortArray"
    JINTARRAY = "jintArray"
    JLONGARRAY = "jlongArray"
    JFLOATARRAY = "jfloatArray"
    JDOUBLEARRAY = "jdoubleArray"
    JOBJECTARRAY = "jobjectArray"

    JVALUE = "jvalue"
    JSIZE = "jsize"

    def rust_mapping(self) -> str:
        match self:
            case JniType.JNIENV:
                return "JniEnv"
            case JniType.JOBJECT:
                return "JObject"
            case JniType.JCLASS:
                return "JClass"
            case JniType.JTHROWABLE:
                return "JThrowable"
            case JniType.JSTRING:
                return "JString"
            case JniType.JWEAK:
                return "JWeak"
            case JniType.JARRAY:
                return "JArray"
            case JniType.JBOOLEANARRAY:
                return "JBooleanArray"
            case JniType.JBYTEARRAY:
                return "JByteArray"
            case JniType.JCHARARRAY:
                return "JCharArray"
            case JniType.JSHORTARRAY:
                return "JShortArray"
            case JniType.JINTARRAY:
                return "JIntArray"
            case JniType.JLONGARRAY:
                return "JLongArray"
            case JniType.JFLOATARRAY:
                return "JFloatArray"
            case JniType.JDOUBLEARRAY:
                return "JDoubleArray"
            case JniType.JOBJECTARRAY:
                return "JObjectArray"
            case _:
                return str(self)


@dataclasses.dataclass
class Ptr:
    inner: Union[Ptr, CType, JniType, str]
    is_const: bool

    def rust_mapping(self) -> str:
        mutability = "const" if self.is_const else "mut"

        if hasattr(self.inner, "rust_mapping"):
            inner_rust_type = self.inner.rust_mapping()
        else:
            inner_rust_type = str(self.inner)

        return f"*{mutability} {inner_rust_type}"

    def __str__(self):
        return self.rust_mapping()


def parse_type(c_type: clang.cindex.Type) -> Union[Ptr, CType, JniType, str]:
    if c_type.kind == clang.cindex.TypeKind.POINTER:
        pointee = c_type.get_pointee()
        is_const = pointee.is_const_qualified()
        inner_type = parse_type(pointee)
        return Ptr(inner=inner_type, is_const=is_const)

    spelling = c_type.spelling.replace("struct ", "").replace("const ", "").strip()
    enum_lookup = spelling.replace(" ", "")

    try:
        return CType(enum_lookup)
    except ValueError:
        pass

    try:
        return JniType(enum_lookup)
    except ValueError:
        pass

    return spelling


@dataclasses.dataclass
class Function:
    name: str
    ret: Union[CType, JniType, str]
    params: list[Union[CType, JniType, str]]

    def __init__(self, node):
        self.name = node.spelling
        self.params = []

        self.ret = parse_type(node.result_type)

        for arg in node.get_arguments():
            self.params.append(parse_type(arg.type))

    def __str__(self):
        if self.ret == CType.VOID:
            ret_str = None
        else:
            ret_str = (
                self.ret.rust_mapping()
                if hasattr(self.ret, "rust_mapping")
                else str(self.ret)
            )

        param_strs = []
        for p in self.params:
            if isinstance(p, Ptr) and p.inner == JniType.JNIENV:
                param_strs.append(JniType.JNIENV.rust_mapping())
                continue

            param_strs.append(
                p.rust_mapping() if hasattr(p, "rust_mapping") else str(p)
            )

        params_joined = ",".join(param_strs)
        return f"{self.name}\t{params_joined}{f"\t-> {ret_str}" if ret_str else ""}"


def get_clang_include_path():
    try:
        result = subprocess.run(
            ["clang", "-print-resource-dir"], capture_output=True, text=True, check=True
        )
        resource_dir = result.stdout.strip()
        return os.path.join(resource_dir, "include")
    except (FileNotFoundError, subprocess.CalledProcessError):
        print("Could not determine clang resource dir", file=sys.stderr)
        sys.exit(1)


def parse(jvm_h_path, temp_dir) -> list[Function]:
    index = clang.cindex.Index.create()

    args = [f"-I{temp_dir}", f"-isystem{get_clang_include_path()}"]
    tu = index.parse(jvm_h_path, args=args)
    for diag in tu.diagnostics:
        print(f"{diag.spelling}")
        sys.exit(1)

    functions = []
    for node in tu.cursor.get_children():
        if node.location.file and node.location.file.name != jvm_h_path:
            continue

        if node.kind == clang.cindex.CursorKind.FUNCTION_DECL:
            functions.append(Function(node))

    return functions


def main():
    if len(sys.argv) != 2:
        print("Expected a git tag", file=sys.stderr)
        sys.exit(1)

    with tempfile.TemporaryDirectory() as temp_dir:
        jvm_h_path = os.path.join(temp_dir, "jvm.h")
        fetch(Path(temp_dir), sys.argv[1])
        functions = parse(jvm_h_path, temp_dir)

        functions.sort(key=lambda fun: fun.name)
        with open(OUTPUT_PATH, "w") as f:
            f.write(VM_FUNCTIONS_TXT_HEADER)
            for func in functions:
                f.write(str(func) + "\n")
    print(f"Updated jvm.h function list at {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
