# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import importlib
import json
import pkgutil
from types import ModuleType

from parsec.api.data import MessageContent
from parsec.api.protocol.types import HumanHandleField
from parsec.serde import (
    BaseSerializer,
    JSONSerializer,
    MsgpackSerializer,
    OneOfSchema,
    ZipMsgpackSerializer,
)
from parsec.serde.fields import (
    BaseEnumField,
    CheckedConstant,
    EnumCheckedConstant,
    List,
    Map,
    Nested,
    Tuple,
)

_SERIALIZER_TO_STR = {
    JSONSerializer: "json",
    MsgpackSerializer: "msgpack",
    ZipMsgpackSerializer: "zip+msgpack",
    BaseSerializer: None,  # Not serializable
}
_BASE_DATA_CLASSES = (MessageContent,)


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


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("what", choices=["api_protocol", "api_data", "core_data"])
    args = parser.parse_args()
    specs = globals()[f"generate_{args.what}_specs"]()
    print(json.dumps(specs, indent=4, sort_keys=True))
