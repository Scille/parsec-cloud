import pytest
from unittest.mock import Mock

from parsec.exceptions import IdentityNotLoadedError
from parsec.core2.backend_api_service import MockedBackendAPIService
from parsec.core2.fs_api import MockedFSAPIService, FSAPIService
from parsec.core2.identity_service import BaseIdentityService


class MockedIdentityService(BaseIdentityService):

    def __init__(self):
        super().__init__()
        self._loaded = False
        self._id = 'mocked_user'
        self._private_key = Mock()
        self._private_key.sign.return_value = b'<mocked_signature>'
        self._private_key.decrypt = lambda x: x
        self._public_key = Mock()
        self._public_key.encrypt = lambda x: x
        self._public_key.verify.return_value = True

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

    @property
    def public_key(self):
        if not self._loaded:
            raise IdentityNotLoadedError('Identity not loaded')
        return self._public_key

    async def load(self, *args):
        self._loaded = True
        self.on_identity_loaded.send(self.id)

    async def unload(self):
        self._loaded = False
        self.on_identity_unloaded.send(self.id)


async def bootstrap_FSAPIService(request, event_loop, unused_tcp_port):
    fs_api = FSAPIService()
    fs_api.identity = MockedIdentityService()
    fs_api.backend = MockedBackendAPIService()
    fs_api.backend.identity = fs_api.identity
    await fs_api.bootstrap()

    def finalize():
        event_loop.run_until_complete(fs_api.teardown())

    request.addfinalizer(finalize)
    return fs_api


async def bootstrap_MockedFSAPIService(request, event_loop, unused_tcp_port):
    fs_api = MockedFSAPIService()
    fs_api.identity = MockedIdentityService()
    return fs_api


@pytest.fixture(params=[bootstrap_MockedFSAPIService, bootstrap_FSAPIService],
                ids=['mocked', 'backend'])
async def fs_api_svc(request, event_loop, unused_tcp_port):
    return await request.param(request, event_loop, unused_tcp_port)


class TestBackendAPIService:
    @pytest.mark.asyncio
    async def test_before_identity_load(self, fs_api_svc):
        with pytest.raises(IdentityNotLoadedError):
            await fs_api_svc.file_create('/foo')
        await fs_api_svc.identity.load()
        await fs_api_svc.wait_for_ready()
        await fs_api_svc.file_create('/foo')

    @pytest.mark.asyncio
    async def test_file(self, fs_api_svc):
        await fs_api_svc.identity.load()
        await fs_api_svc.wait_for_ready()
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
        await fs_api_svc.identity.load()
        await fs_api_svc.wait_for_ready()
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
