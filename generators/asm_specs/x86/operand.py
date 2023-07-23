from enum import StrEnum
from typing import Optional

from generators.asm_specs.util import fatal
from generators.asm_specs.x86 import global_defs
from generators.asm_specs.x86.width import Width
from generators.asm_specs.x86.xtype import XType


class OperandVisibility(StrEnum):
    Explicit = "EXPL"
    Implicit = "IMPL"
    Suppressed = "SUPP"
    Econd = "ECOND"


def is_valid_visibility(s: str) -> bool:
    return s == "EXPL" or s == "IMPL" or s == "SUPP" or s == "ECOND"


class Operand:
    name: str
    action: str
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

        self.name = next(fields_iter)
        self.action = next(fields_iter)

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
        return self.visibility and (self.visibility == OperandVisibility.Explicit or OperandVisibility.Implicit)
