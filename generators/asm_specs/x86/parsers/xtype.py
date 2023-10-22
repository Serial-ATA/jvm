from generators.asm_specs.util import fatal


class XType:
    """A datatype definition consisting of a name which maps to a
    base type and a size in bits.

    The type definitions come from `all-element-types.txt`."""

    name: str
    """The identifier of the datatype"""

    base_type: str
    """The base type of this datatype, as defined in `all-element-type-base.txt`"""

    size_bits: str
    """The size of this datatype in bits"""

    def __init__(self, xtype_line: str):
        tokens = xtype_line.split()
        if len(tokens) != 3:
            fatal("ERROR: Wrong number of tokens for element type definition")

        self.name = tokens[0]
        self.base_type = tokens[1]
        self.size_bits = tokens[2]

    def rust_mapping(self):
        if self.name.startswith("i") or self.name.startswith("u"):
            return self.name

        if self.name == "f32" or self.name == "f64":
            return self.name

        fatal("ERROR: Unimplemented XType: {}".format(self.name))

