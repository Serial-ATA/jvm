from typing import Iterable, Tuple

import re

comment_regex = re.compile(r'#.*$')


def remove_comment_from_line(line: str) -> str:
    return comment_regex.sub("", line).strip()


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
