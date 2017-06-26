from copy import deepcopy
import json
from io import BytesIO

from freezegun import freeze_time
import pytest

from parsec.core import (CoreService, IdentityService, MetaBlockService, MockedBackendAPIService,
                         MockedBlockService, SynchronizerService)
from parsec.core.manifest import GroupManifest, Manifest, UserManifest
from parsec.core.file import File
from parsec.crypto import generate_sym_key
from parsec.exceptions import BlockNotFound, ManifestError, ManifestNotFound
from parsec.server import BaseServer
from parsec.tools import to_jsonb64


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
def identity_svc(event_loop):
    identity = JOHN_DOE_IDENTITY
    identity_key = BytesIO(JOHN_DOE_PRIVATE_KEY)
    service = IdentityService()
    event_loop.run_until_complete(service.load(identity, identity_key.read()))
    return service


@pytest.fixture
def synchronizer_svc():
    return SynchronizerService()


@pytest.fixture
def manifest(core_svc):
    return Manifest(core_svc)


@pytest.fixture
def group_manifest(event_loop, core_svc):
    return event_loop.run_until_complete(GroupManifest.create(core_svc))


@pytest.fixture
def user_manifest(event_loop, core_svc):
    return event_loop.run_until_complete(UserManifest.load(core_svc))  # TODO retrieve key from id?


@pytest.fixture
def user_manifest_with_group(event_loop, core_svc, user_manifest):
    group_manifest = event_loop.run_until_complete(GroupManifest.create(core_svc))
    event_loop.run_until_complete(group_manifest.commit())
    group_vlob = event_loop.run_until_complete(group_manifest.get_vlob())
    event_loop.run_until_complete(user_manifest.import_group_vlob('foo_community', group_vlob))
    return user_manifest


@pytest.fixture
def core_svc(event_loop, identity_svc, synchronizer_svc):
    service = CoreService()
    block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
    backend_service = MockedBackendAPIService()
    event_loop.run_until_complete(
        backend_service._pubkey_service.add_pubkey(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY))
    server = BaseServer()
    server.register_service(service)
    server.register_service(backend_service)
    server.register_service(block_service)
    server.register_service(identity_svc)
    server.register_service(synchronizer_svc)
    event_loop.run_until_complete(server.bootstrap_services())
    # event_loop.run_until_complete(service.load_user_manifest())
    yield service
    event_loop.run_until_complete(server.teardown_services())


class TestManifest:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('id', ['i123', None])
    async def test_init(self, core_svc, id):
        manifest = Manifest(core_svc, id)
        assert manifest.id == id

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_is_dirty(self, synchronizer_svc, manifest):
        assert await manifest.is_dirty() is False
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.add_file('/foo', vlob)
        assert await manifest.is_dirty() is True
        await manifest.delete_file('/foo')
        assert await manifest.is_dirty() is False

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_diff(self, manifest):
        vlob_1 = {'id': 'vlob_1'}
        vlob_2 = {'id': 'vlob_2'}
        vlob_3 = {'id': 'vlob_3'}
        vlob_4 = {'id': 'vlob_4'}
        vlob_5 = {'id': 'vlob_5'}
        vlob_6 = {'id': 'vlob_6'}
        vlob_7 = {'id': 'vlob_7'}
        vlob_8 = {'id': 'vlob_8'}
        vlob_9 = {'id': 'vlob_9'}
        # Empty diff
        diff = await manifest.diff(
            {
                'entries': {'/a': vlob_1, '/b': vlob_2, '/c': vlob_3},
                'dustbin': [vlob_5, vlob_6, vlob_7],
                'versions': {'vlob_1': 1, 'vlob_2': 1, 'vlob_3': 1, 'vlob_4': None}
            },
            {
                'entries': {'/a': vlob_1, '/b': vlob_2, '/c': vlob_3},
                'dustbin': [vlob_5, vlob_6, vlob_7],
                'versions': {'vlob_1': 1, 'vlob_2': 1, 'vlob_3': 1, 'vlob_4': None}
            }
        )
        assert diff == {
            'entries': {'added': {}, 'changed': {}, 'removed': {}},
            'dustbin': {'added': [], 'removed': []},
            'versions': {'added': {}, 'changed': {}, 'removed': {}}
        }
        # Not empty diff
        diff = await manifest.diff(
            {
                'entries': {'/a': vlob_1, '/b': vlob_2, '/c': vlob_3},
                'dustbin': [vlob_5, vlob_6, vlob_7],
                'versions': {'vlob_1': 1, 'vlob_2': 1, 'vlob_3': 1, 'vlob_4': None}
            },
            {
                'entries': {'/a': vlob_6, '/b': vlob_2, '/d': vlob_4},
                'dustbin': [vlob_7, vlob_8, vlob_9],
                'versions': {'vlob_1': 2, 'vlob_3': 1, 'vlob_5': 2, 'vlob_4': None}
            }
        )
        assert diff == {
            'entries': {'added': {'/d': vlob_4},
                        'changed': {'/a': (vlob_1, vlob_6)},
                        'removed': {'/c': vlob_3}},
            'dustbin': {'added': [vlob_8, vlob_9],
                        'removed': [vlob_5, vlob_6]},
            'versions': {'added': {'vlob_5': 2},
                         'changed': {'vlob_1': (1, 2)},
                         'removed': {'vlob_2': 1}}
        }

    @pytest.mark.asyncio
    async def test_patch(self, manifest):
        # TODO too intrusive?
        vlob_1 = {'id': 'vlob_1', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_2 = {'id': 'vlob_2', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_3 = {'id': 'vlob_3', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_4 = {'id': 'vlob_4', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_5 = {'id': 'vlob_5', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_6 = {'id': 'vlob_6', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_7 = {'id': 'vlob_7', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_8 = {'id': 'vlob_8', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        vlob_9 = {'id': 'vlob_9', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest.original_manifest = {
            'entries': {'/A-B-C': vlob_1,  # Conflict between B and C, save C-conflict
                        '/A-B-nil': vlob_2,  # Recover B, save B-deleted
                        '/A-A-nil': vlob_4,  # Delete A
                        '/A-A-A': vlob_5,
                        '/A-nil-A': vlob_6,
                        '/A-nil-B': vlob_7},  # Recover B, save B-recreated
            'dustbin': [vlob_4, vlob_5, vlob_6],
            'versions': {}
        }
        manifest.entries = {'/A-B-C': vlob_2,
                            '/A-B-nil': vlob_3,
                            '/A-A-nil': vlob_4,
                            '/A-A-A': vlob_5,
                            '/nil-A-A': vlob_6,  # Resolve conflict silently
                            '/nil-A-B': vlob_7,  # Conflict between A and B, save B-conflict
                            '/nil-A-nil': vlob_9}  # Recover A, save A-deleted
        manifest.dustbin = [vlob_6, vlob_7, vlob_8]
        # Recreate entries and dustbin from original manifest
        backup_original = deepcopy(manifest.original_manifest)
        new_manifest = json.loads(await manifest.dumps())
        diff = await manifest.diff(backup_original, new_manifest)
        patched_manifest = await manifest.patch(backup_original, diff)
        assert backup_original == manifest.original_manifest
        assert patched_manifest['entries'] == manifest.entries
        assert patched_manifest['dustbin'] == manifest.dustbin
        # Reapply patch on already patched manifest
        patched_manifest_2 = await manifest.patch(patched_manifest, diff)
        assert patched_manifest == patched_manifest_2
        # Apply patch on a different source manifest
        new_manifest = {
            'entries': {'/A-B-C': vlob_3,
                        '/A-A-A': vlob_5,
                        '/nil-A-A': vlob_6,
                        '/nil-A-B': vlob_8,
                        '/A-nil-A': vlob_6,
                        '/A-nil-B': vlob_8},
            'dustbin': [vlob_5, vlob_6, vlob_7],
            'versions': {}
        }
        patched_manifest = await manifest.patch(new_manifest, diff)
        assert patched_manifest == {
            'entries': {'/A-B-C-conflict': vlob_3,
                        '/A-B-C': vlob_2,
                        '/A-B-nil-deleted': vlob_3,
                        '/A-A-A': vlob_5,
                        '/A-nil-B-recreated': vlob_8,
                        '/nil-A-A': vlob_6,
                        '/nil-A-B-conflict': vlob_8,
                        '/nil-A-B': vlob_7,
                        '/nil-A-nil': vlob_9},
            'dustbin': [vlob_6, vlob_7, vlob_8],
            'versions': {}
        }

    @pytest.mark.asyncio
    async def test_get_version(self, manifest):
        assert await manifest.get_version() == 0  # TODO dirty check
        # TODO check after synchronization

    @pytest.mark.asyncio
    async def test_get_vlobs_versions(self, manifest, synchronizer_svc):
        # TODO version
        # File (version 1)
        file_1 = await File.create(synchronizer_svc)
        vlob_1 = await file_1.get_vlob()
        await file_1.write(b'foo', 0)
        await manifest.add_file('/foo', vlob_1)
        # File (version 2)
        file_2 = await File.create(synchronizer_svc)
        vlob_2 = await file_2.commit()
        await file_2.write(b'bar', 0)
        await file_2.commit()
        await file_2.write(b'bar_bar', 0)
        await manifest.add_file('/bar', vlob_2)
        await manifest.delete_file('/bar')
        # Wrong vlob
        wrong_vlob = {'id': 'i123', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        await manifest.add_file('/baz', wrong_vlob)
        # Folder
        await manifest.make_folder('/dir')
        versions = await manifest.get_vlobs_versions()
        assert versions == {vlob_1['id']: 1, vlob_2['id']: 2, wrong_vlob['id']: None}

    @pytest.mark.asyncio
    async def test_dumps_current_manifest(self, synchronizer_svc, manifest):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.add_file('/foo', vlob)
        dump = await manifest.dumps(original_manifest=False)
        dump = json.loads(dump)
        assert dump == {'entries': {'/': None, '/foo': vlob},
                        'dustbin': [],
                        'versions': {vlob['id']: 1}}

    @pytest.mark.asyncio
    async def test_dumps_original_manifest(self, synchronizer_svc, manifest):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.add_file('/foo', vlob)
        dump = await manifest.dumps(original_manifest=True)
        dump = json.loads(dump)
        assert dump == {'entries': {'/': None},
                        'dustbin': [],
                        'versions': {}}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_add_file(self, synchronizer_svc, manifest, final_slash):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.add_file('/test' + final_slash, vlob)
        # Already exists
        with pytest.raises(ManifestError):
            await manifest.add_file('/test', vlob)
        # Parent not found
        with pytest.raises(ManifestNotFound):
            await manifest.add_file('/test_dir/test', vlob)
        # Parent found
        await manifest.make_folder('/test_dir')
        await manifest.add_file('/test_dir/test', vlob)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_rename_file(self, synchronizer_svc, manifest, final_slash):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.make_folder('/test')
        await manifest.add_file('/test/test', vlob)
        # Rename file
        await manifest.rename_file('/test/test' + final_slash, '/test/foo' + final_slash)
        with pytest.raises(ManifestNotFound):
            await manifest.stat('/test/test')
        await manifest.stat('/test/foo')
        # Rename dir
        await manifest.rename_file('/test' + final_slash, '/foo' + final_slash)
        with pytest.raises(ManifestNotFound):
            await manifest.stat('/test')
        with pytest.raises(ManifestNotFound):
            await manifest.stat('/test/foo')
        await manifest.stat('/foo')
        await manifest.stat('/foo/foo')
        # Rename parent and parent not found
        with pytest.raises(ManifestNotFound):
            await manifest.rename_file('/foo/foo' + final_slash, '/test/test' + final_slash)
        await manifest.stat('/foo')
        await manifest.stat('/foo/foo')
        # Rename parent and parent found
        await manifest.make_folder('/test')
        await manifest.rename_file('/foo/foo' + final_slash, '/test/test' + final_slash)
        await manifest.stat('/test')
        await manifest.stat('/test/test')

    @pytest.mark.asyncio
    async def test_rename_file_and_source_not_exists(self, manifest):
        with pytest.raises(ManifestNotFound):
            await manifest.rename_file('/test', '/foo')

    @pytest.mark.asyncio
    async def test_rename_file_and_target_exists(self, synchronizer_svc, manifest):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.add_file('/test', vlob)
        await manifest.add_file('/foo', vlob)
        with pytest.raises(ManifestError):
            await manifest.rename_file('/test', '/foo')
        await manifest.stat('/test')
        await manifest.stat('/foo')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    @pytest.mark.parametrize('synchronize', [True, False])
    async def test_delete_file(self, synchronizer_svc, manifest, path, final_slash, synchronize):
        persistent_file = await File.create(synchronizer_svc)
        persistent_vlob = await persistent_file.get_vlob()
        file = await File.create(synchronizer_svc)
        await file.write(b'foo', 0)
        vlob = await file.get_vlob()
        block_ids = await file.get_blocks()
        await manifest.make_folder('/test_dir')
        if synchronize:
            vlob = await file.commit()
        for persistent_path in ['/persistent', '/test_dir/persistent']:
            await manifest.add_file(persistent_path, persistent_vlob)
        for i in range(1):
            await manifest.add_file(path, vlob)
            await manifest.delete_file(path + final_slash)
            # File not found
            with pytest.raises(ManifestNotFound):
                await manifest.delete_file(path + final_slash)
            # Persistent files
            for persistent_path in ['/persistent', '/test_dir/persistent']:
                await manifest.stat(persistent_path)
        for block_id in block_ids:
            with pytest.raises(BlockNotFound):
                await synchronizer_svc.block_delete(block_id)
        if synchronize:
            for block_id in block_ids:
                await synchronizer_svc.block_read(block_id)
            assert len(manifest.dustbin) == 1
        else:
            assert len(manifest.dustbin) == 0

    @pytest.mark.asyncio
    async def test_delete_not_file(self, manifest):
        await manifest.make_folder('/test')
        with pytest.raises(ManifestError):  # TODO InvalidPath
            await manifest.delete_file('/test')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('synchronize', [True, False])
    async def test_undelete_file(self, synchronizer_svc, manifest, path, synchronize):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await manifest.make_folder('/test_dir')
        if synchronize:
            new_vlob = await file.commit()
            await manifest.add_file(path, new_vlob)
            await manifest.stat(path)
            await manifest.delete_file(path)
            await manifest.remove_folder('/test_dir')
            # Working
            await manifest.undelete_file(new_vlob['id'])
            await manifest.stat(path)
            if path.startswith('/test_dir'):
                await manifest.stat('/test_dir')
            # Not found
            with pytest.raises(ManifestNotFound):
                await manifest.undelete_file(new_vlob['id'])
            # Restore path already used
            await manifest.delete_file(path)
            await manifest.add_file(path, new_vlob)
            with pytest.raises(ManifestError):
                await manifest.undelete_file(new_vlob['id'])
        else:
            await manifest.add_file(path, vlob)
            await manifest.stat(path)
            await manifest.delete_file(path)
            await manifest.remove_folder('/test_dir')
            # Not found
            with pytest.raises(ManifestNotFound):
                await manifest.undelete_file(vlob['id'])

    @pytest.mark.xfail
    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_reencrypt_file(self, synchronizer_svc, core_svc, path, final_slash):
        encoded_content_initial = to_jsonb64('content 1'.encode())
        encoded_content_final = to_jsonb64('content 2'.encode())
        await core_svc.make_folder('/test_dir')
        file_vlob = await core_svc.create_file(path, encoded_content_initial)
        manifest = await core_svc.get_manifest()
        old_vlob = await core_svc.get_properties(path=path)
        assert old_vlob == file_vlob
        await manifest.reencrypt_file(path + final_slash)
        new_vlob = await core_svc.get_properties(path=path)
        for property in old_vlob.keys():
            assert new_vlob[property] != old_vlob[property]
        await file_svc.write(id=new_vlob['id'], version=2, content=encoded_content_final)
        new_file = await synchronizer_svc.read(new_vlob['id'])
        assert new_file == {'content': encoded_content_final, 'version': 2}
        with pytest.raises(ManifestNotFound):
            await manifest.reencrypt_file('/unknown')

    @pytest.mark.asyncio
    async def test_stat(self, synchronizer_svc, manifest):
        with freeze_time('2012-01-01') as frozen_datetime:
            file = await File.create(synchronizer_svc)
            vlob = await file.get_vlob()
            # Create folders
            await manifest.make_folder('/countries')
            await manifest.make_folder('/countries/France')
            await manifest.make_folder('/countries/France/cities')
            await manifest.make_folder('/countries/Belgium')
            await manifest.make_folder('/countries/Belgium/cities')
            # Create multiple files
            await manifest.add_file('/.root', vlob)
            await manifest.add_file('/countries/index', vlob)
            await manifest.add_file('/countries/France/info', vlob)
            await manifest.add_file('/countries/Belgium/info', vlob)
            ret = await manifest.stat('/')
            assert ret == {'type': 'folder', 'items': ['.root', 'countries']}
            for final_slash in ['', '/']:
                ret = await manifest.stat('/countries' + final_slash)
                assert ret == {'type': 'folder', 'items': ['Belgium', 'France', 'index']}
                ret = await manifest.stat('/countries/France/cities' + final_slash)
                assert ret == {'type': 'folder', 'items': []}
                ret = await manifest.stat('/countries/France/info' + final_slash)
                assert ret == {'id': vlob['id'],
                               'type': 'file',
                               'created': frozen_datetime().isoformat(),
                               'updated': frozen_datetime().isoformat(),
                               'size': 0,
                               'version': 1}

        # Test bad list as well
        with pytest.raises(ManifestNotFound):
            await manifest.stat('/dummy')
            await manifest.stat('/countries/dummy')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('parents', ['/', '/parent_1/', '/parent_1/parent_2/'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    @pytest.mark.parametrize('create_parents', [False, True])
    async def test_make_folder(self, manifest, parents, final_slash, create_parents):
        complete_path = parents + 'test_dir' + final_slash
        # Working
        if parents == '/' or create_parents:
            await manifest.make_folder(complete_path, parents=create_parents)
        else:
            # Parents not found
            with pytest.raises(ManifestNotFound):
                await manifest.make_folder(complete_path, parents=create_parents)
        # Already exist
        if create_parents:
            await manifest.make_folder(complete_path, parents=create_parents)
        else:
            with pytest.raises((ManifestError, ManifestNotFound)):
                await manifest.make_folder(complete_path, parents=create_parents)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_remove_folder(self, manifest, final_slash):
        # Working
        await manifest.make_folder('/test_dir')
        await manifest.remove_folder('/test_dir' + final_slash)
        # Not found
        with pytest.raises(ManifestNotFound):
            await manifest.remove_folder('/test_dir')
        with pytest.raises(ManifestNotFound):
            await manifest.remove_folder('/test_dir/')

    @pytest.mark.asyncio
    async def test_cant_remove_root_dir(self, manifest):
        with pytest.raises(ManifestError):
            await manifest.remove_folder('/')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_remove_not_empty_dir(self, manifest, final_slash):
        # Not empty
        await manifest.make_folder('/test_dir')
        await manifest.make_folder('/test_dir/test')
        with pytest.raises(ManifestError):
            await manifest.remove_folder('/test_dir' + final_slash)
        # Empty
        await manifest.remove_folder('/test_dir/test' + final_slash)
        await manifest.remove_folder('/test_dir' + final_slash)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_remove_not_dir(self, manifest, final_slash):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.add_file('/test_dir' + final_slash, file_vlob)
        with pytest.raises(ManifestError):
            await manifest.remove_folder('/test_dir')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    async def test_show_dustbin(self, synchronizer_svc, manifest, path, final_slash):
        file = await File.create(synchronizer_svc)
        vlob = await file.commit()
        # Empty dustbin
        dustbin = await manifest.show_dustbin()
        assert dustbin == []
        await manifest.add_file('/foo', vlob)
        await manifest.delete_file('/foo')
        await manifest.make_folder('/test_dir')
        for i in [1, 2]:
            await manifest.add_file(path, vlob)
            await manifest.delete_file(path)
            # Global dustbin with one additional file
            dustbin = await manifest.show_dustbin()
            assert len(dustbin) == i + 1
            # File in dustbin
            dustbin = await manifest.show_dustbin(path + final_slash)
            assert len(dustbin) == i
            # Not found
            with pytest.raises(ManifestNotFound):
                await manifest.remove_folder('/unknown')

    @pytest.mark.asyncio
    async def test_check_consistency(self, synchronizer_svc, manifest):
        file = await File.create(synchronizer_svc)
        await file.commit()
        good_vlob = await file.get_vlob()
        bad_vlob = {'id': '123', 'key': good_vlob['key'], 'read_trust_seed': '123', 'write_trust_seed': '123'}
        # With good vlobs only
        await manifest.add_file('/foo', good_vlob)
        dump = await manifest.dumps()
        assert await manifest.check_consistency(json.loads(dump)) is True
        await manifest.delete_file('/foo')
        dump = await manifest.dumps()
        assert await manifest.check_consistency(json.loads(dump)) is True
        # With a bad vlob
        await manifest.add_file('/bad', bad_vlob)
        dump = await manifest.dumps()
        assert await manifest.check_consistency(json.loads(dump)) is False
        # TODO check bad block in dustbin


class TestGroupManifest:

    @pytest.mark.asyncio
    async def test_create(self, core_svc):
        manifest = await GroupManifest.create(core_svc)
        vlob = await manifest.get_vlob()
        assert await manifest.get_version() == 0
        new_manifest = await GroupManifest.load(core_svc, **vlob)
        assert await new_manifest.get_version() == 0

    @pytest.mark.asyncio
    async def test_load(self, core_svc, group_manifest):
        vlob = await group_manifest.get_vlob()
        manifest = await GroupManifest.load(core_svc, **vlob)
        assert await manifest.get_version() == 0

    @pytest.mark.asyncio
    async def test_get_vlob(self, core_svc):
        manifest = await GroupManifest.create(core_svc)
        new_vlob = {'id': 'i123',
                    'key': 'DOXnMEmt34+PdAnCLMVgXmKmk3SFjfWxDgx3zeKpbyU=\n',
                    'read_trust_seed': 'rts',
                    'write_trust_seed': 'wts'}
        await manifest.update_vlob(new_vlob)
        vlob = await manifest.get_vlob()
        assert vlob == new_vlob

    @pytest.mark.asyncio
    async def test_update_vlob(self, group_manifest):
        new_vlob = {
            'id': 'i123',
            'key': 'DOXnMEmt34+PdAnCLMVgXmKmk3SFjfWxDgx3zeKpbyU=\n',
            'read_trust_seed': 'rts',
            'write_trust_seed': 'wts'
        }
        await group_manifest.update_vlob(new_vlob)
        assert await group_manifest.get_vlob() == new_vlob

    @pytest.mark.asyncio
    async def test_diff_versions(self, core_svc, group_manifest):
        # Old version (0) and new version (0) of non-commited manifest
        manifest = await GroupManifest.create(core_svc)
        diff = await manifest.diff_versions(0, 0)
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # No old version (use original) and no new version (dump current)
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await group_manifest.add_file('/foo', file_vlob)
        diff = await group_manifest.diff_versions()
        assert diff == {'entries': {'added': {'/foo': file_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {file_vlob['id']: None},
                                     'changed': {},
                                     'removed': {}}}
        # Old version (1) and no new version (dump current)
        await group_manifest.commit()
        await group_manifest.add_file('/bar', file_vlob)
        diff = await group_manifest.diff_versions(1)
        assert diff == {'entries': {'added': {'/bar': file_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # Old version (2) and new version (4)
        await group_manifest.commit()
        await group_manifest.make_folder('/dir')
        await group_manifest.add_file('/dir/foo', file_vlob)
        await group_manifest.commit()
        await group_manifest.add_file('/dir/bar', file_vlob)
        await group_manifest.commit()
        await group_manifest.add_file('/dir/last', file_vlob)
        diff = await group_manifest.diff_versions(2, 4)
        assert diff == {'entries': {'added': {'/dir': None,
                                              '/dir/bar': file_vlob,
                                              '/dir/foo': file_vlob},
                                    'changed': {},
                                    'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # Old version (4) and new version (2)
        diff = await group_manifest.diff_versions(4, 2)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir': None,
                                                '/dir/bar': file_vlob,
                                                '/dir/foo': file_vlob}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # No old version (use original) and new version (3)
        diff = await group_manifest.diff_versions(None, 3)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir/bar': file_vlob}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}

    @pytest.mark.asyncio
    async def test_reload_not_consistent(self, core_svc, group_manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.commit()
        with pytest.raises(ManifestError):
            await group_manifest.reload(reset=True)

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_with_reset_and_new_version(self, core_svc, group_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.commit()
        vlob = await group_manifest.get_vlob()
        group_manifest_2 = await GroupManifest.load(core_svc, **vlob)
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await group_manifest_2.add_file('/bar', file_vlob_2)
        assert await group_manifest_2.get_version() == 0
        await group_manifest_2.reload(reset=True)
        assert await group_manifest_2.get_version() == 2
        diff = await group_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await group_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_with_reset_no_new_version(self, core_svc, group_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.commit()
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await group_manifest.add_file('/bar', file_vlob_2)
        assert await group_manifest.get_version() == 1
        await group_manifest.reload(reset=True)
        assert await group_manifest.get_version() == 1
        diff = await group_manifest.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await group_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_without_reset_and_new_version(self, core_svc, group_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.commit()
        vlob = await group_manifest.get_vlob()
        group_manifest_2 = await GroupManifest.create(core_svc, **vlob)
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await group_manifest_2.add_file('/bar', file_vlob_2)
        assert await group_manifest_2.get_version() == 0
        await group_manifest_2.reload(reset=False)
        assert await group_manifest_2.get_version() == 1
        diff = await group_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'versions': {'added': {file_vlob_2['id']: 1}, 'changed': {}, 'removed': {}}}
        manifest = await group_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_reload_without_reset_and_no_new_version(self, core_svc, group_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.commit()
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await group_manifest.add_file('/bar', file_vlob_2)
        assert await group_manifest.get_version() == 2
        await group_manifest.reload(reset=False)
        assert await group_manifest.get_version() == 2
        diff = await group_manifest.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'versions': {'added': {file_vlob_2['id']: 1}, 'changed': {}, 'removed': {}}}
        manifest = await group_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_commit(self, core_svc):
        # Create group manifest
        group_manifest = await GroupManifest.create(core_svc)
        # Save firt time
        await group_manifest.commit()
        manifest_vlob = await group_manifest.get_vlob()
        assert manifest_vlob['id'] is not None
        assert manifest_vlob['key'] is not None
        assert manifest_vlob['read_trust_seed'] is not None
        assert manifest_vlob['write_trust_seed'] is not None
        # Modify and commit
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.commit()
        assert await group_manifest.get_vlob() == manifest_vlob
        assert await group_manifest.get_version() == 2
        # Save without modifications
        await group_manifest.commit()
        assert await group_manifest.get_version() == 2
        # TODO assert called methods

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reencrypt(self, core_svc, group_manifest):
        foo_file = await File.create(core_svc.synchronizer)
        foo_vlob = await foo_file.get_vlob()
        bar_file = await File.create(core_svc.synchronizer)
        bar_vlob = await bar_file.get_vlob()
        await group_manifest.add_file('/foo', foo_vlob)
        await group_manifest.add_file('/bar', bar_vlob)
        await group_manifest.delete_file('/bar')
        await group_manifest.commit()
        assert await group_manifest.get_version() == 1
        dump = await group_manifest.dumps()
        dump = json.loads(dump)
        old_id = group_manifest.id
        old_key = group_manifest.key
        old_read_trust_seed = group_manifest.read_trust_seed
        old_write_trust_seed = group_manifest.write_trust_seed
        await group_manifest.reencrypt()
        await group_manifest.commit()
        assert group_manifest.id != old_id
        assert group_manifest.key != old_key
        assert group_manifest.read_trust_seed != old_read_trust_seed
        assert group_manifest.write_trust_seed != old_write_trust_seed
        assert await group_manifest.get_version() == 1
        new_group_manifest = await GroupManifest.create(core_svc,
                                                        group_manifest.id,
                                                        group_manifest.key,
                                                        group_manifest.read_trust_seed,
                                                        group_manifest.write_trust_seed)
        new_group_manifest_vlob = await new_group_manifest.get_vlob()
        await core_svc.import_group_vlob('new_foo_community', new_group_manifest_vlob)
        await new_group_manifest.reload(reset=True)
        assert await new_group_manifest.get_version() == 1
        new_dump = await new_group_manifest.dumps()
        new_dump = json.loads(new_dump)
        for file_path, entry in dump['entries'].items():
            if entry['id']:
                for property in entry.keys():
                    assert entry[property] != new_dump['entries'][file_path][property]
        for index, entry in enumerate(dump['dustbin']):
            for property in entry.keys():
                if property in ['path', 'removed_date']:
                    assert entry[property] == new_dump['dustbin'][index][property]
                else:
                    assert entry[property] != new_dump['dustbin'][index][property]

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_restore_manifest(self, core_svc, group_manifest):

        def encode_content(content):
            return to_jsonb64(content.encode())

        dust_file = await File.create(core_svc.synchronizer)
        tmp_file = await File.create(core_svc.synchronizer)
        foo_file = await File.create(core_svc.synchronizer)
        bar_file = await File.create(core_svc.synchronizer)
        baz_file = await File.create(core_svc.synchronizer)
        await dust_file.write(encode_content('v1'), 0)
        await tmp_file.write(encode_content('v1'), 0)
        await foo_file.write(encode_content('v1'), 0)
        await bar_file.write(encode_content('v1'), 0)
        await baz_file.write(encode_content('v1'), 0)
        await dust_file.commit()
        await tmp_file.commit()
        await foo_file.commit()
        await bar_file.commit()
        await baz_file.commit()
        dust_vlob = await dust_file.get_vlob()
        tmp_vlob = await tmp_file.get_vlob()
        foo_vlob = await foo_file.get_vlob()
        bar_vlob = await bar_file.get_vlob()
        baz_vlob = await baz_file.get_vlob()
        # Restore dirty manifest with version 1
        assert await group_manifest.get_version() == 0
        await group_manifest.add_file('/tmp', tmp_vlob)
        await group_manifest.commit()
        assert await group_manifest.get_version() == 1
        await group_manifest.restore()
        assert await group_manifest.get_version() == 1
        children = await group_manifest.stat('/')
        assert sorted(children.keys()) == []
        # Restore previous version
        await group_manifest.add_file('/foo', foo_vlob)
        await group_manifest.add_file('/dust', dust_vlob)
        await group_manifest.delete_file('/dust')
        await group_manifest.commit()
        await group_manifest.add_file('/bar', bar_vlob)
        await foo_file.write(encode_content('v2'))
        await group_manifest.commit()
        await group_manifest.add_file('/baz', baz_vlob)
        await group_manifest.restore_file(dust_vlob['id'])
        await dust_file.write(encode_content('v2'))
        await foo_file.write(encode_content('v3'))
        await bar_file.write(encode_content('v2'))
        await group_manifest.commit()
        assert await group_manifest.get_version() == 4
        await group_manifest.restore()
        assert await group_manifest.get_version() == 5
        children = await group_manifest.stat('/')
        assert sorted(children.keys()) == ['bar', 'foo']
        dust_file = await File.load(core_svc.synchronizer, **dust_vlob)
        assert await dust_file.read() == encode_content('v1')
        assert dust_file.get_version() == 3
        bar_file = await File.load(core_svc.synchronizer, **bar_vlob)
        assert await bar_file.read() == encode_content('v1')
        assert bar_file.get_version() == 3
        foo_file = await File.load(core_svc.synchronizer, **foo_vlob)
        assert await foo_file.read() == encode_content('v2')
        assert foo_file.get_version() == 4
        # Restore old version
        await group_manifest.restore(4)
        assert await group_manifest.get_version() == 6
        children = await group_manifest.stat('/')
        assert sorted(children.keys()) == ['bar', 'baz', 'dust', 'foo']
        dust_file = await File.load(core_svc.synchronizer, **dust_vlob)
        assert await dust_file.read() == encode_content('v2')
        assert dust_file.get_version() == 4
        bar_file = await File.load(core_svc.synchronizer, **bar_vlob)
        assert await bar_file.read() == encode_content('v2')
        assert bar_file.get_version() == 4
        baz_file = await File.load(core_svc.synchronizer, **baz_vlob)
        assert await baz_file.read() == encode_content('v1')
        assert baz_file.get_version() == 1
        foo_file = await File.load(core_svc.synchronizer, **foo_vlob)
        assert await foo_file.read() == encode_content('v3')
        assert foo_file.get_version() == 5
        # Bad version
        with pytest.raises(ManifestError):
            await group_manifest.restore(10)
        # Restore not commited manifest
        new_group_manifest = await GroupManifest.create(group_manifest.core)
        with pytest.raises(ManifestError):
            await new_group_manifest.restore()


class TestUserManifest:

    @pytest.mark.asyncio
    async def test_load(self, core_svc, user_manifest):
        manifest = await UserManifest.load(core_svc)
        assert await manifest.get_version() == 0

    @pytest.mark.asyncio
    async def test_diff_versions(self, core_svc, user_manifest):
        # Old version (0) and new version (0) of non-commited manifest
        manifest = await UserManifest.load(core_svc)
        diff = await manifest.diff_versions(0, 0)
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # No old version (use original) and no new version (dump current)
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        group_manifest = await GroupManifest.create(core_svc)
        await group_manifest.commit()
        group_vlob = await group_manifest.get_vlob()
        await user_manifest.import_group_vlob('share', group_vlob)
        diff = await user_manifest.diff_versions()
        await user_manifest.remove_group('share')
        assert diff == {'entries': {'added': {'/foo': file_vlob}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {'share': group_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {file_vlob['id']: 1}, 'changed': {}, 'removed': {}}}
        # Old version (1) and no new version (dump current)
        await user_manifest.commit()
        await user_manifest.add_file('/bar', file_vlob)
        diff = await user_manifest.diff_versions(1)
        assert diff == {'entries': {'added': {'/bar': file_vlob}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # Old version (2) and new version (4)
        await user_manifest.commit()
        await user_manifest.make_folder('/dir')
        await user_manifest.add_file('/dir/foo', file_vlob)
        await user_manifest.commit()
        await user_manifest.add_file('/dir/bar', file_vlob)
        await user_manifest.commit()
        await user_manifest.add_file('/dir/last', file_vlob)
        diff = await user_manifest.diff_versions(2, 4)
        assert diff == {'entries': {'added': {'/dir': None,
                                              '/dir/bar': file_vlob,
                                              '/dir/foo': file_vlob},
                                    'changed': {},
                                    'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # Old version (4) and new version (2)
        diff = await user_manifest.diff_versions(4, 2)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir': None,
                                                '/dir/bar': file_vlob,
                                                '/dir/foo': file_vlob}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # No old version (use original) and new version (3)
        diff = await user_manifest.diff_versions(None, 3)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir/bar': file_vlob}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}

    @pytest.mark.asyncio
    async def test_dumps_current_manifest(self, core_svc, user_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        group_manifest = await GroupManifest.create(core_svc)
        await group_manifest.commit()
        group_vlob = await group_manifest.get_vlob()
        await user_manifest.import_group_vlob('foo_community', group_vlob)
        dump = await user_manifest.dumps(original_manifest=False)
        dump = json.loads(dump)
        group_vlob = dump['groups']['foo_community']
        assert dump == {'entries': {'/': None,
                                    '/foo': file_vlob},
                        'dustbin': [],
                        'groups': {'foo_community': group_vlob},
                        'versions': {file_vlob['id']: 1}}

    @pytest.mark.asyncio
    async def test_dumps_original_manifest(self, core_svc, user_manifest_with_group):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest_with_group.add_file('/foo', file_vlob)
        dump = await user_manifest_with_group.dumps(original_manifest=True)
        dump = json.loads(dump)
        group_vlob = dump['groups']['foo_community']
        assert dump == {'entries': {'/': None},
                        'dustbin': [],
                        'groups': {'foo_community': group_vlob},
                        'versions': {}}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'share'])
    async def test_get_group_vlobs(self, core_svc, user_manifest_with_group, group):
        await user_manifest_with_group.create_group_manifest('share')
        group_vlobs = await user_manifest_with_group.get_group_vlobs(group)
        if group:
            keys = [group]
        else:
            keys = ['foo_community', 'share']
        assert keys == sorted(list(group_vlobs.keys()))
        for group_vlob in group_vlobs.values():
            keys = ['id', 'read_trust_seed', 'write_trust_seed', 'key']
            assert sorted(keys) == sorted(group_vlob.keys())
        # Not found
        with pytest.raises(ManifestNotFound):
            await user_manifest_with_group.get_group_vlobs('unknown')

    @pytest.mark.asyncio
    async def test_get_group_manifest(self, user_manifest_with_group):
        group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        assert isinstance(group_manifest, GroupManifest)
        # Not found
        with pytest.raises(ManifestNotFound):
            await user_manifest_with_group.get_group_manifest('unknown')

    @pytest.mark.asyncio
    async def test_reencrypt_group_manifest(self, user_manifest_with_group):
        group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        await user_manifest_with_group.reencrypt_group_manifest('foo_community')
        new_group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        assert group_manifest.get_vlob() != new_group_manifest.get_vlob()
        assert isinstance(group_manifest, GroupManifest)
        # Not found
        with pytest.raises(ManifestNotFound):
            await user_manifest_with_group.reencrypt_group_manifest('unknown')

    @pytest.mark.asyncio
    async def test_create_group_manifest(self, user_manifest):
        with pytest.raises(ManifestNotFound):
            await user_manifest.get_group_manifest('share')
        await user_manifest.create_group_manifest('share')
        group_manifest = await user_manifest.get_group_manifest('share')
        assert isinstance(group_manifest, GroupManifest)
        # Already exists
        with pytest.raises(ManifestError):
            await user_manifest.create_group_manifest('share')

    @pytest.mark.asyncio
    async def test_import_group_vlob(self, core_svc, user_manifest):
        group_manifest = await GroupManifest.create(core_svc)
        await group_manifest.commit()
        vlob = await group_manifest.get_vlob()
        await user_manifest.import_group_vlob('share', vlob)
        retrieved_manifest = await user_manifest.get_group_manifest('share')
        assert await retrieved_manifest.get_vlob() == vlob
        await group_manifest.reencrypt()
        new_vlob = await group_manifest.get_vlob()
        await user_manifest.import_group_vlob('share', new_vlob)
        retrieved_manifest = await user_manifest.get_group_manifest('share')
        assert await retrieved_manifest.get_vlob() == new_vlob

    @pytest.mark.asyncio
    async def test_remove_group(self, user_manifest_with_group):
        await user_manifest_with_group.remove_group('foo_community')
        with pytest.raises(ManifestNotFound):
            await user_manifest_with_group.remove_group('foo_community')

    @pytest.mark.asyncio
    async def test_reload_not_consistent(self, core_svc, user_manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.commit()
        with pytest.raises(ManifestError):
            await user_manifest.reload(reset=True)

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_with_reset_and_new_version(self, core_svc, user_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.commit()
        user_manifest_2 = await UserManifest.load(core_svc, JOHN_DOE_IDENTITY, JOHN_DOE_PRIVATE_KEY)
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await user_manifest_2.add_file('/bar', file_vlob_2)
        assert await user_manifest_2.get_version() == 1
        await user_manifest_2.reload(reset=True)
        assert await user_manifest_2.get_version() == 2
        diff = await user_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_with_reset_no_new_version(self, core_svc, user_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.commit()
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await user_manifest.add_file('/bar', file_vlob_2)
        assert await user_manifest.get_version() == 2
        await user_manifest.reload(reset=True)
        assert await user_manifest.get_version() == 2
        diff = await user_manifest.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_without_reset_and_new_version(self, core_svc, user_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.commit()
        user_manifest_2 = await UserManifest.load(core_svc, JOHN_DOE_IDENTITY, JOHN_DOE_PRIVATE_KEY)
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await user_manifest_2.add_file('/bar', file_vlob_2)
        assert await user_manifest_2.get_version() == 0
        await user_manifest_2.reload(reset=False)
        assert await user_manifest_2.get_version() == 2
        diff = await user_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'versions': {'added': {file_vlob_2['id']: 1}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_reload_without_reset_and_no_new_version(self, file_svc, user_manifest):
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.commit()
        file_2 = await File.create(core_svc.synchronizer)
        file_vlob_2 = await file_2.get_vlob()
        await user_manifest.add_file('/bar', file_vlob_2)
        assert await user_manifest.get_version() == 2
        await user_manifest.reload(reset=False)
        assert await user_manifest.get_version() == 2
        diff = await user_manifest.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'versions': {'added': {file_vlob_2['id']: 1}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_commit(self, core_svc, user_manifest):
        # Modify and save
        file = await File.create(core_svc.synchronizer)
        file_vlob = await file.get_vlob()
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.commit()
        assert await user_manifest.get_version() == 1
        # Save without modifications
        await user_manifest.commit()
        assert await user_manifest.get_version() == 1
        # TODO assert called methods

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_restore_manifest(self, core_svc, user_manifest):
        dust_file = await File.create(core_svc.synchronizer)
        tmp_file = await File.create(core_svc.synchronizer)
        foo_file = await File.create(core_svc.synchronizer)
        bar_file = await File.create(core_svc.synchronizer)
        baz_file = await File.create(core_svc.synchronizer)
        await dust_file.write(to_jsonb64('v1'.encode()), 0)
        await tmp_file.write(to_jsonb64('v1'.encode()), 0)
        await foo_file.write(to_jsonb64('v1'.encode()), 0)
        await bar_file.write(to_jsonb64('v1'.encode()), 0)
        await baz_file.write(to_jsonb64('v1'.encode()), 0)
        await dust_file.commit()
        await tmp_file.commit()
        await foo_file.commit()
        await bar_file.commit()
        await baz_file.commit()
        dust_vlob = await dust_file.get_vlob()
        tmp_vlob = await tmp_file.get_vlob()
        foo_vlob = await foo_file.get_vlob()
        bar_vlob = await bar_file.get_vlob()
        baz_vlob = await baz_file.get_vlob()
        # Restore dirty manifest with version 1
        assert await user_manifest.get_version() == 0
        await user_manifest.add_file('/tmp', tmp_vlob)
        await user_manifest.commit()
        assert await user_manifest.get_version() == 1
        await user_manifest.restore()
        assert await user_manifest.get_version() == 1
        children = await user_manifest.stat('/')
        assert sorted(children.keys()) == []
        # Restore previous version
        await user_manifest.add_file('/foo', foo_vlob)
        await user_manifest.add_file('/dust', dust_vlob)
        await user_manifest.delete_file('/dust')
        await user_manifest.commit()
        await user_manifest.add_file('/bar', bar_vlob)
        await foo_file.write(to_jsonb64('v2'.encode()))
        await user_manifest.commit()
        await user_manifest.add_file('/baz', baz_vlob)
        await user_manifest.restore_file(dust_vlob['id'])
        await dust_file.write(to_jsonb64('v2'.encode()))
        await foo_file.write(to_jsonb64('v3'.encode()))
        await bar_file.write(to_jsonb64('v2'.encode()))
        await user_manifest.commit()
        assert await user_manifest.get_version() == 4
        await user_manifest.restore()
        assert await user_manifest.get_version() == 5
        children = await user_manifest.stat('/')
        assert sorted(children.keys()) == ['bar', 'foo']
        dust_file = await File.load(core_svc.synchronizer, **dust_vlob)
        assert await dust_file.read() == to_jsonb64('v1'.encode())
        assert dust_file.get_version() == 3
        bar_file = await File.load(core_svc.synchronizer, **bar_vlob)
        assert await bar_file.read() == to_jsonb64('v1'.encode())
        assert bar_file.get_version() == 3
        foo_file = await File.load(core_svc.synchronizer, **foo_vlob)
        assert await foo_file.read() == to_jsonb64('v2'.encode())
        assert foo_file.get_version() == 4
        # Restore old version
        await user_manifest.restore(4)
        assert await user_manifest.get_version() == 6
        children = await user_manifest.stat('/')
        assert sorted(children.keys()) == ['bar', 'baz', 'dust', 'foo']
        dust_file = await File.load(core_svc.synchronizer, **dust_vlob)
        assert await dust_file.read() == to_jsonb64('v2'.encode())
        assert dust_file.get_version() == 4
        bar_file = await File.load(core_svc.synchronizer, **bar_vlob)
        assert await bar_file.read() == to_jsonb64('v2'.encode())
        assert bar_file.get_version() == 4
        baz_file = await File.load(core_svc.synchronizer, **baz_vlob)
        assert await baz_file.read() == to_jsonb64('v1'.encode())
        assert baz_file.get_version() == 1
        foo_file = await File.load(core_svc.synchronizer, **foo_vlob)
        assert await foo_file.read() == to_jsonb64('v3'.encode())
        assert foo_file.get_version() == 5
        # Bad version
        with pytest.raises(ManifestError):
            await user_manifest.restore(10)
        # Restore not commited manifest
        vlob = await user_manifest.get_vlob()
        new_user_manifest = await UserManifest.load(user_manifest.service, vlob['id'])
        with pytest.raises(ManifestError):
            await new_user_manifest.restore()

    @pytest.mark.asyncio
    async def test_check_consistency(self, core_svc, user_manifest):
        file = await File.create(core_svc.synchronizer)
        await file.commit()
        good_vlob = await file.get_vlob()
        encryptor = generate_sym_key()
        bad_vlob = {'id': '123', 'key': to_jsonb64(encryptor.key), 'read_trust_seed': '123', 'write_trust_seed': '123'}
        # With good vlobs only
        await user_manifest.add_file('/foo', good_vlob)
        await user_manifest.commit()
        await user_manifest.delete_file('/foo')
        await user_manifest.add_file('/bar', good_vlob)
        dump = await user_manifest.dumps()
        assert await user_manifest.check_consistency(json.loads(dump)) is True
        # With a bad vlob
        group_manifest = await GroupManifest.create(core_svc)
        await group_manifest.update_vlob(bad_vlob)
        user_manifest.group_manifests['share'] = group_manifest
        dump = await user_manifest.dumps()
        assert await user_manifest.check_consistency(json.loads(dump)) is False
        await user_manifest.remove_group('share')
        dump = await user_manifest.dumps()
        assert await user_manifest.check_consistency(json.loads(dump)) is True
