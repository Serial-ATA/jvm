import re
from enum import Enum
from typing import Optional

from generators.asm_specs.x86 import global_defs
from generators.asm_specs.x86.operand import Operand
from generators.asm_specs.x86.text_utils import multiform_numeric


class Space(Enum):
    LEGACY = 0
    VEX = 1
    EVEX = 2
    XOP = 3

    def is_legacy(self):
        return self == Space.LEGACY

    def is_vex(self):
        return self == Space.VEX

    def is_evex(self):
        return self == Space.EVEX

    def is_xop(self):
        return self == Space.XOP


def get_space(pat: str) -> Space:
    match pat:
        case "VEXVALID=1":
            return Space.VEX
        case "VEXVALID=2":
            return Space.EVEX
        case "VEXVALID=3":
            return Space.XOP
        case _:
            return Space.LEGACY


MAP_PATTERN = re.compile(r'MAP=(?P<map>[0-9]+)')
OSZ_PATTERN = re.compile(r'OSZ=(?P<prefix>[01])')
REP_PATTERN = re.compile(r'REP=(?P<prefix>[0-3])')
VEX_PREFIX_PATTERN = re.compile(r'VEX_PREFIX=(?P<prefix>[0-9])')
REXW_PATTERN = re.compile(r'REXW=(?P<rexw>[01])')
REG_PATTERN = re.compile(r'REG[\[](?P<reg>[b01]+)]')
RM_PATTERN = re.compile(r'RM[\[](?P<rm>[b01]+)]')
RM_VALUE_SPECIFIED_PATTERN = re.compile(r'RM=(?P<rm>[0-9]+)')
MOD_PATTERN = re.compile(r'MOD[\[](?P<mod>[b01]+)]')
MOD_MEM_REQUIRED_PATTERN = re.compile(r'MOD!=3')
MOD_REG_REQUIRED_PATTERN = re.compile(r'MOD=3')

NOT64_PATTERN = re.compile(r'MODE!=2')
MODE_PATTERN = re.compile(r' MODE=(?P<mode>[012]+)')


class Pattern:
    # the sequence of bits and nonterminals used to decode/encode an instruction.
    pattern: [str] = []
    # the operands, typicall registers,  memory operands and pseudo-resources.
    operands: Optional[list[Operand]] = None
    """(optional) a name for the pattern that starts with the iclass and bakes in the operands. If omitted, 
    xed tries to generate one. We often add custom suffixes to these to disambiguate certain combinations."""
    iform: Optional[str] = None

    space: Space
    opcode: str
    map_num: str | int = 0

    no_prefixes_allowed: bool = False
    ozs_required: bool = False
    f2_required: bool = False
    f3_required: bool = False

    rexw_prefix: Optional[int] = None
    reg_required: Optional[int] = None
    rm_required: Optional[int] = None
    mod_required: Optional[str | int] = None

    has_modrm: bool = False
    mode_restriction: Optional[str | int] = None
    easz: str = "aszall"

    default_64bit: bool = False

    vl: Optional[str] = None

    def _find_legacy_map_opcode(self):
        opcode, map_num = self.pattern[0], 0

        for info in global_defs.map_info:
            if info.space != Space.LEGACY:
                continue

            if self.pattern[0] == info.escape:
                if info.opcode and info.opcode == self.pattern[1]:
                    map_num = multiform_numeric(info.map_id)
                    opcode = self.pattern[info.opcode_pos]
                    break

                if not info.opcode:
                    if info.name == 'amd-3dnow':
                        map_num = 'AMD3DNOW'
                    else:
                        map_num = multiform_numeric(info.map_id)
                    opcode = self.pattern[info.opcode_pos]
                    break

        self.opcode = opcode
        self.map_num = map_num

    def _get_vl(self):
        if self.space != Space.VEX and self.space != Space.EVEX:
            return

        if "VL=0" in self.pattern or "VLX=1" in self.pattern:
            self.vl = "128"
            return
        if "VL=1" in self.pattern or "VLX=2" in self.pattern:
            self.vl = "256"
            return
        if "VL=2" in self.pattern or "VLX=3" in self.pattern or "FIX_ROUND_LEN512" in self.pattern:
            self.vl = "512"
            return

        match self.space:
            case Space.VEX:
                self.vl = "LIG"
            case Space.EVEX:
                self.vl = "LLIG"

    def __init__(self, pattern: str, operands_str: str, iform: Optional[str]):
        # Expand state macros we pulled from `all-state.txt`
        expanded_patterns = []
        for pattern_field in pattern.split():
            expanded = global_defs.states.get(pattern_field)
            if expanded:
                # This pattern field had a macro expansion
                expanded_patterns.append(expanded)
            else:
                expanded_patterns.append(pattern_field)

        self.pattern = expanded_patterns

        self.iform = iform
        if operands_str != "":
            operands = []
            for operand_str in operands_str.split(" "):
                operand = Operand(operand_str)
                if operand.is_visible() and operand.action != "":
                    operands.append(operand)
            if len(operands) > 1:
                self.operands = operands

        self.space = get_space(self.pattern[0])
        if self.space == Space.LEGACY:
            self._find_legacy_map_opcode()
        else:
            self.opcode = self.pattern[1]

        osz = OSZ_PATTERN.search(pattern)
        if osz and osz.group("prefix") == "1":
            self.ozs_required = True

        rep = REP_PATTERN.search(pattern)
        if rep:
            prefix = rep.group("prefix")
            if prefix == "0" and not self.ozs_required:
                self.no_prefixes_allowed = True
            elif prefix == "2":
                self.f2_required = True
            elif prefix == "3":
                self.f3_required = True

        if self.space != Space.LEGACY:
            vexp = VEX_PREFIX_PATTERN.search(pattern)
            if vexp:
                if vexp.group("prefix") == '0':
                    self.no_prefixes_allowed = True
                elif vexp.group("prefix") == '1':
                    self.osz_required = True
                elif vexp.group("prefix") == '2':
                    self.f2_required = True
                elif vexp.group("prefix") == '3':
                    self.f3_required = True

        rexw = REXW_PATTERN.search(pattern)
        if rexw:
            self.rexw_prefix = multiform_numeric(rexw.group("rexw"))

        reg = REG_PATTERN.search(pattern)
        if reg:
            self.reg_required = multiform_numeric(reg.group("reg"))

        rm = RM_PATTERN.search(pattern)
        if rm:
            self.rm_required = multiform_numeric(rm.group("rm"))
        rm = RM_VALUE_SPECIFIED_PATTERN.search(pattern)
        if rm:
            self.rm_required = multiform_numeric(rm.group("rm"))

        mod = MOD_PATTERN.search(pattern)
        if mod:
            self.mod_required = multiform_numeric(mod.group("mod"))
        mod = MOD_MEM_REQUIRED_PATTERN.search(pattern)
        if mod:
            self.mod_required = "00/01/10"
        mod = MOD_REG_REQUIRED_PATTERN.search(pattern)
        if mod:
            self.mod_required = 3

        if "MODRM" in self.pattern or (self.reg_required or self.mod_required):
            self.has_modrm = True
        if self.rm_required and "SRM[" not in self.pattern:
            self.has_modrm = True

        if NOT64_PATTERN.search(pattern):
            self.mode_restriction = "not64"
        else:
            mode = MODE_PATTERN.search(pattern)
            if mode:
                self.mode_restriction = multiform_numeric(mode.group("mode"))

        if "EASZ=1" in self.pattern:
            self.easz = "a16"
        elif "EASZ=2" in self.pattern:
            self.easz = "a32"
        elif "EASZ=3" in self.pattern:
            self.easz = "a64"
            self.mode_restriction = 2
        elif "EASZ!=1" in self.pattern:
            self.easz = "asznot16"

        if 'DF64()' in self.pattern or 'CR_WIDTH()' in self.pattern:
            self.default_64bit = True

        self._get_vl()
