from generators.asm_specs.util import fatal

import re

BITS_REGEX = re.compile("([0-9]+)bits")


class Width:
    name: str
    datatype: str
    widths: dict[int, int] = {}

    def __init__(self, width_line: str):
        tokens = width_line.split()
        if len(tokens) not in [3, 5]:
            fatal("ERROR: Invalid number of tokens for width declaration")

        self.name = tokens[0]
        self.datatype = tokens[1]
        width8 = width16 = width32 = width64 = tokens[2]

        if len(tokens) == 5:
            width8 = "0"
            width16 = tokens[2]
            width32 = tokens[3]
            width64 = tokens[4]

        # Widths are allowed to specify their size in bits or bytes
        # We need to normalize to bits
        for key, original_val in zip([8, 16, 32, 64], [width8, width16, width32, width64]):
            bits_format = BITS_REGEX.match(original_val)
            if bits_format:
                self.widths[key] = int(bits_format.group(1))
            else:
                self.widths[key] = int(original_val)
