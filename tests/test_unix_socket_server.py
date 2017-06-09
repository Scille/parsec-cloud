import os
import json
import pytest
import asyncio
from tempfile import mktemp

from parsec.server import UnixSocketServer
from parsec.service import BaseService, event, cmd


@pytest.fixture
async def unix_socket_server(request, event_loop, unused_tcp_port):
    event_loop.set_debug(True)

    class PingPongService(BaseService):
        name = 'PingPongService'
        on_ping = event('on_ping')

        @cmd('ping')
        async def ping(self, session, cmd):
            self.on_ping.send(cmd['ping'])
            return {'status': 'ok', 'pong': cmd['ping']}

    socket_path = mktemp()
    server = UnixSocketServer()
    server.register_service(PingPongService())
    server_task = await server.start(socket_path, loop=event_loop, block=False)

    def finalize():
        event_loop.run_until_complete(server.teardown_services())
        server_task.close()
        event_loop.run_until_complete(server_task.wait_closed())

    request.addfinalizer(finalize)
    return server, socket_path


@pytest.mark.asyncio
async def test_socket_exists(unix_socket_server):
    _, socket_path = unix_socket_server
    assert os.path.stat.S_ISSOCK(os.stat(socket_path).st_mode)


@pytest.mark.asyncio
async def test_socket_communication(unix_socket_server):
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    try:
        writer.write(b'{"cmd": "ping", "ping": "foo"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {
            "status": "ok", "pong": "foo"}
    finally:
        writer.close()


@pytest.mark.asyncio
async def test_big_socket_communication(unix_socket_server):
    big_payload = b'X' * int(10e6)  # 10mo big payload
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    try:
        writer.write(b'{"cmd": "ping", "ping": "')
        writer.write(big_payload)
        writer.write(b'"}\n')
        resp = b''
        while True:
            resp += await reader.read(65536)
            if resp[-1] == ord(b'\n'):
                break
        assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": big_payload.decode()}
    finally:
        writer.close()


@pytest.mark.asyncio
async def test_notification(unix_socket_server):
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    try:
        # Subscribe event
        writer.write(b'{"cmd": "subscribe", "event": "on_ping", "sender": "hello"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {"status": "ok"}
        # Trigger event
        writer.write(b'{"cmd": "ping", "ping": "hello"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": "hello"}
        # Should receive a notification
        notif = await reader.readline()
        assert json.loads(notif[:-1].decode()) == {'event': 'on_ping', 'sender': 'hello'}
    finally:
        writer.close()


@pytest.mark.asyncio
async def test_unsubscribe_notification(unix_socket_server):
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    try:
        # Subscribe event
        # Subscribe same event with two different senders
        async def subscribe(name):
            writer.write(b'{"cmd": "subscribe", "event": "on_ping", "sender": "%s"}\n' % name.encode())
            resp = await reader.readline()
            assert json.loads(resp[:-1].decode()) == {"status": "ok"}
        await subscribe('alice')
        await subscribe('bob')

        # Then unsubscribe bob's event
        writer.write(b'{"cmd": "unsubscribe", "event": "on_ping", "sender": "bob"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {"status": "ok"}

        # Trigger bob's event
        writer.write(b'{"cmd": "ping", "ping": "bob"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": "bob"}

        # No event should arrive
        with pytest.raises(asyncio.TimeoutError):
            reading = reader.readline()
            resp = await asyncio.wait_for(reading, 0.1)
        try:
            await reading
        except asyncio.CancelledError:
            pass


        # Of courses alice's event should still work
        writer.write(b'{"cmd": "ping", "ping": "alice"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": "alice"}

        reading = reader.readline()
        resp = await asyncio.wait_for(reading, 0.01)
        assert json.loads(resp[:-1].decode()) == {'event': 'on_ping', 'sender': 'alice'}

    finally:
        writer.close()


@pytest.mark.asyncio
async def test_multi_sender_notification(unix_socket_server):
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    try:
        # Subscribe same event with two different senders
        async def subscribe(name):
            writer.write(b'{"cmd": "subscribe", "event": "on_ping", "sender": "%s"}\n' % name.encode())
            resp = await reader.readline()
            assert json.loads(resp[:-1].decode()) == {"status": "ok"}
        await subscribe('alice')
        await subscribe('bob')

        # Trigger event
        async def do_ping(name):
            writer.write(b'{"cmd": "ping", "ping": "%s"}\n' % name.encode())
            resp = await reader.readline()
            assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": name}

        async def check_event(name):
            reading = reader.readline()
            resp = await asyncio.wait_for(reading, 0.01)
            assert json.loads(resp[:-1].decode()) == {'event': 'on_ping', 'sender': name}

        async def check_no_event():
            reading = reader.readline()
            event = None
            try:
                resp = await asyncio.wait_for(reading, 0.1)
                event = json.loads(resp[:-1].decode())
            except asyncio.TimeoutError:
                try:
                    await reading
                except asyncio.CancelledError:
                    pass
            assert not event

        await do_ping('bob')
        await check_event('bob')
        await do_ping('john')  # should not receive this one
        await check_no_event()
        await do_ping('alice')
        await check_event('alice')
    finally:
        writer.close()


@pytest.mark.asyncio
async def test_base_cmds(unix_socket_server):
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    try:
        writer.write(b'{"cmd": "list_cmds"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {
            "status": "ok", "cmds": ['list_cmds', 'list_events', 'ping', 'subscribe', 'unsubscribe']}

        writer.write(b'{"cmd": "list_events"}\n')
        resp = await reader.readline()
        assert json.loads(resp[:-1].decode()) == {"status": "ok", "events": ['on_ping']}
    finally:
        writer.close()
