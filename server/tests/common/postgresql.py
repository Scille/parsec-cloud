# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import atexit
import os
from collections.abc import Awaitable, Callable

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


async def execute_pg_queries(
    url: str, *queries: str | Callable[[AsyncpgConnection], Awaitable[None]]
) -> None:
    conn = await asyncpg.connect(url)
    for query in queries:
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
        if provided_db:
            await execute_pg_queries(_pg_db_url, run_migrations, _Q_RESET_POSTGRESQL_TESTBED)
        else:
            await execute_pg_queries(_pg_db_url, run_migrations)

    asyncio.run(init_db())

    return _pg_db_url


_Q_RESET_POSTGRESQL_TESTBED = """
TRUNCATE TABLE account RESTART IDENTITY CASCADE;

TRUNCATE TABLE account_create_validation_code;
TRUNCATE TABLE account_delete_validation_code;
TRUNCATE TABLE account_recover_validation_code;

TRUNCATE TABLE organization RESTART IDENTITY CASCADE;

TRUNCATE TABLE block_data;

-- Normally, all sequence starts at 0.
-- However this means in the test we basically have all primary key with very low value
-- since we don't insert much.
--
-- By giving different starting value to each table's primary key we are able to more
-- easily detect incorrect comparisons (real word example: `LEFT JOIN human ON device.user_ = human._id`)
-- On top of that it makes things more readable when looking into the database.
SELECT
    SETVAL(PG_GET_SERIAL_SEQUENCE('organization', '_id'), 1000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('sequester_service', '_id'), 2000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('human', '_id'), 3000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('user_', '_id'), 4000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('profile', '_id'), 5000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('device', '_id'), 6000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('shamir_recovery_setup', '_id'), 7000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('shamir_recovery_share', '_id'), 8000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('invitation', '_id'), 9000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('pki_enrollment', '_id'), 11000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm', '_id'), 12000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_user_role', '_id'), 13000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_archiving', '_id'), 14000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_user_change', '_id'), 15000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_keys_bundle', '_id'), 16000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_keys_bundle_access', '_id'), 17000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_sequester_keys_bundle_access', '_id'), 18000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_name', '_id'), 19000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('vlob_atom', '_id'), 20000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_vlob_update', '_id'), 21000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('block', '_id'), 22000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('block_data', '_id'), 23000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('common_topic', '_id'), 24000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('sequester_topic', '_id'), 25000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('shamir_recovery_topic', '_id'), 26000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('realm_topic', '_id'), 27000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('account', '_id'), 28000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('vault', '_id'), 29000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('vault_item', '_id'), 30000),
    SETVAL(PG_GET_SERIAL_SEQUENCE('vault_authentication_method', '_id'), 31000)
;
"""


async def reset_postgresql_testbed() -> None:
    assert _pg_db_url is not None
    await execute_pg_queries(_pg_db_url, _Q_RESET_POSTGRESQL_TESTBED)


_Q_CLEAR_POSTGRESQL_ACCOUNT_DATA = """
TRUNCATE TABLE account RESTART IDENTITY CASCADE;

TRUNCATE TABLE account_create_validation_code;
TRUNCATE TABLE account_delete_validation_code;
TRUNCATE TABLE account_recover_validation_code;
"""


async def clear_postgresql_account_data() -> None:
    """
    Unlike for organizations, accounts are global across the whole database,
    hence each test involving account need to start by clearing those data.

    This is typically done by by the `clear_account_data` fixture, in turn used
    by the `alice_account`/`bob_account`/`anonymous_server` fixtures.
    """
    assert _pg_db_url is not None
    await execute_pg_queries(_pg_db_url, _Q_CLEAR_POSTGRESQL_ACCOUNT_DATA)


_Q_CLEAR_POSTGRESQL_PKI_CERTIFICATE_DATA = """
TRUNCATE TABLE pki_certificate CASCADE;
"""


async def clear_postgresql_pki_certificate_data() -> None:
    """
    Unlike for organizations, pki_certificates are global across the whole database,
    hence each test involving account need to start by clearing those data.

    This is typically done by the `clear_pki_certificate_data` fixture.
    """
    assert _pg_db_url is not None
    await execute_pg_queries(_pg_db_url, _Q_CLEAR_POSTGRESQL_PKI_CERTIFICATE_DATA)


def get_postgresql_url() -> str | None:
    return _pg_db_url


_pg_cluster = None


def bootstrap_pg_cluster() -> str:
    global _pg_cluster
    port = os.environ.get("PG_CLUSTER_PORT", "dynamic")

    if _pg_cluster:
        raise RuntimeError("Postgresql cluster already bootstrapped !")

    _pg_cluster = TempCluster()
    # Make the default superuser name stable.
    _pg_cluster.init(username="postgres")
    _pg_cluster.trust_local_connections()
    _pg_cluster.start(port=port, server_settings={})

    def _shutdown_pg_cluster() -> None:
        assert _pg_cluster is not None
        if _pg_cluster.get_status() == "running":
            _pg_cluster.stop()
        if _pg_cluster.get_status() != "not-initialized":
            _pg_cluster.destroy()

    atexit.register(_shutdown_pg_cluster)
    spec = _pg_cluster.get_connection_spec()
    return f"postgresql://postgres@localhost:{spec['port']}/postgres"
