import pytest
import asyncio
import blinker
import random
from unittest.mock import Mock
from string import ascii_lowercase

from parsec.server import WebSocketServer
from parsec.backend import MockedVlobService
from parsec.core2.fs_api import MockedFSAPIMixin
from parsec.server import BaseServer


# async def bootstrap_FSAPIMixin(request, event_loop, unused_tcp_port):
#     event_loop.set_debug(True)
#     # Start a minimal backend server...
#     server = WebSocketServer()
#     vlob_service = MockedVlobService()
#     # Patch our server not to share it signals with the core (given they should
#     # not share the same interpreter)
#     backend_signal_namespace = blinker.Namespace()
#     _patch_service_event_namespace(vlob_service, backend_signal_namespace)
#     server.register_service(vlob_service)
#     server_task = await server.start('localhost', unused_tcp_port, loop=event_loop, block=False)
#     # ...then create a FSAPIMixin which will connect to
#     backend_api_svc = FSAPIMixin('ws://localhost:%s' % unused_tcp_port)
#     backend_api_svc.identity = MockedIdentityService()
#     await backend_api_svc.bootstrap()
#     # Create base core server
#     # server = BaseServer()
#     # server.register_service(backend_api_svc)
#     # server.register_service(IdentityService())
#     # await server.bootstrap_services()
    

#     def finalize():
#         event_loop.run_until_complete(backend_api_svc.teardown())
#         server_task.close()
#         event_loop.run_until_complete(server_task.wait_closed())

#     request.addfinalizer(finalize)
#     return backend_api_svc


async def bootstrap_MockedFSAPIMixin(request, event_loop, unused_tcp_port):
    return MockedFSAPIMixin()


# @pytest.fixture(params=[bootstrap_MockedFSAPIMixin, bootstrap_FSAPIMixin],
#                 ids=['mocked', 'backend'])
@pytest.fixture(params=[bootstrap_MockedFSAPIMixin],
                ids=['mocked'])
async def fs_api_svc(request, event_loop, unused_tcp_port):
    return await request.param(request, event_loop, unused_tcp_port)


class TestBackendAPIService:

    @pytest.mark.asyncio
    async def test_file(self, fs_api_svc):
        await fs_api_svc.file_create('/foo')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == ['foo']

        ret = await fs_api_svc.stat('/foo')
        assert ret['updated'] == ret['created']
        assert ret['type'] == 'file'
        assert ret['size'] == 0

        await fs_api_svc.file_write('/foo', b'hello world !')
        ret = await fs_api_svc.stat('/foo')
        assert ret['updated'] > ret['created']
        assert ret['type'] == 'file'
        assert ret['size'] == 13

        ret = await fs_api_svc.file_read('/foo')
        assert ret == b'hello world !'

        await fs_api_svc.file_write('/foo', b'!', offset=13)
        await fs_api_svc.file_write('/foo', b'again', offset=6)
        ret = await fs_api_svc.file_read('/foo')
        assert ret == b'hello again !!'

        await fs_api_svc.file_truncate('/foo', length=5)
        ret = await fs_api_svc.file_read('/foo')
        assert ret == b'hello'

        await fs_api_svc.move('/foo', '/bar')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == ['bar']

        await fs_api_svc.delete('/bar')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == []

    @pytest.mark.asyncio
    async def test_folder(self, fs_api_svc):
        await fs_api_svc.folder_create('/foo')
        await fs_api_svc.folder_create('/foo/foo')
        await fs_api_svc.file_create('/foo/bar')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == ['foo']

        ret = await fs_api_svc.stat('/foo')
        assert sorted(ret['children']) == sorted(['foo', 'bar'])
        assert ret['type'] == 'folder'

        await fs_api_svc.move('/foo', '/bar')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == ['bar']
        ret = await fs_api_svc.stat('/bar')
        assert sorted(ret['children']) == sorted(['foo', 'bar'])

        await fs_api_svc.delete('/bar/foo')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == ['bar']

        await fs_api_svc.delete('/bar')
        ret = await fs_api_svc.stat('/')
        assert ret['children'] == []
