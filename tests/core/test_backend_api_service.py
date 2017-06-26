import pytest
import asyncio
import blinker
from io import BytesIO

from parsec.server import WebSocketServer
from parsec.backend import MockedVlobService, InMemoryPubKeyService
from parsec.core.backend_api_service import _patch_service_event_namespace
from parsec.core import BackendAPIService, IdentityService, MockedBackendAPIService
from parsec.server import BaseServer


JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN PRIVATE KEY-----
MIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEAsSle/x4jtr+kaxiv
9BYlL+gffH/VLC+Q/WTTB+1FIU1fdmgdZVGaIlAWJHqr9qEZBfwXYzlQv8pIMn+W
5pqvHQICAQECQDv5FjKAvWW1a3Twc1ia67eQUmDugu8VFTDsV2BRUS0jlxJ0yCL+
TEBpOwH95TFgvfRBYee97APHjhvLLlzmEyECIQDZdjMg/j9N7sw602DER0ciERPI
Ps9rU8RqRXaWPYtWOQIhANCO1h/z7iFjlpENbKDOCinfsXd9ulVsoNYWhKm58gAF
AiEAzMT3XdKFUlljq+/hl/Nt0GPA8lkHDGjG5ZAaAAYnj/ECIQCB125lkuHy61LH
4INhH6azeFaUGnn7aHwJxE6beL6BgQIhALbajJWsBf5LmeO190adM2jAVN94YqVD
aOrHGFFqrjJ3
-----END PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MFswDQYJKoZIhvcNAQEBBQADSgAwRwJBALEpXv8eI7a/pGsYr/QWJS/oH3x/1Swv
kP1k0wftRSFNX3ZoHWVRmiJQFiR6q/ahGQX8F2M5UL/KSDJ/luaarx0CAgEB
-----END PUBLIC KEY-----
"""


async def bootstrap_BackendAPIService(request, event_loop, unused_tcp_port):
    event_loop.set_debug(True)
    # Start a minimal backend server...
    backend_server = WebSocketServer()
    pubkey_svc = InMemoryPubKeyService()
    await pubkey_svc.add_pubkey(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY)
    vlob_svc = MockedVlobService()
    # Patch our server not to share it signals with the core (given they should
    # not share the same interpreter)
    backend_signal_namespace = blinker.Namespace()
    _patch_service_event_namespace(vlob_svc, backend_signal_namespace)
    backend_server.register_service(pubkey_svc)
    backend_server.register_service(vlob_svc)
    server_task = await backend_server.start('localhost', unused_tcp_port, loop=event_loop, block=False)
    # ...then create a BackendAPIService in a core server which will connect to
    backend_api_svc = BackendAPIService('ws://localhost:%s' % unused_tcp_port)
    identity_svc = IdentityService()
    core_server = BaseServer()
    core_server.register_service(backend_api_svc)
    core_server.register_service(identity_svc)
    identity = JOHN_DOE_IDENTITY
    identity_key = BytesIO(JOHN_DOE_PRIVATE_KEY)
    await core_server.bootstrap_services()
    await identity_svc.load(identity, identity_key.read())

    def finalize():
        event_loop.run_until_complete(backend_api_svc.teardown())
        server_task.close()
        event_loop.run_until_complete(server_task.wait_closed())

    request.addfinalizer(finalize)
    return backend_api_svc


async def bootstrap_MockedBackendAPIService(request, event_loop, unused_tcp_port):
    return MockedBackendAPIService()


@pytest.fixture(params=[bootstrap_MockedBackendAPIService, bootstrap_BackendAPIService],
                ids=['mocked', 'backend'])
async def backend_api_svc(request, event_loop, unused_tcp_port):
    return await request.param(request, event_loop, unused_tcp_port)


class TestBackendAPIService:

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_cmd(self, backend_api_svc):
        vlob = await backend_api_svc.vlob_create('foo')
        assert isinstance(vlob, dict)

    @pytest.mark.xfail
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

    @pytest.mark.xfail
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
