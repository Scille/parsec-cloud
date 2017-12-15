import pytest
from trio.testing import trio_test

from tests.common import with_backend


@trio_test
@with_backend()
async def test_connect_as_anonymous(backend):
    async with backend.test_connect('anonymous') as sock:
        await sock.send({'cmd': 'ping', 'ping': 'foo'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'pong': 'foo'}


@trio_test
@pytest.mark.parametrize('cmd', [
    'user_get',
    'user_create',
    'blockstore_post',
    'blockstore_get',
    'blockstore_get_url',
    'vlob_create',
    'vlob_read',
    'vlob_update',
    'user_vlob_read',
    'user_vlob_update',
    'group_read',
    'group_create',
    'group_add_identities',
    'group_remove_identities',
    'message_get',
    'message_new',
    'pubkey_get',
])
@with_backend()
async def test_anonymous_has_limited_access(backend, cmd):
    async with backend.test_connect('anonymous') as sock:
        await sock.send({'cmd': cmd})
        rep = await sock.recv()
        assert rep == {'status': 'bad_cmd', 'reason': 'Unknown command `%s`' % cmd}
