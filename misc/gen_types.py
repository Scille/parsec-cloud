#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


import argparse
import json
import re
from pathlib import Path

FIELDS_TEMPLATE = """
    @property
    def {name}(self) -> {type}: ..."""


def replace_type(typename):
    KEYWORD_REPLACEMENT = {
        "Bytes": "bytes",
        "Float": "float",
        "String": "str",
        "Boolean": "bool",
        "NonZeroInteger": "int",
        "IntegerBetween1And100": "int",
        "Size": "int",
        "Index": "int",
        "Version": "int",
    }
    return KEYWORD_REPLACEMENT.get(typename, typename)


def replace_wrapped_type(typename):
    REGEX_REPLACEMENTS = [
        (r"Option<(.*)>", "Optional[{type}]"),
        (r"List<(.*)>", "Tuple[{type}]"),
        (r"Map<(.*), (.*)>", "dict[{type}, {type2}]"),
    ]

    for (reg, format) in REGEX_REPLACEMENTS:
        match = re.match(reg, typename)

        if match:
            value = match.group(1)
            try:
                value2 = match.group(2)
                return format.format(
                    type=replace_wrapped_type(value), type2=replace_wrapped_type(value2)
                )
            except IndexError:
                return format.format(type=replace_wrapped_type(value))

    return replace_type(typename)


def replace_wrapped_type_signature(typename):
    REGEX_REPLACEMENTS = [
        (r"Option<(.*)>", "Optional[{type}]"),
        (r"List<(.*)>", "Iterable[{type}]"),
        (r"Map<(.*), (.*)>", "dict[{type}, {type2}]"),
    ]

    for (reg, format) in REGEX_REPLACEMENTS:
        match = re.match(reg, typename)

        if match:
            value = match.group(1)
            try:
                value2 = match.group(2)
                return format.format(
                    type=replace_wrapped_type_signature(value),
                    type2=replace_wrapped_type_signature(value2),
                )
            except IndexError:
                return format.format(type=replace_wrapped_type_signature(value))

    return replace_type(typename)


def init_parameters(other_fields):
    if not other_fields:
        return ""

    properties = [
        f"{field['name']}: {replace_wrapped_type_signature(field['type'])}"
        for field in other_fields
    ]

    result = ", "
    for i, var_name in enumerate(properties):
        if i < len(properties) - 1:
            result += f"{var_name}, "
        else:
            result += var_name

    return result


def gen_req(cmd_name, other_fields):
    result = f"""class {cmd_name}Req:
    def __init__(self{init_parameters(other_fields)}) -> None: ...

    def dump(self) -> bytes: ...
"""
    for r in other_fields:
        name, type, *_ = r.values()
        type = replace_wrapped_type(type)
        result += FIELDS_TEMPLATE.format(name=name, type=type)

    return result


def gen_rep(cmd_name, reps):
    result = f"""
class {cmd_name}Rep:
    def __init__(self) -> None: ...
    def dump(self) -> bytes: ...
    def load(buf: bytes) -> {cmd_name}Rep: ...
"""

    for rep in reps:
        status_name = snake_case_to_camel_case(rep["status"])
        rep_result = f"""
class {cmd_name}Rep{status_name}({cmd_name}Rep):
    def __init__(self{init_parameters(rep['other_fields'])}) -> None: ...
"""

        if rep["other_fields"]:
            for field in rep["other_fields"]:
                name, type, *_ = field.values()
                type = replace_wrapped_type(type)
                rep_result += FIELDS_TEMPLATE.format(name=name, type=type)

        result += rep_result

    result += gen_unknown_status_rep(cmd_name)

    return result


def gen_unknown_status_rep(cmd_name):
    result = f"""
class {cmd_name}RepUnknownStatus({cmd_name}Rep):
    def __init__(self, status: str, reason: Optional[str]) -> None: ...
    @property
    def status(self) -> str: ...
    @property
    def reason(self) -> Optional[str]: ...
"""
    return result


def snake_case_to_camel_case(name):
    words = name.split("_")

    if len(words) == 1:
        return words[0].title()

    return words[0].title() + "".join(word.title() for word in words[1:])


def gen_code(data):
    cmd_name = data["label"]
    req_data = data["req"]

    req_code = gen_req(cmd_name, req_data["other_fields"])
    rep_code = gen_rep(cmd_name, data["reps"])

    return f"{req_code}\n\n{rep_code}"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate code from templates")
    parser.add_argument(
        "input",
        metavar="EXTENSION_API_PATH",
        type=Path,
        help="Path to protocol.json file",
        nargs="+",
    )
    args = parser.parse_args()
    for file in args.input:
        data = json.loads(
            "\n".join([x for x in file.read_text().splitlines() if not x.strip().startswith("//")])
        )
        print(gen_code(data))
