# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import json
from datetime import timedelta

import pytest
from click.testing import CliRunner

from parsec.cli.options import Duration
from parsec.cli.tasks import delete_old_organization
from parsec.cli.testbed import TestbedBackend
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from tests.common.client import CoolorgRpcClients


def test_delete_old_organization_cmd(db_args: list[str], testbed: TestbedBackend):
    runner = CliRunner()
    args = [*db_args, "--remove-older-than=9999d", "--blockstore=MOCKED"]
    result = runner.invoke(delete_old_organization.cmd, args)
    # assert result.stderr_bytes == b""
    assert result.exception is None, result.exc_info[1]  # pyright: ignore[reportOptionalSubscript]
    assert result.exit_code == 0
    data = json.loads(result.stdout)
    assert data == []


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


async def test_rm_old_orga(coolorg: CoolorgRpcClients, testbed: TestbedBackend):
    coolorg_org = await testbed.backend.organization.get(coolorg.organization_id)
    assert isinstance(coolorg_org, Organization)

    deletion_date = coolorg_org.created_on.subtract(microseconds=1)
    assert coolorg_org.created_on > deletion_date

    deleted_orgs = await delete_old_organization.delete_old_organizations(
        deletion_date, testbed.backend.organization
    )

    assert coolorg.organization_id not in deleted_orgs

    deletion_date = coolorg_org.created_on
    deleted_orgs = await delete_old_organization.delete_old_organizations(
        deletion_date, testbed.backend.organization
    )
    assert coolorg.organization_id in deleted_orgs

    assert (
        await testbed.backend.organization.get(coolorg.organization_id)
    ) is OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND
