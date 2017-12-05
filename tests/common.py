import trio
from trio.testing import open_stream_to_socket_listener
import socket
import inspect
import attr
import contextlib
from collections import defaultdict
from unittest.mock import Mock, patch
from functools import wraps
from nacl.public import PrivateKey
from nacl.signing import SigningKey

from parsec.handshake import ClientHandshake
from parsec.core.app import CoreApp
from parsec.utils import CookedSocket, to_jsonb64, from_jsonb64, User
from parsec.core.local_storage import BaseLocalStorage
from parsec.core.backend_storage import BaseBackendStorage
from parsec.backend.app import BackendApp

from tests.populate import populate_local_storage_cls, populate_backend


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
    signature.__name__ = wrapped.__name__
    signature.__qualname__ = wrapped.__qualname__
    wrapper = wraps(signature)
    return wrapper


# `unittest.mock.patch` doesn't work as decorator on async functions
def async_patch(*patch_args, **patch_kwargs):
    def decorator(func):
        async def wrapper(*args, **kwargs):
            with patch(*patch_args, **patch_kwargs) as patched:
                return await func(patched, *args, **kwargs)
        return wrapper
    return decorator


def _get_unused_port():
    with contextlib.closing(socket.socket()) as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


### CORE helpers ###


class CoreAppTesting(CoreApp):
    def test_connect(self, auth_as=None):
        return ConnectToCore(self, auth_as)

    @property
    def port(self):
        return self.listeners[0].socket.getsockname()[1]


class ConnectToCore:
    def __init__(self, core, auth_as):
        self.core = core
        self.auth_as = auth_as

    async def __aenter__(self):
        if self.auth_as and (not self.core.auth_user or self.core.auth_user.id != self.auth_as):
            assert self.auth_as in TEST_USERS
            user = TEST_USERS[self.auth_as]
            if self.core.auth_user:
                await self.core.logout()
            await self.core.login(user)
        sockstream = await open_stream_to_socket_listener(self.core.listeners[0])
        self.sock = CookedSocket(sockstream)
        return self.sock

    async def __aexit__(self, exc_type, exc, tb):
        await self.sock.aclose()


def mocked_local_storage_cls_factory():

    @attr.s
    class InMemoryLocalStorage:
        # Can be changed before initialization (that's why we use a factory btw)
        local_manifests = attr.ib(default=attr.Factory(dict))
        local_user_manifest = attr.ib(default=None)
        blocks = attr.ib(default=attr.Factory(dict))
        dirty_blocks = attr.ib(default=attr.Factory(dict))

        def __init__(self, user):
            self.user = user

        def fetch_user_manifest(self):
            return self.local_user_manifest

        def flush_user_manifest(self, blob):
            self.local_user_manifest = blob

        def fetch_manifest(self, id):
            return self.local_manifests.get(id)

        def flush_manifest(self, id, blob):
            self.local_manifests[id] = blob

        def move_manifest(self, id, new_id):
            self.local_manifests[new_id] = self.local_manifests[id]
            del self.local_manifests[id]

        def fetch_block(self, id):
            return self.blocks.get(id)

        def flush_block(self, id, blob):
            self.blocks[id] = blob

        def fetch_dirty_block(self, id):
            return self.dirty_blocks.get(id)

        def flush_dirty_block(self, id, blob):
            self.blocks[id] = blob

    # LocalStorage should store on disk, but faster and easier to do that
    # in memory during tests
    mls_cls = Mock(spec=BaseLocalStorage)
    # Can be changed before initialization (that's why we use a factory btw)
    mls_cls.test_storage = InMemoryLocalStorage()
    mls_instance = mls_cls.return_value

    mls_instance.fetch_user_manifest.side_effect = mls_cls.test_storage.fetch_user_manifest
    mls_instance.flush_user_manifest.side_effect = mls_cls.test_storage.flush_user_manifest
    mls_instance.fetch_manifest.side_effect = mls_cls.test_storage.fetch_manifest
    mls_instance.flush_manifest.side_effect = mls_cls.test_storage.flush_manifest
    mls_instance.move_manifest.side_effect = mls_cls.test_storage.move_manifest
    mls_instance.fetch_block.side_effect = mls_cls.test_storage.fetch_block
    mls_instance.flush_block.side_effect = mls_cls.test_storage.flush_block
    mls_instance.fetch_dirty_block.side_effect = mls_cls.test_storage.fetch_dirty_block
    mls_instance.flush_dirty_block.side_effect = mls_cls.test_storage.flush_dirty_block
    return mls_cls


async def test_core_factory(config=None, mocked_get_user=True):
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
            backend_port = _get_unused_port()
            config['BACKEND_ADDR'] = 'tcp://127.0.0.1:%s' % backend_port
            core = await test_core_factory(config, mocked_get_user)
            core.test_backend = backend

            async def run_test_and_cancel_scope(nursery):
                if mocked_local_storage:
                    mocked_local_storage_cls = mocked_local_storage_cls_factory()
                    core.mocked_local_storage_cls = mocked_local_storage_cls
                    with patch('parsec.core.fs.LocalStorage', mocked_local_storage_cls):
                        await testfunc(core, *args, **kwargs)
                else:
                    await testfunc(core, *args, **kwargs)
                nursery.cancel_scope.cancel()

            async with trio.open_nursery() as nursery:
                if with_backend:
                    backend.listeners = await nursery.start(
                        trio.serve_tcp, backend.handle_client, backend_port)
                await core.init(nursery)
                core.listeners = await nursery.start(
                    trio.serve_tcp, core.handle_client, 0)
                nursery.start_soon(run_test_and_cancel_scope, nursery)
        return wrapper
    return decorator


def with_populated_local_storage(user='alice'):
    if isinstance(user, str):
        user = globals()[user]
    assert isinstance(user, User)

    def decorator(testfunc):
        @wraps(testfunc)
        async def wrapper(core, *args, **kwargs):
            assert isinstance(core, CoreApp), 'missing `@with_core` parent decorator !'
            await populate_backend(user, core.test_backend)
            populate_local_storage_cls(user, core.mocked_local_storage_cls)
            await testfunc(core, *args, **kwargs)
        return wrapper
    return decorator


### BACKEND helpers ###


class BackendAppTesting(BackendApp):
    def test_connect(self, auth_as=None):
        return ConnectToBackend(self, auth_as)

    @property
    def port(self):
        return self.listeners[0].socket.getsockname()[1]


class ConnectToBackend:
    def __init__(self, backend, auth_as):
        self.backend = backend
        self.auth_as = auth_as
        self.sock = None

    async def __aenter__(self):
        sockstream = await open_stream_to_socket_listener(self.backend.listeners[0])
        self.sock = CookedSocket(sockstream)
        if self.auth_as:
            assert self.auth_as in TEST_USERS
            user = TEST_USERS[self.auth_as]
            # Handshake
            ch = ClientHandshake(user)
            challenge_req = await self.sock.recv()
            answer_req = ch.process_challenge_req(challenge_req)
            await self.sock.send(answer_req)
            result_req = await self.sock.recv()
            ch.process_result_req(result_req)
        return self.sock

    async def __aexit__(self, exc_type, exc, tb):
        await self.sock.aclose()


async def _test_backend_factory(config=None):
    config = config or {}
    config['BLOCKSTORE_URL'] = 'backend://'
    backend = BackendAppTesting(config)
    for userid, user in TEST_USERS.items():
        await backend.pubkey.add(userid, user.pubkey.encode(), user.verifykey.encode())
    return backend


def with_backend(config=None, populated_for=None):

    def decorator(testfunc):
        @almost_wraps(testfunc, ['backend_store'])
        async def wrapper(backend_store, *args, **kwargs):
            backend = await _test_backend_factory(config)
            if populated_for:
                await populate_backend(globals()[populated_for], backend)
            async def run_test_and_cancel_scope(nursery):
                await testfunc(backend, *args, **kwargs)
                nursery.cancel_scope.cancel()

            async with trio.open_nursery() as nursery:
                backend.listeners = await nursery.start(trio.serve_tcp, backend.handle_client, 0)
                nursery.start_soon(run_test_and_cancel_scope, nursery)
        return wrapper
    return decorator
