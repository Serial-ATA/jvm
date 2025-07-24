import argparse
from argparse import Namespace

from entry import VmVariant


def args() -> Namespace:
    parser = argparse.ArgumentParser(
        prog="sj-build", description="Build packager for Serial's JVM"
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
        "--force",
        action="store_true",
        help="force re-package, even if nothing has changed",
    )
    return parser.parse_args()
