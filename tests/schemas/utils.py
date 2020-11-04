# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from unittest.mock import ANY


def _assert_backward_compatible_field_upgrade(old_field, new_field):
    if old_field["type"] == "Nested" and new_field["type"] == "Nested":
        expected_field = old_field.copy()
        expected_field["schema"] = ANY
        assert new_field == expected_field
        _assert_backward_compatible_schema_upgrade(old_field["schema"], new_field["schema"])

    elif old_field["type"] == "List" and new_field["type"] == "List":
        expected_field = old_field.copy()
        expected_field["container_type"] = ANY
        assert new_field == expected_field
        _assert_backward_compatible_field_upgrade(
            old_field["container_type"], new_field["container_type"]
        )

    elif old_field["type"] == "Map" and new_field["type"] == "Map":
        expected_field = old_field.copy()
        expected_field["key_type"] = ANY
        expected_field["nested_type"] = ANY
        assert new_field == expected_field
        _assert_backward_compatible_field_upgrade(old_field["key_type"], new_field["key_type"])
        _assert_backward_compatible_field_upgrade(
            old_field["nested_type"], new_field["nested_type"]
        )

    elif old_field["type"] == "Tuple" and new_field["type"] == "Tuple":
        expected_field = old_field.copy()
        expected_field["args_types"] = [ANY] * len(old_field["args_types"])
        assert new_field == expected_field
        for old_sub_type, new_sub_type in zip(old_field["args_types"], new_field["args_types"]):
            _assert_backward_compatible_field_upgrade(old_sub_type, new_sub_type)

    else:
        assert old_field == new_field


def _assert_backward_compatible_schema_upgrade(old_schema, new_schema):
    removed_fields = old_schema["fields"].keys() - new_schema["fields"].keys()
    assert not removed_fields

    for field in old_schema["fields"].keys() ^ new_schema["fields"].keys():
        old_field = deepcopy(old_schema["fields"][field])
        new_field = deepcopy(new_schema["fields"][field])
        _assert_backward_compatible_field_upgrade(old, new)

    added_fields = new_schema["fields"].keys() - old_schema["fields"].keys()
    for field in added_fields:
        assert not field["required"]


def assert_backward_compatible_spec_upgrade(old_spec, new_spec):
    # Compatibility rules:
    # 1. serialization format cannot change
    # 2. fields cannot be removed
    # 3. fields type cannot change, but nested schemas can evolve
    #    according to the current rules
    # 4. only non-required field can be added (wanabe required field should
    #    be marked allow_none=False instead)

    assert old_spec["serializing"] == new_spec["serializing"]

    _assert_backward_compatible_schema_upgrade(old_spec, new_spec)
