# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import trio

from parsec._parsec import InvitationType
from parsec.api.transport import Transport, BytesMessage, Ping, Pong
from parsec.core.types import BackendInvitationAddr
from parsec.core.backend_connection import (
    backend_authenticated_cmds_factory,
    backend_invited_cmds_factory,
)


async def _test_keepalive(frozen_clock, monkeypatch, cmds_factory):
    KEEPALIVE_TIME = 10
    ping_events_sender, ping_events_receiver = trio.open_memory_channel(100)
    transport_ready = trio.Event()
    _vanilla_next_ws_event = Transport._next_ws_event

    async def _mocked_next_ws_event(transport):
        transport_ready.set()
        while True:
            event = await _vanilla_next_ws_event(transport)
            if isinstance(event, BytesMessage):
                # Filter out all commands events except handshake
                if b"handshake" not in event.data:
                    continue
            if isinstance(event, (Ping, Pong)):
                await ping_events_sender.send((transport, event))
            return event

    # Transport is only used by client
    monkeypatch.setattr(Transport, "_next_ws_event", _mocked_next_ws_event)

    async with cmds_factory(keepalive=KEEPALIVE_TIME) as cmds:
        # Backend won't receive our command (remember api level ping has nothing
        # to do with websocket level ping !), so the client will end up sending
        # websocket pings to keep the connection alive while waiting for the
        # never-comming answer
        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(cmds.ping, "Whatever")

            # Wait for the transport to be ready, then wait until ping is requested
            await transport_ready.wait()
            await frozen_clock.sleep_with_autojump(KEEPALIVE_TIME + 1)
            async with frozen_clock.real_clock_timeout():
                client_transport, event = await ping_events_receiver.receive()
                assert isinstance(event, Pong)

            # Wait for another ping, just to be sure...
            await frozen_clock.sleep_with_autojump(KEEPALIVE_TIME + 1)
            async with frozen_clock.real_clock_timeout():
                client_transport2, event = await ping_events_receiver.receive()
                assert isinstance(event, Pong)
                assert client_transport is client_transport2

            nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_authenticated_cmd_keepalive(frozen_clock, monkeypatch, running_backend, alice):
    def _cmds_factory(keepalive):
        return backend_authenticated_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key, keepalive=keepalive
        )

    await _test_keepalive(frozen_clock, monkeypatch, _cmds_factory)


@pytest.mark.trio
async def test_invited_cmd_keepalive(
    frozen_clock, monkeypatch, backend, running_backend, backend_addr, alice
):
    invitation = await backend.invite.new_for_device(
        organization_id=alice.organization_id, greeter_user_id=alice.user_id
    )
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=alice.organization_addr.get_backend_addr(),
        organization_id=alice.organization_id,
        invitation_type=InvitationType.DEVICE(),
        token=invitation.token,
    )

    def _cmds_factory(keepalive):
        return backend_invited_cmds_factory(invitation_addr, keepalive=keepalive)

    await _test_keepalive(frozen_clock, monkeypatch, _cmds_factory)
