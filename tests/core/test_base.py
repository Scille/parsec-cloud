import pytest
from trio.testing import trio_test

from tests.common import with_core, async_patch, TEST_USERS, alice


@trio_test
@with_core()
async def test_connection(core):
    async with core.test_connect() as sock:
        await sock.send({'cmd': 'get_core_state'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
    # Deconnection, then reco
    async with core.test_connect() as sock:
        await sock.send({'cmd': 'get_core_state'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}


@trio_test
@with_core(with_backend=False)
async def test_multi_connections(core):
    async with core.test_connect() as sock1:
        async with core.test_connect() as sock2:
            await sock1.send({'cmd': 'get_core_state'})
            await sock2.send({'cmd': 'get_core_state'})

            rep1 = await sock1.recv()
            rep2 = await sock2.recv()

            assert rep1 == {'status': 'ok'}
            assert rep2 == {'status': 'ok'}
        # Sock 1 should not have been affected by sock 2 leaving
        await sock1.send({'cmd': 'get_core_state'})
        rep = await sock1.recv()
        assert rep == {'status': 'ok'}


@trio_test
@async_patch('parsec.core.app.CoreApp._get_user')
@with_core(with_backend=False, mocked_get_user=False)
async def test_offline_login_and_logout(core, get_user_mock):
    # Backend is not available, hence we can use a user not registered in it
    get_user_mock.return_value = (
        b'\xfcv\xd8t\xac6\xb6&\xab\xf6\xd7\xb1\x1b{:fQ\x86\xcf\x87$\t}\xc6%\x90D\xc1g\xc9|\xf9',
        b'\x16\x0c\x02Nz\xce}\xff\xbf\xc4W\xf7\x16\xa2j\xc7\xb1\x06[#v\x81AR\xe9\xdc\xe4m\x93Y\xcc\xb2'
    )
    async with core.test_connect() as sock:
        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': False, 'id': None}
        # Do the login
        await sock.send({'cmd': 'identity_login', 'id': 'john@test', 'password': '<secret>'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': True, 'id': 'john@test'}

    # Changing socket should not trigger logout
    async with core.test_connect() as sock:
        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': True, 'id': 'john@test'}
        # Actual logout
        await sock.send({'cmd': 'identity_logout'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': False, 'id': None}

    # TODO: make the backend startup and check exception is triggered given
    # user is unknown


@trio_test
@with_core()
async def test_login_and_logout(core):
    async with core.test_connect() as sock:
        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': False, 'id': None}
        # Do the login
        await sock.send({'cmd': 'identity_login', 'id': alice.id, 'password': '<secret>'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': True, 'id': alice.id}

    # Changing socket should not trigger logout
    async with core.test_connect() as sock:
        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': True, 'id': alice.id}
        # Actual logout
        await sock.send({'cmd': 'identity_logout'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        await sock.send({'cmd': 'identity_info'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'loaded': False, 'id': None}


@trio_test
@with_core()
async def test_need_login_cmds(core):
    async with core.test_connect() as sock:
        for cmd in [
                'identity_logout',
                'file_create',
                'file_read',
                'file_write',
                'file_sync',
                'stat',
                'folder_create',
                'move',
                'delete',
                'file_truncate'
            ]:
            await sock.send({'cmd': cmd})
            rep = await sock.recv()
            assert rep == {'status': 'login_required'}
