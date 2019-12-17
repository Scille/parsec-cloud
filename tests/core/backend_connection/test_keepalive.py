# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.api.transport import Transport, BytesMessage, Ping, Pong
from parsec.core.backend_connection import (
    backend_authenticated_cmds_factory,
    backend_anonymous_cmds_factory,
    backend_administration_cmds_factory,
)


@pytest.mark.trio
async def _test_keepalive(mock_clock, monkeypatch, cmds_factory):
    ping_events_sender, ping_events_receiver = trio.open_memory_channel(100)
    _vanilla_next_ws_event = Transport._next_ws_event

    async def _mocked_next_ws_event(transport):
        while True:
            event = await _vanilla_next_ws_event(transport)
            if isinstance(event, BytesMessage):
                # Filter out all commands events except handshake
                if b"handshake" not in event.data:
                    continue
            if isinstance(event, (Ping, Pong)):
                await ping_events_sender.send((transport, event))
            return event

    monkeypatch.setattr(Transport, "_next_ws_event", _mocked_next_ws_event)

    mock_clock.rate = 1
    async with cmds_factory(keepalive=10) as cmds:
        # Backend won't receive our command (remember api level ping has nothing
        # to do with websocket level ping !), so the client will end up sending
        # websocket pings to keep the connection alive while waiting for the
        # never-comming answer
        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(cmds.ping, "Wathever")

            # Now advance time until ping is requested
            await trio.testing.wait_all_tasks_blocked()
            mock_clock.jump(10)
            with trio.fail_after(1):
                backend_transport, event = await ping_events_receiver.receive()
                assert isinstance(event, Ping)
                client_transport, event = await ping_events_receiver.receive()
                assert isinstance(event, Pong)
                assert client_transport is not backend_transport

            # Wait for another ping, just to be sure...
            await trio.testing.wait_all_tasks_blocked()
            mock_clock.jump(10)
            with trio.fail_after(1):
                backend_transport2, event = await ping_events_receiver.receive()
                assert isinstance(event, Ping)
                assert backend_transport is backend_transport2
                client_transport2, event = await ping_events_receiver.receive()
                assert isinstance(event, Pong)
                assert client_transport is client_transport2

            nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_authenticated_cmd_keepalive(mock_clock, monkeypatch, running_backend, alice):
    def _cmds_factory(keepalive):
        return backend_authenticated_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key, keepalive=keepalive
        )

    await _test_keepalive(mock_clock, monkeypatch, _cmds_factory)


@pytest.mark.trio
async def test_anonymous_cmd_keepalive(mock_clock, monkeypatch, running_backend, coolorg):
    def _cmds_factory(keepalive):
        return backend_anonymous_cmds_factory(coolorg.addr, keepalive=keepalive)

    await _test_keepalive(mock_clock, monkeypatch, _cmds_factory)


@pytest.mark.trio
async def test_administration_cmd_keepalive(
    mock_clock, monkeypatch, running_backend, backend_addr, backend
):
    def _cmds_factory(keepalive):
        return backend_administration_cmds_factory(
            backend_addr, backend.config.administration_token, keepalive=keepalive
        )

    await _test_keepalive(mock_clock, monkeypatch, _cmds_factory)
