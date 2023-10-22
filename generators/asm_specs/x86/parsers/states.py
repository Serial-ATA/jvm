import re
from typing import Iterable, Tuple, Optional

STATE_REGEX = re.compile("^\s*(?P<macro>\w+)\s+(?P<replacement>.+?)\s*$")


class StateParser:
    """
    Parser for the "state" definitions from `all-state.txt`.

    A state definition is simply a text replacement macro.

    Example:

        "not64                  MODE!=2"

        Anywhere `not64` is encountered, it should be replaced with `MODE!=2`.
    """

    lines: Iterable[str]

    def __init__(self, lines: Iterable[str]):
        self.lines = lines

    def next(self) -> Optional[Tuple[str, str]]:
        for line in self.lines:
            if len(line) == 0:
                continue
            matches = STATE_REGEX.match(line)
            if not matches:
                continue
            return matches["macro"], matches["replacement"]
        return None
