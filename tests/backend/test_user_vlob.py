import pytest
from effect2.testing import const, perform_sequence, raise_, noop

from parsec.base import EEvent
from parsec.backend.session import EGetAuthenticatedUser
from parsec.backend.backend_api import execute_cmd
from parsec.backend.user_vlob import (
    EUserVlobRead, EUserVlobUpdate, UserVlobAtom, MockedUserVlobComponent)
from parsec.exceptions import UserVlobError
from parsec.tools import to_jsonb64


@pytest.fixture
def component():
    return MockedUserVlobComponent()


class TestUserVlobComponent:

    def test_user_vlob_read_ok(self, component):
        intent = EUserVlobRead()
        eff = component.perform_user_vlob_read(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com'))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == UserVlobAtom('alice@test.com', 0, b'')

    def test_user_vlob_read_previous_version(self, component):
        # Update user vlob
        intent = EUserVlobUpdate(1, b'Next version.')
        eff = component.perform_user_vlob_update(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com')),
            (EEvent('user_vlob_updated', 'alice@test.com'), noop)
        ]
        perform_sequence(sequence, eff)
        # Read previous version
        intent = EUserVlobRead(version=0)
        eff = component.perform_user_vlob_read(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com')),
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == UserVlobAtom('alice@test.com', 0, b'')

    def test_user_vlob_read_wrong_version(self, component):
        intent = EUserVlobRead(version=42)
        with pytest.raises(UserVlobError):
            eff = component.perform_user_vlob_read(intent)
            sequence = [
                (EGetAuthenticatedUser(), const('alice@test.com'))
            ]
            perform_sequence(sequence, eff)

    def test_user_vlob_update_ok(self, component):
        intent = EUserVlobUpdate(1, b'Next version.')
        eff = component.perform_user_vlob_update(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com')),
            (EEvent('user_vlob_updated', 'alice@test.com'), noop)
        ]
        perform_sequence(sequence, eff)
        # Check back the value
        intent = EUserVlobRead(version=1)
        eff = component.perform_user_vlob_read(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com'))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == UserVlobAtom('alice@test.com', 1, b'Next version.')

    def test_user_vlob_update_wrong_version(self, component):
        intent = EUserVlobUpdate(42, b'Next version.')
        with pytest.raises(UserVlobError):
            eff = component.perform_user_vlob_update(intent)
            sequence = [
                (EGetAuthenticatedUser(), const('alice@test.com')),
            ]
            perform_sequence(sequence, eff)

    def test_multiple_users(self, component):
        def _update(user):
                intent = EUserVlobUpdate(1, b'Next version for %s.' % user.encode())
                eff = component.perform_user_vlob_update(intent)
                sequence = [
                    (EGetAuthenticatedUser(), const(user)),
                (EEvent('user_vlob_updated', user), noop)
                ]
                perform_sequence(sequence, eff)

        _update('alice@test.com')
        _update('bob@test.com')

        def _read(user):
            intent = EUserVlobRead()
            eff = component.perform_user_vlob_read(intent)
            sequence = [
                (EGetAuthenticatedUser(), const(user))
            ]
            return perform_sequence(sequence, eff)

        assert _read('alice@test.com') == UserVlobAtom(
            'alice@test.com', 1, b'Next version for alice@test.com.')
        assert _read('bob@test.com') == UserVlobAtom(
            'bob@test.com', 1, b'Next version for bob@test.com.')


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
            (EUserVlobRead(version=42),
                lambda x: raise_(UserVlobError('Wrong blob version.')))
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
                lambda x: raise_(UserVlobError('Wrong blob version.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'user_vlob_error'

# TODO: test event
# TODO: test can't listen other user's events
