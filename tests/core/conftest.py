import pytest
import trio
from async_generator import asynccontextmanager

from parsec.core.backend_connection import backend_cmds_pool_factory
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.fs import FS

from tests.common import InMemoryLocalDB


@pytest.fixture
def encryption_manager_factory(backend_addr):
    @asynccontextmanager
    async def _encryption_manager_factory(device, backend_addr=backend_addr):
        async with backend_cmds_pool_factory(
            backend_addr, device.device_id, device.signing_key
        ) as cmds:
            em = EncryptionManager(device, InMemoryLocalDB(), cmds)
            async with trio.open_nursery() as nursery:
                await em.init(nursery)
                try:
                    yield em
                finally:
                    await em.teardown()

    return _encryption_manager_factory


@pytest.fixture
async def encryption_manager(running_backend, encryption_manager_factory, alice):
    async with encryption_manager_factory(alice) as em:
        yield em


@pytest.fixture
def fs_factory(encryption_manager_factory, event_bus_factory, backend_addr):
    @asynccontextmanager
    async def _fs_factory(device, backend_addr=backend_addr, event_bus=None):
        if not event_bus:
            event_bus = event_bus_factory()

        async with encryption_manager_factory(device, backend_addr=backend_addr) as em:
            fs = FS(device, em.backend_cmds, em, event_bus)
            yield fs

    return _fs_factory


@pytest.fixture
async def alice_fs(fs_factory, alice):
    async with fs_factory(alice) as fs:
        yield fs


@pytest.fixture
async def alice2_fs(fs_factory, alice2):
    async with fs_factory(alice2) as fs:
        yield fs


@pytest.fixture
async def bob_fs(fs_factory, bob):
    async with fs_factory(bob) as fs:
        yield fs


@pytest.fixture
def backend_addr_factory(running_backend, tcp_stream_spy):
    # Creating new addr for backend make it easy be selective on what to
    # turn offline
    counter = 0

    def _backend_addr_factory():
        nonlocal counter
        addr = f"tcp://backend-addr-{counter}.localhost:9999"
        tcp_stream_spy.push_hook(addr, running_backend.connection_factory)
        counter += 1
        return addr

    return _backend_addr_factory


@pytest.fixture
async def alice_backend_cmds(running_backend, alice):
    async with backend_cmds_pool_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def alice2_backend_cmds(running_backend, alice2):
    async with backend_cmds_pool_factory(
        running_backend.addr, alice2.device_id, alice2.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def bob_backend_cmds(running_backend, bob):
    async with backend_cmds_pool_factory(
        running_backend.addr, bob.device_id, bob.signing_key
    ) as cmds:
        yield cmds
