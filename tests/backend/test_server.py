import os
import websockets

from parsec.backend.server import run_websocket_server


async def test_socket_exists(loop, unused_port):
    unused_port = unused_port()

    async def on_connection(ws, route):
        msg = await ws.recv()
        assert msg == b'ping'
        await ws.send(b'pong')

    async with run_websocket_server(
            on_connection=on_connection, port=unused_port, loop=loop):
        async with websockets.connect('ws://localhost:%s' % unused_port, loop=loop) as client:
            await client.send(b'ping')
            ret = await client.recv()
            assert ret == b'pong'
