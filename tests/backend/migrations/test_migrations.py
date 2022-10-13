# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import re
import trio
import trio_asyncio
import triopg
from asyncpg.cluster import TempCluster
from contextlib import contextmanager
import importlib.resources

from parsec.backend.postgresql import migrations as migrations_module
from parsec.backend.postgresql.handler import (
    retrieve_migrations,
    apply_migrations,
    MIGRATION_FILE_PATTERN,
)


@contextmanager
def pg_cluster_factory(data_dir):
    # TempCluster internally creates a new asyncio loop (to test the DB
    # connection) which is not compatible with trio_asyncio, so we must
    # run this from a non-trio context
    pg_cluster = TempCluster(data_dir_parent=data_dir)
    # Make the default superuser name stable.
    print("Creating PostgreSQL cluster...", end="", flush=True)
    pg_cluster.init(username="postgres")
    pg_cluster.trust_local_connections()
    pg_cluster.start(port="dynamic", server_settings={})
    print("Done !")

    try:
        yield pg_cluster

    finally:
        if pg_cluster.get_status() == "running":
            pg_cluster.stop()
        if pg_cluster.get_status() != "not-initialized":
            pg_cluster.destroy()


def url_from_pg_cluster(pg_cluster):
    spec = pg_cluster.get_connection_spec()
    return f"postgresql://postgres@localhost:{spec['port']}/postgres"


def collect_data_patches():
    from tests.backend import migrations as migrations_test_module

    patches = {}
    for file in importlib.resources.files(migrations_test_module).iterdir():
        file_name = file.name
        match = re.search(MIGRATION_FILE_PATTERN, file_name)
        if match:
            idx = int(match.group("id"))
            assert idx not in patches  # Sanity check
            sql = importlib.resources.files(migrations_test_module).joinpath(file_name).read_text()
            patches[idx] = sql

    return patches


@pytest.mark.postgresql
def test_migrations(tmp_path):
    # Use our own cluster to isolate this test from the others (given
    # otherwise it failure would most likely provoke inconsistent DB schemas
    # errors in any subsequent tests)
    with pg_cluster_factory(tmp_path) as pg_cluster:

        postgresql_url = url_from_pg_cluster(pg_cluster)
        pg_dump = pg_cluster._find_pg_binary("pg_dump")
        psql = pg_cluster._find_pg_binary("psql")

        # Need to run trio loop after pg_cluster is ready
        trio_asyncio.run(_trio_test_migration, postgresql_url, pg_dump, psql)


async def _trio_test_migration(postgresql_url, pg_dump, psql):
    async def reset_db_schema():
        # Now drop the database clean...
        async with triopg.connect(postgresql_url) as conn:
            rep = await conn.execute("DROP SCHEMA public CASCADE")
            assert rep == "DROP SCHEMA"
            rep = await conn.execute("CREATE SCHEMA public")
            assert rep == "CREATE SCHEMA"

    async def dump_schema() -> str:
        cmd = [pg_dump, "--schema=public", "--schema-only", postgresql_url]
        print(f"run: {' '.join(cmd)}")
        process = await trio.run_process(cmd, capture_stdout=True)
        return process.stdout.decode()

    async def dump_data() -> str:
        cmd = [pg_dump, "--schema=public", "--data-only", postgresql_url]
        print(f"run: {' '.join(cmd)}")
        process = await trio.run_process(cmd, capture_stdout=True)
        return process.stdout.decode()

    async def restore_data(data: str) -> None:
        cmd = [psql, "--no-psqlrc", "--set", "ON_ERROR_STOP=on", postgresql_url]
        print(f"run: {' '.join(cmd)}")
        process = await trio.run_process(cmd, capture_stdout=True, stdin=data.encode())
        return process.stdout.decode()

    # The schema may start with an automatic comment, something like:
    # `COMMENT ON SCHEMA public IS 'standard public schema';`
    # So we clean everything first
    await reset_db_schema()

    # Now we apply migrations one after another and also insert the provided data
    migrations = retrieve_migrations()
    patches = collect_data_patches()
    async with triopg.connect(postgresql_url) as conn:
        for migration in migrations:
            result = await apply_migrations(postgresql_url, [migration], dry_run=False)
            assert not result.error
            patch_sql = patches.get(migration.idx, "")
            if patch_sql:
                await conn.execute(patch_sql)

    # Save the final state of the database schema
    schema_from_migrations = await dump_schema()

    # Also save the final data
    data_from_migrations = await dump_data()

    # Now drop the database clean...
    await reset_db_schema()

    # ...and reinitialize it with the current datamodel script
    sql = importlib.resources.files(migrations_module).joinpath("datamodel.sql").read_text()
    async with triopg.connect(postgresql_url) as conn:
        await conn.execute(sql)

    # The resulting database schema should be equivalent to what we add after
    # all the migrations
    schema_from_init = await dump_schema()
    assert schema_from_init == schema_from_migrations

    # Final check is to re-import all the data, this requires some cooking first:

    # Remove the migration related data given their should already be in the database
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

    await restore_data(data_from_migrations)
