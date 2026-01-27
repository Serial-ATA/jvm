import os
from pathlib import Path

PROJECT_ROOT = Path(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
BUILD_DIR = PROJECT_ROOT.joinpath("build")
TARGET_DIR = PROJECT_ROOT.joinpath("target")
OUT_DIR = BUILD_DIR.joinpath("out")
DIST_DIR = BUILD_DIR.joinpath("dist")
