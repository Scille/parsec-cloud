# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import json
from importlib import import_module
from pathlib import Path

import pytest


@pytest.mark.xfail(reason="TODO: tests are missing !")
def test_each_cmds_req_reps_have_dedicated_test() -> None:
    schema_dir = (Path(__file__) / "../../../libparsec/crates/protocol/schema/").resolve()

    for family_dir in schema_dir.iterdir():
        assert family_dir.name.endswith("_cmds")
        family_name = family_dir.name[: -len("_cmds")]

        for cmd_json in family_dir.iterdir():
            cmd_specs = json.loads(cmd_json.read_text(encoding="utf8"))

            for cmd_spec in cmd_specs:
                cmd_name = cmd_spec["req"]["cmd"]
                cmd_statuses = [rep_spec["status"] for rep_spec in cmd_spec["reps"]]

                for version in cmd_spec["major_versions"]:
                    api_mod = import_module(f"tests.routes.api_v{version}")

                    for status in cmd_statuses:
                        test_name = f"test_{family_name}_{cmd_name}_{status}"
                        cmd_rep_x_test_fn = getattr(api_mod, test_name, None)

                        assert cmd_rep_x_test_fn is not None and callable(
                            cmd_rep_x_test_fn
                        ), f"`{test_name}` is missing !"
