# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import logging
import sys
from asyncio import InvalidStateError
from datetime import datetime, timezone
from uuid import uuid4

import pytest
import trio
import triopg

from parsec._parsec import DateTime, EntryID
from parsec.backend.cli.run import RetryPolicy, _run_backend
from parsec.backend.config import BackendConfig, PostgreSQLBlockStoreConfig
from parsec.backend.postgresql.handler import handle_datetime, handle_uuid
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
async def test_retry_policy_allow_retry(postgresql_url, asyncio_loop, caplog):
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

    # Ignore error logs that looks like:
    # *** asyncio.exceptions.InvalidStateError: invalid state
    # Traceback (most recent call last):
    # File "asyncio/base_events.py", line 1779, in call_exception_handler
    #     self.default_exception_handler(context)
    # File "site-packages/trio_asyncio/_async.py", line 44, in default_exception_handler
    #     raise exception
    # File "asyncio/selector_events.py", line 868, in _read_ready__data_received
    #     self._protocol.data_received(data)
    # File "site-packages/asyncpg/connect_utils.py", line 674, in data_received
    #     self.on_data.set_result(False)
    # Or like this:
    # *** ConnectionError: unexpected connection_lost() call
    # Traceback (most recent call last):
    # File "asyncio/base_events.py", line 1779, in call_exception_handler
    #     self.default_exception_handler(context)
    # File "site-packages/trio_asyncio/_async.py", line 44, in default_exception_handler
    #     raise exception
    # Those happen about 14% and 5% of the runs, respectively.
    # TODO: Investigate
    for record in caplog.get_records("call"):
        if record.levelno < logging.ERROR or record.name != "asyncio":
            continue
        try:
            _, exc, _ = record.exc_info
        except ValueError:
            continue
        if isinstance(exc, (ConnectionError, InvalidStateError)):
            try:
                caplog.asserted_records.add(record)
            except AttributeError:
                caplog.asserted_records = {record}


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


@pytest.mark.trio
@pytest.mark.postgresql
async def test_rust_uuid_correctly_serialized(postgresql_url, backend_factory):
    id_py = uuid4()
    id_rs = EntryID.from_hex(id_py.hex)

    async with triopg.connect(postgresql_url) as vanilla_conn:
        async with triopg.connect(postgresql_url) as patched_conn:
            await handle_uuid(patched_conn)

            await vanilla_conn.execute(
                f"""
                DROP TABLE IF EXISTS uuid;
                CREATE TABLE IF NOT EXISTS uuid (
                    _id SERIAL PRIMARY KEY,
                    id UUID
                )"""
            )

            # Insert DateTime
            await vanilla_conn.execute(
                "INSERT INTO uuid (_id, id) VALUES (0, $1)",
                id_py,
            )
            await patched_conn.execute(
                "INSERT INTO uuid (_id, id) VALUES (1, $1)",
                id_rs,
            )

            # Retrieve uuid inserted by vanilla
            from_vanilla_to_py = await vanilla_conn.fetchval("SELECT id FROM uuid WHERE _id = 0")
            # Retrieve uuid inserted by patched
            from_patched_to_py = await vanilla_conn.fetchval("SELECT id FROM uuid WHERE _id = 1")
            # Retrieve hex inserted by vanilla
            from_vanilla_to_rs = await patched_conn.fetchval("SELECT id FROM uuid WHERE _id = 0")
            # Retrieve hex inserted by patched
            from_patched_to_rs = await patched_conn.fetchval("SELECT id FROM uuid WHERE _id = 1")

            # Test that we can retrieve our EntryIDs
            # because deserializer doesn't know which ID
            from_vanilla_to_rs = EntryID.from_hex(from_vanilla_to_rs)
            from_patched_to_rs = EntryID.from_hex(from_patched_to_rs)

            assert from_vanilla_to_py == from_patched_to_py
            assert from_vanilla_to_rs == from_patched_to_rs
            assert (
                from_vanilla_to_py.hex
                == from_patched_to_py.hex
                == from_vanilla_to_rs.hex
                == from_patched_to_rs.hex
                == id_py.hex
                == id_rs.hex
            )
