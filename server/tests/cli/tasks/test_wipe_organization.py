# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Literal

import pytest
from click.testing import CliRunner

from parsec.cli.tasks import wipe_organization
from parsec.cli.testbed import TestbedBackend
from parsec.components.organization import (
    Organization,
    OrganizationGetBadOutcome,
    WipeOrganizationBadOutcome,
)
from tests.cli.common import cli_invoke_in_thread
from tests.common.client import MinimalorgRpcClients


# Only check on postgres since the testbed does not share in-memory data with task commands (they spun-up their own backend)
@pytest.mark.postgresql
@pytest.mark.parametrize("confirm_mode", ("input", "option"))
async def test_wipe_organization_cmd(
    db_args: list[str],
    blockstore_args: list[str],
    minimalorg: MinimalorgRpcClients,
    testbed: TestbedBackend,
    confirm_mode: Literal["input"] | Literal["option"],
):
    runner = CliRunner()

    # 1. Invoke with no confirmation should not wipe the org
    args = [*db_args, *blockstore_args, minimalorg.organization_id.str]
    result = await cli_invoke_in_thread(command=wipe_organization.cmd, cmd=args, runner=runner)
    if result.exception is not None:
        raise ValueError("CLI failed with an exception") from result.exception
    assert result.exit_code == 0
    assert result.stdout == (
        f"""Confirm wipe of organization {minimalorg.organization_id} by entering its name: """
        f"""Invalid confirmation for {minimalorg.organization_id}, will not be wiped\n"""
    )

    res = await testbed.backend.organization.get(minimalorg.organization_id)
    assert isinstance(res, Organization)

    # 2. Confirm deletion of org
    input: str | None = None
    match confirm_mode:
        case "input":
            input = f"{minimalorg.organization_id}\n"
        case "option":
            args.extend([f"--confirm={minimalorg.organization_id}"])

    result = await cli_invoke_in_thread(
        command=wipe_organization.cmd, cmd=args, runner=runner, input=input
    )
    if result.exception is not None:
        raise ValueError("CLI failed with an exception") from result.exception
    assert result.exit_code == 0
    assert result.stdout == (
        ""
        if confirm_mode == "option"
        else f"Confirm wipe of organization {minimalorg.organization_id} by entering its name: {minimalorg.organization_id}\n"
    )

    res = await testbed.backend.organization.get(minimalorg.organization_id)
    assert res == OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND


async def test_wipe_organization(minimalorg: MinimalorgRpcClients, testbed: TestbedBackend):
    minimal_org = await testbed.backend.organization.get(minimalorg.organization_id)
    assert isinstance(minimal_org, Organization)

    res_wipe = await wipe_organization.wipe_organization(
        testbed.backend.organization, testbed.backend.blockstore, minimalorg.organization_id
    )

    res_get = await testbed.backend.organization.get(minimalorg.organization_id)

    assert res_get is OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND

    res_wipe = await testbed.backend.organization.wipe_organization(minimalorg.organization_id)

    assert res_wipe is WipeOrganizationBadOutcome.ORGANIZATION_NOT_FOUND
