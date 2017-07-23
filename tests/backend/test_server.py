import os
import pytest
import websockets

from parsec.backend.server import run_websocket_server


@pytest.mark.asyncio
async def test_socket_exists(event_loop, unused_tcp_port):

    async def on_connection(ws, route):
        msg = await ws.recv()
        assert msg == b'ping'
        await ws.send(b'pong')

    async with run_websocket_server(
            on_connection=on_connection, port=unused_tcp_port, loop=event_loop):
        async with websockets.connect('ws://localhost:%s' % unused_tcp_port) as client:
            await client.send(b'ping')
            ret = await client.recv()
            assert ret == b'pong'
