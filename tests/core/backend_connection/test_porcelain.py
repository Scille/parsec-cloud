# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.core.backend_connection import BackendNotAvailable, backend_cmds_pool_factory


@pytest.mark.trio
async def test_backend_offline(alice):
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        with pytest.raises(BackendNotAvailable):
            # Must send a request given the pool lazily creates connections
            await cmds.ping("Whatever")


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, alice, tcp_stream_spy):
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        # First, have a good request to make sure a socket has been opened
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"

        # Current socket is down, but opening another socket
        # should solve the trouble
        async def _broken_send_stream():
            raise trio.BrokenResourceError("Huho!")

        tcp_stream_spy.get_socks(alice.organization_addr)[
            -1
        ].send_stream.send_all_hook = _broken_send_stream
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"

        # Now sockets will never be able to reach the backend no matter what
        with running_backend.offline():
            with pytest.raises(BackendNotAvailable):
                await cmds.ping("Hello World !")

        # Finally make sure we can still connect to the backend
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"


@pytest.mark.trio
@pytest.mark.parametrize("cmds_used", (False, True))
async def test_backend_closed_cmds(cmds_used, running_backend, alice):
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        if cmds_used:
            await cmds.ping("Whatever")
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping("Whatever")


@pytest.mark.trio
async def test_concurrency_sends(running_backend, alice):

    CONCURRENCY = 10
    work_done_counter = 0
    work_all_done = trio.Event()

    async def sender(cmds, x):
        nonlocal work_done_counter
        rep = await cmds.ping(x)
        assert rep == x
        work_done_counter += 1
        if work_done_counter == CONCURRENCY:
            work_all_done.set()

    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:

        async with trio.open_service_nursery() as nursery:
            for x in range(CONCURRENCY):
                nursery.start_soon(sender, cmds, str(x))

        with trio.fail_after(1):
            await work_all_done.wait()
