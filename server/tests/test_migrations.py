# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import importlib.resources
import re
from collections.abc import Iterator

import asyncpg
import pytest
from asyncpg.cluster import Cluster, TempCluster

from parsec.components.postgresql import migrations as migrations_module
from parsec.components.postgresql.handler import (
    apply_migrations,
    retrieve_migrations,
)


@pytest.fixture
def pg_cluster(tmp_path) -> Iterator[Cluster]:
    pg_cluster = TempCluster(data_dir_parent=tmp_path)
    # Make the default superuser name stable.
    pg_cluster.init(username="postgres")
    pg_cluster.trust_local_connections()
    pg_cluster.start(port="dynamic", server_settings={})
    try:
        yield pg_cluster
    finally:
        if pg_cluster.get_status() == "running":
            pg_cluster.stop()
        if pg_cluster.get_status() != "not-initialized":
            pg_cluster.destroy()


@pytest.fixture
def pg_cluster_url(pg_cluster: Cluster) -> str:
    spec = pg_cluster.get_connection_spec()
    return f"postgresql://postgres@localhost:{spec['port']}/postgres"


@pytest.fixture
def pg_dump(pg_cluster: Cluster) -> str:
    return pg_cluster._find_pg_binary("pg_dump")  # type: ignore[attr-defined]


@pytest.fixture
def pg_psql(pg_cluster: Cluster) -> str:
    return pg_cluster._find_pg_binary("psql")  # type: ignore[attr-defined]


def collect_data_patches() -> tuple[dict[int, str], dict[int, str]]:
    PATCH_FILE_PATTERN = r"^(?P<id>\d{4})_(?P<type>before|after).sql$"
    from tests import migrations as migrations_test_module

    before = {}
    after = {}
    for file in importlib.resources.files(migrations_test_module).iterdir():
        file_name = file.name
        match = re.search(PATCH_FILE_PATTERN, file_name)
        assert match or not file_name.endswith(".sql"), (
            f"unknown file `{file_name}`, expects only `xxxx_[before|after].sql`"
        )
        if match:
            index = int(match.group("id"))
            if match.group("type") == "before":
                patches = before
            else:  # after
                patches = after

            assert index not in patches  # Sanity check
            sql = importlib.resources.files(migrations_test_module).joinpath(file_name).read_text()
            patches[index] = sql

    return before, after


@pytest.mark.postgresql
@pytest.mark.asyncio
async def test_migrations(
    pg_cluster_url: str,
    pg_psql: str,
    pg_dump: str,
):
    async def execute_datamodel() -> None:
        sql = importlib.resources.files(migrations_module).joinpath("datamodel.sql").read_text()
        conn = await asyncpg.connect(pg_cluster_url)
        try:
            await conn.execute(sql)
        finally:
            await conn.close()

    async def reset_db_schema() -> None:
        conn = await asyncpg.connect(pg_cluster_url)
        try:
            rep = await conn.execute("DROP SCHEMA public CASCADE")
            assert rep == "DROP SCHEMA"
            rep = await conn.execute("CREATE SCHEMA public")
            assert rep == "CREATE SCHEMA"
        finally:
            await conn.close()

    async def dump_schema() -> str:
        cmd = [pg_dump, "--schema=public", "--schema-only", pg_cluster_url]
        print(f"run: {' '.join(cmd)}")
        process = await asyncio.create_subprocess_exec(*cmd, stdout=asyncio.subprocess.PIPE)
        stdout, _stderr = await process.communicate()
        # Since PostgreSQL 14.19, `pg_dump` add non-deterministic headers&footer
        # (i.e. `\(un)restrict <UUID>`) that we must filter out.
        # (see https://www.postgresql.org/docs/release/14.19/)
        return re.sub(r"\\restrict.*|\\unrestrict.*", "", stdout.decode())

    async def dump_data() -> str:
        cmd = [pg_dump, "--schema=public", "--data-only", pg_cluster_url]
        print(f"run: {' '.join(cmd)}")
        process = await asyncio.create_subprocess_exec(*cmd, stdout=asyncio.subprocess.PIPE)
        stdout, _stderr = await process.communicate()
        # Since PostgreSQL 14.19, `pg_dump` add non-deterministic headers&footer
        # (i.e. `\(un)restrict <UUID>`) that we must filter out.
        # (see https://www.postgresql.org/docs/release/14.19/)
        return re.sub(r"\\restrict.*|\\unrestrict.*", "", stdout.decode())

    async def restore_data(data: str) -> None:
        cmd = [pg_psql, "--no-psqlrc", "--set", "ON_ERROR_STOP=on", pg_cluster_url]
        print(f"run: {' '.join(cmd)}")
        process = await asyncio.create_subprocess_exec(*cmd, stdin=asyncio.subprocess.PIPE)
        await process.communicate(data.encode())
        assert process.returncode == 0

    # The schema may start with an automatic comment, something like:
    # `COMMENT ON SCHEMA public IS 'standard public schema';`
    # So we clean everything first
    await reset_db_schema()

    # Now we apply migrations one after another and also insert the provided data
    migrations = retrieve_migrations()
    patches_before, patches_after = collect_data_patches()
    conn_for_patches = await asyncpg.connect(pg_cluster_url)
    try:
        for migration in migrations:
            if sql := patches_before.get(migration.index):
                await conn_for_patches.execute(sql)

            result = await apply_migrations(pg_cluster_url, [migration], dry_run=False)
            assert not result.error

            if sql := patches_after.get(migration.index):
                await conn_for_patches.execute(sql)
    finally:
        await conn_for_patches.close()

    # Save the final state of the database schema
    schema_from_migrations = await dump_schema()

    # Also save the final data
    data_from_migrations = await dump_data()

    # Now drop the database clean...
    await reset_db_schema()

    # ...and reinitialize it with the current datamodel script
    await execute_datamodel()

    # The resulting database schema should be equivalent to what we add after
    # all the migrations
    schema_from_init = await dump_schema()
    assert schema_from_init == schema_from_migrations

    # Final check is to re-import all the data, this requires some cooking first:

    # Remove the migration-related data given they should already be in the database
    data_from_migrations = re.sub(
        r"COPY public.migration[^\\]*\\.", "", data_from_migrations, flags=re.DOTALL
    )
    # Modify the foreign key constraint between `user_` and `device` to be
    # checked at the end of the transaction. This is needed because `user_`
    # table is entirely populated before switching to `device`. So the
    # constraint should break as soon as an `user_` row references a device_id.
    data_from_migrations = f"""
ALTER TABLE public."user_" ALTER CONSTRAINT fk_user_device_user_certifier DEFERRABLE;
ALTER TABLE public."user_" ALTER CONSTRAINT fk_user_device_revoked_user_certifier DEFERRABLE;

BEGIN;

SET CONSTRAINTS fk_user_device_user_certifier DEFERRED;
SET CONSTRAINTS fk_user_device_revoked_user_certifier DEFERRED;

{data_from_migrations}

COMMIT;
"""

    # All the data should be restored without errors (restore_data will fail on first error)
    await restore_data(data_from_migrations)
