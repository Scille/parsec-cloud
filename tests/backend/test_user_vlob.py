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
async def test_user_vlob_read_bad_version(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'user_vlob_read', 'version': 42})
        rep = await sock.recv()
        assert rep == {'status': 'user_vlob_error', 'reason': 'Wrong blob version.'}


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


# TODO: test bad params for user_vlob_update and user_vlob_read
