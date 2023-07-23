from typing import Optional

from generators.asm_specs.x86 import global_defs
from generators.asm_specs.x86.operand import Operand


class Pattern:
    # the sequence of bits and nonterminals used to decode/encode an instruction.
    pattern: dict[str, bool] = {}
    # the operands, typicall registers,  memory operands and pseudo-resources.
    operands: Optional[list[Operand]] = None
    """(optional) a name for the pattern that starts with the iclass and bakes in the operands. If omitted, 
    xed tries to generate one. We often add custom suffixes to these to disambiguate certain combinations."""
    iform: Optional[str] = None

    def __init__(self, pattern: str, operands_str: str, iform: Optional[str]):
        # Expand state macros we pulled from `all-state.txt`
        for pattern_field in pattern.split():
            expanded = global_defs.states.get(pattern_field)
            if expanded:
                # This pattern field had a macro expansion
                self.pattern[expanded] = True
            else:
                self.pattern[pattern_field] = True

        self.iform = iform
        if operands_str != "":
            operands = []
            for operand_str in operands_str.split(" "):
                operand = Operand(operand_str)
                if operand.is_visible() and operand.action != "":
                    operands.append(operand)
            if len(operands) > 1:
                self.operands = operands
