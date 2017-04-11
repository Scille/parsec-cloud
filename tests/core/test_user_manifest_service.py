from os import path

import gnupg
import pytest

from parsec.server import BaseServer
from parsec.core import (BackendAPIService, CryptoService, FileService, IdentityService,
                         GNUPGPubKeysService, UserManifestService)


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def user_manifest_svc(event_loop):
    service = UserManifestService()
    crypto_service = CryptoService()
    crypto_service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/alice')
    identity_service = IdentityService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(crypto_service)
    server.register_service(identity_service)
    server.register_service(BackendAPIService('localhost', 6777))
    server.register_service(FileService())
    server.register_service(GNUPGPubKeysService())
    server.bootstrap_services()
    event_loop.run_until_complete(identity_service.load_identity())
    event_loop.run_until_complete(service.load_user_manifest())
    return service


class TestUserManifestService:

    @pytest.mark.asyncio
    async def test_create_dir(self, user_manifest_svc):
        # Working
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'make_dir', 'path': '/test_dir'})
        assert ret['status'] == 'ok'
        # Already exist
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'make_dir', 'path': '/test_dir'})
        assert ret == {'status': 'already_exist', 'label': 'Target already exists.'}

    @pytest.mark.asyncio
    async def test_remove_dir(self, user_manifest_svc):
        # Working
        await user_manifest_svc.make_dir('/test_dir')
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'ok'}
        # Not found
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    async def test_cant_remove_root_dir(self, user_manifest_svc):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'remove_dir', 'path': '/'})
        assert ret == {'status': 'cannot_remove_root', 'label': 'Cannot remove root directory.'}

    @pytest.mark.asyncio
    async def test_remove_not_empty_dir(self, user_manifest_svc):
        # Not empty
        await user_manifest_svc.make_dir('/test_dir')
        await user_manifest_svc.create_file('/test_dir/test')
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'directory_not_empty', 'label': 'Directory not empty.'}
        # Empty
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'delete_file', 'path': '/test_dir/test'})
        assert ret == {'status': 'ok'}
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'remove_dir', 'path': '/test_dir'})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_list_dir(self, user_manifest_svc):
        # Create folders
        await user_manifest_svc.make_dir('/countries')
        await user_manifest_svc.make_dir('/countries/France')
        await user_manifest_svc.make_dir('/countries/France/cities')
        await user_manifest_svc.make_dir('/countries/Belgium')
        await user_manifest_svc.make_dir('/countries/Belgium/cities')
        # Create multiple files
        await user_manifest_svc.create_file('/.root')
        await user_manifest_svc.create_file('/countries/index')
        await user_manifest_svc.create_file('/countries/France/info')
        await user_manifest_svc.create_file('/countries/Belgium/info')

        # Finally do some lookup
        async def assert_ls(path, expected_childrens):
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'list_dir', 'path': path})
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
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'list_dir', 'path': '/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

        ret = await user_manifest_svc.dispatch_msg({'cmd': 'list_dir', 'path': '/countries/dummy'})
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    async def test_create_file(self, path, user_manifest_svc):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret['status'] == 'ok'
        assert ret['id'] is not None
        # Already exist
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret == {'status': 'already_exist', 'label': 'Target already exists.'}

    @pytest.mark.asyncio
    async def test_rename_file(self, user_manifest_svc):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'create_file', 'path': '/test'})
        assert ret['status'] == 'ok'
        assert ret['id'] is not None
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'rename_file',
                                                    'old_path': '/test',
                                                    'new_path': '/foo'})
        assert ret['status'] == 'ok'
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'delete_file', 'path': '/test'})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'delete_file', 'path': '/foo'})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    async def test_delete_file(self, path, user_manifest_svc):
        await user_manifest_svc.create_file('/test')
        await user_manifest_svc.make_dir('/test_dir')
        await user_manifest_svc.create_file('/test_dir/test')
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'ok'}
        # File not found
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'delete_file', 'path': path})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}

    # TODO tests for load from file, dump to file, check consistency

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_history(self, user_manifest_svc):
        raise NotImplementedError()
