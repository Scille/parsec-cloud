# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import json
import importlib.resources

import tests.schemas
from tests.schemas.builder import (
    generate_api_protocol_specs,
    generate_api_data_specs,
    generate_core_data_specs,
)


# /!\ Those tests only deal with pure Python schemas (even if running with --runrust, /!\
# /!\ even if the Rust schemas are the one exposed by core and api packages)          /!\


def test_api_protocol_compat():
    specs = json.loads(
        importlib.resources.files(tests.schemas).joinpath("api_protocol.json").read_text()
    )
    assert generate_api_protocol_specs() == specs


def test_api_data_compat():
    specs = json.loads(
        importlib.resources.files(tests.schemas).joinpath("api_data.json").read_text()
    )
    assert generate_api_data_specs() == specs


def test_core_data_compat():
    specs = json.loads(
        importlib.resources.files(tests.schemas).joinpath("core_data.json").read_text()
    )
    assert generate_core_data_specs() == specs
