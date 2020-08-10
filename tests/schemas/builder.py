# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import json
import pkgutil
import importlib

from parsec.serde import BaseSerializer, JSONSerializer, MsgpackSerializer, ZipMsgpackSerializer
from parsec.serde.fields import List, Map, Tuple, Nested, CheckedConstant, EnumCheckedConstant
from parsec.api.data.base import BaseSignedData, BaseData
from parsec.core.types.base import BaseLocalData


_SERIALIZER_TO_STR = {
    JSONSerializer: "json",
    MsgpackSerializer: "msgpack",
    ZipMsgpackSerializer: "zip+msgpack",
    BaseSerializer: None,  # Not serializable
}
_BASE_DATA_CLASSES = (BaseData, BaseSignedData, BaseLocalData)


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
    if isinstance(field, EnumCheckedConstant):
        spec["value"] = field.constant.value
    elif isinstance(field, Nested):
        spec["schema"] = schema_to_spec(field.schema)
    elif isinstance(field, List):
        spec["container_type"] = field_to_spec(field.container)
    elif isinstance(field, Map):
        spec["key_type"] = field_to_spec(field.key_field)
        spec["nested_type"] = field_to_spec(field.nested_field)
    elif isinstance(field, Tuple):
        spec["args_types"] = [field_to_spec(x) for x in field.args]
    # Note Dict type correspond to a schemaless field, so nothing more to document

    return spec


def schema_to_spec(schema):
    return {"fields": {k: field_to_spec(v) for k, v in schema.fields.items()}}


def data_class_to_spec(data_class):
    return {
        "serializing": _SERIALIZER_TO_STR[data_class.SERIALIZER_CLS],
        **schema_to_spec(data_class.SCHEMA_CLS()),
    }


def collect_data_classes_from_module(mod):
    data_classes = []
    for item_name in dir(mod):
        item = getattr(mod, item_name)
        # Ignore non-data classes
        if not isinstance(item, type) or not issubclass(item, _BASE_DATA_CLASSES):
            continue
        if item in _BASE_DATA_CLASSES:
            continue
        # Data classes with default serializer cannot be serialized, hence no need
        # to check them (note they will be checked if they used in Nested field)
        if item.SERIALIZER_CLS is BaseSerializer:
            continue
        data_classes.append(item)
    return data_classes


def generate_api_data_specs():
    import parsec.api.data

    data_classes = collect_data_classes_from_module(parsec.api.data)
    return {data_cls.__name__: data_class_to_spec(data_cls) for data_cls in data_classes}


def generate_core_data_specs():
    import parsec.core.types

    package = parsec.core.types

    data_classes = set()
    for submod_info in pkgutil.walk_packages(package.__path__, prefix=f"{package.__name__}."):
        submod = importlib.import_module(submod_info.name)
        data_classes.update(collect_data_classes_from_module(submod))

    return {data_cls.__name__: data_class_to_spec(data_cls) for data_cls in data_classes}


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("what", choices=["api_data", "core_data"])
    args = parser.parse_args()
    if args.what == "api_data":
        specs = generate_api_data_specs()
    else:
        specs = generate_core_data_specs()
    print(json.dumps(specs, indent=4, sort_keys=True))
