import os
import sys
import argparse
from argparse import Namespace

from entry import VmVariant


def args() -> Namespace:
    parser = argparse.ArgumentParser(
        prog="sj-build", description="Build packager for Serial's JVM"
    )
    parser.add_argument(
        "--boot-jdk",
        type=str,
        help="path to the boot JDK",
    )
    parser.add_argument(
        "--profile",
        type=str,
        help="the Cargo build profile",
        default="release",
    )
    parser.add_argument(
        "--variant",
        choices=[x for x in VmVariant.__members__.keys()],
        help="the variant of the VM",
        default=str(VmVariant.SERVER),
    )
    parser.add_argument(
        "--no-native-libs",
        action="store_true",
        help="don't use sj native libraries, copy the libraries from the BOOT_JDK",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="force re-package, even if nothing has changed",
    )

    ret = parser.parse_args()
    if not ret.boot_jdk:
        boot_jdk_home = os.environ.get("BOOT_JDK")
        if boot_jdk_home is None:
            print(
                f"Boot JDK not specified (set `BOOT_JDK` environment variable or --boot-jdk)",
                file=sys.stderr,
            )
            exit(1)
        ret.boot_jdk = boot_jdk_home

    return ret
