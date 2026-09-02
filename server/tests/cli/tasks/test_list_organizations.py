# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import json

from click.testing import CliRunner
from pydantic import TypeAdapter

from parsec.cli.tasks import list_organization
from parsec.cli.testbed import TestbedBackend
from parsec.config import BaseDatabaseConfig, PostgreSQLDatabaseConfig
from tests.common.client import CoolorgRpcClients


def test_list_organization_cmd(db_config: BaseDatabaseConfig, db_args: list[str]):
    runner = CliRunner()
    args = db_args
    use_pg = isinstance(db_config, PostgreSQLDatabaseConfig)
    result = runner.invoke(list_organization.cmd, args)
    assert result.exception is None, result.exc_info
    assert result.exit_code == 0
    assert result.stderr_bytes == b""
    data = json.loads(result.stdout)
    if not use_pg:
        assert data == {}
    else:
        assert isinstance(data, dict)
        assert list(data.keys()) != []


async def test_list_organization(coolorg: CoolorgRpcClients, testbed: TestbedBackend):
    orgs = await list_organization.list_organizations(testbed.backend.organization)

    assert coolorg.organization_id in orgs
    assert orgs[coolorg.organization_id] == list_organization.OrganizationInfo(
        id=coolorg.organization_id,
        is_expired=False,
        is_bootstrapped=True,
        user_profile_outsider_allowed=True,
        realm_minimum_archiving_period_before_deletion=testbed.backend.config.organization_initial_realm_deletion_min_archiving_period,
    )


async def test_serialization(coolorg: CoolorgRpcClients):
    info = list_organization.OrganizationInfo(
        id=coolorg.organization_id,
        is_expired=False,
        is_bootstrapped=True,
        user_profile_outsider_allowed=True,
        realm_minimum_archiving_period_before_deletion=5,
    )

    adapter = TypeAdapter(list_organization.OrganizationInfo)
    serialized = adapter.dump_json(info)
    raw_info = json.loads(serialized)
    print(raw_info)
    assert raw_info["id"] == info.id.str

    got = adapter.validate_json(serialized)
    assert got == info
