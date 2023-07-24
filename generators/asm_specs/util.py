import os
from pathlib import Path

import requests


def generated_directory_for(arch: str) -> str:
    return "generated/asm_specs/" + arch


def fatal(message: str):
    print(message)
    exit(1)


def download_pdf(path: str, generated_dir: str):
    if not Path(path).exists():
        try:
            os.makedirs(generated_dir)
        except OSError as e:
            fatal("ERROR: Failed to create generated directories: " + repr(e))

        print("WARN: Missing instruction set reference, downloading from Intel.")

        try:
            response = requests.get("https://cdrdv2.intel.com/v1/dl/getContent/671110")
            response.raise_for_status()

            pdf_file = open(path, "wb+")
            pdf_file.write(response.content)
            pdf_file.close()
        except requests.exceptions.RequestException as e:
            fatal("ERROR: Failed to download instruction set reference from Intel: " + repr(e))
        except OSError as e:
            fatal("ERROR: Failed to create/write to instruction set PDF: " + repr(e))
