# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import sys
from unittest.mock import patch

import pytest
from click.testing import CliRunner

from parsec.cli import cli
from parsec.components.postgresql import MigrationItem
from tests.common import execute_pg_queries


@pytest.mark.postgresql
@pytest.mark.skipif(sys.platform == "win32", reason="Hard to test on Windows...")
def test_migrate(postgresql_url: str, monkeypatch: pytest.MonkeyPatch) -> None:
    # Must use a fake migration table, otherwise we will break the database for
    # the other tests (or ourself is the database is kept between pytest runs)
    asyncio.run(
        execute_pg_queries(
            postgresql_url,
            """
CREATE TABLE IF NOT EXISTS migration_dummy (
    _id INTEGER PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    applied TIMESTAMPTZ NOT NULL
);
TRUNCATE TABLE migration_dummy RESTART IDENTITY CASCADE;
""",
        )
    )
    monkeypatch.setattr("parsec.components.postgresql.handler.MIGRATION_TABLE", "migration_dummy")

    sql = "SELECT current_database();"  # Dummy migration content
    dry_run_args = f"migrate --db {postgresql_url} --dry-run"
    apply_args = f"migrate --db {postgresql_url}"

    with patch("parsec.cli.migration.retrieve_migrations") as retrieve_migrations:
        retrieve_migrations.return_value = [
            MigrationItem(
                index=100001, name="migration1", file_name="100001_migration1.sql", sql=sql
            ),
            MigrationItem(
                index=100002, name="migration2", file_name="100002_migration2.sql", sql=sql
            ),
        ]
        runner = CliRunner()
        result = runner.invoke(cli, dry_run_args)
        assert result.exit_code == 0, result.output
        assert "100001_migration1.sql ✔" in result.output
        assert "100002_migration2.sql ✔" in result.output

        runner = CliRunner()
        result = runner.invoke(cli, apply_args)
        assert result.exit_code == 0, result.output
        assert "100001_migration1.sql ✔" in result.output
        assert "100002_migration2.sql ✔" in result.output

        retrieve_migrations.return_value.append(
            MigrationItem(
                index=100003, name="migration3", file_name="100003_migration3.sql", sql=sql
            )
        )

        result = runner.invoke(cli, dry_run_args)
        assert result.exit_code == 0, result.output
        assert "100001_migration1.sql (already applied)" in result.output
        assert "100002_migration2.sql (already applied)" in result.output
        assert "100003_migration3.sql ✔" in result.output

        result = runner.invoke(cli, apply_args)
        assert result.exit_code == 0, result.output
        assert "100001_migration1.sql (already applied)" in result.output
        assert "100002_migration2.sql (already applied)" in result.output
        assert "100003_migration3.sql ✔" in result.output

        result = runner.invoke(cli, apply_args)
        assert result.exit_code == 0, result.output
        assert "100001_migration1.sql (already applied)" in result.output
        assert "100002_migration2.sql (already applied)" in result.output
        assert "100003_migration3.sql (already applied)" in result.output
