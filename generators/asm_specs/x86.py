import os
from pathlib import Path

import requests
import re
import fitz
from fitz import Page

GENERATED_DIR = "../../generated/asm_specs/x86"
PDF_PATH = GENERATED_DIR + "/vol-2abcd.pdf"


def fatal(message: str):
    print(message)
    exit(1)


def download_pdf():
    if not Path(PDF_PATH).exists():
        try:
            os.makedirs(GENERATED_DIR)
        except OSError as e:
            fatal("ERROR: Failed to create generated directories: " + repr(e))

        print("WARN: Missing instruction set reference, downloading from Intel.")

        try:
            response = requests.get("https://cdrdv2.intel.com/v1/dl/getContent/671110")
            response.raise_for_status()

            pdf_file = open(PDF_PATH, "wb+")
            pdf_file.write(response.content)
            pdf_file.close()
        except requests.exceptions.RequestException as e:
            fatal("ERROR: Failed to download instruction set reference from Intel: " + repr(e))
        except OSError as e:
            fatal("ERROR: Failed to create/write to instruction set PDF: " + repr(e))


DATE_REGEX = re.compile(
    r"\b(January|February|March|April|May|June|July|August|September|October|November|December) (20[0-9][0-9])\b")
ORDER_NUMBER_REGEX = re.compile(r"Order Number:\s*?([\w\-\w]+)")


def check_first_page(page: Page):
    all_text: str = page.get_text()

    if len(all_text) == 0:
        fatal("ERROR: Title page contains no content!")

    published = DATE_REGEX.search(all_text)
    if published is None:
        fatal("ERROR: Unable to determine PDF publish date")

    order_num = ORDER_NUMBER_REGEX.search(all_text)
    if order_num is None:
        fatal("ERROR: Unable to determine PDF order number")

    print(
        f"INFO: Determined PDF to be \"Instruction Set Reference A-Z, Order Number {order_num.group(1)}\" (Published: {' '.join(published.groups())})")


def main():
    print("INFO: Starting parse of Intel instruction set \"Instruction Set Reference A-Z\" PDF")
    download_pdf()

    pdf = fitz.Document(PDF_PATH, filetype="pdf")
    if pdf.page_count == 0:
        fatal("ERROR: PDF has no pages!")

    check_first_page(pdf[0])

    for page in pdf.pages(start=1):
        texts = []

        blocks = page.get_text("dict")["blocks"]
        for b in blocks:
            for line in b["lines"]:
                for span in line["spans"]:
                    texts.append(span)


if __name__ == "__main__":
    main()
