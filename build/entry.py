import os
import shutil
import sys

import cli
from enum import StrEnum

from includes import BINARIES


class ModuleType(StrEnum):
    BIN = "bin"
    CONF = "conf"
    INCLUDE = "include"
    JMODS = "jmods"
    LEGAL = "legal"
    LIB = "lib"
    MAN = "man"


PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BUILD_DIR = os.path.join(PROJECT_ROOT, "build")
TARGET_DIR = os.path.join(PROJECT_ROOT, "target")
OUT_DIR = os.path.join(BUILD_DIR, "out")
DIST_DIR = os.path.join(BUILD_DIR, "dist")


def needs_repackage() -> bool:
    return True


def mk_dist_dir_if_needed():
    if os.path.exists(DIST_DIR):
        shutil.rmtree(DIST_DIR)

    os.mkdir(DIST_DIR)

    for mod in ModuleType.__members__.values():
        os.mkdir(os.path.join(DIST_DIR, mod.value))


def main():
    args = cli.args()

    if args.force is False:
        if not needs_repackage():
            print("No files changed, exiting...")
            return

    print(f"Packaging for profile `{args.profile}`")
    target_dir = os.path.join(TARGET_DIR, args.profile)
    if not os.path.isdir(target_dir):
        print(
            f"Project not built for profile `{args.profile}` (searched {str(target_dir)})",
            file=sys.stderr,
        )
        exit(1)

    mk_dist_dir_if_needed()

    bin_dir = os.path.join(DIST_DIR, "bin")
    for binary, packaged_binary_name in BINARIES:
        src = os.path.join(target_dir, binary)
        dest = os.path.join(bin_dir, packaged_binary_name)
        shutil.copy(src, dest)


if __name__ == "__main__":
    main()
