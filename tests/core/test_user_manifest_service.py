from asynctest import mock, MagicMock
import pytest

from parsec.server import BaseServer
from parsec.core import (CryptoService, FileService, IdentityService, PubKeysService,
                         UserManifestService)


class AsyncMock(MagicMock):
    async def __call__(self, *args, **kwargs):
        return super(AsyncMock, self).__call__(*args, **kwargs)


class BaseTestUserManifestService:

    @pytest.mark.asyncio
    async def test_create_dir(self):
        # Working
        ret = await self.service.dispatch_msg({'cmd': 'make_dir', 'path': '/test_dir'})
        assert ret['status'] == 'ok'
        # Already exist
        ret = await self.service.dispatch_msg({'cmd': 'make_dir', 'path': '/test_dir'})
        assert ret == {'status': 'already_exist', 'label': 'Target already exists.'}

    @pytest.mark.asyncio
    async def test_remove_dir(self):
        # Working
        await self.service.make_dir('/test_dir')
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'ok'}
        # Not found
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    async def test_cant_remove_root_dir(self):
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/'})
        assert ret == {'status': 'cannot_remove_root', 'label': 'Cannot remove root directory.'}

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_remove_not_empty_dir(self):
        # Not empty
        await self.service.make_dir('/test_dir')
        await self.service.create_file('/test_dir/test')
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'directory_not_empty', 'label': 'Directory not empty.'}
        # Empty
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': '/test_dir/test'})
        assert ret == {'status': 'ok'}
        ret = await self.service.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'ok'}

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_list_dir(self):
        # Create folders
        await self.service.make_dir('/countries')
        await self.service.make_dir('/countries/France')
        await self.service.make_dir('/countries/France/cities')
        await self.mkservice.make_dirdir('/countries/Belgium')
        await self.service.make_dir('/countries/Belgium/cities')
        # Create multiple files
        await self.service.create_file('/.root')
        await self.service.create_file('/countries/index')
        await self.service.create_file('/countries/France/info')
        await self.service.create_file('/countries/Belgium/info')

        # Finally do some lookup
        async def assert_ls(path, expected_childrens):
            ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': path})
            assert ret['status'] == 'ok'
            for name in expected_childrens:
                keys = ['id', 'read_trust_seed', 'write_trust_seed', 'key']
                assert list(sorted(keys)) == list(sorted(ret['current'].keys()))
                assert list(sorted(keys)) == list(sorted(ret['childrens'][name].keys()))

        await assert_ls('/', ['.root', 'countries'])
        await assert_ls('/countries', ['index', 'Belgium', 'France'])
        await assert_ls('/countries/France/cities', [])
        await assert_ls('/countries/France/info', [])

        # Test bad list as well
        ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': '/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

        ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': '/countries/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.xfail
    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    async def test_create_file(self, path):
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret['status'] == 'ok'
        assert ret['file']['id'] is not None

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_rename_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret['status'] == 'ok'
        assert ret['file']['id'] is not None
        ret = await self.service.dispatch_msg({'cmd': 'rename_file',
                                               'old_path': '/test',
                                               'new_path': '/foo'})
        assert ret['status'] == 'ok'
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': '/test'})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': '/foo'})
        assert ret == {'status': 'ok'}

    @pytest.mark.xfail
    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    async def test_delete_file(self, path):
        await self.service.create_file('/test')
        await self.service.make_dir('/test_dir')
        await self.service.create_file('/test_dir/test')
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'ok'}
        # File not found
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}

    # TODO tests for load from file, dump to file, check consistency

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_history(self):
        raise NotImplementedError()


class TestUserManifestService(BaseTestUserManifestService):

    def setup_method(self):
        self.service = UserManifestService('localhost', 6777)
        mock.patch.object(self.service, 'save_user_manifest', new=AsyncMock()).start()  # remove?
        server = BaseServer()
        server.register_service(self.service)
        server.register_service(CryptoService())
        server.register_service(FileService('localhost', 6777))
        server.register_service(IdentityService('localhost', 6777))
        server.register_service(PubKeysService())
        server.bootstrap_services()
