import pytest
from trio.testing import trio_test
from nacl.public import PrivateKey

from parsec.utils import to_jsonb64

from tests.common import with_backend, async_patch


@trio_test
@with_backend(populated_for='alice')
async def test_user_vlob_read_ok(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_vlob_read', 'version': 1})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'blob': to_jsonb64(backend.test_populate_data['user_vlobs']['alice@test'][0]),
            'version': 1
        }


@trio_test
@with_backend(populated_for='alice')
async def test_user_vlob_read_last_version(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_vlob_read'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'blob': to_jsonb64(backend.test_populate_data['user_vlobs']['alice@test'][-1]),
            'version': 3
        }

@trio_test
@with_backend(populated_for='alice')
async def test_user_vlob_read_bad_version(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_vlob_read', 'version': 42})
        rep = await sock.recv()
        assert rep == {'status': 'user_vlob_error', 'reason': 'Wrong blob version.'}


@pytest.mark.parametrize('bad_msg', [
    {'version': None},
    {'version': '42x'},
    {'bad_field': 'foo'}
])
@trio_test
@with_backend()
async def test_user_vlob_read_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_vlob_read', **bad_msg})
        rep = await sock.recv()
        # assert rep == {}
        assert rep['status'] == 'bad_message'


@trio_test
@with_backend(populated_for='alice')
async def test_user_vlob_update_ok(backend):
    next_version = len(backend.test_populate_data['user_vlobs']['alice@test']) + 1
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'user_vlob_update',
            'version': next_version,
            'blob': to_jsonb64(b'fooo')
        })
        rep = await sock.recv()
        assert rep == {'status': 'ok'}


@trio_test
@with_backend(populated_for='alice')
async def test_user_vlob_update_bad_version(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'user_vlob_update',
            'version': 42,
            'blob': to_jsonb64(b'fooo')
        })
        rep = await sock.recv()
        assert rep == {'status': 'user_vlob_error', 'reason': 'Wrong blob version.'}


@pytest.mark.parametrize('bad_msg', [
    {'version': 42, 'blob': to_jsonb64(b'...'), 'bad_field': 'foo'},
    {'version': '42x', 'blob': to_jsonb64(b'...')},
    {'version': None, 'blob': to_jsonb64(b'...')},
    {'version': 42, 'blob': 42},
    {'version': 42, 'blob': None},
    {'version': 42, 'blob': '<not a b64>'},
    {}
])
@trio_test
@with_backend()
async def test_user_vlob_update_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_vlob_update', **bad_msg})
        rep = await sock.recv()
        # Id and trust_seed are invalid anyway, but here we test another layer
        # so it's not important as long as we get our `bad_message` status
        assert rep['status'] == 'bad_message'
