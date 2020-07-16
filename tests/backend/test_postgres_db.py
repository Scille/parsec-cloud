# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_ok(postgresql_url, backend_factory, blockstore):
    async with backend_factory(config={"db_url": postgresql_url}):
        pass


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_not_ok(
    postgresql_url, backend_factory, caplog, unused_tcp_port
):
    postgresql_url = f"postgresql://localhost:{unused_tcp_port}/dummy"
    with pytest.raises(OSError) as exc:
        async with backend_factory(config={"db_url": postgresql_url}):
            pass
    assert "[Errno 111] Connect call failed" in str(exc.value)
    # assert len(caplog.records) == 1
    assert [caplog.records[0].msg, caplog.records[1].msg] == "test"  # TODO DELETE
    assert caplog.records[0].levelname == "ERROR"
    assert "initial db connection failed" in caplog.records[0].message


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_not_ok_retrying(
    postgresql_url, backend_factory, caplog, unused_tcp_port, autojump_clock
):
    tries_number = 4
    tries_sleep = 3
    postgresql_url = f"postgresql://localhost:{unused_tcp_port}/dummy"
    with pytest.raises(OSError) as exc:
        with trio.fail_after((tries_number - 1) * tries_sleep + 1):
            async with backend_factory(
                config={
                    "db_url": postgresql_url,
                    "db_first_tries_number": tries_number,
                    "db_first_tries_sleep": tries_sleep,
                }
            ):
                pass
    assert "[Errno 111] Connect call failed" in str(exc.value)
    assert len(caplog.records) == 4
    for record in caplog.records[:3]:
        assert record.levelname == "WARNING"
        assert "initial db connection failed" in record.message
    assert caplog.records[3].levelname == "ERROR"
    assert "initial db connection failed" in caplog.records[3].message
