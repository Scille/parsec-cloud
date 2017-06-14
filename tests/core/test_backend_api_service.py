import pytest
import asyncio
import blinker
from unittest.mock import Mock

from parsec.server import WebSocketServer
from parsec.backend import MockedVlobService
from parsec.backend.pubkey_service import BasePubKeyService
from parsec.core2.backend_api_service import (
    _patch_service_event_namespace, BackendAPIService, MockedBackendAPIService)
from parsec.core2.identity_service import BaseIdentityService
from parsec.server import BaseServer
from parsec.exceptions import IdentityNotLoadedError


class MockedPubKeyService(BasePubKeyService):
    async def get_pubkey(self, id, raw=False):
        pubkey = Mock()
        pubkey.verify.return_value = True
        return pubkey


class MockedIdentityService(BaseIdentityService):

    def __init__(self):
        super().__init__()
        self._loaded = False
        self._id = 'mocked_user'
        self._private_key = Mock()
        self._private_key.sign.return_value = b'mocked_signature'

    @property
    def id(self):
        if not self._loaded:
            raise IdentityNotLoadedError('Identity not loaded')
        return self._id

    @property
    def private_key(self):
        if not self._loaded:
            raise IdentityNotLoadedError('Identity not loaded')
        return self._private_key

    async def load(self, *args):
        self._loaded = True
        self.on_identity_loaded.send(self.id)

    async def unload(self):
        self._loaded = False
        self.on_identity_unloaded.send(self.id)


async def bootstrap_BackendAPIService(request, event_loop, unused_tcp_port):
    event_loop.set_debug(True)
    # Start a minimal backend server...
    vlob_service = MockedVlobService()
    pubkey_service = MockedPubKeyService()
    server = WebSocketServer(handshake=pubkey_service.handshake)
    # Patch our server not to share it signals with the core (given they should
    # not share the same interpreter)
    backend_signal_namespace = blinker.Namespace()
    _patch_service_event_namespace(vlob_service, backend_signal_namespace)
    _patch_service_event_namespace(pubkey_service, backend_signal_namespace)
    server.register_service(vlob_service)
    server.register_service(pubkey_service)
    server_task = await server.start('localhost', unused_tcp_port, loop=event_loop, block=False)
    # ...then create a BackendAPIService which will connect to
    backend_api_svc = BackendAPIService('ws://localhost:%s' % unused_tcp_port)
    backend_api_svc.identity = MockedIdentityService()

    # Create base core server
    server = BaseServer()
    server.register_service(backend_api_svc)
    server.register_service(MockedIdentityService())
    await server.bootstrap_services()


    def finalize():
        event_loop.run_until_complete(backend_api_svc.teardown())
        server_task.close()
        event_loop.run_until_complete(server_task.wait_closed())

    request.addfinalizer(finalize)
    return backend_api_svc


async def bootstrap_MockedBackendAPIService(request, event_loop, unused_tcp_port):
    svc = MockedBackendAPIService()
    svc.identity = MockedIdentityService()
    return svc


@pytest.fixture(params=[bootstrap_MockedBackendAPIService, bootstrap_BackendAPIService],
                ids=['mocked', 'backend'])
async def backend_api_svc(request, event_loop, unused_tcp_port):
    return await request.param(request, event_loop, unused_tcp_port)


class TestBackendAPIService:
    @pytest.mark.asyncio
    async def test_before_identity_load(self, backend_api_svc):
        with pytest.raises(IdentityNotLoadedError):
            await backend_api_svc.vlob_create('Impossible yet !')
        await backend_api_svc.identity.load()
        await backend_api_svc.wait_for_connection_ready()
        await backend_api_svc.vlob_create("Now it's ok")

    @pytest.mark.asyncio
    async def test_cmd(self, backend_api_svc):
        await backend_api_svc.identity.load()
        await backend_api_svc.wait_for_connection_ready()
        vlob = await backend_api_svc.vlob_create('foo')
        assert isinstance(vlob, dict)

    @pytest.mark.asyncio
    async def test_no_backend_leak_event(self, backend_api_svc):
        await backend_api_svc.identity.load()
        await backend_api_svc.wait_for_connection_ready()
        # Backend and core both run in the same interpreter and on the same
        # event loop for the tests, however in reality they are not supposed
        # to share the same events module.

        vlob = await backend_api_svc.vlob_create('First version')

        def _on_vlob_updated(sender):
            assert False, 'Backend callback should not have been called'

        blinker.signal('on_vlob_updated').connect(_on_vlob_updated)
        await backend_api_svc.vlob_update(vlob['id'], 2, vlob['write_trust_seed'], 'Next version')

    @pytest.mark.asyncio
    async def test_event(self, backend_api_svc):
        await backend_api_svc.identity.load()
        await backend_api_svc.wait_for_connection_ready()
        vlob = await backend_api_svc.vlob_create('First version')

        is_callback_called = asyncio.Future()

        def _on_vlob_updated(sender):
            nonlocal is_callback_called
            is_callback_called.set_result(sender)

        await backend_api_svc.connect_event('on_vlob_updated', vlob['id'], _on_vlob_updated)
        await backend_api_svc.vlob_update(vlob['id'], 2, vlob['write_trust_seed'], 'Next version')
        ret = await is_callback_called
        assert ret == vlob['id']
