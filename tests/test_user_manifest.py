import pytest

from parsec.core.user_manifest_service import UserManifestService
from parsec.core.file_service import FileService


class BaseTestUserManifestService:

    # Helpers

    async def mkdir(self, path):
        return await self.service.make_dir(path=path)

    async def mkfile(self, path):
        return await self.service.create_file(path=path)

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
        # Cannot remove already destroyed directory
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
        await self.mkfile('/countries/index')
        await self.mkfile('/countries/France/info')
        await self.mkfile('/countries/Belgium/info')

        # Finally do some lookup
        async def assert_ls(path, expected_childrens):
            ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': path})
            assert ret['status'] == 'ok'
            for name in expected_childrens:
                assert list(sorted(['id', 'read_seed', 'write_seed', 'key'])) == list(sorted(ret['current'].keys()))
                assert list(sorted(['id', 'read_seed', 'write_seed', 'key'])) == list(sorted(ret['childrens'][name].keys()))

        await assert_ls('/', ['.root', 'countries'])
        await assert_ls('/countries', ['index', 'Belgium', 'France'])
        await assert_ls('/countries/France/cities', [])
        await assert_ls('/countries/France/info', [])

        # Test bad list as well
        ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': '/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

        ret = await self.service.dispatch_msg({'cmd': 'list_dir', 'path': '/countries/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    async def test_create_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret['status'] == 'ok'
        assert ret['file']['id'] is not None
        # Try to create file inside directory as well
        await self.mkdir('/test_dir')
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test_dir/test'})
        assert ret['status'] == 'ok'
        assert ret['file']['id'] is not None

    @pytest.mark.asyncio
    async def test_rename_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret['status'] == 'ok'
        assert ret['file']['id'] is not None
        ret = await self.service.dispatch_msg({'cmd': 'rename_file', 'old_path': '/test', 'new_path': '/foo'})
        assert ret['status'] == 'ok'
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': '/test'})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': '/foo'})
        assert ret == {'status': 'ok'}

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    @pytest.mark.asyncio
    async def test_delete_file(self, path):
        await self.mkfile('/test')
        await self.mkdir('/test_dir')
        await self.mkfile('/test_dir/test')
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'ok'}
        # Make sure the file is no longer there
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}

    # TODO tests for load from file, dump to file, check consistency

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_history(self):
        raise NotImplementedError()


class TestUserManifestService(BaseTestUserManifestService):

    def setup_method(self):
        self.file_service = FileService()
        self.service = UserManifestService(self.file_service)
