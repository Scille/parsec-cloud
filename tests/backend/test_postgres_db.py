# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import platform
import triopg


def records_filter_debug(records):
    return [record for record in records if record.levelname != "DEBUG"]


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

            async with backend_factory(config={"db_url": postgresql_url}) as back:
                row = await back.ping.dbh.notification_conn.fetchrow("SELECT pg_backend_pid()")
                pid = row[0]
                row = await conn.fetchrow(f"SELECT pg_terminate_backend({pid})")
                assert list(row) == [True]
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
async def test_postgresql_connection_not_ok_retrying():
    # TODO - test the CLI interface
    return
