# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import json
import importlib_resources

import tests.schemas
from tests.schemas.builder import generate_api_data_specs, generate_core_data_specs


def test_api_data_compat():
    specs = json.loads(importlib_resources.read_text(tests.schemas, "api_data.json"))
    assert generate_api_data_specs() == specs


def test_core_data_compat():
    specs = json.loads(importlib_resources.read_text(tests.schemas, "core_data.json"))
    assert generate_core_data_specs() == specs
