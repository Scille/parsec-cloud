import trio
import socket
import inspect
import attr
from collections import defaultdict
from unittest.mock import Mock, patch
from functools import wraps
from nacl.public import PrivateKey
from nacl.signing import SigningKey

from parsec.core.app import CoreApp
from parsec.utils import CookedSocket, to_jsonb64, from_jsonb64, User
from parsec.core.local_storage import BaseLocalStorage
from parsec.backend.app import BackendApp

from tests.populate import populate_local_storage_cls


alice = User(
    'alice@test',
    b'\xceZ\x9f\xe4\x9a\x19w\xbc\x12\xc8\x98\xd1CB\x02vS\xa4\xfe\xc8\xc5\xa6\xcd\x87\x90\xd7\xabJ\x1f$\x87\xc4',
    b'\xa7\n\xb2\x94\xbb\xe6\x03\xd3\xd0\xd3\xce\x95\xe6\x8b\xfe5`(\x15\xc0UL\xe9\x1dTf^ m\xb7\xbc\\'
)

bob = User(
    'bob@test',
    b'\xc3\xc9(\xf7\\\xd2\xb4[\x85\xe5\xfa\xd3\xad\xbc9\xc6Y\xa3%G{\x08ks\xc5\xff\xb3\x97\xf6\xdf\x8b\x0f',
    b'!\x94\x93\xda\x0cC\xc6\xeb\x80\xbc$\x8f\xaf\xeb\x83\xcb`T\xcf\x96R\x97{\xd5Nx\x0c\x04\xe96a\xb0'
)

mallory = User(
    'mallory@test',
    b'sD\xae\x91^\xae\xcc\xe7.\x89\xc8\x91\x9f\xa0t>B\x93\x07\xe7\xb5\xb0\x81\xb1\x07\xf0\xe5\x9b\x91\xd0`:',
    b'\xcd \x7f\xf5\x91\x17=\xda\x856Sz\xe0\xf9\xc6\x82!O7g9\x01`s\xdd\xeeoj\xcb\xe7\x0e\xc5'
)

TEST_USERS = {user.id: user for user in (alice, bob, mallory)}


def almost_wraps(wrapped, to_add=(), to_remove=1):
    # Test function using decorator like `with_core` will have one more param
    # (the `core` object that we are going to provide), hence we have to
    # fake the list of parameters seen by pytest to avoid it thinking this
    # `core` param is an unknown fixture
    args = [*inspect.getargspec(wrapped).args[to_remove:], *to_add]
    signature = eval("""lambda %s: None""" % ', '.join(args))
    return wraps(signature)


# `unittest.mock.patch` doesn't work as decorator on async functions
def async_patch(*patch_args, **patch_kwargs):
    def decorator(func):
        async def wrapper(*args, **kwargs):
            with patch(*patch_args, **patch_kwargs) as patched:
                return await func(patched, *args, **kwargs)
        return wrapper
    return decorator


def _get_unused_port():
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(('', 0))
    port = sock.getsockname()[1]
    sock.close()
    return port


### CORE helpers ###


class CoreAppTesting(CoreApp):
    def test_connect(self, auth_as=None):
        return ConnectToCore(self, auth_as)


class ConnectToCore:
    def __init__(self, core, auth_as):
        self.core = core
        self.auth_as = auth_as

    async def __aenter__(self):
        self.sock = trio.socket.socket()
        if self.auth_as and (not self.core.auth_user or self.core.auth_user.id != self.auth_as):
            assert self.auth_as in TEST_USERS
            user = TEST_USERS[self.auth_as]
            if self.core.auth_user:
                await self.core.logout()
            await self.core.login(user)
        await self.sock.connect(self.core.socket_bind_opts)
        print('client sock', self.sock)
        cookedsock = CookedSocket(self.sock)
        return cookedsock

    async def __aexit__(self, exc_type, exc, tb):
        self.sock.close()


def mocked_local_storage_cls_factory():

    @attr.s
    class InMemoryStorage:
        # Can be changed before initialization (that's why we use a factory btw)
        blocks = attr.ib(default=attr.Factory(dict))
        dirty_blocks = attr.ib(default=attr.Factory(dict))
        dirty_file_manifests = attr.ib(default=attr.Factory(dict))
        placeholder_file_manifests = attr.ib(default=attr.Factory(dict))
        file_manifests = attr.ib(default=attr.Factory(dict))
        local_user_manifest = attr.ib(default=None)

        def get_block(self, id):
            return self.blocks.get(id)

        def get_file_manifest(self, id):
            return self.file_manifests.get(id)

        def get_local_user_manifest(self):
            return self.local_user_manifest

        def save_local_user_manifest(self, data):
            self.local_user_manifest = data

        def get_dirty_block(self, id):
            return self.dirty_blocks.get(id)

        def save_dirty_block(self, id, data):
            self.dirty_blocks[id] = data

        def get_dirty_file_manifest(self, id):
            return self.dirty_file_manifests.get(id)

        def save_dirty_file_manifest(self, id, data):
            self.dirty_file_manifests[id] = data

        def get_placeholder_file_manifest(self, id):
            return self.placeholder_file_manifests.get(id)

        def save_placeholder_file_manifest(self, id, data):
            self.placeholder_file_manifests[id] = data

    # LocalStorage should store on disk, but faster and easier to do that
    # in memory during tests
    mls_cls = Mock(spec=BaseLocalStorage)
    # Can be changed before initialization (that's why we use a factory btw)
    mls_cls.test_storage = InMemoryStorage()
    mls_instance = mls_cls.return_value

    mls_instance.get_block.side_effect = mls_cls.test_storage.get_block
    mls_instance.get_file_manifest.side_effect = mls_cls.test_storage.get_file_manifest
    mls_instance.get_local_user_manifest.side_effect = mls_cls.test_storage.get_local_user_manifest
    mls_instance.save_local_user_manifest.side_effect = mls_cls.test_storage.save_local_user_manifest
    mls_instance.get_dirty_block.side_effect = mls_cls.test_storage.get_dirty_block
    mls_instance.save_dirty_block.side_effect = mls_cls.test_storage.save_dirty_block
    mls_instance.get_dirty_file_manifest.side_effect = mls_cls.test_storage.get_dirty_file_manifest
    mls_instance.save_dirty_file_manifest.side_effect = mls_cls.test_storage.save_dirty_file_manifest
    mls_instance.get_placeholder_file_manifest.side_effect = mls_cls.test_storage.get_placeholder_file_manifest
    mls_instance.save_placeholder_file_manifest.side_effect = mls_cls.test_storage.save_placeholder_file_manifest

    return mls_cls


async def _test_core_factory(config=None, mocked_get_user=True):
    config = config or {}
    config['ADDR'] = 'tcp://127.0.0.1:%s' % _get_unused_port()
    core = CoreAppTesting(config)

    def _get_user(id, password):
        user = TEST_USERS.get(id)
        if user:
            return user.privkey.encode(), user.signkey.encode()

    if mocked_get_user:
        core._get_user = _get_user
    return core


def with_core(config=None, backend_config=None, mocked_get_user=True, mocked_local_storage=True, with_backend=True):
    config = config or {}

    def decorator(testfunc):
        @almost_wraps(testfunc)
        async def wrapper(*args, **kwargs):
            backend = await _test_backend_factory(backend_config)
            config['BACKEND_ADDR'] = 'tcp://127.0.0.1:%s' % backend.port
            core = await _test_core_factory(config, mocked_get_user)

            async def run_test_and_cancel_scope(nursery):
                if mocked_local_storage:
                    mocked_local_storage_cls = mocked_local_storage_cls_factory()
                    core.mocked_local_storage_cls = mocked_local_storage_cls
                    with patch('parsec.core.local_fs.LocalStorage', mocked_local_storage_cls):
                        await testfunc(core, *args, **kwargs)
                else:
                    await testfunc(core, *args, **kwargs)
                nursery.cancel_scope.cancel()

            async with trio.open_nursery() as nursery:
                if with_backend:
                    nursery.start_soon(backend.run)
                    with trio.move_on_after(1) as cancel_scope:
                        await backend.server_ready.wait()
                    assert not cancel_scope.cancelled_caught, 'Backend starting timeout...'
                nursery.start_soon(core.run)
                with trio.move_on_after(1) as cancel_scope:
                    await core.server_ready.wait()
                assert not cancel_scope.cancelled_caught, 'Core starting timeout...'
                nursery.start_soon(run_test_and_cancel_scope, nursery)
        return wrapper
    return decorator


def with_populated_local_storage(user='alice'):
    if isinstance(user, str):
        user = globals()[user]
    assert isinstance(user, User)

    def decorator(testfunc):
        # @wraps(testfunc)
        async def wrapper(core, *args, **kwargs):
            assert isinstance(core, CoreAppTesting), 'missing `@with_core` parent decorator !'
            populate_local_storage_cls(user, core.mocked_local_storage_cls)
            await testfunc(core, *args, **kwargs)
        return wrapper
    return decorator


### BACKEND helpers ###


class BackendAppTesting(BackendApp):
    def test_connect(self, auth_as=None):
        return ConnectToBackend(self, auth_as)


class ConnectToBackend:
    def __init__(self, backend, auth_as):
        self.backend = backend
        self.auth_as = auth_as

    async def __aenter__(self):
        self.sock = trio.socket.socket()
        await self.sock.connect((self.backend.host, self.backend.port))
        cookedsock = CookedSocket(self.sock)
        if self.auth_as:
            assert self.auth_as in TEST_USERS
            user = TEST_USERS[self.auth_as]
            # Handshake
            hds1 = await cookedsock.recv()
            assert hds1['handshake'] == 'challenge', hds1
            answer = user.signkey.sign(from_jsonb64(hds1['challenge']))
            hds2 = {
                'handshake': 'answer',
                'identity': self.auth_as,
                'answer': to_jsonb64(answer)
            }
            await cookedsock.send(hds2)
            hds3 = await cookedsock.recv()
            assert hds3 == {'status': 'ok', 'handshake': 'done'}, hds3
        return cookedsock

    async def __aexit__(self, exc_type, exc, tb):
        self.sock.close()


async def _test_backend_factory(config=None):
    config = config or {}
    config['HOST'] = '127.0.0.1'
    config.setdefault('PORT', _get_unused_port())
    backend = BackendAppTesting(config)
    for userid, user in TEST_USERS.items():
        await backend.pubkey.add(userid, user.pubkey.encode(), user.verifykey.encode())
    return backend


def with_backend(config=None):

    def decorator(testfunc):
        @almost_wraps(testfunc, ['backend_store'])
        async def wrapper(backend_store, *args, **kwargs):
            backend = await _test_backend_factory(config)

            async def run_test_and_cancel_scope(nursery):
                await testfunc(backend, *args, **kwargs)
                nursery.cancel_scope.cancel()

            async with trio.open_nursery() as nursery:
                nursery.start_soon(backend.run)
                with trio.move_on_after(1) as cancel_scope:
                    await backend.server_ready.wait()
                assert not cancel_scope.cancelled_caught, 'Backend starting timeout...'
                nursery.start_soon(run_test_and_cancel_scope, nursery)
        return wrapper
    return decorator
