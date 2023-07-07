# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import BackendEventMessageReceived, DateTime, authenticated_cmds
from parsec.backend.asgi import app_factory
from parsec.backend.config import PostgreSQLBlockStoreConfig
from tests.backend.common import (
    apiv2v3_events_listen,
    apiv2v3_events_listen_nowait,
    apiv2v3_events_subscribe,
    apiv2v3_message_get,
)

ApiV2V3_APIEventMessageReceived = authenticated_cmds.v3.events_listen.APIEventMessageReceived
ApiV2V3_EventsListenRepNoEvents = authenticated_cmds.v3.events_listen.RepNoEvents
ApiV2V3_EventsListenRepOk = authenticated_cmds.v3.events_listen.RepOk
ApiV2V3_Message = authenticated_cmds.v3.message_get.Message
ApiV2V3_MessageGetRepOk = authenticated_cmds.v3.message_get.RepOk


@pytest.mark.trio
async def test_message_from_bob_to_alice(backend, alice, bob, alice_ws):
    await apiv2v3_events_subscribe(alice_ws)
    d1 = DateTime(2000, 1, 1)
    async with apiv2v3_events_listen(alice_ws) as listen:
        await backend.message.send(
            bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from Bob !"
        )

    assert listen.rep == ApiV2V3_EventsListenRepOk(unit=ApiV2V3_APIEventMessageReceived(1))

    rep = await apiv2v3_message_get(alice_ws)
    assert rep == ApiV2V3_MessageGetRepOk(
        messages=[
            ApiV2V3_Message(body=b"Hello from Bob !", sender=bob.device_id, timestamp=d1, count=1)
        ],
    )


@pytest.mark.trio
async def test_message_get_with_offset(backend, alice, bob, alice_ws):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d1, b"1")
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d1, b"2")
    await backend.message.send(bob.organization_id, bob.device_id, alice.user_id, d2, b"3")

    rep = await apiv2v3_message_get(alice_ws, 1)
    assert rep == ApiV2V3_MessageGetRepOk(
        messages=[
            ApiV2V3_Message(body=b"2", sender=bob.device_id, timestamp=d1, count=2),
            ApiV2V3_Message(body=b"3", sender=bob.device_id, timestamp=d2, count=3),
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
            await apiv2v3_events_subscribe(alice_ws)
            async with apiv2v3_events_listen(alice_ws) as listen:
                await backend_2.message.send(
                    bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from Bob !"
                )

            assert listen.rep == ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventMessageReceived(1))

            rep = await apiv2v3_message_get(alice_ws)
            assert rep == ApiV2V3_MessageGetRepOk(
                messages=[
                    ApiV2V3_Message(
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
    await apiv2v3_events_subscribe(alice_ws)

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
        await apiv2v3_events_listen_nowait(alice_ws),
        await apiv2v3_events_listen_nowait(alice_ws),
        await apiv2v3_events_listen_nowait(alice_ws),
    ]
    assert reps == [
        ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventMessageReceived(1)),
        ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventMessageReceived(2)),
        ApiV2V3_EventsListenRepNoEvents(),
    ]

    # Message to self also trigger event (not as silly as at sound: see workspace reencryption)
    with backend.event_bus.listen() as spy:
        await backend.message.send(
            alice.organization_id, alice.device_id, alice.user_id, d1, b"Hello to myself"
        )

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEventMessageReceived])

    reps = [
        await apiv2v3_events_listen_nowait(alice_ws),
        await apiv2v3_events_listen_nowait(alice_ws),
    ]
    assert reps == [
        ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventMessageReceived(3)),
        ApiV2V3_EventsListenRepNoEvents(),
    ]
