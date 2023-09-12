import re
from enum import Enum, auto
from typing import Iterable, Optional

from generators.asm_specs.util import fatal


class RegisterClass(Enum):
    """
    The classification of a `Register`.

    Possible register types:

        * `GPR` - General Purpose Register
        * `IP` - Instruction Pointer
        * `FLAGS` - Flag Register
        * `SR` - Segment Register
        * `CR` - Control Register
        * `DR` - Debug Register
        * `X87` - x87 Float Register
        * `PSEUDO` - Pseudo Register
        * `PSEUDO_X87` - Pseudo x87 Float Register
        * `MMX` -
        * `XMM` -
        * `YMM` -
        * `ZMM` -
        * `MASK` - Mask Register
        * `XCR` - Extended Control Register
        * `MXCSR` - Multimedia eXtension Control and Status Register
        * `MSR` - Model-Specific Register
        * `TREG` - Test Register
        * `TMP` - Dummy Register
        * `INVALID` - Invalid Register (Used for errors)
    """

    GPR = auto()
    IP = auto()
    FLAGS = auto()
    SR = auto()
    CR = auto()
    DR = auto()

    X87 = auto()

    PSEUDO = auto()
    PSEUDO_X87 = auto()

    MMX = auto()
    XMM = auto()
    YMM = auto()
    ZMM = auto()
    MASK = auto()
    XCR = auto()
    MXCSR = auto()

    BOUND = auto()
    BNDCFG = auto()
    BNDSTAT = auto()

    MSR = auto()
    TREG = auto()
    TMP = auto()
    INVALID = auto()
    UIF = auto()


def register_class(klass: str) -> RegisterClass:
    match klass.upper():
        case "GPR":
            return RegisterClass.GPR
        case "IP":
            return RegisterClass.IP
        case "FLAGS":
            return RegisterClass.FLAGS
        case "SR":
            return RegisterClass.SR
        case "CR":
            return RegisterClass.CR
        case "DR":
            return RegisterClass.DR
        case "X87":
            return RegisterClass.X87
        case "PSEUDO":
            return RegisterClass.PSEUDO
        case "PSEUDOX87":
            return RegisterClass.PSEUDO_X87
        case "MMX":
            return RegisterClass.MMX
        case "XMM":
            return RegisterClass.XMM
        case "YMM":
            return RegisterClass.YMM
        case "ZMM":
            return RegisterClass.ZMM
        case "MASK":
            return RegisterClass.MASK
        case "XCR":
            return RegisterClass.XCR
        case "MXCSR":
            return RegisterClass.MXCSR
        case "BOUND":
            return RegisterClass.BOUND
        case "BNDCFG":
            return RegisterClass.BNDCFG
        case "BNDSTAT":
            return RegisterClass.BNDSTAT
        case "MSR":
            return RegisterClass.MSR
        case "TREG":
            return RegisterClass.TREG
        case "TMP":
            return RegisterClass.TMP
        case "INVALID":
            return RegisterClass.INVALID
        case "UIF":
            return RegisterClass.UIF
        case _:
            fatal("Unknown register class encountered: {}".format(klass))


REGISTER_REGEX = re.compile(
    """^\s*(?P<name>\w+)\s+(?P<class>\w+)
\s+((?P<width32>\d+)(/(?P<width64>\d+))?|NA)
(
    \s+(?P<parent64>\w+)(/(?P<parent32>\w+))?
(
    \s+(?P<id>\d+)
    (\s+((?P<h>h)|-\s*(st\(\d\)|mm\d)))?
)?
)?\s*$""",
    re.RegexFlag.S | re.RegexFlag.X,
)


class Register:
    """
    Represents a register definition from `all-registers.txt`.

    The format is:

    `name class width max-enclosing-reg-64b/32b-mode regid [h]`

    Example:

        `BH  gpr  8   RBX/EBX   7 h`

        * Name: BH
        * Class: gpr
        * Width: 8 bits
        * Parent 64bit register: RBX
        * Parent 32bit register: EBX
        * Register ID: 7
        * Is high byte: True
    """

    name: str
    klass: RegisterClass
    width32: Optional[int]
    width64: Optional[int]
    max_enclosing_64bit_reg_str: str
    max_enclosing_32bit_reg_str: str
    reg_id: Optional[int]
    is_high_byte: bool

    def __init__(
            self,
            name: str,
            klass: RegisterClass,
            width32: Optional[int],
            width64: Optional[int],
            max_enclosing_64bit_reg_str: str,
            max_enclosing_32bit_reg_str: Optional[str],
            reg_id: Optional[int],
            is_high_byte: bool,
    ):
        self.name = name
        self.klass = klass
        self.width32 = width32
        if width64:
            self.width64 = width64
        else:
            self.width64 = self.width32
        self.max_enclosing_64bit_reg_str = max_enclosing_64bit_reg_str
        self.max_enclosing_32bit_reg_str = max_enclosing_32bit_reg_str
        self.reg_id = reg_id
        self.is_high_byte = is_high_byte


def parse_register(line: str) -> Register:
    matches = REGISTER_REGEX.match(line)
    if not matches:
        fatal("Malformed register line: {}".format(line))

    return Register(
        name=matches["name"],
        klass=register_class(matches["class"]),
        width32=int(matches["width32"]) if matches["width32"] else None,
        width64=int(matches["width64"]) if matches["width64"] else None,
        max_enclosing_64bit_reg_str=matches["parent64"],
        max_enclosing_32bit_reg_str=matches["parent32"],
        reg_id=int(matches["id"]) if matches["id"] else None,
        is_high_byte=len(matches.groups()) == 7,
    )


class Registers:
    by_name: dict[str, Register] = {}
    by_index: list[Register] = []

    def create_parent_references(self):
        for reg in self.by_index:
            if reg.max_enclosing_64bit_reg_str:
                setattr(reg, "parent64", self.by_name[reg.max_enclosing_64bit_reg_str])
            if reg.max_enclosing_32bit_reg_str:
                setattr(reg, "parent32", self.by_name[reg.max_enclosing_32bit_reg_str])


def parse_registers(lines: Iterable[str]) -> Registers:
    table = Registers()
    for line in lines:
        if len(line) == 0:
            continue

        reg = parse_register(line)
        table.by_name[reg.name] = reg
        table.by_index.append(reg)

    table.create_parent_references()
    return table
