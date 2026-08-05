# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest
from click.testing import CliRunner

from parsec.cli import cli


@pytest.mark.parametrize(
    "args",
    (
        ["render_email", "account-create"],
        ["render_email", "account-delete"],
        # ["render_email", "account-recover"],  # TODO: does not exist yet
        ["render_email", "async-enroll"],
        ["render_email", "invite"],
        ["render_email", "totp-reset"],
    ),
    ids=[
        "account-create",
        "account-delete",
        # "account-recover",  # TODO: does not exist yet
        "async-enroll",
        "invite",
        "totp-reset",
    ],
)
def test_render_email(args: list[str]) -> None:
    runner = CliRunner()
    result = runner.invoke(cli, args)
    # Minimal test, just check the command does not fail
    assert result.exit_code == 0, result.output
