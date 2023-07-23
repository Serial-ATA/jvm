from pathlib import Path
from typing import Optional, Iterable, Tuple

from generators.asm_specs.util import generated_directory_for, fatal

import re

from generators.asm_specs.x86 import global_defs
from generators.asm_specs.x86.flag import Flags
from generators.asm_specs.x86.operand import Operand
from generators.asm_specs.x86.width import Width
from generators.asm_specs.x86.xtype import XType

GENERATED_DIR = generated_directory_for("x86")
DGEN_DIR = Path(GENERATED_DIR).joinpath("dgen")

comment_regex = re.compile(r'#.*$')


def remove_comment_from_line(line: str) -> str:
    return comment_regex.sub("", line).strip()


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


def key_value_pair(line: str) -> Tuple[str, str]:
    key: str
    val: str
    key, val = line.split(":", 1)
    return key.strip(), val.strip()


def handle_continuations(lines: Iterable[str]) -> list[str]:
    new_lines: list[str] = []
    current_line: str = ""
    for line in lines:
        line = remove_comment_from_line(line)
        if line == "":
            continue
        if line.endswith("\\"):
            current_line += line[:-1]
            continue
        new_lines.append(current_line + line)
        current_line = ""
    del lines
    return new_lines


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
    patterns: list[Pattern] = []


class InstructionParser:
    lines: Iterable[str]

    _filters = ["INSTRUCTIONS()::", "XOP_INSTRUCTIONS()::", "AVX_INSTRUCTIONS()::", "EVEX_INSTRUCTIONS()::"]

    def __init__(self, instruction_lines: Iterable[str]):
        expanded_continuations = handle_continuations(instruction_lines)

        self.lines = iter([x for x in expanded_continuations if x not in self._filters and not x.startswith("UDELETE")])

    def parse(self) -> Optional[Instruction]:
        open_curly = next(self.lines, None)
        if not open_curly:
            return None
        if open_curly != "{":
            fatal("ERROR: Expected instruction start, found: " + open_curly)

        instruction = Instruction()

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
                        instruction.patterns.append(Pattern(current_pattern[0], current_pattern[1], current_pattern[2]))
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
            instruction.patterns.append(Pattern(current_pattern[0], current_pattern[1], current_pattern[2]))

        if instruction.current_privilege_level not in [0, 3]:
            fatal("ERROR: Invalid CPL value: " + str(instruction.current_privilege_level))

        return instruction


def parse_instructions_from(path: Path) -> list[Instruction]:
    print("INFO: Parsing Intel XED instruction definitions from: " + str(path.resolve()))

    instructions = []
    unstable_instructions = 0
    with open(path, "r") as file:
        parser = InstructionParser(iter(file.readlines()))
        while True:
            instruction = parser.parse()
            if not instruction:
                break
            if not instruction.real_opcode:
                unstable_instructions += 1
                continue
            instructions.append(instruction)
    print("INFO: Parsed {} instruction definitions ({} unstable)".format(len(instructions), unstable_instructions))
    return instructions


def parse_widths_from(path: Path):
    print("INFO: Parsing Intel XED width definitions from: " + str(path.resolve()))
    with open(path, "r") as file:
        lines: Iterable[str] = iter(file.readlines())
        for line in map(remove_comment_from_line, lines):
            if len(line) == 0:
                continue

            width = Width(line)
            global_defs.widths[width.name] = width
    print("INFO: Parsed " + str(len(global_defs.widths)) + " width definitions")


def parse_states_from(path: Path):
    print("INFO: Parsing Intel XED state definitions from: " + str(path.resolve()))
    with open(path, "r") as file:
        lines: Iterable[str] = iter(file.readlines())
        for line in map(remove_comment_from_line, lines):
            if len(line) == 0:
                continue

            tokens = line.split(" ", 1)
            name = tokens.pop(0)
            global_defs.states[name] = tokens[0]
    print("INFO: Parsed " + str(len(global_defs.states)) + " state definitions")


def parse_xtypes_from(path: Path):
    print("INFO: Parsing Intel XED XType definitions from: " + str(path.resolve()))
    with open(path, "r") as file:
        lines: Iterable[str] = iter(file.readlines())
        for line in map(remove_comment_from_line, lines):
            if len(line) == 0:
                continue

            xtype = XType(line)
            global_defs.xtypes[xtype.name] = xtype
    print("INFO: Parsed " + str(len(global_defs.xtypes)) + " XType definitions")


def main():
    all_widths_path: Path = DGEN_DIR.joinpath("all-widths.txt")
    parse_widths_from(all_widths_path)

    all_states_path: Path = DGEN_DIR.joinpath("all-state.txt")
    parse_states_from(all_states_path)

    all_element_types_path: Path = DGEN_DIR.joinpath("all-element-types.txt")
    parse_xtypes_from(all_element_types_path)

    all_dec_intructions_path: Path = DGEN_DIR.joinpath("all-dec-instructions.txt")
    instructions = parse_instructions_from(all_dec_intructions_path)


if __name__ == "__main__":
    main()
