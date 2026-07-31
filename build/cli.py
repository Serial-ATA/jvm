import os
import re
from typing import Optional

import sys
import argparse
from argparse import Namespace

from entry import VmVariant


def check_release_file(java_home: str) -> bool:
    expected_version = os.environ.get("JAVA_VERSION")
    assert expected_version, "`JAVA_VERSION` should be set by `justfile`"

    release_path = os.path.join(java_home, "release")
    if not os.path.exists(release_path):
        return False

    with open(release_path) as f:
        java_version_re = re.compile(f"JAVA_VERSION=\\\"({expected_version}\\.\\d+\\.\\d+)\\\"")
        if not java_version_re.search(f.read()):
            return False
        return True


def check_java_home() -> Optional[str]:
    java_home = os.environ.get("JAVA_HOME")
    if not java_home:
        return None

    if check_release_file(java_home):
        print(
            f"[WARN] Using `JAVA_HOME` ({java_home}) as boot JDK",
            file=sys.stderr,
        )
        return java_home
    return None


def args() -> Namespace:
    parser = argparse.ArgumentParser(
        prog="sj-build", description="Build packager for Serial's JVM"
    )
    parser.add_argument(
        "--boot-jdk",
        type=str,
        help="path to the boot JDK. if unspecified, `JAVA_HOME` will be checked",
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
            boot_jdk_home = check_java_home()
        else:
            check_release_file(boot_jdk_home)

        if not boot_jdk_home:
            print(
                f"Boot JDK not specified (set `BOOT_JDK` environment variable or --boot-jdk)",
                file=sys.stderr,
            )
            exit(1)
        ret.boot_jdk = boot_jdk_home

    return ret
