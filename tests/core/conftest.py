import pytest

from parsec.signals import Namespace as SignalNamespace
from parsec.core.backend_cmds_sender import BackendCmdsSender
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.fs import FSManager


@pytest.fixture
def signal_ns_factory():
    return SignalNamespace


@pytest.fixture
def encryption_manager_factory(nursery, alice, backend_cmds_sender_factory):
    async def _encryption_manager_factory(device, backend_addr=None):
        bcs = await backend_cmds_sender_factory(alice, backend_addr=backend_addr)
        em = EncryptionManager(alice, bcs)
        await em.init(nursery)
        return em

    return _encryption_manager_factory


@pytest.fixture
async def encryption_manager(encryption_manager_factory, alice):
    return await encryption_manager_factory(alice)


@pytest.fixture
def backend_cmds_sender_factory(nursery, running_backend):
    async def _backend_cmds_sender_factory(device, backend_addr=None):
        if not backend_addr:
            backend_addr = running_backend.addr
        bcs = BackendCmdsSender(device, backend_addr)
        await bcs.init(nursery)
        return bcs

    return _backend_cmds_sender_factory


@pytest.fixture
def fs_factory(nursery, backend_cmds_sender_factory, encryption_manager_factory, signal_ns_factory):
    async def _fs_factory(device, backend_addr=None, signal_ns=None):
        if not signal_ns:
            signal_ns = signal_ns_factory()
        encryption_manager = await encryption_manager_factory(device, backend_addr=backend_addr)
        backend_cmds_sender = await backend_cmds_sender_factory(device, backend_addr=backend_addr)
        fs = FSManager(device, backend_cmds_sender, encryption_manager, signal_ns)
        await fs.init(nursery)
        return fs

    return _fs_factory


@pytest.fixture
async def backend_cmds_sender(alice):
    return backend_cmds_sender_factory(alice)


@pytest.fixture
async def fs(fs_factory, alice):
    return fs_factory(alice)


@pytest.fixture
def backend_addr_factory(running_backend, tcp_stream_spy):
    # Creating new addr for backend make it easy be selective on what to
    # turn offline
    counter = 0

    def _backend_addr_factory():
        nonlocal counter
        addr = f"tcp://{counter}.placeholder.com:9999"
        tcp_stream_spy.push_hook(addr, running_backend.connection_factory)
        counter += 1
        return addr

    return _backend_addr_factory
