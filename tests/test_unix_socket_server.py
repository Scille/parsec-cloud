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
    writer.write(b'{"cmd": "list_cmds"}\n')
    resp = await reader.readline()
    assert json.loads(resp[:-1].decode()) == {"status": "ok", "cmds": ["list_cmds", "ping", "subscribe"]}
    writer.close()


@pytest.mark.asyncio
async def test_big_socket_communication(unix_socket_server):
    big_payload = b'X' * int(10e6)  # 10mo big payload
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
    writer.write(b'{"cmd": "ping", "ping": "')
    writer.write(big_payload)
    writer.write(b'"}\n')
    resp = b''
    while True:
        resp += await reader.read(65536)
        if resp[-1] == ord(b'\n'):
            break
    assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": big_payload.decode()}
    writer.close()


@pytest.mark.asyncio
async def test_notification(unix_socket_server):
    server, socket_path = unix_socket_server
    reader, writer = await asyncio.open_unix_connection(socket_path)
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
    writer.close()
