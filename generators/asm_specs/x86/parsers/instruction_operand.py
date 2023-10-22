import re
from enum import StrEnum, Enum, auto
from typing import Optional, Self, Tuple

from generators.asm_specs.util import fatal
from generators.asm_specs.x86 import global_defs
from generators.asm_specs.x86.parsers.width import Width
from generators.asm_specs.x86.parsers.xtype import XType

REGISTERS = {
    # Byte
    "AL": "R8::AL",
    "CL": "R8::CL",
    "DL": "R8::DL",
    "BL": "R8::BL",
    "SPL": "R8::SPL",
    "BPL": "R8::BPL",
    "SIL": "R8::SIL",
    "DIL": "R8::DIL",
    "R8": "R8::R8",
    "R9": "R8::R9",
    "R10": "R8::R10",
    "R11": "R8::R11",
    "R12": "R8::R12",
    "R13": "R8::R13",
    "R14": "R8::R14",
    "R15": "R8::R15",
    "AH": "Rh::AH",
    "CH": "Rh::CH",
    "DH": "Rh::DH",
    "BH": "Rh::BH",
    # Word
    "AX": "R16::AX",
    "CX": "R16::CX",
    "DX": "R16::DX",
    "BX": "R16::BX",
    "SP": "R16::SP",
    "BP": "R16::BP",
    "SI": "R16::SI",
    "DI": "R16::DI",
    "R8W": "R16::R8W",
    "R9W": "R16::R9W",
    "R10W": "R16::R10W",
    "R11W": "R16::R11W",
    "R12W": "R16::R12W",
    "R13W": "R16::R13W",
    "R14W": "R16::R14W",
    "R15W": "R16::R15W",
    # Doubleword
    "EAX": "R32::EAX",
    "ECX": "R32::ECX",
    "EDX": "R32::EDX",
    "EBX": "R32::EBX",
    "ESP": "R32::ESP",
    "EBP": "R32::EBP",
    "ESI": "R32::ESI",
    "EDI": "R32::EDI",
    "R8D": "R32::R8D",
    "R9D": "R32::R9D",
    "R10D": "R32::R10D",
    "R11D": "R32::R11D",
    "R12D": "R32::R12D",
    "R13D": "R32::R13D",
    "R14D": "R32::R14D",
    "R15D": "R32::R15D",
    # Quadword
    "RAX": "R64::RAX",
    "RBX": "R64::RBX",
    "RCX": "R64::RCX",
    "RDX": "R64::RDX",
    "RDI": "R64::RDI",
    "RSI": "R64::RSI",
    "RBP": "R64::RBP",
    "RSP": "R64::RSP",
    "R8": "R64::R8",
    "R9": "R64::R9",
    "R10": "R64::R10",
    "R11": "R64::R11",
    "R12": "R64::R12",
    "R13": "R64::R13",
    "R14": "R64::R14",
    "R15": "R64::R15",
    # Flags
    "RFLAGS": "Flags::RFlags",
    # Instruction pointer
    "IP": "Ip::IP",
    "EIP": "Ip::EIP",
    "RIP": "Ip::RIP",
    "ST0": "St::ST0",
}

OPERAND_TYPE_PATTERN = re.compile(
    "^(?P<name>[A-Z]+\d?) (= (?P<value>\w+) (?P<parens>\(\))? )?$",
    re.RegexFlag.X | re.RegexFlag.I,
)


class OperandType(StrEnum):
    REG = "REG"
    MEM = "MEM"
    SEGMENT = "SEG"
    BASE = "BASE"
    INDEX = "INDEX"
    SCALE = "SCALE"
    ADDRESS_GENERATION = "AGEN"
    IMM = "IMM"
    PTR = "PTR"
    REL_BRANCH = "RELBR"
    BROADCAST = "BCAST"


class OperandValue:
    """
    The value and type of an operand

    Operands can be defined with multiple types:
        * Constant: (Ex. `RELBR`)
        * Nonterminal function call: (Ex. `REG0=XMM_R()`)

    Constant values can be standalone in an operand:
        `RELBR:r:b:i8`

    While a nonterminal function call must be bound like so:
        `REG0=XMM_R():rw:q:f32`
    """

    name: str
    implied_value: bool = False
    constant: Optional[int | str] = None
    lookup_fn: Optional[str] = None

    @classmethod
    def parse(cls, line: str) -> Tuple[Self, OperandType]:
        matches = OPERAND_TYPE_PATTERN.match(line)
        if not matches:
            fatal("Unable to determine operand type from: {}".format(line))

        # Some types are indexed, such as REG0 or SEG0, remove the index now
        name = matches["name"]
        name = name[:-1] if name[-1].isdigit() else name

        if name.endswith("()"):
            fatal("Unbound lookup function: {}".format(name))

        ty = OperandType(name)
        op_val = OperandValue()

        # First check if a value was specified
        value = matches.group("value")
        if not value:
            # If we have a bare value such as `RELBR`, we can exit early
            # as we have all the information needed at this point.
            op_val.implied_value = True
            return op_val, ty

        try:
            value = int(value)
            op_val.constant = value
        except Exception:
            if matches.group("parens"):
                # We have encountered a function call
                op_val.lookup_fn = value
            else:
                # We (should) have a register
                op_val.constant = global_defs.registers.by_name[
                    value.removeprefix("XED_REG_")
                ].name

        return op_val, ty


class OperandAccessType(Enum):
    """
    The Operand's access mode, whether it can never, conditionally, or always read/write
    """

    Nothing = auto()
    Conditional = auto()
    Always = auto()


class OperandAccess:
    """
    This simply holds information on the Operand's read/write ability
    """

    read: OperandAccessType = OperandAccessType.Nothing
    write: OperandAccessType = OperandAccessType.Nothing

    def __init__(self, line: str):
        if line.startswith("cr"):
            self.read = OperandAccessType.Conditional
            line = line[2:]
        elif line.startswith("r"):
            self.read = OperandAccessType.Always
            line = line[1:]

        if not line:
            return

        self.write = (
            OperandAccessType.Conditional if line == "cw" else OperandAccessType.Always
        )

    def is_nothing(self) -> bool:
        return (
            self.read == OperandAccessType.Nothing
            and self.write == OperandAccessType.Nothing
        )


class OperandVisibility(StrEnum):
    """
    How the operand is represented both in assembly and to the user.

    Types:
        * `Explicit`: Represented in the assembly and specified by the user
        * `Implicit`: Represented in the assembly, but not specified by the user
        * `Suppressed`: Not represented in the assembly nor specified by the user
        * `Econd`: Not sure?
    """

    Explicit = "EXPL"
    Implicit = "IMPL"
    Suppressed = "SUPP"
    Econd = "ECOND"


def is_valid_visibility(s: str) -> bool:
    return s == "EXPL" or s == "IMPL" or s == "SUPP" or s == "ECOND"


class Operand:
    name: str = ""
    ty: OperandType
    value: OperandValue
    access: OperandAccess
    width: Optional[Width] = None
    xtype: Optional[XType] = None  # User-specified type
    attributes: Optional[list[str]] = None
    visibility: Optional[OperandVisibility] = None

    def __init__(self, operand: str):
        fields: list[str] = operand.split(":")

        if len(fields) == 0:
            fatal("ERROR: Operand field is empty")
        if len(fields) == 1:
            self.name = fields[0]
            return

        fields_iter = iter(fields)

        self.value, self.ty = OperandValue.parse(next(fields_iter))
        self.action = OperandAccess(next(fields_iter))

        for field in fields_iter:
            if field in global_defs.widths and not self.width:
                self.width = global_defs.widths[field]
                continue

            if is_valid_visibility(field):
                self.visibility = OperandVisibility(field)
                continue

            if field in global_defs.xtypes and not self.xtype:
                self.xtype = global_defs.xtypes[field]
                continue

            if not self.attributes:
                self.attributes = [field]
                continue

            self.attributes.append(field)

    def is_visible(self):
        return self.visibility and (
            self.visibility == OperandVisibility.Explicit
            or self.visibility == OperandVisibility.Implicit
        )

    def rust_argument_type(self) -> str:
        if self.xtype:
            return self.xtype.rust_mapping()

        match self.ty:
            case OperandType.IMM:
                assert False, "immediate values not done yet"
            case OperandType.REG:
                if self.value.lookup_fn:
                    assert False, "lookup functions not done yet"

                register_name = self.value.constant.removeprefix(
                    "XED_REG_"
                ).removesuffix("()")

                rust_mapped_register = REGISTERS.get(register_name.upper())
                if not rust_mapped_register:
                    fatal("Encountered unknown register: {}".format(register_name))
                return rust_mapped_register
            case OperandType.MEM:
                assert False, "memory values not done yet"
            case _:
                assert False, "{} not done yet".format(self.ty)
