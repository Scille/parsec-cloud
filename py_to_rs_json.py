# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Tuple, List
import sys
import json
from pathlib import Path


CUSTOM_TRANSLATES = {
    ("invite_delete", "req", "reason"): "InvitationDeletedReason",
    ("events_listen", "rep"): "APIEvent",
    ("human_find", "rep", "results"): "HumanFindResultItem",
    ("organization_stats", "rep", "users_per_profile_detail"): "UsersPerProfileDetailItem",
    ("invite_list", "rep", "invitations"): "InviteListItem",
    ("message_get", "rep", "messages"): "Message",
    ("vlob_maintenance_save_reencryption_batch", "req", "batch"): "ReencryptionBatchEntry",
    ("vlob_maintenance_get_reencryption_batch", "rep", "batch"): "ReencryptionBatchEntry",
    ("realm_status", "rep", "maintenance_type"): "MaintenanceType",
    ("user_get", "rep", "trustchain"): "Trustchain",
    ("invite_new", "req"): "UserOrDevice",
    ("invite_new", "rep", "email_sent"): "InvitationEmailSentStatus",
    ("invite_info", "rep"): "UserOrDevice",
}


nested_name_ctx = []
nested_counter = 0


def name_nested():
    global nested_counter
    nested_counter += 1
    return CUSTOM_TRANSLATES.get(tuple(nested_name_ctx), f"Nested{nested_counter}")


def snake_to_camel(snake: str) -> str:
    out = ""
    next_is_upper = True
    for c in snake:
        if c == "_":
            next_is_upper = True
            continue
        elif next_is_upper:
            c = c.upper()
        out += c
        next_is_upper = False
    return out


def translate_type(field: dict) -> Tuple[str, List[dict]]:
    nested_types = []

    type_name = field["type"]

    if "enum_type" in field:
        translated_type_name = field["enum_type"]
        nested_types.append(
            {
                "label": field["enum_type"],
                "variants": [
                    {
                        "name": v,
                        "discriminant_value": v,
                        "fields": [],
                    }
                    for v in field["enum_allowed_values"]
                ],
            }
        )
    elif type_name == "String":
        translated_type_name = "String"
    elif type_name == "Boolean":
        translated_type_name = "Boolean"
    elif type_name == "Integer":
        translated_type_name = "Integer"
    elif type_name == "DateTime":
        translated_type_name = "DateTime"
    elif type_name == "UUID":
        translated_type_name = "UUID"
    elif type_name == "VerifyKey":
        translated_type_name = "VerifyKey"
    elif type_name == "PublicKey":
        translated_type_name = "PublicKey"
    elif type_name == "bytesField":
        translated_type_name = "Bytes"
    elif type_name == "DeviceIDField":
        translated_type_name = "DeviceID"
    elif type_name == "HashDigestField":
        translated_type_name = "HashDigest"
    elif type_name == "UserProfileField":
        translated_type_name = "UserProfile"
    elif type_name == "UserIDField":
        translated_type_name = "UserID"
    elif type_name == "VlobIDField":
        translated_type_name = "VlobID"
    elif type_name == "RealmIDField":
        translated_type_name = "RealmID"
    elif type_name == "RealmRoleField":
        translated_type_name = "RealmRole"
    elif type_name == "HumanHandleField":
        translated_type_name = "HumanHandle"
    elif type_name == "BlockIDField":
        translated_type_name = "BlockID"
    elif type_name == "InvitationTokenField":
        translated_type_name = "InvitationToken"
    elif type_name == "SequesterServiceIDField":
        translated_type_name = "SequesterServiceID"
    elif type_name == "Tuple":
        args_types = []
        for arg in field["args_types"]:
            arg_type, arg_nested_types = translate_type(arg)
            nested_types += arg_nested_types
            args_types.append(arg_type)
        translated_type_name = f"({', '.join(args_types)})"
    elif type_name == "Map":
        key_type, sub_nested_types = translate_type(field["key_type"])
        nested_types += sub_nested_types
        value_type, sub_nested_types = translate_type(field["nested_type"])
        nested_types += sub_nested_types
        translated_type_name = f"Map<{key_type}, {value_type}>"
    elif type_name == "List":
        sub_type, sub_nested_types = translate_type(field["container_type"])
        nested_types += sub_nested_types
        translated_type_name = f"List<{sub_type}>"
    elif type_name == "Nested":
        translated_type_name = name_nested()
        nested_schema = field["schema"]
        if "oneof_schemas" in nested_schema:
            # Variant nested schema
            variants = []
            discriminant_field = nested_schema["oneof_field"]
            assert nested_schema["oneof_fallback_schema"] is None  # Sanity check
            for variant_discriminant_value, variant_schema in nested_schema[
                "oneof_schemas"
            ].items():
                variant_fields = []
                for variant_field_name, variant_field in variant_schema["fields"].items():
                    if variant_field_name == discriminant_field:
                        assert variant_field["type"] == "EnumCheckedConstant"
                        assert variant_field["value"] == variant_discriminant_value
                    else:
                        variant_field_type, sub_nested_types = translate_type(variant_field)
                        nested_types += sub_nested_types
                        variant_fields.append(
                            {
                                "name": variant_field_name,
                                "type": variant_field_type,
                            }
                        )
                variants.append(
                    {
                        "name": variant_discriminant_value.capitalize(),
                        "discriminant_value": variant_discriminant_value,
                        "fields": variant_fields,
                    }
                )
            nested_types.append(
                {
                    "label": translated_type_name,
                    "discriminant_field": discriminant_field,
                    "variants": variants,
                }
            )

        else:
            # Regular nested schema
            nested_fields = []
            for nested_field_name, nested_field in nested_schema["fields"].items():
                translated_nested_field_type_name, nested_nested_types = translate_type(
                    nested_field
                )
                nested_types += nested_nested_types
                nested_fields.append(
                    {
                        "name": nested_field_name,
                        "type": translated_nested_field_type_name,
                    }
                )
            nested_types.append(
                {
                    "label": translated_type_name,
                    "fields": nested_fields,
                }
            )
    elif type_name == "EnumCheckedConstant":
        raise RuntimeError("EnumCheckedConstant should only be present for variant")
    elif type_name == "CheckedConstant":
        raise RuntimeError("CheckedConstant should only be present for `status: ok` rep field")
    else:
        raise RuntimeError(f"Unknown py type {type_name!r}")

    if field["allow_none"]:
        translated_type_name = f"Option<{translated_type_name}>"

    # TODO: Check `required` field and provide a placeholder `introduced_in` ?

    return translated_type_name, nested_types


def translate_req(spec: dict) -> Tuple[dict, List[dict]]:
    nested_name_ctx.append("req")
    try:
        if "oneof_schemas" in spec:
            # Variant
            # Remove `cmd` fields from each variant schema
            for v in spec["oneof_schemas"].values():
                variant_cmd_field = v["fields"].pop("cmd")
                # Sanity check
                assert variant_cmd_field["type"] == "String"
            to_translate = {"schema": spec, "type": "Nested", "allow_none": False}
            translated, nested_types = translate_type(to_translate)
            return {
                "cmd": cmd_name,
                "unit": translated,
                "other_fields": [],
            }, nested_types

        else:
            nested_types = []
            other_fields = []
            for field_name, field in spec["fields"].items():
                nested_name_ctx.append(field_name)
                try:
                    if field_name == "cmd":
                        continue
                    field_type, field_nested_types = translate_type(field)
                    nested_types += field_nested_types
                    other_fields.append(
                        {
                            "name": field_name,
                            "type": field_type,
                        }
                    )
                finally:
                    nested_name_ctx.pop()
            return {"cmd": cmd_name, "other_fields": other_fields}, nested_types
    finally:
        nested_name_ctx.pop()


def translate_ok_rep(spec: dict) -> Tuple[dict, List[dict]]:
    nested_name_ctx.append("rep")
    try:
        if "oneof_schemas" in spec:
            # Variant
            # Remove `status` fields from each variant schema
            for v in spec["oneof_schemas"].values():
                variant_status_field = v["fields"].pop("status")
                # Sanity checks
                assert variant_status_field["type"] == "CheckedConstant"
                assert variant_status_field["value"] == "ok"
            to_translate = {"schema": spec, "type": "Nested", "allow_none": False}
            translated, nested_types = translate_type(to_translate)
            return {
                "status": "ok",
                "unit": translated,
                "other_fields": [],
            }, nested_types

        else:
            nested_types = []
            other_fields = []
            for field_name, field in spec["fields"].items():
                nested_name_ctx.append(field_name)
                try:
                    if field_name == "status":
                        # Sanity check
                        assert field["type"] == "CheckedConstant"
                        assert field["value"] == "ok"
                        continue
                    translated_field, field_nested_types = translate_type(field)
                    nested_types += field_nested_types
                    other_fields.append(
                        {
                            "name": field_name,
                            "type": translated_field,
                        }
                    )
                finally:
                    nested_name_ctx.pop()
            return {"status": "ok", "other_fields": other_fields}, nested_types
    finally:
        nested_name_ctx.pop()


def translate_cmd(cmd_name: str, spec: dict) -> dict:
    nested_name_ctx.append(cmd_name)
    try:
        rep = spec["rep"]
        req = spec["req"]

        # Sanity check
        assert rep["serializing"] == "msgpack"
        assert req["serializing"] == "msgpack"

        nested_types = []
        translated_rep, rep_nested_types = translate_ok_rep(rep)
        nested_types += rep_nested_types
        translated_req, req_nested_types = translate_req(req)
        nested_types += req_nested_types

        # Remove duplicated nested types
        unique_nested_types = []
        for nested_type in nested_types:
            if all(x["label"] != nested_type["label"] for x in unique_nested_types):
                unique_nested_types.append(nested_type)

        return [
            {
                "label": snake_to_camel(cmd_name),
                "major_version": [],  # TODO
                "req": translated_req,
                "reps": [
                    translated_rep,
                    # api_protocol.json doesn't define the error responses...
                ],
                "nested_types": unique_nested_types,
            }
        ]
    finally:
        nested_name_ctx.pop()


if __name__ == "__main__":

    if len(sys.argv) < 3:
        raise SystemExit("usage: py_to_rs_json.py <api_protocol.json> <output_dir>")

    py = json.loads(Path(sys.argv[1]).read_text())
    output_dir = Path(sys.argv[2])

    py_apiv2 = py["APIv2"]
    py_apiv2_authenticated = py_apiv2["AUTHENTICATED"]
    py_apiv2_invited = py_apiv2["INVITED"]

    output_authenticated_dir = output_dir / "authenticated_cmds"
    output_invited_dir = output_dir / "invited_cmds"

    output_dir.mkdir(exist_ok=True)
    output_authenticated_dir.mkdir(exist_ok=True)
    output_invited_dir.mkdir(exist_ok=True)

    for cmd_name, py_cmd in py_apiv2_authenticated.items():
        rs_cmd = translate_cmd(cmd_name, py_cmd)
        (output_authenticated_dir / f"{cmd_name}.json").write_text(json.dumps(rs_cmd, indent=4))

    for cmd_name, py_cmd in py_apiv2_invited.items():
        rs_cmd = translate_cmd(cmd_name, py_cmd)
        (output_invited_dir / f"{cmd_name}.json").write_text(json.dumps(rs_cmd, indent=4))
