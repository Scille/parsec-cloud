# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import importlib.resources
import json

import tests.schemas
from tests.schemas.builder import generate_core_data_specs


# /!\ Those tests only deal with pure Python schemas (even if running with --runrust, /!\
# /!\ even if the Rust schemas are the one exposed by core and api packages)          /!\


def test_core_data_compat():
    specs = json.loads(
        importlib.resources.files(tests.schemas).joinpath("core_data.json").read_text()
    )
    assert generate_core_data_specs() == specs
