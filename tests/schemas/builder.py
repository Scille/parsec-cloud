# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
import json
import pkgutil
import importlib
from types import ModuleType
from parsec.serde import (
    BaseSerializer,
    JSONSerializer,
    MsgpackSerializer,
    ZipMsgpackSerializer,
    OneOfSchema,
)
from parsec.serde.fields import (
    List,
    Map,
    Tuple,
    Nested,
    CheckedConstant,
    EnumCheckedConstant,
    BaseEnumField,
)
from parsec.api.protocol.types import HumanHandleField

from parsec.api.protocol.base import CmdSerializer
from parsec.api.data.base import BaseData, BaseAPIData, BaseSignedData, BaseAPISignedData
from parsec.api.data import MessageContent

from parsec.core.types.base import BaseLocalData


_SERIALIZER_TO_STR = {
    JSONSerializer: "json",
    MsgpackSerializer: "msgpack",
    ZipMsgpackSerializer: "zip+msgpack",
    BaseSerializer: None,  # Not serializable
}
_BASE_DATA_CLASSES = (
    BaseData,
    BaseAPIData,
    BaseSignedData,
    BaseAPISignedData,
    BaseLocalData,
    MessageContent,
)


def field_to_spec(field):
    # Dump/load only fields not supported yet
    assert not field.dump_only
    assert not field.load_only
    # Missing/default parameters are ignored given they describe *how* to
    # load/dump the data and not *what* are the data
    spec = {
        "type": type(field).__name__,
        "required": field.required,
        "allow_none": field.allow_none,
    }

    # Handle nested/meta/special types
    if isinstance(field, CheckedConstant):
        spec["value"] = field.constant
    elif isinstance(field, EnumCheckedConstant):
        spec["value"] = field.constant.value
        spec["enum_type"] = type(field.constant).__name__
    elif isinstance(field, BaseEnumField):
        spec["enum_type"] = field.ENUM.__name__
        # It might seem strange to embed the enum definition in the schema.
        # This is done to easily see which schema is impacted on enum
        # modification (for instance adding a value in an enum doesn't impact
        # a schema using an `EnumCheckedConstant` on this enum).
        spec["enum_allowed_values"] = [v.value for v in field.ENUM]
    elif isinstance(field, Nested):
        spec["schema"] = schema_to_spec(field.schema)
    elif isinstance(field, List):
        spec["container_type"] = field_to_spec(field.container)
    elif isinstance(field, Map):
        spec["key_type"] = field_to_spec(field.key_field)
        spec["nested_type"] = field_to_spec(field.nested_field)
    elif isinstance(field, (Tuple, HumanHandleField)):
        spec["args_types"] = [field_to_spec(x) for x in field.args]
    # Note Dict type correspond to a schemaless field, so nothing more to document

    return spec


def schema_to_spec(schema):
    if isinstance(schema, OneOfSchema):
        return {
            "oneof_field": schema.type_field,
            "oneof_schemas": {k.value: schema_to_spec(v) for k, v in schema.type_schemas.items()},
            "oneof_fallback_schema": schema_to_spec(schema.fallback_type_schema)
            if schema.fallback_type_schema
            else None,
        }
    else:
        return {"fields": {k: field_to_spec(v) for k, v in schema.fields.items()}}


def data_class_to_spec(data_class):
    return {
        "serializing": _SERIALIZER_TO_STR[data_class.SERIALIZER_CLS],
        **schema_to_spec(data_class.SCHEMA_CLS()),
    }


def collect_data_classes_from_module(mod: ModuleType):
    data_classes = []
    for item_name in dir(mod):
        item = getattr(mod, item_name)
        # Ignore non-data classes
        if (
            not isinstance(item, type) or not issubclass(item, _BASE_DATA_CLASSES)
        ) or item in _BASE_DATA_CLASSES:
            continue
        # Data classes with default serializer cannot be serialized, hence no need
        # to check them (note they will be checked if they are used in Nested field)
        if item.SERIALIZER_CLS is BaseSerializer:
            continue
        # Ignore imported classes (avoid to populate current module collection
        # with external imported schema.
        # Example: Avoid to add imported api schemas while generating parsec.core.types)
        if not item.__module__.startswith(mod.__name__):
            continue
        # Skip classes that are prefixed with '_Py'
        # The prefix '_Py' indicated that the class was replaced by rust impl
        if item.__name__.startswith("_Py"):
            continue
        data_classes.append(item)
    return data_classes


def generate_api_data_specs():
    import parsec.api.data

    package = parsec.api.data

    data_classes = set()
    for sub_module_info in pkgutil.walk_packages(package.__path__, prefix=f"{package.__name__}."):
        sub_module = importlib.import_module(sub_module_info.name)
        data_classes.update(collect_data_classes_from_module(sub_module))

    return {data_cls.__name__: data_class_to_spec(data_cls) for data_cls in data_classes}


def generate_core_data_specs():
    import parsec.core.types
    from parsec.core.local_device import key_file_serializer, legacy_key_file_serializer

    package = parsec.core.types
    data_classes = set()
    for sub_mod_info in pkgutil.walk_packages(package.__path__, prefix=f"{package.__name__}."):
        sub_mod = importlib.import_module(sub_mod_info.name)
        data_classes.update(collect_data_classes_from_module(sub_mod))

    specs = {data_cls.__name__: data_class_to_spec(data_cls) for data_cls in data_classes}

    # Hack to include device file serialization

    def _file_serialization_specs(serializer):
        assert serializer.__class__.__name__.startswith("Msgpack")
        return {
            serializer.schema.__class__.__name__: {
                "serializing": "msgpack",
                **schema_to_spec(serializer.schema),
            }
        }

    for serializer in (key_file_serializer, legacy_key_file_serializer):
        specs.update(_file_serialization_specs(serializer))
    return specs


def collect_cmd_serializers_from_module(mod):
    data_classes = []
    for item_name in dir(mod):
        item = getattr(mod, item_name)
        # Ignore non-data classes
        if not isinstance(item, type) or not issubclass(item, _BASE_DATA_CLASSES):
            continue
        if item in _BASE_DATA_CLASSES:
            continue
        # Data classes with default serializer cannot be serialized, hence no need
        # to check them (note they will be checked if they are used in Nested field)
        if item.SERIALIZER_CLS is BaseSerializer:
            continue
        # Ignore imported classes (avoid to populate current module collection
        # with external imported schema.
        # Example: Avoid to add imported api schemas while generating parsec.core.types)
        if not item.__module__.startswith(mod.__name__):
            continue
        data_classes.append(item)
    return data_classes


def cmd_serializer_to_spec(cmd_serializer: CmdSerializer):
    assert isinstance(cmd_serializer, CmdSerializer)
    # Serialization should always be msgpack, but add it just to be sure...
    return {
        "req": {
            "serializing": _SERIALIZER_TO_STR[type(cmd_serializer._req_serializer)],
            **schema_to_spec(cmd_serializer._req_serializer.schema),
        },
        "rep": {
            "serializing": _SERIALIZER_TO_STR[type(cmd_serializer._rep_serializer)],
            **schema_to_spec(cmd_serializer.rep_noerror_schema),
        },
    }


def collect_cmd_serializer_from_module(mod, cmd_serializers):
    for item_name in dir(mod):
        item = getattr(mod, item_name)
        if not isinstance(item, CmdSerializer):
            continue
        # This is where things start to be hacky: given we don't use enum
        # for command name, and given we sometime use the same schema for
        # APIv1 and v2 definitions, we rely on a weak naming convention
        # to do the identification...
        match = re.match(r"^(?P<name>(apiv1_)?[a-z][a-z0-9_]+)_serializer$", item_name)
        assert match, f"Invalid name `{item_name}` for CmdSerializer !"
        cmd_serializers[match.group("name")] = item
    return cmd_serializers


def generate_api_protocol_specs():
    # First collect all command serializers
    import parsec.api.protocol

    package = parsec.api.protocol

    cmd_serializers = {}
    for sub_mod_info in pkgutil.walk_packages(package.__path__, prefix=f"{package.__name__}."):
        sub_mod = importlib.import_module(sub_mod_info.name)
        collect_cmd_serializer_from_module(sub_mod, cmd_serializers)

    # Now retrieve the per-family commands sets and generate specs
    from parsec.api.protocol import cmds as cmds_mod

    specs = {"APIv1": {}, "APIv2": {}}
    used_cmds_serializers = set()
    for item_name in dir(cmds_mod):
        item = getattr(cmds_mod, item_name)
        # Only keep command sets
        if item_name.startswith("_") or not isinstance(item, set):
            continue

        # Just like for command serializers, commands sets have a naming convention
        match = re.match(r"^(?P<apiv1>APIV1_)?(?P<name>[A-Z][A-Z0-9_]+)_CMDS", item_name)
        assert match, f"Invalid name `{item_name}` for command set !"
        cmds_set_name = match.group("name")
        cmds_set_is_apiv1 = bool(match.group("apiv1"))
        cmds_set_specs = {}

        # Build command specs for the current commands sets
        for cmd_name in item:
            # Retrieve the corresponding command serializer
            if cmds_set_is_apiv1:
                # apiv1 commands serializer have the "apiv1_" prefix when the
                # command name is clashing with a apiv2
                try:
                    cmd_serializer = cmd_serializers[f"apiv1_{cmd_name}"]
                except KeyError:
                    cmd_serializer = cmd_serializers[cmd_name]
            else:
                try:
                    cmd_serializer = cmd_serializers[cmd_name]
                except KeyError:
                    # Due to oxidation, there is no more schema
                    assert cmd_name in [
                        "block_create",
                        "block_read",
                        "events_subscribe",
                        "events_listen",
                        "invite_1_claimer_wait_peer",
                        "invite_1_greeter_wait_peer",
                        "invite_2a_claimer_send_hashed_nonce",
                        "invite_2a_greeter_get_hashed_nonce",
                        "invite_2b_claimer_send_nonce",
                        "invite_2b_greeter_send_nonce",
                        "invite_2b_greeter_send_nonce",
                        "invite_3a_claimer_signify_trust",
                        "invite_3a_greeter_wait_peer_trust",
                        "invite_3b_claimer_wait_peer_trust",
                        "invite_3b_greeter_signify_trust",
                        "invite_4_claimer_communicate",
                        "invite_4_greeter_communicate",
                        "invite_delete",
                        "invite_info",
                        "invite_list",
                        "invite_new",
                        "message_get",
                        "organization_config",
                        "organization_stats",
                        "ping",
                        "pki_enrollment_info",
                        "pki_enrollment_accept",
                        "pki_enrollment_reject",
                        "pki_enrollment_list",
                        "realm_create",
                        "realm_finish_reencryption_maintenance",
                        "realm_get_role_certificates",
                        "realm_start_reencryption_maintenance",
                        "realm_stats",
                        "realm_status",
                        "realm_update_roles",
                        "realm_start_reencryption_maintenance",
                        "realm_finish_reencryption_maintenance",
                        "ping",
                        "user_get",
                        "user_create",
                        "user_revoke",
                        "device_create",
                        "human_find",
                        "vlob_create",
                        "vlob_list_versions",
                        "vlob_maintenance_get_reencryption_batch",
                        "vlob_maintenance_save_reencryption_batch",
                        "vlob_poll_changes",
                        "vlob_read",
                        "vlob_update",
                    ]
                    continue
            used_cmds_serializers.add(cmd_serializer)  # Keep track for sanity check
            cmds_set_specs[cmd_name] = cmd_serializer_to_spec(cmd_serializer)

        # Don't generate category when spec_set is empty
        if len(cmds_set_specs) > 0:
            specs["APIv1" if cmds_set_is_apiv1 else "APIv2"][cmds_set_name] = cmds_set_specs

    # Finally ensure no command serializer has been omitted
    unused_cmds_serializers = set(cmd_serializers.values()) - used_cmds_serializers
    assert (
        not unused_cmds_serializers
    ), f"Command serializer declared but not part of a commands family group: {unused_cmds_serializers}"

    return specs


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("what", choices=["api_protocol", "api_data", "core_data"])
    args = parser.parse_args()
    specs = globals()[f"generate_{args.what}_specs"]()
    print(json.dumps(specs, indent=4, sort_keys=True))
