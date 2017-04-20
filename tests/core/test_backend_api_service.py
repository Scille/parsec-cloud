import pytest

from parsec.server import WebSocketServer
from parsec.backend.vlob_service import Vlob
from parsec.backend import MockedVlobService
from parsec.core import MockedBackendAPIService, BackendAPIService


async def bootstrap_BackendAPIService(request, event_loop, unused_tcp_port):
    event_loop.set_debug(True)
    # Start a minimal backend server...
    server = WebSocketServer()
    server.register_service(MockedVlobService())
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


@pytest.fixture(params=[bootstrap_MockedBackendAPIService, bootstrap_BackendAPIService])
async def backend_api_svc(request, event_loop, unused_tcp_port):
    return await request.param(request, event_loop, unused_tcp_port)


class TestBackendAPIService:

    @pytest.mark.asyncio
    async def test_cmd(self, backend_api_svc):
        vlob = await backend_api_svc.vlob_create('foo')
        assert isinstance(vlob, Vlob)

    @pytest.mark.asyncio
    async def test_event(self, backend_api_svc):
        vlob = await backend_api_svc.vlob_create('First version')

        is_callback_called = False

        def _on_vlob_updated(sender):
            nonlocal is_callback_called
            is_callback_called = sender

        backend_api_svc.on_vlob_updated.connect(_on_vlob_updated)
        await backend_api_svc.vlob_update(vlob.id, 2, vlob.write_trust_seed, 'Next version')
        assert is_callback_called == vlob.id
