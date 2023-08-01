# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
    authenticated_cmds,
)
from parsec.backend.asgi import app_factory
from parsec.backend.config import PostgreSQLBlockStoreConfig
from tests.backend.common import (
    message_get,
)
from tests.common import AuthenticatedRpcApiClient, real_clock_timeout

Message = authenticated_cmds.latest.message_get.Message
MessageGetRepOk = authenticated_cmds.latest.message_get.RepOk


@pytest.mark.trio
async def test_message_from_bob_to_alice(backend, alice, bob, alice_rpc):
    d1 = DateTime(2000, 1, 1)
    await backend.message.send(
        bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from Bob !"
    )

    rep = await message_get(alice_rpc)
    assert rep == MessageGetRepOk(
        messages=[
            Message(
                body=b"Hello from Bob !",
                sender=bob.device_id,
                timestamp=d1,
                index=1,
                certificate_index=12,
            )
        ],
    )


@pytest.mark.trio
async def test_message_get_with_offset(backend, alice, bob, alice_rpc):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d1, b"1")
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d1, b"2")
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d2, b"3")

    rep = await message_get(alice_rpc, 1)
    assert rep == MessageGetRepOk(
        messages=[
            Message(body=b"2", sender=bob.device_id, timestamp=d1, index=2, certificate_index=12),
            Message(body=b"3", sender=bob.device_id, timestamp=d2, index=3, certificate_index=12),
        ],
    )


@pytest.mark.trio
@pytest.mark.postgresql
async def test_message_from_bob_to_alice_multi_backends(
    postgresql_url, alice, bob, backend_factory, backend_authenticated_ws_factory
):
    d1 = DateTime(2000, 1, 1)
    async with backend_factory(
        config={"blockstore_config": PostgreSQLBlockStoreConfig(), "db_url": postgresql_url}
    ) as backend_1, backend_factory(
        populated=False,
        config={"blockstore_config": PostgreSQLBlockStoreConfig(), "db_url": postgresql_url},
    ) as backend_2:
        await backend_2.message.send(
            bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from Bob !"
        )
        app1 = app_factory(backend_1)
        alice_rpc = AuthenticatedRpcApiClient(app1.test_client(), alice)
        async with real_clock_timeout():
            while True:
                rep = await message_get(alice_rpc)
                assert rep == MessageGetRepOk(
                    messages=[
                        Message(
                            body=b"Hello from Bob !",
                            sender=bob.device_id,
                            timestamp=d1,
                            index=1,
                            certificate_index=12,
                        )
                    ],
                )
