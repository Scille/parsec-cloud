# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys
from datetime import datetime, timezone

import trio
import pytest
import triopg

from parsec._parsec import DateTime
from parsec.backend.cli.run import _run_backend, RetryPolicy
from parsec.backend.config import BackendConfig, PostgreSQLBlockStoreConfig
from parsec.backend.postgresql.handler import handle_datetime

from tests.common import real_clock_timeout


def records_filter_debug(records):
    return [record for record in records if record.levelname != "DEBUG"]


async def wait_for_listeners(conn, to_terminate=False):
    async with real_clock_timeout():
        while True:
            rows = await conn.fetch(
                "SELECT pid FROM pg_stat_activity WHERE query ILIKE 'listen %' AND state ILIKE 'idle'"
            )
            if (not to_terminate and rows) or (to_terminate and not rows):
                return [r["pid"] for r in rows]


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_ok(postgresql_url, backend_factory):
    async with backend_factory(config={"db_url": postgresql_url}):
        pass


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_notification_listener_terminated(postgresql_url, backend_factory):

    async with triopg.connect(postgresql_url) as conn:

        with pytest.raises(ConnectionError):

            async with backend_factory(config={"db_url": postgresql_url}):
                (pid,) = await wait_for_listeners(conn)
                (value,) = await conn.fetchrow("SELECT pg_terminate_backend($1)", pid)
                assert value
                # Wait to get cancelled by the backend app
                async with real_clock_timeout():
                    await trio.sleep_forever()


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_not_ok(postgresql_url, backend_factory, unused_tcp_port):
    postgresql_url = f"postgresql://127.0.0.1:{unused_tcp_port}/dummy"
    with pytest.raises(OSError) as exc:
        async with backend_factory(config={"db_url": postgresql_url}):
            pass
    if sys.platform == "darwin":
        errno = 61
    else:
        errno = 111
    assert f"[Errno {errno}] Connect call failed" in str(exc.value)


@pytest.mark.trio
@pytest.mark.postgresql
async def test_retry_policy_no_retry(postgresql_url, asyncio_loop):
    app_config = BackendConfig(
        administration_token="s3cr3t",
        db_min_connections=1,
        db_max_connections=5,
        debug=False,
        blockstore_config=PostgreSQLBlockStoreConfig(),
        email_config=None,
        backend_addr=None,
        forward_proto_enforce_https=None,
        organization_spontaneous_bootstrap=False,
        organization_bootstrap_webhook_url=None,
        db_url=postgresql_url,
    )

    # No retry
    retry_policy = RetryPolicy(maximum_attempts=0, pause_before_retry=0)

    # Expect a connection error
    with pytest.raises(ConnectionError):
        async with trio.open_nursery() as nursery:
            # Run backend in the background
            nursery.start_soon(
                lambda: _run_backend(
                    host="127.0.0.1",
                    port=0,
                    ssl_certfile=None,
                    ssl_keyfile=None,
                    retry_policy=retry_policy,
                    app_config=app_config,
                )
            )
            # Connect to PostgreSQL database
            async with triopg.connect(postgresql_url) as conn:
                # Wait for the backend to be connected
                (pid,) = await wait_for_listeners(conn)
                # Terminate the backend listener connection
                (value,) = await conn.fetchrow("SELECT pg_terminate_backend($1)", pid)
                assert value
                # Wait to get cancelled by the connection error `_run_backend`
                async with real_clock_timeout():
                    await trio.sleep_forever()


@pytest.mark.trio
@pytest.mark.postgresql
async def test_retry_policy_allow_retry(postgresql_url, asyncio_loop):
    app_config = BackendConfig(
        administration_token="s3cr3t",
        db_min_connections=1,
        db_max_connections=5,
        debug=False,
        blockstore_config=PostgreSQLBlockStoreConfig(),
        email_config=None,
        backend_addr=None,
        forward_proto_enforce_https=None,
        organization_spontaneous_bootstrap=False,
        organization_bootstrap_webhook_url=None,
        db_url=postgresql_url,
    )
    # Allow to retry once
    retry_policy = RetryPolicy(maximum_attempts=1, pause_before_retry=0)
    async with trio.open_nursery() as nursery:
        # Run backend in the background
        nursery.start_soon(
            lambda: _run_backend(
                host="127.0.0.1",
                port=0,
                ssl_certfile=None,
                ssl_keyfile=None,
                retry_policy=retry_policy,
                app_config=app_config,
            )
        )
        # Connect to PostgreSQL database
        async with triopg.connect(postgresql_url) as conn:

            # Test for 10 cycles
            pid = None
            for _ in range(10):
                # Wait for the backend to be connected
                (new_pid,) = await wait_for_listeners(conn)
                # Make sure a new connection has been created
                assert new_pid != pid
                pid = new_pid
                # Terminate the backend listener connection
                (value,) = await conn.fetchrow("SELECT pg_terminate_backend($1)", pid)
                assert value
                # Wait for the listener to terminate
                await wait_for_listeners(conn, to_terminate=True)

            # Cancel the backend nursery
            nursery.cancel_scope.cancel()


@pytest.mark.trio
@pytest.mark.postgresql
async def test_rust_datetime_correctly_serialized(postgresql_url, backend_factory):
    now_py = datetime.now(timezone.utc)
    now_rs = DateTime.from_timestamp(now_py.timestamp())

    async with triopg.connect(postgresql_url) as vanilla_conn:
        async with triopg.connect(postgresql_url) as patched_conn:
            await handle_datetime(patched_conn)

            await vanilla_conn.execute(
                f"""
                DROP TABLE IF EXISTS datetime;
                CREATE TABLE IF NOT EXISTS datetime (
                    _id SERIAL PRIMARY KEY,
                    timestamp TIMESTAMPTZ
                )"""
            )

            # Insert DateTime
            await vanilla_conn.execute(
                "INSERT INTO datetime (_id, timestamp) VALUES (0, $1)",
                now_py,
            )
            await patched_conn.execute(
                "INSERT INTO datetime (_id, timestamp) VALUES (1, $1)",
                now_rs,
            )

            # Retrieve datetime inserted by vanilla
            from_vanilla_to_py = await vanilla_conn.fetchval(
                "SELECT timestamp FROM datetime WHERE _id = 0"
            )
            # Retrieve datetime inserted by patched
            from_patched_to_py = await vanilla_conn.fetchval(
                "SELECT timestamp FROM datetime WHERE _id = 1"
            )
            # Retrieve Datetime inserted by vanilla
            from_vanilla_to_rs = await patched_conn.fetchval(
                "SELECT timestamp FROM datetime WHERE _id = 0"
            )
            # Retrieve Datetime inserted by patched
            from_patched_to_rs = await patched_conn.fetchval(
                "SELECT timestamp FROM datetime WHERE _id = 1"
            )

            assert from_vanilla_to_py == from_patched_to_py
            assert from_vanilla_to_rs == from_patched_to_rs
            assert (
                from_vanilla_to_py.timestamp()
                == from_patched_to_py.timestamp()
                == from_vanilla_to_rs.timestamp()
                == from_patched_to_rs.timestamp()
                == now_py.timestamp()
                == now_rs.timestamp()
            )
