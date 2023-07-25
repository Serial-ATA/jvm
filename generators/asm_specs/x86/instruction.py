from copy import deepcopy
from typing import Optional, Tuple, Iterable

from generators.asm_specs.util import fatal
from generators.asm_specs.x86.flag import Flags
from generators.asm_specs.x86.pattern import Pattern
from generators.asm_specs.x86.text_utils import handle_continuations, key_value_pair


class Instruction:
    # instruction name
    name: str
    version: Optional[int] = None
    # (optional) substituted name when a simple conversion from iclass is inappropriate
    disambiguation: Optional[str] = None
    disambiguation_intel: Optional[str] = None
    disambiguation_att: Optional[str] = None
    # (optional) names for bits in the binary attributes field
    attributes: Optional[list[str]] = None
    # (optional) unique name used for deleting / replacing instructions.
    unique_name: Optional[str] = None
    # current privilege level. Valid values: 0, 3.
    current_privilege_level: int
    # ad-hoc categorization of instructions
    category: str
    # ad-hoc grouping of instructions.  If no ISA_SET is specified, this is used instead.
    extension: str
    exceptions: Optional[str] = None
    """(optional) name for the group of instructions that introduced this feature. On the older stuff, we used the 
    EXTENSION field but that got too complicated."""
    isa_set: Optional[str] = None
    real_opcode: bool = True
    # (optional) read/written flag bit values.
    flags: Optional[list[Flags]] = None
    # (optional) a hopefully useful comment
    comment: Optional[str] = None
    pattern: Pattern

    scalar: bool = False


class InstructionParser:
    lines: Iterable[str]

    _filters = ["INSTRUCTIONS()::", "XOP_INSTRUCTIONS()::", "AVX_INSTRUCTIONS()::", "EVEX_INSTRUCTIONS()::"]

    def __init__(self, instruction_lines: Iterable[str]):
        expanded_continuations = handle_continuations(instruction_lines)

        self.lines = iter([x for x in expanded_continuations if x not in self._filters and not x.startswith("UDELETE")])

    def parse(self) -> Optional[list[Instruction]]:
        """Parse an instruction definition, returning multiple if there
        is more than one PATTERN encountered, or None if there is nothing
        left in the reader."""

        open_curly = next(self.lines, None)
        if not open_curly:
            return None
        if open_curly != "{":
            fatal("ERROR: Expected instruction start, found: " + open_curly)

        instruction = Instruction()

        # Patterns, operands, and iforms are repeatable
        # They are all combined into the `Pattern` class and stored here.
        # A new `Instruction` will be created for each `Pattern` at the end of parsing.
        patterns = []

        current_pattern: Optional[Tuple[str, str, Optional[str]]] = None
        for line in self.lines:
            if line == "}":
                break

            key, val = key_value_pair(line)

            if val.startswith(":"):
                fatal("ERROR: Encountered double colon in instruction key value pair")

            match key:
                case "ICLASS":
                    instruction.name = val
                case "VERSION":
                    instruction.version = int(val)
                case "DISASM":
                    instruction.disambiguation = val
                case "DISASM_INTEL":
                    instruction.disambiguation_intel = val
                case "DISASM_ATTSV":
                    instruction.disambiguation_att = val
                case "ATTRIBUTES":
                    if not instruction.attributes:
                        instruction.attributes = [val]
                    else:
                        instruction.attributes.append(val)
                case "UNAME":
                    instruction.unique_name = val
                case "CPL":
                    instruction.current_privilege_level = int(val)
                case "CATEGORY":
                    instruction.category = val
                case "EXTENSION":
                    instruction.extension = val
                case "EXCEPTIONS":
                    instruction.exceptions = val
                case "ISA_SET":
                    instruction.isa_set = val
                case "REAL_OPCODE":
                    instruction.real_opcode = val == "Y"
                case "FLAGS":
                    instruction.flags = [Flags(x.strip()) for x in val.split(",")]
                case "COMMENT":
                    instruction.comment = val
                case "PATTERN":
                    if current_pattern:
                        patterns.append(Pattern(current_pattern[0], current_pattern[1], current_pattern[2]))
                    current_pattern = val, "", None
                case "OPERANDS":
                    if not current_pattern:
                        fatal("ERROR: Found key 'OPERAND' outside of pattern")
                    current_pattern = current_pattern[0], val, current_pattern[2]
                case "IFORM":
                    if not current_pattern:
                        fatal("ERROR: Found key 'IFORM' outside of pattern")
                    current_pattern = current_pattern[0], current_pattern[1], val
                case _:
                    fatal("ERROR: Unknown key in instruction definition: \"" + key + "\"")

        if current_pattern:
            patterns.append(Pattern(current_pattern[0], current_pattern[1], current_pattern[2]))

        if instruction.current_privilege_level not in [0, 3]:
            fatal("ERROR: Invalid CPL value: " + str(instruction.current_privilege_level))

        if instruction.attributes and "scalar" in instruction.attributes:
            instruction.scalar = True

        instructions = []
        for pat in patterns:
            copied = deepcopy(instruction)
            copied.pattern = pat
            instructions.append(copied)

        return instructions
