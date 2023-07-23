from generators.asm_specs.util import fatal


class Width:
    name: str
    datatype: str
    width8: str
    width16: str
    width32: str
    width64: str

    def __init__(self, width_line: str):
        tokens = width_line.split()
        if len(tokens) not in [3, 5]:
            fatal("ERROR: Invalid number of tokens for width declaration")

        self.name = tokens[0]
        self.datatype = tokens[1]
        self.width8 = self.width16 = self.width32 = self.width64 = tokens[2]

        if len(tokens) == 5:
            self.width8 = "0"
            self.width16 = tokens[2]
            self.width32 = tokens[3]
            self.width64 = tokens[4]
