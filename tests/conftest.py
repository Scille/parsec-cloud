import os
from tempfile import mkdtemp
from shutil import rmtree
import pytest
import socket
import contextlib
from freezegun import freeze_time
from unittest.mock import patch

from parsec.backend.app import BackendApp
from parsec.backend.config import Config as BackendConfig
from parsec.core.app import CoreApp
from parsec.core.config import Config as CoreConfig

from tests.common import (
    TEST_USERS, connect_backend, connect_core, run_app)
from tests.populate import populate_factory
from tests.open_tcp_stream_mock_wrapper import OpenTCPStreamMockWrapper




def pytest_addoption(parser):
    parser.addoption("--no-postgresql", action="store_true",
                     help="Don't run tests making use of PostgreSQL")


DEFAULT_POSTGRESQL_TEST_URL = '/parsec_test'


def postgresql_url():
    return os.environ.get('PARSEC_POSTGRESQL_TEST_URL', DEFAULT_POSTGRESQL_TEST_URL)


@pytest.fixture(params=['mocked', 'postgresql'])
def backend_store(request):
    if request.param == 'postgresql':
        if pytest.config.getoption('--no-postgresql'):
            pytest.skip('`--no-postgresql` option provided')
        pytest.importorskip('parsec.backend.postgresql')
        return postgresql_url()
    else:
        return 'mocked://'


@pytest.fixture(scope='session')
def alice():
    return TEST_USERS['alice@test']


@pytest.fixture(scope='session')
def bob():
    return TEST_USERS['bob@test']


@pytest.fixture(scope='session')
def mallory():
    return TEST_USERS['mallory@test']


@pytest.fixture
def alice_data(alice):
    return populate_factory(alice)


@pytest.fixture
def bob_data(bob):
    return populate_factory(bob)


@pytest.fixture
def mallory_data(mallory):
    return populate_factory(mallory)


@pytest.fixture
def unused_tcp_port():
    """Find an unused localhost TCP port from 1024-65535 and return it."""
    with contextlib.closing(socket.socket()) as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


@pytest.fixture
def default_users(alice, bob):
    return (alice, bob)


@pytest.fixture
async def backend(default_users, config={}):
    config = BackendConfig(**{
        'blockstore_url': 'backend://',
        **config
    })
    backend = BackendApp(config)
    with freeze_time('2000-01-01'):
        for user in default_users:
            userid, deviceid = user.id.split('@')
            await backend.user.create(
                author='<backend-fixture>',
                id=userid,
                broadcast_key=user.pubkey.encode(),
                devices=[(deviceid, user.verifykey.encode())]
            )
    return backend


@pytest.fixture
async def backend_addr(tcp_stream_spy):
    return 'tcp://placeholder.com:9999'


@pytest.fixture
async def tcp_stream_spy():
    open_tcp_stream_mock_wrapper = OpenTCPStreamMockWrapper()
    with patch('trio.open_tcp_stream', new=open_tcp_stream_mock_wrapper):
        yield open_tcp_stream_mock_wrapper


@pytest.fixture
async def running_backend(tcp_stream_spy, backend, backend_addr):
    async with run_app(backend) as backend_connection_factory:

        async def _open_tcp_stream(*args):
            return await backend_connection_factory()

        tcp_stream_spy.install_hook(backend_addr, _open_tcp_stream)
        yield (backend, backend_addr, backend_connection_factory)
        tcp_stream_spy.install_hook(backend_addr, None)


@pytest.fixture
async def alice_backend_sock(backend, alice):
    async with connect_backend(backend, auth_as=alice.id) as sock:
        yield sock


@pytest.fixture
async def core(backend_addr, default_users, config={}):
    base_settings_path = mkdtemp()
    config = CoreConfig(**{
        'backend_addr': backend_addr,
        'base_settings_path': base_settings_path,
        **config
    })
    core = CoreApp(config)

    if default_users:
        core.mocked_users_list = list(default_users)

        def _get_user(id, password):
            for user in core.mocked_users_list:
                if user.id == id:
                    return user.privkey.encode(), user.signkey.encode()

        core._get_user = _get_user

    yield core

    rmtree(base_settings_path)


@pytest.fixture
async def alice_core_sock(core, alice):
    await core.login(alice)
    async with connect_core(core) as sock:
        yield sock
