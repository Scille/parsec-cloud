# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re

import pytest

from parsec.backend.config import PostgreSQLBlockStoreConfig


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_ok(
    postgresql_url, alice, bob, backend_factory, backend_sock_factory
):
    async with backend_factory(
        config={"blockstore_config": PostgreSQLBlockStoreConfig(), "db_url": postgresql_url}
    ):
        pass


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_not_ok(
    postgresql_url, alice, bob, backend_factory, backend_sock_factory, caplog
):
    postgresql_url = re.sub(
        ":(\\d+)/", lambda m: f":{str(int(m.group()[1:-1])+1)}/", postgresql_url
    )
    with pytest.raises(OSError):
        async with backend_factory(
            config={"blockstore_config": PostgreSQLBlockStoreConfig(), "db_url": postgresql_url}
        ):
            pass
    assert len(caplog.records) == 1
    assert caplog.records[0].levelname == "ERROR"
    assert "initial db connection failed" in caplog.records[0].message


@pytest.mark.trio
@pytest.mark.postgresql
async def test_postgresql_connection_not_ok_retrying(
    postgresql_url, alice, bob, backend_factory, backend_sock_factory, recwarn, caplog
):
    postgresql_url = re.sub(
        ":(\\d+)/", lambda m: f":{str(int(m.group()[1:-1])+1)}/", postgresql_url
    )
    with pytest.raises(OSError):
        async with backend_factory(
            config={
                "blockstore_config": PostgreSQLBlockStoreConfig(),
                "db_url": postgresql_url,
                "db_first_tries_number": 4,
                "db_first_tries_sleep": 0,
            }
        ):
            pass
    assert len(caplog.records) == 4
    for record in caplog.records[:3]:
        assert record.levelname == "WARNING"
        assert "initial db connection failed" in record.message
    assert caplog.records[3].levelname == "ERROR"
    assert "initial db connection failed" in caplog.records[3].message
