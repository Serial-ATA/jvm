import os
import shutil
import sys
from pathlib import Path

import cli
from enum import StrEnum

from includes import BINARIES, LIBRARIES, VM_LIBRARIES


class ModuleType(StrEnum):
    BIN = "bin"
    CONF = "conf"
    INCLUDE = "include"
    JMODS = "jmods"
    LEGAL = "legal"
    LIB = "lib"
    MAN = "man"


class VmVariant(StrEnum):
    SERVER = "server"


PROJECT_ROOT = Path(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
BUILD_DIR = PROJECT_ROOT.joinpath("build")
TARGET_DIR = PROJECT_ROOT.joinpath("target")
OUT_DIR = BUILD_DIR.joinpath("out")
DIST_DIR = BUILD_DIR.joinpath("dist")


def needs_repackage() -> bool:
    return True


def mk_dist_dir_if_needed():
    if DIST_DIR.exists():
        shutil.rmtree(DIST_DIR)

    DIST_DIR.mkdir(parents=True)

    for mod in ModuleType.__members__.values():
        new_dir = DIST_DIR.joinpath(mod.value)
        new_dir.mkdir(parents=True)


if sys.platform.startswith("linux"):
    LIBRARY_PREFIX = "lib"
    LIBRARY_SUFFIX = ".so"
elif sys.platform.startswith("darwin"):
    LIBRARY_PREFIX = "lib"
    LIBRARY_SUFFIX = ".dylib"
elif sys.platform.startswith("win32"):
    LIBRARY_PREFIX = ""
    LIBRARY_SUFFIX = ".dll"
else:
    print(
        f"Unable to determine the current platform ({sys.platform})",
        file=sys.stderr,
    )
    exit(1)

BIN_DIR: Path = DIST_DIR.joinpath("bin")
LIB_DIR: Path = DIST_DIR.joinpath("lib")


def copy_native_libs(target_dir: Path, variant: VmVariant):
    for lib in LIBRARIES:
        packaged_lib_name = LIBRARY_PREFIX + lib + LIBRARY_SUFFIX
        src = os.path.join(target_dir, packaged_lib_name)

        # Special case for VM libraries, copy them into a Hotspot-style directory (<out>/lib/<vm_variant>/<lib_name>)
        if lib in VM_LIBRARIES:
            out = LIB_DIR.joinpath(variant)
            out.mkdir(parents=True, exist_ok=True)

            dest = os.path.join(out, packaged_lib_name)
        else:
            dest = os.path.join(LIB_DIR, packaged_lib_name)
        shutil.copy(src, dest)


def copy_native_libs_from_boot_jdk(
    target_dir: Path, variant: VmVariant, boot_jdk: Path
):
    # Always copy VM libraries from sj
    for lib in VM_LIBRARIES:
        packaged_lib_name = LIBRARY_PREFIX + lib + LIBRARY_SUFFIX
        src = os.path.join(target_dir, packaged_lib_name)

        out = LIB_DIR.joinpath(variant)
        out.mkdir(parents=True, exist_ok=True)

        dest = os.path.join(out, packaged_lib_name)
        shutil.copy(src, dest)

    # For all other libraries, copy them from the boot JDK
    boot_jdk_libs = boot_jdk.joinpath("lib")
    for entry in os.listdir(boot_jdk_libs):
        path = boot_jdk_libs.joinpath(entry)
        if not path.is_file() or not path.name.endswith(LIBRARY_SUFFIX):
            continue
        shutil.copy(path, LIB_DIR)


def main():
    args = cli.args()

    if args.force is False:
        if not needs_repackage():
            print("No files changed, exiting...")
            return

    print(f"Packaging for profile `{args.profile}`")
    target_dir = TARGET_DIR.joinpath(args.profile)
    if not os.path.isdir(target_dir):
        print(
            f"Project not built for profile `{args.profile}` (searched {str(target_dir)})",
            file=sys.stderr,
        )
        exit(1)

    mk_dist_dir_if_needed()

    for binary, packaged_binary_name in BINARIES:
        src = os.path.join(target_dir, binary)
        dest = os.path.join(BIN_DIR, packaged_binary_name)
        shutil.copy(src, dest)

    boot_jdk = Path(args.boot_jdk)
    boot_jdk_modules = boot_jdk.joinpath("lib").joinpath("modules")
    if not boot_jdk_modules.exists():
        print(
            f"Boot JDK modules not found (searched {str(boot_jdk_modules)})",
            file=sys.stderr,
        )
        exit(1)

    shutil.copy(boot_jdk_modules, LIB_DIR)

    if args.no_native_libs:
        copy_native_libs_from_boot_jdk(target_dir, args.variant, boot_jdk)
    else:
        copy_native_libs(target_dir, args.variant)


if __name__ == "__main__":
    main()
