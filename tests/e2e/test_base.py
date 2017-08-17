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


@pytest.mark.parametrize('cmd', [
    'backend_status',
    'identity_info',
    'identity_unload',
    'synchronize'
])
async def test_identity_not_loaded(client, cmd):
    ret = await client.send_cmd(cmd)
    assert ret == {'label': 'Identity not loaded', 'status': 'identity_not_loaded'}


async def test_identity_load(client, johndoe):
    print('a')
    ret = await client.send_cmd('identity_load', id=johndoe.id, key=johndoe.privkey)
    assert ret == {'status': 'ok'}
    print('a')
    ret = await client.send_cmd('identity_info')
    assert ret == {'id': 'John_Doe', 'loaded': True, 'status': 'ok'}
    print('a')
    ret = await client.send_cmd('identity_unload')
    assert ret == {'status': 'ok'}
    print('a')


async def test_backend_status(johndoe_client):
    ret = await johndoe_client.send_cmd('backend_status')
    assert ret == {'status': 'ok'}
