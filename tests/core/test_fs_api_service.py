import pytest
import asyncio
from unittest.mock import Mock

from parsec.core2.backend_api_service import MockedBackendAPIService
from parsec.core2.fs_api import MockedFSAPIMixin, FSAPIMixin

from tests.core.test_backend_api_service import MockedIdentityService


async def bootstrap_FSAPIMixin(request, event_loop, unused_tcp_port):
    fs_api = FSAPIMixin()
    fs_api.identity = MockedIdentityService()
    fs_api.backend = MockedBackendAPIService()
    await fs_api.identity.load()
    await fs_api.backend.wait_for_connection_ready()
    return fs_api


async def bootstrap_MockedFSAPIMixin(request, event_loop, unused_tcp_port):
    return MockedFSAPIMixin()


@pytest.fixture(params=[bootstrap_MockedFSAPIMixin, bootstrap_FSAPIMixin],
                ids=['mocked', 'backend'])
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
