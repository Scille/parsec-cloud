# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import trio
import pytest
from functools import partial

from parsec._parsec import DateTime, AuthenticatedPingRepOk, EventsListenRepOkPinged
from parsec.backend.utils import ClientType
from parsec.api.transport import Transport, Ping, Pong
from parsec.api.data import RevokedUserCertificate
from parsec.api.protocol import ServerHandshake, AUTHENTICATED_CMDS, OrganizationID
from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendConnectionRefused,
    backend_authenticated_cmds_factory,
)
from parsec.backend.backend_events import BackendEvent

from tests.common import correct_addr
from tests.core.backend_connection.common import ALL_CMDS


@pytest.mark.trio
async def test_backend_offline(alice):
    with pytest.raises(BackendNotAvailable):
        async with backend_authenticated_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key
        ) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, alice):
    async with backend_authenticated_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        await cmds.ping()
        with running_backend.offline():
            with pytest.raises(BackendNotAvailable):
                await cmds.ping()


@pytest.mark.trio
async def test_backend_closed_cmds(running_backend, alice):
    async with backend_authenticated_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping()


@pytest.mark.trio
async def test_ping(running_backend, alice):
    async with backend_authenticated_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        rep = await cmds.ping("Hello World !")
        assert rep == AuthenticatedPingRepOk("Hello World !")


@pytest.mark.trio
async def test_handshake_unknown_device(running_backend, alice, mallory):
    with pytest.raises(BackendConnectionRefused) as exc:
        async with backend_authenticated_cmds_factory(
            alice.organization_addr, mallory.device_id, mallory.signing_key
        ) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Invalid handshake information"


@pytest.mark.trio
async def test_handshake_unknown_organization(running_backend, alice):
    unknown_org_addr = BackendOrganizationAddr.build(
        backend_addr=alice.organization_addr.get_backend_addr(),
        organization_id=OrganizationID("dummy"),
        root_verify_key=alice.organization_addr.root_verify_key,
    )
    with pytest.raises(BackendConnectionRefused) as exc:
        async with backend_authenticated_cmds_factory(
            unknown_org_addr, alice.device_id, alice.signing_key
        ) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Invalid handshake information"


@pytest.mark.trio
async def test_handshake_rvk_mismatch(running_backend, alice, otherorg):
    bad_rvk_org_addr = BackendOrganizationAddr.build(
        backend_addr=alice.organization_addr.get_backend_addr(),
        organization_id=alice.organization_id,
        root_verify_key=otherorg.root_verify_key,
    )
    with pytest.raises(BackendConnectionRefused) as exc:
        async with backend_authenticated_cmds_factory(
            bad_rvk_org_addr, alice.device_id, alice.signing_key
        ) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Root verify key for organization differs between client and server"


@pytest.mark.trio
async def test_handshake_revoked_device(running_backend, alice, bob):
    revoked_user_certificate = RevokedUserCertificate(
        author=alice.device_id, timestamp=DateTime.now(), user_id=bob.user_id
    ).dump_and_sign(alice.signing_key)
    await running_backend.backend.user.revoke_user(
        organization_id=alice.organization_id,
        user_id=bob.user_id,
        revoked_user_certificate=revoked_user_certificate,
        revoked_user_certifier=alice.device_id,
    )

    with pytest.raises(BackendConnectionRefused) as exc:
        async with backend_authenticated_cmds_factory(
            bob.organization_addr, bob.device_id, bob.signing_key
        ) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Device has been revoked"


@pytest.mark.trio
async def test_organization_expired(running_backend, alice, expiredorg):

    with running_backend.backend.event_bus.listen() as spy:
        with pytest.raises(BackendConnectionRefused) as exc:
            async with backend_authenticated_cmds_factory(
                expiredorg.addr, alice.device_id, alice.signing_key
            ) as cmds:
                await cmds.ping()
        await spy.wait_with_timeout(BackendEvent.ORGANIZATION_EXPIRED)
    assert str(exc.value) == "Trial organization has expired"


@pytest.mark.trio
async def test_backend_disconnect_during_handshake(alice):
    client_answered = False

    async def poorly_serve_client(stream):
        nonlocal client_answered

        transport = await Transport.init_for_server(stream)
        handshake = ServerHandshake()
        await transport.send(handshake.build_challenge_req())
        await transport.recv()
        # Close connection during handshake
        await stream.aclose()

        client_answered = True

    async with trio.open_service_nursery() as nursery:

        listeners = await nursery.start(
            partial(trio.serve_tcp, poorly_serve_client, port=0, host="127.0.0.1")
        )

        organization_addr = correct_addr(
            alice.organization_addr, listeners[0].socket.getsockname()[1]
        )
        with pytest.raises(BackendNotAvailable):
            async with backend_authenticated_cmds_factory(
                organization_addr, alice.device_id, alice.signing_key
            ) as cmds:
                await cmds.ping()

        nursery.cancel_scope.cancel()

    assert client_answered


@pytest.mark.trio
async def test_events_listen_wait_has_watchdog(monkeypatch, frozen_clock, running_backend, alice):
    KEEPALIVE_TIME = 10
    # Spy on the transport events to detect the Pings/Pongs
    # (Note we are talking about websocket ping, not our own higher-level ping api)
    transport_events_sender, transport_events_receiver = trio.open_memory_channel(100)
    _vanilla_next_ws_event = Transport._next_ws_event

    async def _mocked_next_ws_event(transport):
        event = await _vanilla_next_ws_event(transport)
        await transport_events_sender.send((transport, event))
        return event

    monkeypatch.setattr(Transport, "_next_ws_event", _mocked_next_ws_event)

    async def next_ping_related_event():
        while True:
            transport, event = await transport_events_receiver.receive()
            if isinstance(event, (Ping, Pong)):
                return (transport, event)

    # Highjack the backend api to control the wait time and the final
    # event that will be returned to the client
    backend_received_cmd = trio.Event()
    backend_client_ctx = None
    vanilla_api_events_listen = running_backend.backend.apis[ClientType.AUTHENTICATED][
        "events_listen"
    ]

    async def _mocked_api_events_listen(client_ctx, msg):
        nonlocal backend_client_ctx
        backend_client_ctx = client_ctx
        backend_received_cmd.set()
        return await vanilla_api_events_listen(client_ctx, msg)

    _mocked_api_events_listen._api_info = vanilla_api_events_listen._api_info

    running_backend.backend.apis[ClientType.AUTHENTICATED][
        "events_listen"
    ] = _mocked_api_events_listen

    events_listen_rep = None
    async with backend_authenticated_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key, keepalive=KEEPALIVE_TIME
    ) as cmds:
        async with trio.open_service_nursery() as nursery:

            async def _cmd():
                nonlocal events_listen_rep
                events_listen_rep = await cmds.events_listen(wait=True)

            nursery.start_soon(_cmd)

            # Wait for the connection to be established with the backend
            async with frozen_clock.real_clock_timeout():
                await backend_received_cmd.wait()

            # Now advance time until ping is requested
            await frozen_clock.sleep_with_autojump(KEEPALIVE_TIME + 1)
            async with frozen_clock.real_clock_timeout():
                client_transport, event = await next_ping_related_event()
                assert isinstance(event, Pong)

            # Wait for another ping, just to be sure...
            await frozen_clock.sleep_with_autojump(KEEPALIVE_TIME + 1)
            async with frozen_clock.real_clock_timeout():
                client_transport2, event = await next_ping_related_event()
                assert isinstance(event, Pong)
                assert client_transport is client_transport2

            await backend_client_ctx.send_events_channel.send(EventsListenRepOkPinged("foo"))

    assert events_listen_rep == EventsListenRepOkPinged("foo")


@pytest.mark.trio
async def test_authenticated_cmds_has_right_methods(running_backend, alice):
    async with backend_authenticated_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        for method_name in AUTHENTICATED_CMDS:
            assert hasattr(cmds, method_name)
        for method_name in ALL_CMDS - AUTHENTICATED_CMDS:
            assert not hasattr(cmds, method_name)
