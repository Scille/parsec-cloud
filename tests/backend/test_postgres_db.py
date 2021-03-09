# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import platform
import triopg

from parsec.backend.cli.run import _run_backend, RetryPolicy
from parsec.backend.config import BackendConfig, PostgreSQLBlockStoreConfig


def records_filter_debug(records):
    return [record for record in records if record.levelname != "DEBUG"]


async def wait_for_listener(conn, to_terminate=False, timeout=3.0):
    with trio.fail_after(timeout):
        while True:
            rows = await conn.fetch(
                "SELECT pid FROM pg_stat_activity WHERE query ILIKE 'listen %' AND state ILIKE 'idle'"
            )
            if not to_terminate and rows or to_terminate and not rows:
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
                pid, = await wait_for_listener(conn)
                value, = await conn.fetchrow("SELECT pg_terminate_backend($1)", pid)
                assert value
                await trio.sleep(1)
                assert False, "Should be cancelled by now"


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_not_ok(
    postgresql_url, backend_factory, caplog, unused_tcp_port
):
    postgresql_url = f"postgresql://localhost:{unused_tcp_port}/dummy"
    with pytest.raises(OSError) as exc:
        async with backend_factory(config={"db_url": postgresql_url}):
            pass
    if platform.system() == "Darwin":
        errno = 61
    else:
        errno = 111
    assert f"[Errno {errno}] Connect call failed" in str(exc.value)


@pytest.mark.trio
@pytest.mark.postgresql
async def test_retry_policy_no_retry(postgresql_url, unused_tcp_port, asyncio_loop):
    host = "localhost"
    port = unused_tcp_port
    app_config = BackendConfig(
        administration_token="s3cr3t",
        db_min_connections=1,
        db_max_connections=5,
        debug=False,
        blockstore_config=PostgreSQLBlockStoreConfig(),
        email_config=None,
        backend_addr=None,
        forward_proto_enforce_https=None,
        ssl_context=False,
        spontaneous_organization_bootstrap=False,
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
                    host, port, ssl_context=False, retry_policy=retry_policy, app_config=app_config
                )
            )
            # Connect to PostgreSQL database
            async with triopg.connect(postgresql_url) as conn:
                # Wait for the backend to be connected
                pid, = await wait_for_listener(conn)
                # Terminate the backend listener connection
                value, = await conn.fetchrow(f"SELECT pg_terminate_backend($1)", pid)
                assert value
                # The _run_backend should raise a Connection error and cancel the nursery
                await trio.sleep(3)
                assert False


@pytest.mark.trio
@pytest.mark.postgresql
async def test_retry_policy_allow_retry(postgresql_url, unused_tcp_port, asyncio_loop):
    host = "localhost"
    port = unused_tcp_port
    app_config = BackendConfig(
        administration_token="s3cr3t",
        db_min_connections=1,
        db_max_connections=5,
        debug=False,
        blockstore_config=PostgreSQLBlockStoreConfig(),
        email_config=None,
        backend_addr=None,
        forward_proto_enforce_https=None,
        ssl_context=False,
        spontaneous_organization_bootstrap=False,
        organization_bootstrap_webhook_url=None,
        db_url=postgresql_url,
    )
    # Allow to retry once
    retry_policy = RetryPolicy(maximum_attempts=1, pause_before_retry=0)
    async with trio.open_nursery() as nursery:
        # Run backend in the background
        nursery.start_soon(
            lambda: _run_backend(
                host, port, ssl_context=False, retry_policy=retry_policy, app_config=app_config
            )
        )
        # Connect to PostgreSQL database
        async with triopg.connect(postgresql_url) as conn:

            # Test for 10 cycles
            for _ in range(10):
                # Wait for the backend to be connected
                pid, = await wait_for_listener(conn)
                # Terminate the backend listener connection
                value, = await conn.fetchrow("SELECT pg_terminate_backend($1)", pid)
                assert value
                # Wait for the listener to terminate
                await wait_for_listener(conn, to_terminate=True)

            # Cancel the backend nursery
            nursery.cancel_scope.cancel()
