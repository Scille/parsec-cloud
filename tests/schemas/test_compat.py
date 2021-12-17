# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import json

import importlib_resources

import tests.schemas
from tests.schemas.builder import (
    generate_api_protocol_specs,
    generate_api_data_specs,
    generate_core_data_specs,
)


def test_api_protocol_compat():
    specs = json.loads(
        importlib_resources.files(tests.schemas).joinpath("api_protocol.json").read_text()
    )
    assert generate_api_protocol_specs() == specs


def test_api_data_compat():
    specs = json.loads(
        importlib_resources.files(tests.schemas).joinpath("api_data.json").read_text()
    )
    assert generate_api_data_specs() == specs


def test_core_data_compat():
    specs = json.loads(
        importlib_resources.files(tests.schemas).joinpath("core_data.json").read_text()
    )
    assert generate_core_data_specs() == specs
