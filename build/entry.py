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

    bin_dir = DIST_DIR.joinpath("bin")
    for binary, packaged_binary_name in BINARIES:
        src = os.path.join(target_dir, binary)
        dest = os.path.join(bin_dir, packaged_binary_name)
        shutil.copy(src, dest)

    lib_dir = DIST_DIR.joinpath("lib")
    for lib in LIBRARIES:
        packaged_lib_name = LIBRARY_PREFIX + lib + LIBRARY_SUFFIX
        src = os.path.join(target_dir, packaged_lib_name)

        # Special case for VM libraries, copy them into a Hotspot-style directory (<out>/lib/<vm_variant>/<lib_name>)
        if lib in VM_LIBRARIES:
            out = lib_dir.joinpath(args.variant)
            out.mkdir(parents=True, exist_ok=True)

            dest = os.path.join(out, packaged_lib_name)
        else:
            dest = os.path.join(lib_dir, packaged_lib_name)
        shutil.copy(src, dest)


if __name__ == "__main__":
    main()
