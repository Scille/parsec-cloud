# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import atexit
import asyncio
import asyncpg
from asyncpg.cluster import TempCluster

from parsec.backend.drivers.postgresql.handler import _init_db


def _patch_url_if_xdist(url):
    xdist_worker = os.environ.get("PYTEST_XDIST_WORKER")
    if xdist_worker:
        return f"{url}_{xdist_worker}"
    else:
        return url


def _execute_pg_query(url, query):
    async def _execute_query():
        conn = await asyncpg.connect(url)
        if callable(query):
            await query(conn)
        else:
            await conn.execute(query)
        await conn.close()

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(_execute_query())


_pg_db_url = None


def bootstrap_postgresql_testbed():
    global _pg_db_url

    provided_db = os.environ.get("PG_URL")
    if not provided_db:
        print("Creating PostgreSQL cluster...")
        _pg_db_url = bootstrap_pg_cluster()

    else:
        _pg_db_url = _patch_url_if_xdist(provided_db)

    print("PostgreSQL url: ", _pg_db_url)
    # Finally initialiaze the database
    # In theory we should use TrioPG here to do db init, but:
    # - Duck typing and similar api makes `_init_db` compatible with both
    # - AsyncPG should be slightly faster than TrioPG
    # - Most important: a trio loop is potentially already started inside this
    #   thread (i.e. if the test is mark as trio). Hence we would have to spawn
    #   another thread just to run the new trio loop.
    _execute_pg_query(_pg_db_url, _init_db)


def reset_postgresql_testbed():
    # Cleanup the db tables from previous tests
    _execute_pg_query(
        _pg_db_url,
        """
TRUNCATE TABLE
    organizations,

    users,
    devices,

    user_invitations,
    device_invitations,

    message,

    beacons,
    beacon_accesses,
    vlobs,
    vlob_atoms,
    beacon_vlob_atom_updates,

    blockstore
RESTART IDENTITY CASCADE
""",
    )


def get_postgresql_url():
    return _pg_db_url


_pg_cluster = None


def bootstrap_pg_cluster():
    global _pg_cluster

    if _pg_cluster:
        raise RuntimeError("Postgresql cluster already bootstrapped !")

    _pg_cluster = TempCluster()
    # Make the default superuser name stable.
    print(_pg_cluster.init(username="postgres"))
    _pg_cluster.trust_local_connections()
    _pg_cluster.start(port="dynamic", server_settings={})

    def _shutdown_pg_cluster():
        if _pg_cluster.get_status() == "running":
            _pg_cluster.stop()
        if _pg_cluster.get_status() != "not-initialized":
            _pg_cluster.destroy()

    atexit.register(_shutdown_pg_cluster)
    spec = _pg_cluster.get_connection_spec()
    return f"postgresql://postgres@localhost:{spec['port']}/postgres"
