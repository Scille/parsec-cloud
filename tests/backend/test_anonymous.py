import pytest

from tests.common import connect_backend


@pytest.mark.trio
async def test_connect_as_anonymous(backend):
    async with connect_backend(backend, auth_as='anonymous') as sock:
        await sock.send({'cmd': 'ping', 'ping': 'foo'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'pong': 'foo'}


@pytest.mark.trio
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
async def test_anonymous_has_limited_access(backend, cmd):
    async with connect_backend(backend, auth_as='anonymous') as sock:
        await sock.send({'cmd': cmd})
        rep = await sock.recv()
        assert rep == {'status': 'unknown_command'}
