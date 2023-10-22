from generators.asm_specs.x86.parsers.map_info import MapInfo
from generators.asm_specs.x86.parsers.register import Registers
from generators.asm_specs.x86.parsers.width import Width
from generators.asm_specs.x86.parsers.xtype import XType

widths: dict[str, Width] = {}
states: dict[str, str] = {}
xtypes: dict[str, XType] = {}
map_info: [MapInfo] = []
registers: Registers = {}
