from pathlib import Path
from typing import Iterable

from generators.asm_specs.util import generated_directory_for

from generators.asm_specs.x86 import global_defs
from generators.asm_specs.x86.instruction import InstructionParser, Instruction
from generators.asm_specs.x86.map_info import MapInfoParser
from generators.asm_specs.x86.text_utils import remove_comment_from_line
from generators.asm_specs.x86.width import Width
from generators.asm_specs.x86.xtype import XType

GENERATED_DIR = generated_directory_for("x86")
DGEN_DIR = Path(GENERATED_DIR).joinpath("dgen")


def parse_instructions_from(path: Path) -> list[Instruction]:
    print("INFO: Parsing Intel XED instruction definitions from: " + str(path.resolve()))

    instructions = []
    unstable_instructions = 0
    skipped_instructions = 0
    definitions_count = 0
    with open(path, "r") as file:
        parser = InstructionParser(iter(file.readlines()))
        while True:
            parsed_instructions = parser.parse()
            if not parsed_instructions:
                break
            if not parsed_instructions[0].real_opcode:
                unstable_instructions += 1
                continue
            if parsed_instructions[0].extension == "3DNOW" or parsed_instructions[0].category == "3DNOW":
                skipped_instructions += 1
                continue
            definitions_count += 1
            instructions.extend(parsed_instructions)
    print("INFO: Parsed {} instructions ({} definitions, {} unstable, {} skipped)".format(len(instructions), definitions_count,
                                                                              unstable_instructions, skipped_instructions))
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
            name = tokens.pop(0).strip()
            global_defs.states[name] = tokens[0].strip()
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


def parse_map_descriptions_from(path: Path):
    print("INFO: Parsing Intel XED map descriptions from: " + str(path.resolve()))
    with open(path, "r") as file:
        lines: Iterable[str] = map(remove_comment_from_line, iter(file.readlines()))
        parser = MapInfoParser(lines)
        while True:
            parsed_map_info = parser.parse()
            if not parsed_map_info:
                break
            global_defs.map_info.append(parsed_map_info)
    print("INFO: Parsed " + str(len(global_defs.map_info)) + " map descriptions")


def main():
    all_widths_path: Path = DGEN_DIR.joinpath("all-widths.txt")
    parse_widths_from(all_widths_path)

    all_states_path: Path = DGEN_DIR.joinpath("all-state.txt")
    parse_states_from(all_states_path)

    all_element_types_path: Path = DGEN_DIR.joinpath("all-element-types.txt")
    parse_xtypes_from(all_element_types_path)

    all_map_descriptions: Path = DGEN_DIR.joinpath("all-map-descriptions.txt")
    parse_map_descriptions_from(all_map_descriptions)

    all_dec_intructions_path: Path = DGEN_DIR.joinpath("all-dec-instructions.txt")
    instructions = parse_instructions_from(all_dec_intructions_path)


if __name__ == "__main__":
    main()
