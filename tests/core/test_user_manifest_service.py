from os import path

import gnupg
import pytest

from parsec.server import BaseServer
from parsec.core import (CryptoService, FileService, IdentityService, GNUPGPubKeysService,
                         MetaBlockService, MockedBackendAPIService, MockedBlockService,
                         ShareService, UserManifestService)


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def user_manifest_svc(event_loop):
    identity = '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'
    service = UserManifestService()
    block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
    crypto_service = CryptoService()
    crypto_service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
    identity_service = IdentityService()
    share_service = ShareService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(block_service)
    server.register_service(crypto_service)
    server.register_service(identity_service)
    server.register_service(MockedBackendAPIService())
    server.register_service(FileService())
    server.register_service(GNUPGPubKeysService())
    server.register_service(share_service)
    event_loop.run_until_complete(server.bootstrap_services())
    event_loop.run_until_complete(identity_service.load_identity(identity=identity))
    event_loop.run_until_complete(service.load_user_manifest())
    event_loop.run_until_complete(share_service.group_create('foo_community'))
    yield service
    event_loop.run_until_complete(server.teardown_services())


class TestUserManifestService:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_create_dir(self, user_manifest_svc, group):
        # Working
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_make_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret['status'] == 'ok'
        # Already exist
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_make_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'already_exist', 'label': 'Directory already exists.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_remove_dir(self, user_manifest_svc, group):
        # Working
        await user_manifest_svc.make_dir('/test_dir', group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'ok'}
        # Not found
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'not_found', 'label': 'Directory or file not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_cant_remove_root_dir(self, user_manifest_svc, group):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/',
                                                    'group': group})
        assert ret == {'status': 'cannot_remove_root', 'label': 'Cannot remove root directory.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_remove_not_empty_dir(self, user_manifest_svc, group):
        # Not empty
        await user_manifest_svc.make_dir('/test_dir', group)
        await user_manifest_svc.create_file('/test_dir/test', group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'directory_not_empty', 'label': 'Directory not empty.'}
        # Empty
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_delete_file',
                                                    'path': '/test_dir/test',
                                                    'group': group})
        assert ret == {'status': 'ok'}
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_list_dir(self, user_manifest_svc, group):
        # Create folders
        await user_manifest_svc.make_dir('/countries', group)
        await user_manifest_svc.make_dir('/countries/France', group)
        await user_manifest_svc.make_dir('/countries/France/cities', group)
        await user_manifest_svc.make_dir('/countries/Belgium', group)
        await user_manifest_svc.make_dir('/countries/Belgium/cities', group)
        # Create multiple files
        await user_manifest_svc.create_file('/.root', group=group)
        await user_manifest_svc.create_file('/countries/index', group=group)
        await user_manifest_svc.create_file('/countries/France/info', group=group)
        await user_manifest_svc.create_file('/countries/Belgium/info', group=group)

        # Finally do some lookup
        async def assert_ls(path, expected_children):
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_list_dir',
                                                        'path': path,
                                                        'group': group})
            assert ret['status'] == 'ok'
            for name in expected_children:
                keys = ['id', 'read_trust_seed', 'write_trust_seed', 'key']
                assert list(sorted(keys)) == list(sorted(ret['current'].keys()))
                assert list(sorted(keys)) == list(sorted(ret['children'][name].keys()))

        await assert_ls('/', ['.root', 'countries'])
        await assert_ls('/countries', ['index', 'Belgium', 'France'])
        await assert_ls('/countries/France/cities', [])
        await assert_ls('/countries/France/info', [])

        # Test bad list as well
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_list_dir',
                                                    'path': '/dummy',
                                                    'group': group})
        assert ret == {'status': 'not_found', 'label': 'Directory or file not found.'}

        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_list_dir',
                                                    'path': '/countries/dummy',
                                                    'group': group})
        assert ret == {'status': 'not_found', 'label': 'Directory or file not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_create_file(self, user_manifest_svc, group, path):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret['status'] == 'ok'
        assert ret['id'] is not None
        # Already exist
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret == {'status': 'already_exist', 'label': 'File already exists.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_rename_file(self, user_manifest_svc, group):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret['status'] == 'ok'
        assert ret['id'] is not None
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_rename_file',
                                                    'old_path': '/test',
                                                    'new_path': '/foo',
                                                    'group': group})
        assert ret['status'] == 'ok'
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_delete_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_delete_file',
                                                    'path': '/foo',
                                                    'group': group})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_delete_file(self, user_manifest_svc, group, path):
        await user_manifest_svc.make_dir('/test_dir', group)
        for persistent_path in ['/persistent', '/test_dir/persistent']:
            await user_manifest_svc.create_file(persistent_path, group=group)
        for i in [1, 2]:
            await user_manifest_svc.create_file(path, group=group)
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_delete_file',
                                                        'path': path,
                                                        'group': group})
            assert ret == {'status': 'ok'}
            # File not found
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_delete_file',
                                                        'path': path,
                                                        'group': group})
            assert ret == {'status': 'not_found', 'label': 'File not found.'}
            # Persistent files
            for persistent_path in ['/persistent', '/test_dir/persistent']:
                await user_manifest_svc.list_dir(persistent_path, group)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_show_dustbin(self, user_manifest_svc, group, path):
        # Empty dustbin
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_show_dustbin',
                                                    'group': group})
        assert ret == {'status': 'ok', 'dustbin': []}
        await user_manifest_svc.create_file('/foo', group=group)
        await user_manifest_svc.delete_file('/foo', group)
        await user_manifest_svc.make_dir('/test_dir', group)
        for i in [1, 2]:
            await user_manifest_svc.create_file(path, group=group)
            await user_manifest_svc.delete_file(path, group)
            # Global dustbin with one additional file
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_show_dustbin',
                                                        'group': group})
            assert ret['status'] == 'ok'
            assert len(ret['dustbin']) == i + 1
            # File in dustbin
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_show_dustbin',
                                                        'path': path,
                                                        'group': group})
            assert ret['status'] == 'ok'
            assert len(ret['dustbin']) == i
            # Not found
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_show_dustbin',
                                                        'path': '/unknown',
                                                        'group': group})
            assert ret == {'status': 'not_found', 'label': 'Path not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_restore_file(self, user_manifest_svc, group, path):
        await user_manifest_svc.make_dir('/test_dir', group)
        await user_manifest_svc.create_file(path, group=group)
        current, _ = await user_manifest_svc.list_dir(path, group)
        vlob_id = current['id']
        await user_manifest_svc.delete_file(path, group)
        # Working
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore_file',
                                                    'vlob': vlob_id,
                                                    'group': group})
        assert ret['status'] == 'ok'
        await user_manifest_svc.list_dir(path, group)
        # Not found
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore_file',
                                                    'vlob': vlob_id,
                                                    'group': group})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}
        # Restoration path already used
        await user_manifest_svc.delete_file(path, group)
        await user_manifest_svc.create_file(path, group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore_file',
                                                    'vlob': vlob_id,
                                                    'group': group})
        assert ret == {'status': 'not_found', 'label': 'Restoration path already used.'}

    # TODO tests for load from file, dump to file, check consistency

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_history(self, user_manifest_svc):
        raise NotImplementedError()
