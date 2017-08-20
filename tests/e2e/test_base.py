import pytest
import asyncio
import aiohttp


async def test_run_backend(backend, backend_host):
    assert backend.started
    session = aiohttp.ClientSession()
    async with session.ws_connect(backend_host) as ws:
        ws.ping()


async def test_run_core(core, core_socket):
    reader, writer = await asyncio.open_unix_connection(path=core_socket)
    writer.write(b'{"cmd": "ping", "ping": "hello"}\n')
    raw_resp = await reader.readline()
    assert raw_resp == b'{"pong": "hello", "status": "ok"}\n'


async def test_backend_status(johndoe_client):
    ret = await johndoe_client.send_cmd('backend_status')
    assert ret == {'status': 'ok'}
