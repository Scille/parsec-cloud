# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    BackendEventMessageReceived,
    DateTime,
)
from parsec.api.protocol import (
    APIEventMessageReceived,
    EventsListenRepNoEvents,
    EventsListenRepOk,
    Message,
    MessageGetRepOk,
)
from parsec.backend.asgi import app_factory
from parsec.backend.config import PostgreSQLBlockStoreConfig
from tests.backend.common import message_get
from tests.backend.test_events import events_listen, events_listen_nowait, events_subscribe


@pytest.mark.trio
async def test_message_from_bob_to_alice(backend, alice, bob, alice_ws):
    await events_subscribe(alice_ws)
    d1 = DateTime(2000, 1, 1)
    async with events_listen(alice_ws) as listen:
        await backend.message.send(
            bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from Bob !"
        )

    assert listen.rep == EventsListenRepOk(unit=APIEventMessageReceived(1))

    rep = await message_get(alice_ws)
    assert rep == MessageGetRepOk(
        messages=[Message(body=b"Hello from Bob !", sender=bob.device_id, timestamp=d1, count=1)],
    )


@pytest.mark.trio
async def test_message_get_with_offset(backend, alice, bob, alice_ws):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d1, b"1")
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d1, b"2")
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d2, b"3")

    rep = await message_get(alice_ws, 1)
    assert rep == MessageGetRepOk(
        messages=[
            Message(body=b"2", sender=bob.device_id, timestamp=d1, count=2),
            Message(body=b"3", sender=bob.device_id, timestamp=d2, count=3),
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
        app_1 = app_factory(backend_1)
        async with backend_authenticated_ws_factory(app_1, alice) as alice_ws:
            await events_subscribe(alice_ws)
            async with events_listen(alice_ws) as listen:
                await backend_2.message.send(
                    bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from Bob !"
                )

            assert listen.rep == EventsListenRepOk(APIEventMessageReceived(1))

            rep = await message_get(alice_ws)
            assert rep == MessageGetRepOk(
                messages=[
                    Message(
                        body=b"Hello from Bob !",
                        sender=bob.device_id,
                        timestamp=d1,
                        count=1,
                    )
                ],
            )


@pytest.mark.trio
async def test_message_received_event(backend, alice_ws, alice, bob):
    d1 = DateTime(2000, 1, 1)
    await events_subscribe(alice_ws)

    # Good message
    with backend.event_bus.listen() as spy:
        await backend.message.send(
            bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from bob to alice"
        )
        await backend.message.send(
            bob.organization_id, bob.device_id, alice.user_id, d1, b"Goodbye from bob to alice"
        )

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout(
            [BackendEventMessageReceived, BackendEventMessageReceived]
        )

    reps = [
        await events_listen_nowait(alice_ws),
        await events_listen_nowait(alice_ws),
        await events_listen_nowait(alice_ws),
    ]
    assert reps == [
        EventsListenRepOk(APIEventMessageReceived(1)),
        EventsListenRepOk(APIEventMessageReceived(2)),
        EventsListenRepNoEvents(),
    ]

    # Message to self also trigger event (not as silly as at sound: see workspace reencryption)
    with backend.event_bus.listen() as spy:
        await backend.message.send(
            alice.organization_id, alice.device_id, alice.user_id, d1, b"Hello to myself"
        )

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEventMessageReceived])

    reps = [await events_listen_nowait(alice_ws), await events_listen_nowait(alice_ws)]
    assert reps == [
        EventsListenRepOk(APIEventMessageReceived(3)),
        EventsListenRepNoEvents(),
    ]
