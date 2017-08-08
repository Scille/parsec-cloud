import pytest
import asyncio
from effect2.testing import const, conste, noop, perform_sequence, asyncio_perform_sequence

from parsec.base import EEvent
from parsec.backend.session import EGetAuthenticatedUser
from parsec.backend.backend_api import execute_cmd
from parsec.backend.user_vlob import (
    EUserVlobRead, EUserVlobUpdate, UserVlobAtom, MockedUserVlobComponent)
from parsec.exceptions import UserVlobError
from parsec.tools import to_jsonb64

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLUserVlobComponent(request, loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    conn = module.PostgreSQLConnection(url)
    await conn.open_connection()

    def finalize():
        loop.run_until_complete(conn.close_connection())

    request.addfinalizer(finalize)
    return module.PostgreSQLUserVlobComponent(conn)


@pytest.fixture(params=[MockedUserVlobComponent, bootstrap_PostgreSQLUserVlobComponent],
                ids=['mocked', 'postgresql'])
def component(request, loop):
    if asyncio.iscoroutinefunction(request.param):
        return loop.run_until_complete(request.param(request, loop))
    else:
        return request.param()


class TestUserVlobComponent:

    async def test_user_vlob_read_ok(self, component):
        intent = EUserVlobRead()
        eff = component.perform_user_vlob_read(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com'))
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == UserVlobAtom('alice@test.com', 0, b'')

    async def test_user_vlob_read_previous_version(self, component):
        # Update user vlob
        intent = EUserVlobUpdate(1, b'Next version.')
        eff = component.perform_user_vlob_update(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com')),
            (EEvent('user_vlob_updated', 'alice@test.com'), noop)
        ]
        await asyncio_perform_sequence(sequence, eff)
        # Read previous version
        intent = EUserVlobRead(version=0)
        eff = component.perform_user_vlob_read(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com')),
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == UserVlobAtom('alice@test.com', 0, b'')

    async def test_user_vlob_read_wrong_version(self, component):
        intent = EUserVlobRead(version=42)
        with pytest.raises(UserVlobError):
            eff = component.perform_user_vlob_read(intent)
            sequence = [
                (EGetAuthenticatedUser(), const('alice@test.com'))
            ]
            await asyncio_perform_sequence(sequence, eff)

    async def test_user_vlob_update_ok(self, component):
        intent = EUserVlobUpdate(1, b'Next version.')
        eff = component.perform_user_vlob_update(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com')),
            (EEvent('user_vlob_updated', 'alice@test.com'), noop)
        ]
        await asyncio_perform_sequence(sequence, eff)
        # Check back the value
        intent = EUserVlobRead(version=1)
        eff = component.perform_user_vlob_read(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com'))
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == UserVlobAtom('alice@test.com', 1, b'Next version.')

    async def test_user_vlob_update_wrong_version(self, component):
        intent = EUserVlobUpdate(42, b'Next version.')
        with pytest.raises(UserVlobError):
            eff = component.perform_user_vlob_update(intent)
            sequence = [
                (EGetAuthenticatedUser(), const('alice@test.com')),
            ]
            await asyncio_perform_sequence(sequence, eff)

    async def test_multiple_users(self, component):
        async def _update(user):
                intent = EUserVlobUpdate(1, b'Next version for %s.' % user.encode())
                eff = component.perform_user_vlob_update(intent)
                sequence = [
                    (EGetAuthenticatedUser(), const(user)),
                    (EEvent('user_vlob_updated', user), noop)
                ]
                await asyncio_perform_sequence(sequence, eff)

        await _update('alice@test.com')
        await _update('bob@test.com')

        async def _read(user):
            intent = EUserVlobRead()
            eff = component.perform_user_vlob_read(intent)
            sequence = [
                (EGetAuthenticatedUser(), const(user))
            ]
            return await asyncio_perform_sequence(sequence, eff)

        alice_usa = await _read('alice@test.com')
        assert alice_usa == UserVlobAtom('alice@test.com', 1, b'Next version for alice@test.com.')

        bob_usa = await _read('bob@test.com')
        assert bob_usa == UserVlobAtom('bob@test.com', 1, b'Next version for bob@test.com.')


class TestUserVlobServiceAPI:

    def test_user_vlob_read_ok(self):
        eff = execute_cmd('user_vlob_read', {})
        sequence = [
            (EUserVlobRead(),
                const(UserVlobAtom('alice@test.com', 42, b'content v42')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
            'blob': to_jsonb64(b'content v42'),
            'version': 42
        }

    def test_user_vlob_read_bad_version(self):
        eff = execute_cmd('user_vlob_read', {'version': 42})
        sequence = [
            (EUserVlobRead(version=42), conste(UserVlobError('Wrong blob version.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'user_vlob_error'

    def test_user_vlob_update_ok(self):
        eff = execute_cmd('user_vlob_update', {'version': 42, 'blob': to_jsonb64(b'Next version.')})
        sequence = [
            (EUserVlobUpdate(version=42, blob=b'Next version.'), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
        }

    def test_user_vlob_update_bad_version(self):
        eff = execute_cmd('user_vlob_update', {'version': 42, 'blob': to_jsonb64(b'Next version.')})
        sequence = [
            (EUserVlobUpdate(version=42, blob=b'Next version.'),
                conste(UserVlobError('Wrong blob version.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'user_vlob_error'

# TODO: test event
# TODO: test can't listen other user's events
