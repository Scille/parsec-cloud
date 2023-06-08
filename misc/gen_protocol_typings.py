#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Callable

BASEDIR = Path(__file__).parent.parent


def snake_case_to_camel_case(name: str) -> str:
    words = name.split("_")

    if len(words) == 1:
        return words[0].title()

    return words[0].title() + "".join(word.title() for word in words[1:])


def camel_case_to_snake_case(name: str) -> str:
    out = ""
    for c in name:
        if c.isupper():
            out += "_"
        out += c.lower()
    if out.startswith("_"):
        return out[1:]
    return out


def cook_field_type(
    raw_type: str, field_parse_callback: Callable[[str | None, bool | None], None]
) -> str:
    for candidate, py_type_name in [
        ("Boolean", "bool"),
        ("String", "str"),
        ("Bytes", "bytes"),
        ("Integer", "int"),
        ("Float", "float"),
        ("Version", "int"),
        ("Size", "int"),
        ("Index", "int"),
        ("NonZeroInteger", "int"),
        ("IntegerBetween1And100", "int"),
    ]:
        if raw_type == candidate:
            return py_type_name

    def _test_container(container: str):
        assert container.endswith("<")
        if raw_type.startswith(container):
            assert raw_type.endswith(">")
            return raw_type[len(container) : -1]

    is_list = _test_container("List<")
    if is_list:
        return f"list[{cook_field_type(is_list, field_parse_callback)}]"

    is_set = _test_container("Set<")
    if is_set:
        return f"set[{cook_field_type(is_set, field_parse_callback)}]"

    is_required_option = _test_container("RequiredOption<")
    if is_required_option:
        return f"{cook_field_type(is_required_option, field_parse_callback)} | None"

    is_non_required_option = _test_container("NonRequiredOption<")
    if is_non_required_option:
        return f"{cook_field_type(is_non_required_option, field_parse_callback)} | None"

    is_option = _test_container("Option<")
    if is_option:
        return f"{cook_field_type(is_option, field_parse_callback)} | None"

    is_map = _test_container("Map<")
    if is_map:
        key_type, value_type = [
            cook_field_type(x.strip(), field_parse_callback) for x in is_map.split(",", 1)
        ]
        return f"dict[{key_type}, {value_type}]"

    if raw_type.startswith("("):
        assert raw_type.endswith(")")
        items_types = [
            cook_field_type(x.strip(), field_parse_callback) for x in raw_type[1:-1].split(",")
        ]
        return f"tuple[{', '.join(items_types)}]"

    field_parse_callback(raw_type, None)
    return raw_type


def cook_field(
    field: dict, field_parse_callback: Callable[[str | None, bool | None], None]
) -> tuple[str, str]:
    field_parse_callback(None, "introduced_in" in field)
    return (field["name"], cook_field_type(field["type"], field_parse_callback))


def gen_req(
    req: dict,
    collected_items: list[str],
    field_parse_callback: Callable[[str | None, bool | None], None],
) -> str:
    collected_items.append("Req")

    fields = [cook_field(f, field_parse_callback) for f in req.get("fields", ())]
    unit_type_name = req.get("unit")
    if unit_type_name:
        fields.append(("unit", unit_type_name))

    code = f"""class Req:
    def __init__(self, {','.join(n + ': ' + t for n, t in fields)}) -> None: ...

    def dump(self) -> bytes: ...
"""
    for field_name, field_type in fields:
        code += f"""
    @property
    def {field_name}(self) -> {field_type}: ...
"""

    return code


def gen_reps(
    reps: dict,
    collected_items: list[str],
    field_parse_callback: Callable[[str | None, bool | None], None],
) -> str:
    collected_items.append("Rep")
    collected_items.append("RepUnknownStatus")

    code = """class Rep:
    @staticmethod
    def load(raw: bytes) -> Rep: ...
    def dump(self) -> bytes: ...


class RepUnknownStatus(Rep):
    def __init__(self, status: str, reason: str | None) -> None: ...

    @property
    def status(self) -> str: ...

    @property
    def reason(self) -> str | None: ...
"""

    for rep in reps:
        rep_cls_name = f"Rep{snake_case_to_camel_case(rep['status'])}"
        collected_items.append(rep_cls_name)

        fields = [cook_field(f, field_parse_callback) for f in rep.get("fields", ())]
        unit_type_name = rep.get("unit")
        if unit_type_name:
            fields.append(("unit", unit_type_name))

        code += f"""

class {rep_cls_name}(Rep):
    def __init__(self, {','.join(n + ': ' + t for n, t in fields)}) -> None: ...
"""

        for field_name, field_type in fields:
            code += f"""
    @property
    def {field_name}(self) -> {field_type}: ...
"""

    return code


def gen_nested_type(
    nested_type: dict,
    collected_items: list[str],
    field_parse_callback: Callable[[str | None, bool | None], None],
) -> str:
    if "variants" in nested_type:
        return gen_nested_type_variant(nested_type, collected_items, field_parse_callback)
    else:
        return gen_nested_type_struct(nested_type, collected_items, field_parse_callback)


def gen_nested_type_variant(
    nested_type: dict,
    collected_items: list[str],
    field_parse_callback: Callable[[str | None, bool | None], None],
) -> str:
    class_name = nested_type["name"]
    collected_items.append(class_name)

    # Is a literal variant ?
    if all(not variants.get("fields", ()) for variants in nested_type["variants"]):
        code = f"""class {class_name}:
    VALUES: tuple[{class_name}]
"""
        for variant in nested_type["variants"]:
            name = camel_case_to_snake_case(variant["name"]).upper()
            code += f"    {name}: {class_name}\n"

        code += f"""
    @classmethod
    def from_str(cls, value: str) -> {class_name}: ...
    @property
    def str(self) -> str: ...
"""

    else:
        code = f"""class {class_name}:
    pass
"""
        for variant in nested_type["variants"]:
            subclass_name = f"{class_name}{variant['name']}"
            collected_items.append(subclass_name)
            fields = [cook_field(f, field_parse_callback) for f in variant.get("fields", ())]

            code += f"""
class {subclass_name}({class_name}):
    def __init__(self, {','.join(n + ': ' + t for n, t in fields)}) -> None: ...
"""

            for field_name, field_type in fields:
                code += f"""
    @property
    def {field_name}(self) -> {field_type}: ...
"""

    return code


def gen_nested_type_struct(
    nested_type: dict,
    collected_items: list[str],
    field_parse_callback: Callable[[str | None, bool | None], None],
) -> str:
    class_name = nested_type["name"]
    collected_items.append(class_name)

    fields = [cook_field(f, field_parse_callback) for f in nested_type.get("fields", ())]

    code = f"""class {class_name}:
    def __init__(self, {','.join(n + ': ' + t for n, t in fields)}) -> None: ...
"""

    for field_name, field_type in fields:
        code += f"""
    @property
    def {field_name}(self) -> {field_type}: ...
"""

    return code


def gen_single_version_cmd_spec(
    output_dir: Path,
    family_name: str,
    spec: dict,
    v_version: str,
    collected_items: dict[str, dict[str, list[str]]],
) -> bool:
    # Protocol code generator force separation if a field is marked `introduced_in`
    can_be_reused = True

    need_import_types = set()

    def _field_parse_callback(field_type: str | None, introduced_in: bool | None):
        nonlocal can_be_reused
        if introduced_in is True:
            can_be_reused = False
        if field_type is not None:
            for nested_type in spec.get("nested_types", ()):
                if field_type == nested_type["name"]:
                    return
            need_import_types.add(field_type)

    cmd_name = spec["req"]["cmd"]

    cmd_collected_items: list[str] = []
    collected_items[v_version][cmd_name] = cmd_collected_items
    version_dir = output_dir / family_name / v_version
    version_dir.mkdir(exist_ok=True, parents=True)

    typing_file = version_dir / f"{cmd_name}.pyi"

    code = ""
    for nested_type in spec.get("nested_types", ()):
        code += gen_nested_type(nested_type, cmd_collected_items, _field_parse_callback)
        code += "\n\n"
    code += gen_req(spec["req"], cmd_collected_items, _field_parse_callback)
    code += "\n\n"
    code += gen_reps(spec["reps"], cmd_collected_items, _field_parse_callback)

    code_prefix = "from __future__ import annotations\n\n"
    if need_import_types:
        code_prefix += f"from parsec._parsec import {', '.join(need_import_types)}\n\n"
    code_prefix += "\n"

    typing_file.write_text(code_prefix + code, encoding="utf8")

    return can_be_reused


def gen_pyi_file_for_cmd_spec(
    output_dir: Path,
    family_name: str,
    spec: dict,
    collected_items: dict[str, dict[str, list[str]]],
) -> None:
    first_version = None
    other_versions: list[str] = []
    for version in sorted(spec["major_versions"]):
        v_version = f"v{version}"
        collected_items.setdefault(v_version, {})
        if first_version:
            other_versions.append(v_version)
        else:
            first_version = v_version
    assert first_version is not None

    can_be_reused = gen_single_version_cmd_spec(
        output_dir, family_name, spec, first_version, collected_items
    )
    if not can_be_reused:
        for other_version in other_versions:
            gen_single_version_cmd_spec(
                output_dir, family_name, spec, other_version, collected_items
            )

    else:
        cmd_name = spec["req"]["cmd"]
        for other_version in other_versions:
            reexported_items = collected_items[first_version][cmd_name]
            collected_items[other_version][cmd_name] = reexported_items
            code = (
                f"""
from ..{first_version}.{cmd_name} import {', '.join(reexported_items)}


__all__ = ["""
                + ", ".join(f'"{x}"' for x in reexported_items)
                + "]\n"
            )

            version_dir = output_dir / family_name / other_version
            version_dir.mkdir(exist_ok=True, parents=True)
            typing_file = version_dir / f"{cmd_name}.pyi"
            typing_file.write_text(code, encoding="utf8")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate code from templates")
    parser.add_argument(
        "--input",
        type=Path,
        help="Path to api schema folder",
        default=BASEDIR / "oxidation/libparsec/crates/protocol/schema/",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="Path to protocol typing dir",
        default=BASEDIR / "server/parsec/_parsec_pyi/protocol",
    )
    args = parser.parse_args()
    # {<family>: {<version>: {<cmd>: [<collected items>]}}}
    collected_items: dict[str, dict[str, dict[str, list[str]]]] = {}
    for family in args.input.iterdir():
        family_name = family.name
        collected_items[family_name] = {}
        for cmd in family.glob("*.json5"):
            cmd_specs = json.loads(
                "\n".join(
                    [
                        x
                        for x in cmd.read_text(encoding="utf8").splitlines()
                        if not x.strip().startswith("//")
                    ]
                )
            )
            for cmd_spec in cmd_specs:
                gen_pyi_file_for_cmd_spec(
                    args.output, family.name, cmd_spec, collected_items[family_name]
                )

    protocol_code = "from __future__ import annotations\n\n"
    for family in collected_items.keys():
        protocol_code += f"from . import {family}\n"
    protocol_code += """
from parsec._parsec import VlobID


class ProtocolErrorFields:
    @classmethod
    def NotHandled(cls) -> ProtocolErrorFields: ...
    @classmethod
    def BadRequest(cls, exc: str) -> ProtocolErrorFields: ...
    @property
    def exc(self) -> ProtocolErrorFields: ...


class ProtocolError(BaseException, ProtocolErrorFields): ...


class ReencryptionBatchEntry:
    def __init__(self, vlob_id: VlobID, version: int, blob: bytes) -> None: ...
    @property
    def vlob_id(self) -> VlobID: ...
    @property
    def version(self) -> int: ...
    @property
    def blob(self) -> bytes: ...


class ActiveUsersLimit:
    NO_LIMIT: ActiveUsersLimit

    @classmethod
    def FromOptionalInt(cls, count: int | None) -> ActiveUsersLimit: ...
    @classmethod
    def LimitedTo(cls, user_count_limit: int) -> ActiveUsersLimit: ...
    def to_int(self) -> int | None: ...
    "Returns the user limit count as an integer or None if there's no limit specified"

    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
"""
    protocol_code += (
        '\n\n__all__ =["ReencryptionBatchEntry", "ActiveUsersLimit", '
        + ", ".join(f'"{f}"' for f in collected_items.keys())
        + "]\n"
    )
    (args.output / "__init__.py").write_text(protocol_code, encoding="utf8")

    for family, versions in collected_items.items():
        ordered_versions = sorted(versions.keys())
        family_code = "from __future__ import annotations\n\n"
        version = ""
        for version in ordered_versions:
            family_code += f"from . import {version}\n"
        family_code += f"from . import {version} as latest\n"
        family_code += (
            '\n\n__all__ =["latest", ' + ", ".join(f'"{v}"' for v in ordered_versions) + "]\n"
        )
        (args.output / family / "__init__.py").write_text(family_code, encoding="utf8")

        for version, cmds in versions.items():
            version_code = """from __future__ import annotations

"""
            cmds_names = sorted(cmds.keys())
            for cmd in cmds_names:
                version_code += f"from . import {cmd}\n"

            version_code += f"""

class AnyCmdReq:
    @classmethod
    def load(cls, raw: bytes) -> {'|'.join(cmd + '.Req' for cmd in cmds_names)}: ...
"""
            version_code += (
                '\n\n__all__ =["AnyCmdReq", ' + ", ".join(f'"{c}"' for c in cmds_names) + "]\n"
            )
            (args.output / family / version / "__init__.py").write_text(
                version_code, encoding="utf8"
            )
