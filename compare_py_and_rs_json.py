# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Any, Dict, List, Union
import sys
import json
from pathlib import Path


INTEGER_ALIASES = {
    "Integer",
    "Index",
    "Size",
    "NonZeroInteger",
    "Version",
    "Option<Integer>",
    "Option<Version>",
    "Option<Size>",
}
# Uncomment this to show different integer errors
# INTEGER_ALIASES = {}
# List of types that are already define in the rust code
# And shouldn't be defined in "nested_types"
BUILTIN_RUST_TYPES = set({"InvitationStatus", "RealmRole", "UserProfile", "ReencryptionBatchEntry"})

ErrorList = List[Union[Dict[str, Any], str]]


def strip_comment(txt: str) -> str:
    return "\n".join(x for x in txt.splitlines() if not x.strip().startswith("//"))


def compare_file(item_name: str, py_scheme: Path, rs_scheme: Path) -> bool:
    py_api = json.loads(strip_comment(py_scheme.read_text()))
    rs_api = json.loads(strip_comment(rs_scheme.read_text()))
    assert len(py_api) > 0
    assert len(rs_api) > 0
    ok = True
    errors: ErrorList = list()

    if len(py_api) != len(rs_api):
        errors.append(f"number of items differs {len(py_api)!r} vs {len(rs_api)!r}")
        ok = False

    py_api = py_api[0]
    rs_api = rs_api[0]

    if py_api["label"] != rs_api["label"]:
        errors.append(f"label differs {py_api['label']!r} vs {rs_api['label']!r}")
        ok = False

    # Check req
    py_req = py_api["req"]
    rs_req = rs_api["req"]
    ok &= compare_req(errors, py_req, rs_req)

    # Check reps (only status can be checked)

    py_rep_ok = next(x for x in py_api["reps"] if x["status"] == "ok")
    rs_rep_ok = next(x for x in rs_api["reps"] if x["status"] == "ok")
    ok &= compare_rep(errors, py_rep_ok, rs_rep_ok)

    # Nested types

    api1_nested_types = {x["label"]: x for x in py_api.get("nested_types", [])}
    api2_nested_types = {x["label"]: x for x in rs_api.get("nested_types", [])}
    ok &= compare_nested_types(
        item_name, py_api, rs_api, errors, api1_nested_types, api2_nested_types
    )

    if ok is False:
        display_errors(py_scheme, rs_scheme, errors)
    return ok


def display_errors(py_scheme, rs_scheme, errors):
    def _display_errors(indentation: str, errors: ErrorList):
        for error in errors:
            if isinstance(error, str):
                print(f"{indentation}{error}")
            elif isinstance(error, dict):
                sub_part = error["item"]
                sub_errors = error["errors"]
                print(f"{indentation}{sub_part}:")
                _display_errors(indentation * 2, sub_errors)
            else:
                raise RuntimeError(f"expected error {error}")

    print(f"\x1b[1m{py_scheme} vs {rs_scheme}:\x1b[0m")
    _display_errors("\t", errors)


def compare_nested_types(
    item_name: str,
    py_api: Dict[str, Any],
    rs_api: Dict[str, Any],
    errors: ErrorList,
    py_nested_types: List[Dict[str, Any]],
    rs_nested_types: List[Dict[str, Any]],
):
    ok = True
    # Sanity check to make sure a schemas doesn't have multiple nested types with the same name
    assert len(py_nested_types) == len(py_api.get("nested_types", []))
    assert len(rs_nested_types) == len(rs_api.get("nested_types", []))

    # Check redefinition of nested types by the rust scheme
    rs_nested_types_keys = set(rs_nested_types.keys())
    redefined_nested_types = BUILTIN_RUST_TYPES & rs_nested_types_keys
    for type in redefined_nested_types:
        errors.append(f"Builtin type shouldn't be redefined as nested_type: {type}")
        ok = False

    # Check for different definition of nested_types
    different_nested_types = py_nested_types.keys() ^ rs_nested_types.keys()
    for different_nested_type in different_nested_types:
        if different_nested_type not in BUILTIN_RUST_TYPES:
            errors.append(f"different nested types: {different_nested_type}")
            ok = False

    # Compare each nested_types
    for field in py_nested_types.keys() & rs_nested_types.keys():
        py_nested = py_nested_types[field]
        rs_nested = rs_nested_types[field]

        if "variants" in py_nested and "variants" in rs_nested:
            # Variant
            if py_nested.get("discriminant_field") != rs_nested.get("discriminant_field"):
                errors.append(
                    f"nested type {field}'s discriminant_field differs {py_nested['discriminant_field']!r} vs {rs_nested['discriminant_field']!r}"
                )
                ok = False
            py_variants = {v["discriminant_value"]: v for v in py_nested["variants"]}
            rs_variants = {v["discriminant_value"]: v for v in rs_nested["variants"]}
            if py_variants.keys() ^ rs_variants.keys():
                errors.append(
                    f"nested type {field} variants differs {py_variants.keys() ^ rs_variants.keys()!r}"
                )
                ok = False
            for variant in py_variants.keys() & rs_variants.keys():
                py_variant = py_variants[variant]
                rs_variant = rs_variants[variant]
                if py_variant["discriminant_value"] != rs_variant["discriminant_value"]:
                    errors.append(
                        f"nested type {field} variant {variant} discriminant value differs {py_variant['discriminant_value']!r} vs {rs_variant['discriminant_value']!r}"
                    )
                    ok = False
                ok &= compare_fields(
                    errors,
                    f"nested type {field} variant {variant}",
                    py_variant["fields"],
                    rs_variant["fields"],
                )

        elif "fields" in py_nested and "fields" in rs_nested:
            # Regular schema
            ok &= compare_fields(errors, item_name, py_nested["fields"], rs_nested["fields"])

        else:
            errors.append(f"nested type {field} has different types !")
            ok = False
    return ok


def compare_rep(errors: ErrorList, py_rep: Dict[str, Any], rs_rep: Dict[str, Any]):
    ok: bool = compare_fields(
        errors, f"ok rep fields", py_rep["other_fields"], rs_rep["other_fields"]
    )
    if py_rep.get("unit") != rs_rep.get("unit"):
        errors.append(f"rep _okunit differs {py_rep.get('unit')!r} vs {rs_rep.get('unit')!r}")
        ok = False
    return ok


def compare_req(errors: ErrorList, py_req: Dict[str, Any], rs_req: Dict[str, Any]) -> bool:
    ok = True
    if py_req["cmd"] != rs_req["cmd"]:
        errors.append(f"req cmd differs {py_req['cmd']!r} vs {rs_req['cmd']!r}")
        ok = False

    if py_req.get("unit") != rs_req.get("unit"):
        errors.append(f"req unit differs {py_req.get('cmd')!r} vs {rs_req.get('cmd')!r}")
        ok = False

    py_req_fields = {x["name"]: x for x in py_req["other_fields"]}
    rs_req_field = {x["name"]: x for x in rs_req["other_fields"]}
    if py_req_fields.keys() ^ rs_req_field.keys():
        errors.append(
            f"different other_fields in req: {py_req_fields.keys() ^ rs_req_field.keys()!r}"
        )
        ok = False
    for field in py_req_fields.keys() & rs_req_field.keys():
        t1 = py_req_fields[field]["type"]
        t2 = rs_req_field[field]["type"]
        if t1 != t2 and not (t1 in INTEGER_ALIASES and t2 in INTEGER_ALIASES):
            errors.append(f"req field {field!r} type differs {t1!r} vs {t2!r}")
            ok = False
    return ok


def compare_fields(
    errors: ErrorList, item_name: str, py_fields: List[dict], rs_fields: List[dict]
) -> bool:
    ok = True
    py_fields = {x["name"]: x for x in py_fields}
    rs_fields = {x["name"]: x for x in rs_fields}
    local_errors = list()
    if py_fields.keys() ^ rs_fields.keys():
        local_errors.append(f"different fields: {py_fields.keys() ^ rs_fields.keys()!r}")
        ok = False
    for field in py_fields.keys() & rs_fields.keys():
        py_type = py_fields[field]["type"]
        rs_type = rs_fields[field]["type"]
        if py_type != rs_type and not (py_type in INTEGER_ALIASES and rs_type in INTEGER_ALIASES):
            local_errors.append(f"field {field!r} type differs {py_type!r} vs {rs_type!r}")
            ok = False
    if ok is False:
        errors.append({"item": item_name, "errors": local_errors})
    return ok


if __name__ == "__main__":

    if len(sys.argv) < 3:
        raise SystemExit("usage: py_to_rs_json.py <api folder 1> <api folder 2>")

    api1 = Path(sys.argv[1])
    api2 = Path(sys.argv[2])
    ok = True
    if api1.is_dir() and api2.is_dir():
        for family in ("authenticated_cmds", "invited_cmds"):
            api1_family = api1 / family
            api2_family = api2 / family
            items = {
                *(x.name for x in api1_family.iterdir()),
                *(x.name for x in api2_family.iterdir()),
            }
            for item in items:
                api1_item = api1_family / item
                api2_item = api2_family / item
                if not api1_item.is_file():
                    print(f"{api1}/{api1_item.relative_to(api1)}: missing file definition")
                    ok = False
                elif not api2_item.is_file():
                    print(f"{api2}/{api2_item.relative_to(api2)}: missing file definition")
                    ok = False
                else:
                    comp_res = compare_file(api1_item.stem, api1_item, api2_item)
                    if comp_res is False:
                        print()
                    ok &= comp_res

    elif api1.is_file() and api2.is_file():
        ok = compare_file(api1.stem, api1, api2)

    else:
        raise SystemExit("Must compare two dir or two files")

    raise SystemExit(0 if ok else 1)
