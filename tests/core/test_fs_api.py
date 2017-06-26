from base64 import encodebytes
from copy import deepcopy
from io import BytesIO
import json
import random

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from freezegun import freeze_time
import pytest

from parsec.core.core_service import core_service_factory
from parsec.core import (CoreService, IdentityService, MetaBlockService, MockedBackendAPIService,
                         MockedBlockService, SynchronizerService)
from parsec.core.fs_api import MockedFSAPIMixin, FSAPIMixin
from parsec.core.manifest import GroupManifest, Manifest, UserManifest
from parsec.core.file import File
from parsec.crypto import load_sym_key
from parsec.exceptions import ManifestError, ManifestNotFound
from parsec.server import BaseServer
from parsec.tools import from_jsonb64, to_jsonb64



JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICWgIBAAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO04KJSq1cH87KtmkqI3qewvXtW
qFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/mG/U9gURDi4aTTXT02RbHESBp
yMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49SLhqGNmNAnH2E3lxAgMBAAEC
gYBY2S0QFJG8AwCdfKKUK+t2q+UO6wscwdtqSk/grBg8MWXTb+8XjteRLy3gD9Eu
E1IpwPStjj7KYEnp2blAvOKY0E537d2a4NLrDbSi84q8kXqvv0UeGMC0ZB2r4C89
/6BTZii4mjIlg3iPtkbRdLfexjqmtELliPkHKJIIMH3fYQJBAKd/k1hhnoxEx4sq
GRKueAX7orR9iZHraoR9nlV69/0B23Na0Q9/mP2bLphhDS4bOyR8EXF3y6CjSVO4
LBDPOmUCQQCV5hr3RxGPuYi2n2VplocLK/6UuXWdrz+7GIqZdQhvhvYSKbqZ5tvK
Ue8TYK3Dn4K/B+a7wGTSx3soSY3RBqwdAkAv94jqtooBAXFjmRq1DuGwVO+zYIAV
GaXXa2H8eMqr2exOjKNyHMhjWB1v5dswaPv25tDX/caCqkBFiWiVJ8NBAkBnEnqo
Xe3tbh5btO7+08q4G+BKU9xUORURiaaELr1GMv8xLhBpkxy+2egS4v+Y7C3zPXOi
1oB9jz1YTnt9p6DhAkBy0qgscOzo4hAn062MAYWA6hZOTkvzRbRpnyTRctKwZPSC
+tnlGk8FAkuOm/oKabDOY1WZMkj5zWAXrW4oR3Q2
-----END RSA PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
SLhqGNmNAnH2E3lxAgMBAAE=
-----END PUBLIC KEY-----
"""


@pytest.fixture
def backend_svc(identity_svc):
    service = MockedBackendAPIService()
    service._pubkey_service.add_pubkey(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY)
    return service


@pytest.fixture
def identity_svc(event_loop):
    identity = JOHN_DOE_IDENTITY
    identity_key = BytesIO(JOHN_DOE_PRIVATE_KEY)
    service = IdentityService()
    event_loop.run_until_complete(service.load(identity, identity_key.read()))
    return service


# @pytest.fixture
# def user_vlob_svc(backend_svc):
#     return BufferedUserVlob(backend_svc)


# @pytest.fixture
# def vlob_svc(backend_svc):
#     return BufferedVlob(backend_svc)


# @pytest.fixture(params=[core_service_factory([MockedFSAPIMixin]),
#                         core_service_factory([FSAPIMixin])],
#                 ids=['mocked_fsapi', 'fsapi'])
@pytest.fixture(params=[core_service_factory([FSAPIMixin])],
                ids=['fsapi'])
def core_svc(request, event_loop, backend_svc, identity_svc):
    service = request.param()
    server = BaseServer()
    server.register_service(service)
    server.register_service(backend_svc)
    server.register_service(identity_svc)
    server.register_service(MetaBlockService(backends=[MockedBlockService, MockedBlockService]))
    server.register_service(SynchronizerService())
    event_loop.run_until_complete(server.bootstrap_services())
    # event_loop.run_until_complete(service.load_user_manifest())
    # event_loop.run_until_complete(service.group_create('foo_community'))
    yield service
    event_loop.run_until_complete(server.teardown_services())


# # @pytest.fixture
# # def manifest(event_loop, backend_svc, core_svc, identity_svc, user_vlob_svc, vlob_svc):
# #     manifest = Manifest(backend_svc, core_svc, identity_svc, user_vlob_svc, vlob_svc)
# #     return manifest

# # @pytest.fixture
# # def user_manifest_with_group(event_loop, backend_svc, core_svc, identity_svc, user_vlob_svc, vlob_svc):
# #     manifest = UserManifest(backend_svc, core_svc, identity_svc, user_vlob_svc, vlob_svc, JOHN_DOE_IDENTITY)
# #     event_loop.run_until_complete(manifest.create_group_manifest('foo_community'))
# #     return manifest


class TestFsAPI:

    @pytest.mark.asyncio
    # @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_file_create(self, core_svc, path):
        ret = await core_svc.dispatch_msg({'cmd': 'file_create',
                                                    'path': '/test'})
        assert ret == {'status': 'ok'}
        # Already exist
        ret = await core_svc.dispatch_msg({'cmd': 'file_create',
                                                    'path': '/test'})
        assert ret == {'status': 'already_exists', 'label': 'File already exists.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_create', 'group': 'share'},
        {'cmd': 'file_create', 'path': 42},
        {'cmd': 'file_create', 'path': '/test', 'group': 'share', 'bad_field': 'foo'},
        {'cmd': 'file_create'}, {}])
    async def test_bad_msg_file_create(self, core_svc, bad_msg):
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_file_write_not_found(self, core_svc):
        content = encodebytes('foo'.encode()).decode()
        ret = await core_svc.dispatch_msg({'cmd': 'file_write',
                                           'path': '/test',
                                           'content': content})
        assert ret == {'status': 'file_not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_file_write(self, core_svc):
        await core_svc.file_create('/test')
        # Check with empty and not empty file
        for content in [b'this is v2 content', b'this is v3 content']:
            encoded_data = encodebytes(content).decode()
            ret = await core_svc.dispatch_msg({'cmd': 'file_write',
                                               'path': '/test',
                                               'content': encoded_data})
            assert ret == {'status': 'ok'}
            file = await core_svc.file_read('/test')
            assert file == content
        # Offset
        encoded_data = encodebytes('v4'.encode()).decode()
        ret = await core_svc.dispatch_msg({'cmd': 'file_write',
                                           'path': '/test',
                                           'content': encoded_data,
                                           'offset': 8})
        assert ret == {'status': 'ok'}
        file = await core_svc.file_read('/test')
        assert file == b'this is v4 content'

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_write', 'path': None},
        {'cmd': 'file_write', 'path': '/test', 'offset': 0},
        {'cmd': 'file_write', 'path': '/test', 'content': 'YQ==\n', 'offset': -1},
        {'cmd': 'file_write', 'path': '/test', 'content': 'YQ==\n', 'offset': 0, 'bad_field': 'foo'},
        {'cmd': 'file_write'}, {}])
    async def test_bad_msg_file_write(self, core_svc, bad_msg):
        await core_svc.file_create('/test')
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_file_read_not_found(self, core_svc):
        ret = await core_svc.dispatch_msg({'cmd': 'file_read',
                                           'path': '/test'})
        assert ret == {'status': 'file_not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_file_read(self, core_svc):
        await core_svc.file_create('/test')
        # Empty file
        ret = await core_svc.dispatch_msg({'cmd': 'file_read', 'path': '/test'})
        assert ret == {'status': 'ok', 'content': ''}
        # Not empty file
        content = b'This is a test content.'
        await core_svc.file_write('/test', content, 0)
        ret = await core_svc.dispatch_msg({'cmd': 'file_read', 'path': '/test'})
        encoded_content = encodebytes(content).decode()
        assert ret == {'status': 'ok', 'content': encoded_content}
        # Offset
        offset = 5
        ret = await core_svc.dispatch_msg({'cmd': 'file_read', 'path': '/test', 'offset': offset})
        encoded_content = encodebytes(content[offset:]).decode()
        assert ret == {'status': 'ok', 'content': encoded_content}
        # Size
        size = 9
        ret = await core_svc.dispatch_msg({'cmd': 'file_read',
                                           'path': '/test',
                                           'size': size,
                                           'offset': offset})
        encoded_content = encodebytes(content[offset:][:size]).decode()
        assert ret == {'status': 'ok', 'content': encoded_content}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_read', 'path': None},
        {'cmd': 'file_read', 'path': '/test', 'offset': -1},
        {'cmd': 'file_read', 'path': '/test', 'size': 1, 'offset': -1},
        {'cmd': 'file_read', 'path': '/test', 'size': 1, 'offset': 0, 'bad_field': 'foo'},
        {'cmd': 'file_read'}, {}])
    async def test_bad_msg_file_read(self, core_svc, bad_msg):
        await core_svc.file_create('/test')
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_stat_not_found(self, core_svc):
        ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/test'})
        assert ret == {'status': 'manifest_not_found', 'label': 'Folder or file not found.'}

    @pytest.mark.asyncio
    async def test_stat_file(self, core_svc):
        # Good file
        with freeze_time('2012-01-01') as frozen_datetime:
            await core_svc.file_create('/test')
            ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/test'})
            ctime = frozen_datetime().isoformat()
            assert ret == {'status': 'ok',
                           'type': 'file',
                           'id': ret['id'],
                           'created': ctime,
                           'updated': ctime,
                           'size': 0,
                           'version': 1}
            frozen_datetime.tick()
            mtime = frozen_datetime().isoformat()
            await core_svc.file_write('/test', b'foo', 0)
            ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/test'})
            assert ret == {'status': 'ok',
                           'type': 'file',
                           'id': ret['id'],
                           'created': mtime,
                           'updated': mtime,
                           'size': 3,
                           'version': 1}
            frozen_datetime.tick()
            await core_svc.file_read('/test')  # TODO useless if atime is not modified
            ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/test'})
            assert ret == {'status': 'ok',
                           'type': 'file',
                           'id': ret['id'],
                           'created': mtime,
                           'updated': mtime,
                           'size': 3,
                           'version': 1}


    @pytest.mark.asyncio
    async def test_stat_folder(self, core_svc):
        # Create folders
        await core_svc.folder_create('/countries/France/cities', parents=True)
        await core_svc.folder_create('/countries/Belgium/cities', parents=True)
        # Create multiple files
        await core_svc.file_create('/.root')
        await core_svc.file_create('/countries/index')
        await core_svc.file_create('/countries/France/info')
        await core_svc.file_create('/countries/Belgium/info')
        # Test folders
        ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/'})
        assert ret == {'type': 'folder', 'items': ['.root', 'countries'], 'status': 'ok'}
        ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/countries'})
        assert ret == {'type': 'folder', 'items': ['Belgium', 'France', 'index'], 'status': 'ok'}
        ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/countries/France/cities'})
        assert ret == {'type': 'folder', 'items': [], 'status': 'ok'}
        # Test bad list as well
        ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/dummy'})
        assert ret == {'status': 'manifest_not_found', 'label': 'Folder or file not found.'}
        ret = await core_svc.dispatch_msg({'cmd': 'stat', 'path': '/countries/dummy'})
        assert ret == {'status': 'manifest_not_found', 'label': 'Folder or file not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'stat', 'path': 42},
        {'cmd': 'stat', 'path': None},
        {'cmd': 'stat', 'path': '/test', 'bad_field': 'foo'},
        {'cmd': 'stat'}, {}])
    async def test_bad_msg_stat(self, core_svc, bad_msg):
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_folder_create(self, core_svc):
        # Working
        ret = await core_svc.dispatch_msg({'cmd': 'folder_create', 'path': '/test_folder'})
        assert ret['status'] == 'ok'
        # Already exist
        ret = await core_svc.dispatch_msg({'cmd': 'folder_create', 'path': '/test_folder'})
        assert ret == {'status': 'already_exists', 'label': 'Folder already exists.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'folder_create', 'path': '/foo', 'bad_field': 'foo'},
        {'cmd': 'folder_create', 'path': 42},
        {'cmd': 'folder_create'}, {}])
    async def test_bad_msg_folder_create(self, core_svc, bad_msg):
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_move_file(self, core_svc):
        await core_svc.file_create('/test')
        ret = await core_svc.dispatch_msg({'cmd': 'move', 'src': '/test', 'dst': '/foo'})
        assert ret['status'] == 'ok'
        with pytest.raises(ManifestNotFound):
            await core_svc.stat('/test')
        await core_svc.stat('/foo')

    @pytest.mark.asyncio
    async def test_move_file_and_target_exist(self, core_svc):
        await core_svc.file_create('/test')
        await core_svc.file_create('/foo')
        ret = await core_svc.dispatch_msg({'cmd': 'move', 'src': '/test', 'dst': '/foo'})
        assert ret == {'status': 'already_exists', 'label': 'File already exists.'}
        await core_svc.stat('/test')
        await core_svc.stat('/foo')

    @pytest.mark.asyncio
    async def test_move_folder(self, core_svc):
        await core_svc.folder_create('/test', parents=False)
        ret = await core_svc.dispatch_msg({'cmd': 'move', 'src': '/test', 'dst': '/foo'})
        assert ret['status'] == 'ok'
        with pytest.raises(ManifestNotFound):
            await core_svc.stat('/test')
        await core_svc.stat('/foo')

    @pytest.mark.asyncio
    async def test_move_folder_and_target_exist(self, core_svc):
        await core_svc.folder_create('/test', parents=False)
        await core_svc.folder_create('/foo', parents=False)
        ret = await core_svc.dispatch_msg({'cmd': 'move', 'src': '/test', 'dst': '/foo'})
        assert ret == {'status': 'already_exists', 'label': 'File already exists.'}
        await core_svc.stat('/test')
        await core_svc.stat('/foo')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'move', 'src': '/foo', 'dst': '/bar', 'bad_field': 'foo'},
        {'cmd': 'move', 'src': '/foo', 'dst': 42},
        {'cmd': 'move', 'src': 42, 'dst': '/bar'},
        {'cmd': 'move'}, {}])
    async def test_bad_msg_move(self, core_svc, bad_msg):
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_delete_file(self, core_svc, path):
        await core_svc.folder_create('/test_dir', parents=False)
        for persistent_path in ['/persistent', '/test_dir/persistent']:
            await core_svc.file_create(persistent_path)
        for i in [1, 2]:
            await core_svc.file_create(path)
            ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': path})
            assert ret == {'status': 'ok'}
            # File not found
            ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': path})
            assert ret == {'status': 'manifest_not_found', 'label': 'File not found.'}
            # Persistent files
            for persistent_path in ['/persistent', '/test_dir/persistent']:
                await core_svc.stat(persistent_path)

    @pytest.mark.asyncio
    async def test_remove_folder(self, core_svc):
        # Working
        await core_svc.folder_create('/test_folder', parents=False)
        ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': '/test_folder'})
        assert ret == {'status': 'ok'}
        # Not found
        ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': '/test_folder'})
        assert ret == {'status': 'manifest_not_found', 'label': 'File not found.'}  # TODO Folder?

    @pytest.mark.asyncio
    async def test_cant_remove_root_folder(self, core_svc):
        ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': '/'})
        assert ret == {'status': 'cannot_remove_root', 'label': 'Cannot remove root folder.'}

    @pytest.mark.asyncio
    async def test_remove_not_empty_folder(self, core_svc):
        # Not empty
        await core_svc.folder_create('/test_folder', parents=False)
        await core_svc.file_create('/test_folder/test')
        ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': '/test_folder'})
        assert ret == {'status': 'folder_not_empty', 'label': 'Folder not empty.'}
        # Empty
        ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': '/test_folder/test'})
        assert ret == {'status': 'ok'}
        ret = await core_svc.dispatch_msg({'cmd': 'delete', 'path': '/test_folder'})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'delete', 'path': '/foo', 'bad_field': 'foo'},
        {'cmd': 'delete', 'path': 42},
        {'cmd': 'delete'}, {}])
    async def test_bad_msg_delete(self, core_svc, bad_msg):
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.xfail
    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_undelete_file(self, core_svc, path):
        await core_svc.folder_create('/test_dir', parents=False)
        await core_svc.file_create(path)
        await core_svc.delete(path)
        # Working
        ret = await core_svc.dispatch_msg({'cmd': 'undelete', 'path': path})
        assert ret['status'] == 'ok'
        await core_svc.stat(path)
        # Not found
        ret = await core_svc.dispatch_msg({'cmd': 'undelete', 'path': path})
        assert ret == {'status': 'manifest_not_found', 'label': 'Vlob not found.'}
        # Restore path already used
        await core_svc.delete(path)
        await core_svc.file_create(path)
        ret = await core_svc.dispatch_msg({'cmd': 'undelete', 'path': path})
        assert ret == {'status': 'already_exists', 'label': 'Restore path already used.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'undelete', 'path': '/test', 'bad_field': 'foo'},
        {'cmd': 'undelete', 'id': 42},
        {'cmd': 'undelete'}, {}])
    async def test_bad_msg_undelete_file(self, core_svc, bad_msg):
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_file_truncate_not_found(self, core_svc):
        ret = await core_svc.dispatch_msg({'cmd': 'file_truncate', 'path': '/test', 'length': 7})
        assert ret == {'status': 'file_not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_file_truncate(self, core_svc):
        await core_svc.file_create('/foo')
        file_vlob = await core_svc.get_properties(path='/foo')
        # Encoded contents
        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, block_size + 1)])
        file = await File.load(core_svc.synchronizer, **file_vlob)
        await file.write(content, 0)
        # Truncate full length
        ret = await core_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'path': '/foo',
                                           'length': block_size + 1})
        assert ret == {'status': 'ok'}
        ret = await core_svc.file_read('/foo')
        assert ret == content[:block_size + 1]
        # Truncate block length
        ret = await core_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'path': '/foo',
                                           'length': block_size})
        assert ret == {'status': 'ok'}
        ret = await core_svc.file_read('/foo')
        assert ret == content[:block_size]
        # Truncate shorter than block length
        ret = await core_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'path': '/foo',
                                           'length': block_size - 1})
        assert ret == {'status': 'ok'}
        ret = await core_svc.file_read('/foo')
        assert ret == content[:block_size - 1]
        # Truncate empty
        ret = await core_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'path': '/foo',
                                           'length': 0})
        assert ret == {'status': 'ok'}
        ret = await core_svc.file_read('/foo')
        assert ret == b''

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_truncate', 'path': None},
        {'cmd': 'file_truncate', 'path': '/foo', 'length': -1},
        {'cmd': 'file_truncate', 'path': '/foo', 'length': 0, 'bad_field': 'foo'},
        {'cmd': 'file_truncate'}, {}])
    async def test_bad_msg_file_truncate(self, core_svc, bad_msg):
        await core_svc.file_create('/test')
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_history_not_found(self, core_svc):
        ret = await core_svc.dispatch_msg({'cmd': 'history', 'path': '1234'})
        assert ret == {'status': 'manifest_not_found', 'label': 'Folder or file not found.'}

    @pytest.mark.asyncio
    async def test_history(self, core_svc):
        with freeze_time('2012-01-01') as frozen_datetime:
            import pdb; pdb.set_trace()
            file_vlob = await core_svc.file_create('/test')
            file = await File.load(core_svc.synchronizer, **file_vlob)
            await file.commit()
            original_time = frozen_datetime().timestamp()
            for content in [b'this is v2', b'this is v3...']:
                frozen_datetime.tick()
                await core_svc.file_write('/test', content, 0)
                await file.commit()
        # Full history
        ret = await core_svc.dispatch_msg({'cmd': 'history', 'path': '/test'})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 1,
                    'ctime': original_time,
                    'mtime': original_time,
                    'atime': original_time,
                    'size': 0
                },
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                },
                {
                    'version': 3,
                    'ctime': original_time + 2,
                    'mtime': original_time + 2,
                    'atime': original_time + 2,
                    'size': 13
                }
            ]
        }
        # Partial history starting at version 2
        ret = await core_svc.dispatch_msg({'cmd': 'file_history', 'id': id, 'first_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                },
                {
                    'version': 3,
                    'ctime': original_time + 2,
                    'mtime': original_time + 2,
                    'atime': original_time + 2,
                    'size': 13
                }
            ]
        }
        # Partial history ending at version 2
        ret = await core_svc.dispatch_msg({'cmd': 'file_history', 'id': id, 'last_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 1,
                    'ctime': original_time,
                    'mtime': original_time,
                    'atime': original_time,
                    'size': 0
                },
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                }
            ]
        }
        # First version = last version
        ret = await core_svc.dispatch_msg({'cmd': 'file_history',
                                           'id': id,
                                           'first_version': 2,
                                           'last_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                }
            ]
        }
        # First version > last version
        ret = await core_svc.dispatch_msg({'cmd': 'file_history',
                                           'id': id,
                                           'first_version': 3,
                                           'last_version': 2})
        assert ret == {'status': 'bad_versions',
                       'label': 'First version number higher than the second one.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'history', 'path': 42},
        {'cmd': 'history', 'path': '/foo', 'first_version': 0},
        {'cmd': 'history', 'path': '/foo', 'last_version': 0},
        {'cmd': 'history', 'path': '/foo', 'first_version': 1, 'last_version': 1,
         'bad_field': 'foo'},
        {'cmd': 'history'}, {}])
    async def test_bad_msg_history(self, core_svc, bad_msg):
        await core_svc.file_create('/test')
        ret = await core_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

#     @pytest.mark.asyncio
#     async def test_restore_not_found(self, core_svc):
#         ret = await core_svc.dispatch_msg({'cmd': 'file_restore', 'id': '1234', 'version': 10})
#         assert ret == {'status': 'file_not_found', 'label': 'Vlob not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_restore_file(self, core_svc):
#         encoded_content = encodebytes('initial'.encode()).decode()
#         file_vlob = await core_svc.file_create('/test', encoded_content)
#         id = file_vlob['id']
#         fd = await core_svc.file_open('/test')
#         # Restore file with version 1
#         file = await core_svc.file_read(fd)
#         assert file == {'content': encoded_content, 'version': 1}
#         ret = await core_svc.dispatch_msg({'cmd': 'file_restore', 'id': id})
#         await core_svc.synchronize_manifest()
#         fd = await core_svc.file_open('/test')
#         file = await core_svc.file_read(fd)
#         assert file == {'content': encoded_content, 'version': 1}
#         # Restore previous version
#         for version, content in enumerate(('this is v2', 'this is v3', 'this is v4'), 2):
#             encoded_content = encodebytes(content.encode()).decode()
#             await core_svc.file_write(fd, encoded_content, 0)
#             await core_svc.synchronize_manifest()
#             fd = await core_svc.file_open('/test')
#         file = await core_svc.file_read(fd)
#         encoded_content = encodebytes('this is v4'.encode()).decode()
#         assert file == {'content': encoded_content, 'version': 4}
#         ret = await core_svc.dispatch_msg({'cmd': 'file_restore', 'id': id})
#         await core_svc.synchronize_manifest()
#         fd = await core_svc.file_open('/test')
#         file = await core_svc.file_read(fd)
#         encoded_content = encodebytes('this is v3'.encode()).decode()
#         assert file == {'content': encoded_content, 'version': 5}
#         # Restore old version
#         ret = await core_svc.dispatch_msg({'cmd': 'file_restore', 'id': id, 'version': 2})
#         await core_svc.synchronize_manifest()
#         fd = await core_svc.file_open('/test')
#         file = await core_svc.file_read(fd)
#         encoded_content = encodebytes('this is v2'.encode()).decode()
#         assert file == {'content': encoded_content, 'version': 6}
#         # Bad version
#         ret = await core_svc.dispatch_msg({'cmd': 'file_restore', 'id': id, 'version': 10})
#         assert ret == {'status': 'bad_version', 'label': 'Bad version number.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'file_restore', 'id': 42},
#         {'cmd': 'file_restore', 'id': '<id-here>', 'version': 0},
#         {'cmd': 'file_restore', 'id': '<id-here>', 'version': 1, 'bad_field': 'foo'},
#         {'cmd': 'file_restore'}, {}])
#     async def test_bad_msg_restore_file(self, core_svc, bad_msg):
#         file_vlob = await core_svc.file_create('/test')
#         if bad_msg.get('id') == '<id-here>':
#             bad_msg['id'] = file_vlob['id']
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.asyncio
#     async def test_reencrypt_not_found(self, core_svc):
#         ret = await core_svc.dispatch_msg({'cmd': 'file_reencrypt', 'id': '1234'})
#         assert ret == {'status': 'file_not_found', 'label': 'Vlob not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_reencrypt(self, core_svc):
#         encoded_content_initial = encodebytes('content 1'.encode()).decode()
#         encoded_content_final = encodebytes('content 2'.encode()).decode()
#         old_vlob = await core_svc.file_create('/foo', encoded_content_initial)
#         ret = await core_svc.dispatch_msg({'cmd': 'file_reencrypt', 'id': old_vlob['id']})
#         assert ret['status'] == 'ok'
#         del ret['status']
#         new_vlob = ret
#         for property in old_vlob.keys():
#             assert old_vlob[property] != new_vlob[property]
#         await core_svc.import_file_vlob('/bar', new_vlob)
#         fd = await core_svc.file_open('/bar')
#         await core_svc.file_write(fd, encoded_content_final, 0)
#         old_fd = await core_svc.file_open('/foo')
#         file = await core_svc.file_read(old_fd)
#         assert file == {'content': encoded_content_initial, 'version': 1}
#         file = await core_svc.file_read(fd)
#         assert file == {'content': encoded_content_final, 'version': 1}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'file_reencrypt', 'id': 42},
#         {'cmd': 'file_reencrypt', 'id': '<id-here>', 'bad_field': 'foo'},
#         {'cmd': 'file_reencrypt'}, {}])
#     async def test_bad_msg_reencrypt(self, core_svc, bad_msg):
#         file_vlob = await core_svc.file_create('/test')
#         if bad_msg.get('id') == '<id-here>':
#             bad_msg['id'] = file_vlob['id']
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'
