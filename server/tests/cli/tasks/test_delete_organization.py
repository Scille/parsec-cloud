# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from datetime import timedelta

import pytest
from click.testing import CliRunner

from parsec.cli.options import Duration
from parsec.cli.tasks import delete_organization
from parsec.cli.testbed import TestbedBackend
from parsec.components.organization import (
    DeleteOrganizationBadOutcome,
    Organization,
    OrganizationGetBadOutcome,
)
from tests.common.client import MinimalorgRpcClients


def test_delete_organization_cmd(
    db_args: list[str],
    blockstore_args: list[str],
    minimalorg: MinimalorgRpcClients,
    testbed: TestbedBackend,
):
    runner = CliRunner()
    args = [*db_args, *blockstore_args, minimalorg.organization_id.str]
    result = runner.invoke(delete_organization.cmd, args)
    if result.exception is not None:
        raise ValueError("CLI failed with an exception") from result.exception
    assert result.exit_code == 0
    assert result.stdout == ""


@pytest.mark.parametrize(
    ("value", "expected"),
    (
        pytest.param("1s", timedelta(seconds=1)),
        pytest.param("1m", timedelta(minutes=1)),
        pytest.param("1h", timedelta(hours=1)),
        pytest.param("1d", timedelta(days=1)),
        pytest.param("60s", timedelta(minutes=1)),
        pytest.param("1m30s", timedelta(minutes=1, seconds=30)),
    ),
)
def test_parse_duration(value: str, expected: timedelta):
    parser = Duration()
    got = parser.convert(value, None, None)
    assert got == expected


async def test_delete_organization(minimalorg: MinimalorgRpcClients, testbed: TestbedBackend):
    minimal_org = await testbed.backend.organization.get(minimalorg.organization_id)
    assert isinstance(minimal_org, Organization)

    res_delete = await delete_organization.delete_organization(
        testbed.backend.organization, testbed.backend.blockstore, minimalorg.organization_id
    )

    res_get = await testbed.backend.organization.get(minimalorg.organization_id)

    assert res_get is OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND

    res_delete = await testbed.backend.organization.delete_organization(minimalorg.organization_id)

    assert res_delete is DeleteOrganizationBadOutcome.ORGANIZATION_NOT_FOUND
