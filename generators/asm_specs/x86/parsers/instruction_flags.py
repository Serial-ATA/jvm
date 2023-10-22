import sys
from enum import StrEnum
from typing import Optional

from generators.asm_specs.util import fatal

import re


def valid_flag_semantics(s: str) -> bool:
    return s == "MAY" or "MUST" or "READONLY"


def valid_flag_qualifier(s: str) -> bool:
    return s == "REP" or "NOREP" or "IMM0" or "IMM1" or "IMMx"


class FlagSemantics(StrEnum):
    MAY = "MAY"
    MUST = "MUST"
    READONLY = "READONLY"


class FlagQualifier(StrEnum):
    REP = "REP"
    NOREP = "NOREP"
    IMM0 = "IMM0"
    IMM1 = "IMM1"
    IMMx = "IMMx"


class FlagSet(object):
    field_pairs = [('cf', 1), ('must_be_1', 1),
                   ('pf', 1), ('must_be_0a', 1),
                   ('af', 1), ('must_be_0b', 1),
                   ('zf', 1), ('sf', 1),
                   ('tf', 1), ('_if', 1),
                   ('df', 1), ('of', 1),
                   ('iopl', 2),  # 2b wide field
                   ('nt', 1), ('must_be_0c', 1),
                   ('rf', 1), ('vm', 1),
                   ('ac', 1), ('vif', 1),
                   ('vip', 1), ('id', 1),
                   ('must_be_0d', 2),
                   ('must_be_0e', 4),

                   # not part of [er]flags, just stored that way for convenience.
                   ('fc0', 1),
                   ('fc1', 1),
                   ('fc2', 1),
                   ('fc3', 1)]
    field_names = [x[0] for x in field_pairs]

    def __init__(self, very_technically_accurate: bool = False):
        for (f, w) in FlagSet.field_pairs:
            if very_technically_accurate and f.startswith('must_be_1'):
                setattr(self, f, 1)
            else:
                setattr(self, f, 0)

    def set(self, fld: str, val: int = 1):
        if fld == 'if':
            fld = '_if'  # recode this one to avoid keyword clash
        if fld == 'iopl':
            val = 3  # turn on both bits for IOPL. Just a convention

        if fld in FlagSet.field_names:
            setattr(self, fld, val)
        else:
            fatal("ERROR: Bad flags field name: " + fld)

    def as_integer(self) -> int:
        s = 0
        n = 0
        for (f, w) in FlagSet.field_pairs:
            mask = (1 << w) - 1
            s = s | (getattr(self, f) & mask) << n
            n = n + w
        return s

    def as_hex(self):
        return hex(self.as_integer())


class FlagAction(object):
    """Simple flag/actions pairs. If the input is 'nothing' we do not have any flag action"""

    valid_flag_actions = ['mod', 'tst', 'u', '0', '1', 'ah', 'pop']  # FIXME: x86 specific

    def __init__(self, s):
        self.flag = None
        self.action = None  # Could be mod, tst, u, 0, 1, ah, pop
        if s != 'nothing':
            (self.flag, self.action) = s.lower().split('-')
            if self.action not in FlagAction.valid_flag_actions:
                fatal("ERROR: Invalid flag_action_t in " + s)

    def __str__(self):
        if self.flag is None:
            return 'nothing'
        return "%s-%s" % (self.flag, self.action)

    def is_nothing(self):
        if self.flag is None:
            return True
        return False

    def reads_flag(self):
        if self.action == 'tst':
            return True
        return False

    def writes_flag(self):
        if self.action != 'tst':
            return True
        return False

    def makes_flag_undefined(self):
        return self.action == 'u'


class Flags:
    semantics: FlagSemantics
    qualifier: Optional[FlagQualifier] = None

    flag_actions: list[FlagAction] = []
    read_set: FlagSet = FlagSet()
    write_set: FlagSet = FlagSet()
    undefined_set: FlagSet = FlagSet()

    _flag_pattern = re.compile(r"\s*(?P<qualifiers>.*)\s+[\[](?P<flags>.*)[\]]")

    def __init__(self, flags: str):
        matches = self._flag_pattern.search(flags)
        if not matches:
            fatal("ERROR: Invalid flag string: " + flags)

        flags_input = matches.group("flags").strip().split()
        qualifiers = matches.group("qualifiers").strip().split()

        if len(qualifiers) == 0 or len(qualifiers) > 2:
            fatal("ERROR: Wrong number of flag qualifiers: " + str(len(qualifiers)))

        if len(qualifiers) > 1:
            if not valid_flag_qualifier(qualifiers[0]):
                fatal("ERROR: Invalid flag qualifier: " + qualifiers[0])
            self.qualifier = FlagQualifier(qualifiers.pop(0))

        specifier_str: str = qualifiers[0]
        if not valid_flag_semantics(specifier_str):
            fatal("ERROR: Invalid flags specification: " + flags)

        self.semantics = FlagSemantics(specifier_str)

        for flag_action_str in flags_input:
            fa = FlagAction(flag_action_str)
            self.flag_actions.append(fa)
            if fa.flag:
                if fa.reads_flag():
                    self.read_set.set(fa.flag)
                if fa.writes_flag():
                    self.write_set.set(fa.flag)
                if fa.makes_flag_undefined():
                    self.undefined_set.set(fa.flag)
            else:
                sys.stderr.write("WARN: Unknown flag: {}\n".format(flag_action_str))

    def is_nothing(self) -> bool:
        return len(self.flag_actions) == 1 and self.flag_actions[0].is_nothing()

    def reads_flags(self) -> bool:
        for action in self.flag_actions:
            if action.reads_flag():
                return True
        return False

    def writes_flags(self) -> bool:
        for action in self.flag_actions:
            if action.writes_flag():
                return True
        return False

    def conditional_writes_flags(self) -> bool:
        return self.writes_flags() and self.semantics == FlagSemantics.MAY

    def is_x86(self) -> bool:
        """Return True if any of the flags are x86 flags. False otherwise"""

        for action in self.flag_actions:
            s = action.flag
            if s != 'fc0' and s != 'fc1' and s != 'fc2' and s != 'fc3':
                return True
        return False

    def rw_action(self) -> str:
        """Return one of: r, w, cw, rcw or rw. This is the r/w action
        for a rFLAGS() NTLUF."""

        r = ''
        w = ''
        c = ''
        has_nothing_record = False
        for action in self.flag_actions:
            if action.is_nothing():
                has_nothing_record = True
            if action.reads_flag():
                r = 'r'
            if action.writes_flag():
                w = 'w'
                if self.conditional_writes_flags():
                    # things that are conditional writes are also writes
                    c = 'c'

        if has_nothing_record:
            c = 'c'
        retval = "%s%s%s" % (r, c, w)
        return retval


class FlagCollection:
    """A container to act on sets of Flags"""

    flags: list[Flags]

    def __init__(self, flags: list[Flags]):
        self.flags = flags

    def reads_flags(self):
        for flags in self.flags:
            if flags.reads_flags():
                return True
        return False

    def writes_flags(self):
        for flags in self.flags:
            if flags.writes_flags():
                return True
        return False

    def x86_flags(self):
        """Return True if any flags are x86 flags"""

        for flags in self.flags:
            if flags.is_x86():
                return True
        return False

    def rw_action(self):
        """Return one of: r, w, cw, rcw or rw. This is the r/w action
        for a rFLAGS() NTLUF."""

        r = ''
        w = ''
        c = ''
        has_nothing_record = False
        for flags in self.flags:
            if flags.is_nothing():
                has_nothing_record = True
            if flags.reads_flags():
                r = 'r'
            if flags.writes_flags():
                w = 'w'
                if flags.conditional_writes_flags():
                    # things that are conditional writes are also writes
                    c = 'c'

        if has_nothing_record:
            c = 'c'
        retval = "%s%s%s" % (r, c, w)
        return retval
