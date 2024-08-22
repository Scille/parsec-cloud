# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import atexit
import os
from typing import Awaitable, Callable

import asyncpg
from asyncpg.cluster import TempCluster

from parsec.components.postgresql.handler import (
    AsyncpgConnection,
    _apply_migrations,
    retrieve_migrations,
)


def _patch_url_if_xdist(url: str) -> str:
    xdist_worker = os.environ.get("PYTEST_XDIST_WORKER")
    if xdist_worker:
        return f"{url}_{xdist_worker}"
    else:
        return url


_pg_db_url = None


async def run_migrations(conn: AsyncpgConnection) -> None:
    result = await _apply_migrations(conn, retrieve_migrations(), dry_run=False)
    if result.error:
        migration, msg = result.error
        raise RuntimeError(f"Error while applying migration {migration.file_name}: {msg}")


async def _execute_pg_query(
    url: str, query: str | Callable[[AsyncpgConnection], Awaitable[None]]
) -> None:
    conn = await asyncpg.connect(url)
    if callable(query):
        await query(conn)
    else:
        await conn.execute(query)
    await conn.close()


def bootstrap_postgresql_testbed() -> str:
    global _pg_db_url

    provided_db = os.environ.get("PG_URL")
    if not provided_db:
        print("Creating PostgreSQL cluster...")
        _pg_db_url = bootstrap_pg_cluster()

    else:
        _pg_db_url = _patch_url_if_xdist(provided_db)

    print("PostgreSQL url: ", _pg_db_url)

    async def init_db():
        assert _pg_db_url is not None
        await _execute_pg_query(_pg_db_url, run_migrations)
        if provided_db:
            await reset_postgresql_testbed()

    asyncio.run(init_db())

    return _pg_db_url


async def reset_postgresql_testbed() -> None:
    assert _pg_db_url is not None
    await _execute_pg_query(
        _pg_db_url,
        """
TRUNCATE TABLE organization
RESTART IDENTITY CASCADE;
""",
    )


def get_postgresql_url() -> str | None:
    return _pg_db_url


_pg_cluster = None


def bootstrap_pg_cluster() -> str:
    global _pg_cluster

    if _pg_cluster:
        raise RuntimeError("Postgresql cluster already bootstrapped !")

    _pg_cluster = TempCluster()
    # Make the default superuser name stable.
    _pg_cluster.init(username="postgres")
    _pg_cluster.trust_local_connections()
    _pg_cluster.start(port="dynamic", server_settings={})

    def _shutdown_pg_cluster() -> None:
        assert _pg_cluster is not None
        if _pg_cluster.get_status() == "running":
            _pg_cluster.stop()
        if _pg_cluster.get_status() != "not-initialized":
            _pg_cluster.destroy()

    atexit.register(_shutdown_pg_cluster)
    spec = _pg_cluster.get_connection_spec()
    return f"postgresql://postgres@localhost:{spec['port']}/postgres"
