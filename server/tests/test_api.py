# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import json
from importlib import import_module
from pathlib import Path
from typing import Any


def parse_json_with_comment(raw: str) -> Any:
    return json.loads(
        "\n".join(line for line in raw.splitlines() if not line.strip().startswith("//"))
    )


def test_each_cmd_req_rep_has_dedicated_test() -> None:
    schema_dir = (Path(__file__) / "../../../libparsec/crates/protocol/schema/").resolve()

    missing = []

    for family_dir in schema_dir.iterdir():
        assert family_dir.name.endswith("_cmds")
        family_name = family_dir.name[: -len("_cmds")]

        for cmd_json in family_dir.iterdir():
            cmd_specs = parse_json_with_comment(cmd_json.read_text(encoding="utf8"))

            for cmd_spec in cmd_specs:
                cmd_name = cmd_spec["cmd"]
                cmd_statuses = [rep_spec["status"] for rep_spec in cmd_spec["reps"]]
                # Add fake status corresponding to common Parsec HTTP errors such as 404/460/461/462
                # The corresponding test function should test that all common errors are returned
                # by the RPC API command
                cmd_statuses.append("http_common_errors")

                for version in cmd_spec["major_versions"]:
                    api_mod = import_module(f"tests.api_v{version}")

                    for status in cmd_statuses:
                        test_name = f"test_{family_name}_{cmd_name}_{status}"
                        cmd_rep_x_test_fn = getattr(api_mod, test_name, None)

                        if cmd_rep_x_test_fn is None or not callable(cmd_rep_x_test_fn):
                            # Last chance: the command might have multiple tests and hence have its name used as prefix
                            tests = [x for x in dir(api_mod) if x.startswith(test_name)]
                            if tests:
                                assert len(tests) > 1, (
                                    f"Test {tests[0]} should be simply named {test_name}"
                                )
                            else:
                                missing.append(test_name)

    assert not missing, f"Missing {len(missing)} tests:\n" + "\n".join(sorted(missing))
