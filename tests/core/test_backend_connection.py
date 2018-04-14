import pytest
import trio
from async_generator import asynccontextmanager
import blinker

from parsec.core.backend_connection import BackendConnection, BackendNotAvailable

from tests.open_tcp_stream_mock_wrapper import offline


@asynccontextmanager
async def open_backend_connection(user, backend_addr):
    signal_ns = blinker.Namespace()
    conn = BackendConnection(user, backend_addr, signal_ns)
    async with trio.open_nursery() as nursery:
        await conn.init(nursery)
        yield conn

        await conn.teardown()


@pytest.mark.trio
async def test_base(running_backend, backend_addr, alice):
    async with open_backend_connection(alice, backend_addr) as conn:
        rep = await conn.send({"cmd": "ping", "ping": "hello"})
        assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_concurrency_sends(running_backend, backend_addr, alice):
    count = 0

    async def sender(conn, x):
        nonlocal count
        rep = await conn.send({"cmd": "ping", "ping": x})
        assert rep == {"status": "ok", "pong": x}
        count += 1

    async with open_backend_connection(alice, backend_addr) as conn:
        async with trio.open_nursery() as nursery:
            for x in range(5):
                nursery.start_soon(sender, conn, str(x))

    assert count == 5


@pytest.mark.trio
async def test_backend_offline(unused_tcp_port, alice):
    addr = "tcp://127.0.0.1:%s" % unused_tcp_port

    signal_ns = blinker.Namespace()
    conn = BackendConnection(alice, addr, signal_ns)
    async with trio.open_nursery() as nursery:
        await conn.init(nursery)

        with pytest.raises(BackendNotAvailable):
            await conn.send({"cmd": "ping", "ping": "hello"})
        with pytest.raises(BackendNotAvailable):
            await conn.send({"cmd": "ping", "ping": "hello"})

        await conn.teardown()


@pytest.mark.trio
async def test_backend_switch_offline(
    running_backend, backend_addr, alice, tcp_stream_spy
):
    async with open_backend_connection(alice, backend_addr) as conn:

        # Connection ok

        rep = await conn.send({"cmd": "ping", "ping": "hello"})
        assert rep == {"status": "ok", "pong": "hello"}

        # Current socket is down, but opening another socket
        # should solve the trouble

        tcp_stream_spy.socks[backend_addr][-1].send_stream.close()

        rep = await conn.send({"cmd": "ping", "ping": "hello"})
        assert rep == {"status": "ok", "pong": "hello"}

        # Now sockets will never be able to reach the backend no matter what

        with offline(backend_addr):
            with pytest.raises(BackendNotAvailable):
                await conn.send({"cmd": "ping", "ping": "hello"})

        # Finally make sure we can still connect to the backend

        rep = await conn.send({"cmd": "ping", "ping": "hello"})
        assert rep == {"status": "ok", "pong": "hello"}


# @pytest.mark.trio
# async def test_backend_event_passthrough(running_backend, backend_addr, alice, alice_backend_sock):
#     backend, _, _ = running_backend

#     async with open_backend_connection(alice, backend_addr) as conn:
#         # Dummy event (not provided by backend)
#         await conn.subscribe_event('ping')
#         await conn.send({'cmd': 'event_subscribe', 'event': 'foo'})
#         rep = await conn.recv()
#         assert rep == {'status': 'ok'}
#         # Event passed by backend
#         await conn.send({'cmd': 'event_subscribe', 'event': 'ping', 'subject': 'back'})
#         rep = await conn.recv()
#         assert rep == {'status': 'ok'}

#         backend.signal_ns.send('foo', 'bar')
#         backend.signal_ns.send('foo', 'bar')
#         await conn.send({'cmd': 'event_listen', 'wait': False})
#         rep = await conn.recv()
#         rep == {'status': 'ok'}

#         backend.signal_ns.send('ping', 'bar')
#         await conn.send({'cmd': 'event_listen', 'wait': False})
#         rep = await conn.recv()
#         rep == {'status': 'ok', 'event': 'ping', 'subject': 'bar'}


@pytest.mark.trio
async def test_backend_event_passthrough_not_configured(
    running_backend, backend_addr, alice
):
    backend, _, _ = running_backend

    async with open_backend_connection(alice, backend_addr) as conn:
        # Make sure connection with backend is up and running
        await conn.send({"cmd": "ping"})

        received_by_core_events = []
        conn.signal_ns.signal("ping").connect(
            lambda *args: received_by_core_events.append(args)
        )

        # Trigger events inside the backend, core should not be notified of them
        backend.signal_ns.signal("ping").send()
        backend.signal_ns.signal("ping").send("bar")

        # Don't like to do that, but given we test that nothing happened...
        await trio.sleep(0.01)

        assert not received_by_core_events


@pytest.mark.trio
async def test_backend_event_passthrough_subscribing(
    running_backend, backend_addr, alice
):
    backend, _, _ = running_backend

    async with open_backend_connection(alice, backend_addr) as conn:
        # Make sure connection with backend is up and running
        await conn.send({"cmd": "ping", "ping": "too early"})

        core_ping_events = []

        def collect_core_ping_events(sender):
            core_ping_events.append(sender)

        conn.signal_ns.signal("ping").connect(collect_core_ping_events)

        await conn.subscribe_event("ping")
        # TODO: backend_connection event pump is really ugly so far,
        # need to improve code to avoid this sleep
        await trio.sleep(0.1)

        backend.signal_ns.signal("dummy").send()
        backend.signal_ns.signal("ping").send()
        backend.signal_ns.signal("ping").send("bar")

        # Don't like to do that, but needed to have events dispatched
        await trio.sleep(0.1)

        assert core_ping_events == [None, "bar"]

        # Now forget about the event

        await conn.unsubscribe_event("ping")
        core_ping_events.clear()

        backend.signal_ns.signal("dummy").send()
        backend.signal_ns.signal("ping").send()
        backend.signal_ns.signal("ping").send("too late")

        # Don't like to do that, but given we test that nothing happened...
        await trio.sleep(0.01)

        assert not core_ping_events
