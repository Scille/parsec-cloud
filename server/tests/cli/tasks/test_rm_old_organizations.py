# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import json
from datetime import timedelta

import pytest
from click.testing import CliRunner

from parsec.cli.options import Duration
from parsec.cli.tasks import delete_old_organization
from parsec.cli.testbed import TestbedBackend
from parsec.config import BaseDatabaseConfig, PostgreSQLDatabaseConfig


def test_delete_old_organization_cmd(
    db_config: BaseDatabaseConfig, db_args: list[str], testbed: TestbedBackend
):
    runner = CliRunner()
    args = [*db_args, "--remove-older-than=1d", "--blockstore=MOCKED"]
    use_pg = isinstance(db_config, PostgreSQLDatabaseConfig)
    result = runner.invoke(delete_old_organization.cmd, args)
    # assert result.stderr_bytes == b""
    assert result.exception is None, result.exc_info[1]
    assert result.exit_code == 0
    data = json.loads(result.stdout)
    if not use_pg:
        assert data == {}
    else:
        assert isinstance(data, dict)
        assert list(data.keys()) != []


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
