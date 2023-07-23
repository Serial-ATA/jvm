from generators.asm_specs.util import fatal


class XType:
    name: str
    base_type: str
    size_bits: str

    def __init__(self, xtype_line: str):
        tokens = xtype_line.split()
        if len(tokens) != 3:
            fatal("ERROR: Wrong number of tokens for element type definition")

        self.name = tokens[0]
        self.base_type = tokens[1]
        self.size_bits = tokens[2]