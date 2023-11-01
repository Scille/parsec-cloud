# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest
from click.testing import CliRunner

from parsec import __version__ as parsec_version
from parsec.cli import cli


@pytest.mark.parametrize(
    "args",
    (
        ["--version"],
        ["run", "--version"],
        ["sequester", "--version"],
        ["sequester", "list_services", "--version"],
    ),
    ids=[
        "root",
        "run",
        "sequester",
        "sequester_list_services",
    ],
)
def test_version(args: list[str]) -> None:
    runner = CliRunner()
    result = runner.invoke(cli, args)
    assert result.exit_code == 0, result.output
    assert f"parsec, version {parsec_version}\n" in result.output
