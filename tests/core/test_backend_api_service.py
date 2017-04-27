import pytest
import asyncio
import blinker

from parsec.server import WebSocketServer
from parsec.backend import MockedVlobService
from parsec.core.backend_api_service import _patch_service_event_namespace
from parsec.core import MockedBackendAPIService, BackendAPIService


async def bootstrap_BackendAPIService(request, event_loop, unused_tcp_port):
    event_loop.set_debug(True)
    # Start a minimal backend server...
    server = WebSocketServer()
    vlob_service = MockedVlobService()
    # Patch our server not to share it signals with the core (given they should
    # not share the same interpreter)
    backend_signal_namespace = blinker.Namespace()
    _patch_service_event_namespace(vlob_service, backend_signal_namespace)
    server.register_service(vlob_service)
    server_task = await server.start('localhost', unused_tcp_port, loop=event_loop, block=False)
    # ...then create a BackendAPIService which will connect to
    backend_api_svc = BackendAPIService('ws://localhost:%s' % unused_tcp_port)
    await backend_api_svc.bootstrap()

    def finalize():
        event_loop.run_until_complete(backend_api_svc.teardown())
        server_task.close()
        event_loop.run_until_complete(server_task.wait_closed())

    request.addfinalizer(finalize)
    return backend_api_svc


async def bootstrap_MockedBackendAPIService(request, event_loop, unused_tcp_port):
    return MockedBackendAPIService()


@pytest.fixture(params=[bootstrap_MockedBackendAPIService, bootstrap_BackendAPIService], ids=['mocked', 'backend'])
async def backend_api_svc(request, event_loop, unused_tcp_port):
    return await request.param(request, event_loop, unused_tcp_port)


class TestBackendAPIService:

    @pytest.mark.asyncio
    async def test_cmd(self, backend_api_svc):
        vlob = await backend_api_svc.vlob_create('foo')
        assert isinstance(vlob, dict)

    @pytest.mark.asyncio
    async def test_no_backend_leak_event(self, backend_api_svc):
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
        vlob = await backend_api_svc.vlob_create('First version')

        is_callback_called = asyncio.Future()

        def _on_vlob_updated(sender):
            nonlocal is_callback_called
            is_callback_called.set_result(sender)

        await backend_api_svc.connect_event('on_vlob_updated', vlob['id'], _on_vlob_updated)
        await backend_api_svc.vlob_update(vlob['id'], 2, vlob['write_trust_seed'], 'Next version')
        ret = await is_callback_called
        assert ret == vlob['id']
