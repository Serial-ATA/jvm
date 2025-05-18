import argparse
from argparse import Namespace


def args() -> Namespace:
    parser = argparse.ArgumentParser(
        prog="sj-build", description="Build packager for Serial's JVM"
    )
    parser.add_argument(
        "--profile",
        nargs=1,
        type=str,
        help="the Cargo build profile",
        default="release",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="force re-package, even if nothing has changed",
    )
    return parser.parse_args()
