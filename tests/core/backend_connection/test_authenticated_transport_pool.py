# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.api.transport import Transport, TransportError
from parsec.core.backend_connection import authenticated_transport_pool_factory


@pytest.fixture
def mocked_transport_ping(monkeypatch):
    # Remember Parsec API's ping (basically a regular Parsec command)
    # and Websocket ping (executed at lower layer, can be send during
    # a Parsec command without breaking request/reply pattern) are
    # two completly different things !
    send_ping, recv_ping = trio.open_memory_channel(1)
    vanilla_ping = Transport.ping

    async def _mocked_ping(transport):
        await send_ping.send(transport)
        hook = getattr(recv_ping, "hook", None)
        if hook:
            await hook(transport)
        return await vanilla_ping(transport)

    monkeypatch.setattr(Transport, "ping", _mocked_ping)

    return recv_ping


@pytest.mark.trio
async def test_keepalive_basic(mocked_transport_ping, mock_clock, running_backend, alice):
    async with authenticated_transport_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key, max=1, watchdog_time=2
    ) as pool:
        # No connection started so far
        async with pool.acquire() as expected_transport:
            # Acquire a connection to force it creation
            pass
        # Now watchdog should take care of the connection after some time
        mock_clock.jump(2)
        mock_clock.rate = 1
        with trio.fail_after(1):
            transport = await mocked_transport_ping.receive()
            assert transport is expected_transport


@pytest.mark.trio
async def test_keepalive_no_concurrency(mocked_transport_ping, mock_clock, running_backend, alice):
    async with authenticated_transport_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key, max=2, watchdog_time=2
    ) as pool:
        # Create 2 transports
        async with pool.acquire() as t1:
            pass
        async with pool.acquire(force_fresh=True) as t2:
            pass

        async with pool.acquire() as taken_transport:
            # The watchdog does deal with an acquired transport
            expected_watched_transport = t1 if taken_transport is t2 else t2
            mock_clock.jump(2)
            mock_clock.rate = 1
            with trio.fail_after(1):
                transport = await mocked_transport_ping.receive()
                await trio.testing.wait_all_tasks_blocked()
                assert mocked_transport_ping.statistics().current_buffer_used == 0
                assert transport is expected_watched_transport

        # Now make sure all transport are watched once released
        mock_clock.jump(2)
        with trio.fail_after(1):
            watched_a = await mocked_transport_ping.receive()
            watched_b = await mocked_transport_ping.receive()
            await trio.testing.wait_all_tasks_blocked()
            if watched_a == t1:
                assert watched_b == t2
            else:
                assert watched_a == t2
                assert watched_b == t1


@pytest.mark.trio
async def test_keepalive_cant_acquire_watched_transport(
    mocked_transport_ping, mock_clock, running_backend, alice
):
    async def _hook(transport):
        await trio.sleep_forever()

    mocked_transport_ping.hook = _hook

    async with authenticated_transport_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key, max=2, watchdog_time=2
    ) as pool:
        # Create a transport
        async with pool.acquire() as t1:
            pass

        mock_clock.jump(2)
        mock_clock.rate = 1
        with trio.fail_after(1):
            t_watched = await mocked_transport_ping.receive()
            assert t_watched is t1
            # Now we have t1 in a middle of a ping, it should not be available
            # for acquiring then
            async with pool.acquire() as t2:
                assert t2 is not t1


@pytest.mark.trio
async def test_failed_keepalive_drop_transport(
    mocked_transport_ping, mock_clock, running_backend, alice
):
    async def _hook(transport):
        raise TransportError("Tough...")

    mocked_transport_ping.hook = _hook

    async with authenticated_transport_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key, max=2, watchdog_time=2
    ) as pool:
        # Create a transport
        async with pool.acquire() as t1:
            pass

        # New acquire should reuse opened transport...
        async with pool.acquire() as t_reuse:
            assert t_reuse is t1

        # ...unless watchdog has failed
        mock_clock.jump(2)
        mock_clock.rate = 1
        with trio.fail_after(1):
            t_watched = await mocked_transport_ping.receive()
            assert t_watched is t1
            async with pool.acquire() as t_new:
                assert t_reuse is not t_new
