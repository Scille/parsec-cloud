# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import anyio
from click.testing import CliRunner

from parsec.cli import cli


async def test_erase_organization_ok() -> None:
    runner = CliRunner()
    result = await anyio.to_thread.run_sync(
        lambda: runner.invoke(
            cli,
            "erase_organization --organization MinimalOrgTemplate --db MOCKED --with-testbed minimal",
            input="MinimalOrgTemplate\n",
            env={"DEBUG": "1"},
        )
    )
    assert result.exit_code == 0, result.output
    assert "Removing from database..." in result.output


async def test_erase_organization_not_found() -> None:
    runner = CliRunner()
    result = await anyio.to_thread.run_sync(
        lambda: runner.invoke(
            cli,
            "erase_organization --organization NonExistentOrg --db MOCKED",
            input="NonExistentOrg\n",
            env={"DEBUG": "1"},
        )
    )
    assert result.exit_code != 0


async def test_erase_organization_confirmation_mismatch() -> None:
    runner = CliRunner()
    result = await anyio.to_thread.run_sync(
        lambda: runner.invoke(
            cli,
            "erase_organization --organization MinimalOrgTemplate --db MOCKED --with-testbed minimal",
            input="WrongName\n",
            env={"DEBUG": "1"},
        )
    )
    assert result.exit_code != 0
    assert "does not match" in result.output
