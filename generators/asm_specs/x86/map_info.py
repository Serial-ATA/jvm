import shlex
from enum import StrEnum
from typing import Optional, Iterable

from generators.asm_specs.util import fatal
from generators.asm_specs.x86.pattern import Space, get_space


class VariableBool(StrEnum):
    YES = "yes"
    NO = "no"
    VARIABLE = "var"


MAP_INFO_FIELD_COUNT: int = 10


class MapInfo:
    name: str
    space: Space
    escape: Optional[int] = None
    opcode: Optional[int] = None
    map_id: str
    has_modrm: VariableBool
    has_disp: VariableBool
    imm: str
    opcode_position: int
    pattern: str

    priority: int


class MapInfoParser:
    lines: Iterable[str]

    def __init__(self, lines: Iterable[str]):
        self.lines = iter([x for x in lines if x != ""])

    def parse(self) -> Optional[MapInfo]:
        """Parses a map description definition"""

        line = next(self.lines, None)
        if not line:
            return None

        columns = shlex.split(line.strip())
        if len(columns) != MAP_INFO_FIELD_COUNT:
            fatal("ERROR: Wrong number of columns in map description line: " + line)

        map_info = MapInfo()

        columns = iter(columns)
        map_info.name = next(columns)
        map_info.space = get_space(next(columns))

        escape = next(columns)
        if escape != "N/A":
            if map_info.space != Space.LEGACY:
                fatal("ERROR: Encountered legacy escape in non-legacy map description")
            map_info.escape = int(escape, 16)

        opcode = next(columns)
        if opcode != "N/A":
            if map_info.space != Space.LEGACY:
                fatal("ERROR: Encountered legacy opcode in non-legacy map description")
            map_info.opcode = int(escape, 16)

        map_info.map_id = next(columns)

        modrm = next(columns)
        if modrm not in ["yes", "no", "var"]:
            fatal("ERROR: Invalid descriptor for modrm: " + modrm)
        map_info.has_modrm = VariableBool(modrm)

        disp = next(columns)
        if disp not in ["yes", "no", "var"]:
            fatal("ERROR: Invalid descriptor for disp: " + disp)
        map_info.has_disp = VariableBool(disp)

        map_info.imm = next(columns)
        if map_info.imm not in ["var", "0", "1", "2", "3", "4"]:
            fatal("ERROR: Invalid imm specifier: " + map_info.imm)

        map_info.opcode_position = int(next(columns))

        map_info.pattern = next(columns)

        map_info.priority = 100 - len(map_info.pattern)
        return map_info
