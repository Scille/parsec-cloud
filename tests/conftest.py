import os
import pytest
import socket
import contextlib
from unittest.mock import patch

from parsec.backend.app import BackendApp
from parsec.backend.config import Config as BackendConfig
from parsec.core.devices_manager import Device

from tests.common import freeze_time, core_factory, connect_backend, connect_core, run_app
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
        pytest.importorskip('parsec.backend.drivers.postgresql')
        return postgresql_url()
    else:
        return 'mocked://'


@pytest.fixture
def alice(tmpdir):
    return Device(
        'alice@test',
        (b'\xceZ\x9f\xe4\x9a\x19w\xbc\x12\xc8\x98\xd1CB\x02vS\xa4\xfe\xc8\xc5'
         b'\xa6\xcd\x87\x90\xd7\xabJ\x1f$\x87\xc4'),
        (b'\xa7\n\xb2\x94\xbb\xe6\x03\xd3\xd0\xd3\xce\x95\xe6\x8b\xfe5`('
         b'\x15\xc0UL\xe9\x1dTf^ m\xb7\xbc\\'),
        tmpdir.join('alice@test.sqlite').strpath,
    )


@pytest.fixture
def bob(tmpdir):
    return Device(
        'bob@test',
        (b'\xc3\xc9(\xf7\\\xd2\xb4[\x85\xe5\xfa\xd3\xad\xbc9\xc6Y\xa3%G{\x08ks'
         b'\xc5\xff\xb3\x97\xf6\xdf\x8b\x0f'),
        (b'!\x94\x93\xda\x0cC\xc6\xeb\x80\xbc$\x8f\xaf\xeb\x83\xcb`T\xcf'
         b'\x96R\x97{\xd5Nx\x0c\x04\xe96a\xb0'),
        tmpdir.join('bob@test.sqlite').strpath,
    )


@pytest.fixture
def mallory(tmpdir):
    return Device(
        'mallory@test',
        (b'sD\xae\x91^\xae\xcc\xe7.\x89\xc8\x91\x9f\xa0t>B\x93\x07\xe7\xb5'
         b'\xb0\x81\xb1\x07\xf0\xe5\x9b\x91\xd0`:'),
        (b'\xcd \x7f\xf5\x91\x17=\xda\x856Sz\xe0\xf9\xc6\x82!O7g9\x01`s\xdd'
         b'\xeeoj\xcb\xe7\x0e\xc5'),
        tmpdir.join('mallory@test.sqlite').strpath,
    )


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
def default_devices(alice, bob):
    return (alice, bob)


@pytest.fixture
async def backend(default_devices, config={}):
    config = BackendConfig(**{
        'blockstore_url': 'backend://',
        **config
    })
    backend = BackendApp(config)
    with freeze_time('2000-01-01'):
        for device in default_devices:
            await backend.user.create(
                author='<backend-fixture>',
                user_id=device.user_id,
                broadcast_key=device.user_pubkey.encode(),
                devices=[(device.device_name, device.device_verifykey.encode())]
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
    async with connect_backend(backend, auth_as=alice) as sock:
        yield sock


@pytest.fixture
async def core(backend_addr, tmpdir, default_devices, config={}):
    core = core_factory(**{
        'base_settings_path': tmpdir.mkdir('core_fixture').strpath,
        'backend_addr': backend_addr,
        **config
    })

    for device in default_devices:
        core.devices_manager.register_new_device(
            device.id,
            device.user_privkey.encode(),
            device.device_signkey.encode(),
            '<secret>'
        )

    return core


@pytest.fixture
async def alice_core_sock(core, alice):
    await core.login(alice)
    async with connect_core(core) as sock:
        yield sock
