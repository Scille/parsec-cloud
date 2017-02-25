import pytest
import tempfile

from .common import b64

from parsec.vfs import VFSServiceMock, VFSServiceInMemoryMock


class BaseTestVFSService:

    # Helpers

    async def mkdir(self, path):
        return await self.service.make_dir(path=path)

    async def mkfile(self, path, content=b''):
        return await self.service.create_file(path=path, content=content)

    async def rmfile(self, path):
        return await self.service.delete_file(path=path)

    async def rmdir(self, path):
        return await self.service.remove_dir(path=path)

    # Tests

    @pytest.mark.asyncio
    async def test_create_dir(self):
        ret = await self.service.dispatch_msg({'cmd': 'make_dir', 'path': '/test_dir'})
        assert ret['status'] == 'ok'
        # Make sure target has been created
        ret = await self.service.dispatch_msg({'cmd': 'make_dir', 'path': '/test_dir'})
        assert ret == {'status': 'already_exist', 'label': 'Target already exists.'}

    @pytest.mark.asyncio
    async def test_remove_dir(self):
        await self.mkdir('/test_dir')
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'ok'}
        # Cannot remove already destroyed folder
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    async def test_cant_remove_root_dir(self):
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/'})
        assert ret == {'status': 'cannot_remove_root', 'label': 'Cannot remove root directory.'}

    @pytest.mark.asyncio
    async def test_remove_not_empty_dir(self):
        await self.mkdir('/test_dir')
        await self.mkfile('/test_dir/test')
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'directory_not_empty', 'label': 'Directory not empty.'}
        # Delete file so we can remove the folder
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': '/test_dir/test'})
        assert ret == {'status': 'ok'}
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_list_dir(self):
        # Create folders
        await self.mkdir('/countries')
        await self.mkdir('/countries/France')
        await self.mkdir('/countries/France/cities')
        await self.mkdir('/countries/Belgium')
        await self.mkdir('/countries/Belgium/cities')
        # Create multiple files
        await self.mkfile('/.root')
        await self.mkfile('/countries/index', b'1 - Belgium\n2 - France')
        await self.mkfile('/countries/France/info', b'good=Wine, bad=Beer')
        await self.mkfile('/countries/Belgium/info', b'good=Beer, bad=Wine')

        # Finally do some lookup
        async def assert_ls(path, expected):
            ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': path})
            assert ret['status'] == 'ok'
            assert list(sorted(ret['list'])) == list(sorted(expected))

        await assert_ls('/', ['.root', 'countries'])
        await assert_ls('/countries', ['index', 'Belgium', 'France'])
        await assert_ls('/countries/France/cities', [])

        # Test bad list as well
        ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': '/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

        ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': '/countries/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    async def test_create_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test', 'content': b64(b'test')})
        assert ret == {'status': 'ok', 'size': 4}
        # Try to create file inside directory as well
        await self.mkdir('/test_dir')
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test_dir/test', 'content': b64(b'test')})
        assert ret == {'status': 'ok', 'size': 4}

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    @pytest.mark.asyncio
    async def test_read_file(self, path):
        await self.mkfile('/test', content=b'foo')
        await self.mkdir('/test_dir')
        await self.mkfile('/test_dir/test', content=b'foo')
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'path': path})
        assert ret == {'status': 'ok', 'content': b64(b'foo')}

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    @pytest.mark.asyncio
    async def test_write_file(self, path):
        await self.mkdir('/test_dir')
        ret = await self.service.dispatch_msg({'cmd': 'write_file', 'path': path, 'content': b64(b'bar')})
        assert ret == {'status': 'ok', 'size': 3}
        # Make sure file has been changed
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'path': path})
        assert ret == {'status': 'ok', 'content': b64(b'bar')}

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    @pytest.mark.asyncio
    async def test_delete_file(self, path):
        await self.mkfile('/test')
        await self.mkdir('/test_dir')
        await self.mkfile('/test_dir/test')
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'ok'}
        # Make sure the file is no longer there
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'path': path})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    @pytest.mark.asyncio
    async def test_stat_file(self, path):
        await self.mkfile('/test', content=b'foo')
        await self.mkdir('/test_dir')
        await self.mkfile('/test_dir/test', content=b'foo')
        ret = await self.service.dispatch_msg({'cmd': 'stat', 'path': path})
        assert ret['status'] == 'ok'
        assert ret['stat']['is_dir'] is False
        assert ret['stat']['size'] == len(b'foo')
        assert ret['stat']['atime']
        assert ret['stat']['ctime']
        assert ret['stat']['mtime']
        # Remove file and make sure stat is altered as well
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'ok'}
        ret = await self.service.dispatch_msg({'cmd': 'stat', 'path': path})
        assert ret == {'status': 'not_found', 'label': 'Target not found.'}

    @pytest.mark.asyncio
    async def test_stat_dir(self):
        await self.mkfile('/test')
        await self.mkdir('/test_dir')
        await self.mkfile('/test_dir/test')
        ret = await self.service.dispatch_msg({'cmd': 'stat', 'path': '/test_dir'})
        assert ret['status'] == 'ok'
        assert ret['stat']['is_dir'] is True
        assert ret['stat']['size'] == 0
        assert ret['stat']['atime']
        assert ret['stat']['ctime']
        assert ret['stat']['mtime']
        # Remove dir and make sure stat is altered as well
        await self.rmfile('/test_dir/test')
        await self.rmdir('/test_dir')
        ret = await self.service.dispatch_msg({'cmd': 'stat', 'path': '/test_dir'})
        assert ret == {'status': 'not_found', 'label': 'Target not found.'}

    @pytest.mark.asyncio
    async def test_bad_stat(self):
        ret = await self.service.dispatch_msg({'cmd': 'stat', 'path': '/unknown'})
        assert ret == {'status': 'not_found', 'label': 'Target not found.'}



# class TestVFSService(BaseTestVFSService):

#     def setup_method(self):
#         params = {
#             'override': 'I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE'
#         }
#         crypto_service = CryptoEngineService(symetric=MockSymCipher(**params),
#                                              asymetric=MockAsymCipher(**params))
#         crypto_client = LocalCryptoClient(service=crypto_service)
#         crypto_client.load_key(b'123456789')
#         volume_client = LocalVolumeClient(VolumeServiceInMemoryMock())
#         self.service = VFSService(volume_client, crypto_client)


class TestVFSServiceMock(BaseTestVFSService):

    def setup_method(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.service = VFSServiceMock(self.tmpdir.name)


class TestVFSInMemoryServiceMock(BaseTestVFSService):

    def setup_method(self):
        self.service = VFSServiceInMemoryMock()
