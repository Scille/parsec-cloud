from base64 import encodebytes
from copy import deepcopy
import json
from os import path

from freezegun import freeze_time
import gnupg
import pytest

from parsec.core import (CryptoService, FileService, IdentityService, GNUPGPubKeysService,
                         MetaBlockService, MockedBackendAPIService, MockedBlockService,
                         ShareService, UserManifestService)
from parsec.core.user_manifest_service import (GroupManifest, Manifest, UserManifest,
                                               UserManifestError, UserManifestNotFound)
from parsec.server import BaseServer


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def manifest(event_loop, user_manifest_svc):
    manifest = Manifest(user_manifest_svc)
    return manifest


@pytest.fixture
def group_manifest(event_loop, user_manifest_svc):
    manifest = GroupManifest(user_manifest_svc)
    event_loop.run_until_complete(manifest.save())
    return manifest


@pytest.fixture
def user_manifest(event_loop, user_manifest_svc):
    manifest = UserManifest(user_manifest_svc, '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9')
    event_loop.run_until_complete(manifest.reload(reset=True))
    return manifest


@pytest.fixture
def user_manifest_with_group(event_loop, share_svc, user_manifest):
    event_loop.run_until_complete(share_svc.group_create('foo_community'))
    event_loop.run_until_complete(user_manifest.reload(reset=True))
    return user_manifest


@pytest.fixture
def file_svc():
    return FileService()


@pytest.fixture
def identity_svc():
    return IdentityService()


@pytest.fixture
def share_svc():
    return ShareService()


@pytest.fixture
def user_manifest_svc(event_loop, file_svc, identity_svc, share_svc):
    identity = '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'
    service = UserManifestService()
    block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
    crypto_service = CryptoService()
    crypto_service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
    server = BaseServer()
    server.register_service(service)
    server.register_service(block_service)
    server.register_service(crypto_service)
    server.register_service(file_svc)
    server.register_service(identity_svc)
    server.register_service(share_svc)
    server.register_service(GNUPGPubKeysService())
    server.register_service(MockedBackendAPIService())
    event_loop.run_until_complete(server.bootstrap_services())
    event_loop.run_until_complete(identity_svc.load_identity(identity=identity))
    event_loop.run_until_complete(service.load_user_manifest())
    yield service
    event_loop.run_until_complete(server.teardown_services())


class TestManifest:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('payload', [
        {'id': 'i123', 'key': 'k123', 'read_trust_seed': 'r123', 'write_trust_seed': 'w123'},
        {'id': None, 'key': None, 'read_trust_seed': None, 'write_trust_seed': None}])
    async def test_init(self, user_manifest_svc, payload):
        manifest = GroupManifest(user_manifest_svc, **payload)
        assert await manifest.get_vlob() == payload

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_is_dirty(self, manifest):
        assert await manifest.is_dirty() is False
        vlob = {'id': 'i123', 'key': 'k123', 'read_trust_seed': 'r123', 'write_trust_seed': 'w123'}
        await manifest.add_file('/foo', vlob)
        assert await manifest.is_dirty() is True
        await manifest.delete_file('/foo')
        assert await manifest.is_dirty() is True

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
                'dustbin': [vlob_5, vlob_6, vlob_7]
            },
            {
                'entries': {'/a': vlob_1, '/b': vlob_2, '/c': vlob_3},
                'dustbin': [vlob_5, vlob_6, vlob_7]
            }
        )
        assert diff == {
            'entries': {'added': {}, 'changed': {}, 'removed': {}},
            'dustbin': {'added': [], 'removed': []}
        }
        # Not empty diff
        diff = await manifest.diff(
            {
                'entries': {'/a': vlob_1, '/b': vlob_2, '/c': vlob_3},
                'dustbin': [vlob_5, vlob_6, vlob_7]
            },
            {
                'entries': {'/a': vlob_6, '/b': vlob_2, '/d': vlob_4},
                'dustbin': [vlob_7, vlob_8, vlob_9]
            }
        )
        assert diff == {
            'entries': {'added': {'/d': vlob_4},
                        'changed': {'/a': (vlob_1, vlob_6)},
                        'removed': {'/c': vlob_3}},
            'dustbin': {'added': [vlob_8, vlob_9],
                        'removed': [vlob_5, vlob_6]}
        }

    @pytest.mark.asyncio
    async def test_patch(self, manifest):
        # TODO too intrusive?
        vlob_1 = {'id': 'vlob_1'}
        vlob_2 = {'id': 'vlob_2'}
        vlob_3 = {'id': 'vlob_3'}
        vlob_4 = {'id': 'vlob_4'}
        vlob_5 = {'id': 'vlob_5'}
        vlob_6 = {'id': 'vlob_6'}
        vlob_7 = {'id': 'vlob_7'}
        vlob_8 = {'id': 'vlob_8'}
        vlob_9 = {'id': 'vlob_9'}
        manifest.original_manifest = {
            'entries': {'/A-B-C': vlob_1,  # Conflict between B and C, save C-conflict
                        '/A-B-nil': vlob_2,  # Recover B, save B-deleted
                        '/A-A-nil': vlob_4,  # Delete A
                        '/A-A-A': vlob_5,
                        '/A-nil-A': vlob_6,
                        '/A-nil-B': vlob_7},  # Recover B, save B-recreated
            'dustbin': [vlob_4, vlob_5, vlob_6]
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
            'dustbin': [vlob_5, vlob_6, vlob_7]
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
            'dustbin': [{'id': 'vlob_6'}, {'id': 'vlob_7'}, {'id': 'vlob_8'}]}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('payload', [
        {'id': 'i123', 'key': 'k123', 'read_trust_seed': 'r123', 'write_trust_seed': 'w123'},
        {'id': None, 'key': None, 'read_trust_seed': None, 'write_trust_seed': None}])
    async def test_get_vlob(self, payload):
        manifest = GroupManifest(user_manifest_svc, **payload)
        assert await manifest.get_vlob() == payload

    @pytest.mark.asyncio
    async def test_get_version(self, manifest):
        assert await manifest.get_version() == 0

    @pytest.mark.asyncio
    async def test_dumps_current_manifest(self, file_svc, manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await manifest.add_file('/foo', file_vlob)
        dump = await manifest.dumps(original_manifest=False)
        dump = json.loads(dump)
        assert dump == {'entries': {'/': {'id': None,
                                          'key': None,
                                          'read_trust_seed': None,
                                          'write_trust_seed': None},
                                    '/foo': file_vlob},
                        'dustbin': []}

    @pytest.mark.asyncio
    async def test_dumps_original_manifest(self, file_svc, manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await manifest.add_file('/foo', file_vlob)
        dump = await manifest.dumps(original_manifest=True)
        dump = json.loads(dump)
        assert dump == {'entries': {'/': {'id': None,
                                          'key': None,
                                          'read_trust_seed': None,
                                          'write_trust_seed': None}
                                    },
                        'dustbin': []}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_add_file(self, manifest, path):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.add_file('/test', file_vlob)
        with pytest.raises(UserManifestError):
            await manifest.add_file('/test', file_vlob)

    @pytest.mark.asyncio
    async def test_rename_file(self, manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.add_file('/test', file_vlob)
        await manifest.rename_file('/test', '/foo')
        with pytest.raises(UserManifestNotFound):
            await manifest.list_dir('/test')
        await manifest.list_dir('/foo')

    @pytest.mark.asyncio
    async def test_rename_file_and_source_not_exists(self, manifest):
        with pytest.raises(UserManifestNotFound):
            await manifest.rename_file('/test', '/foo')

    @pytest.mark.asyncio
    async def test_rename_file_and_target_exists(self, manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.add_file('/test', file_vlob)
        await manifest.add_file('/foo', file_vlob)
        with pytest.raises(UserManifestError):
            await manifest.rename_file('/test', '/foo')
        await manifest.list_dir('/test')
        await manifest.list_dir('/foo')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_delete_file(self, manifest, path):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.make_dir('/test_dir')
        for persistent_path in ['/persistent', '/test_dir/persistent']:
            await manifest.add_file(persistent_path, file_vlob)
        for i in [1, 2]:
            await manifest.add_file(path, file_vlob)
            await manifest.delete_file(path)
            # File not found
            with pytest.raises(UserManifestNotFound):
                await manifest.delete_file(path)
            # Persistent files
            for persistent_path in ['/persistent', '/test_dir/persistent']:
                await manifest.list_dir(persistent_path)

    @pytest.mark.asyncio
    async def test_delete_not_file(self, manifest):
        await manifest.make_dir('/test')
        with pytest.raises(UserManifestError):
            await manifest.delete_file('/test')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_restore_file(self, manifest, path):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.make_dir('/test_dir')
        await manifest.add_file(path, file_vlob)
        await manifest.list_dir(path)
        await manifest.delete_file(path)
        # Working
        await manifest.restore_file(file_vlob['id'])
        await manifest.list_dir(path)
        # Not found
        with pytest.raises(UserManifestNotFound):
            await manifest.restore_file(file_vlob['id'])
        # Restore path already used
        await manifest.delete_file(path)
        await manifest.add_file(path, file_vlob)
        with pytest.raises(UserManifestError):
            await manifest.restore_file(file_vlob['id'])

    @pytest.mark.asyncio
    async def test_list_dir(self, manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        # Create folders
        await manifest.make_dir('/countries')
        await manifest.make_dir('/countries/France')
        await manifest.make_dir('/countries/France/cities')
        await manifest.make_dir('/countries/Belgium')
        await manifest.make_dir('/countries/Belgium/cities')
        # Create multiple files
        await manifest.add_file('/.root', file_vlob)
        await manifest.add_file('/countries/index', file_vlob)
        await manifest.add_file('/countries/France/info', file_vlob)
        await manifest.add_file('/countries/Belgium/info', file_vlob)

        # Finally do some lookup
        async def assert_ls(path, expected_children):
            returned_children = await manifest.list_dir(path, children=True)
            assert sorted(expected_children) == sorted(returned_children.keys())
            keys = ['id', 'key', 'read_trust_seed', 'write_trust_seed']
            for children in returned_children.values():
                assert keys == sorted(children.keys())
            file = await manifest.list_dir(path, children=False)
            assert keys == sorted(file.keys())

        await assert_ls('/', ['.root', 'countries'])
        await assert_ls('/countries', ['index', 'Belgium', 'France'])
        await assert_ls('/countries/France/cities', [])
        await assert_ls('/countries/France/info', [])

        # Test bad list as well
        with pytest.raises(UserManifestNotFound):
            await manifest.list_dir('/dummy')
            await manifest.list_dir('/countries/dummy')

    @pytest.mark.asyncio
    async def test_make_dir(self, manifest):
        # Working
        await manifest.make_dir('/test_dir')
        # Already exist
        with pytest.raises(UserManifestError):
            await manifest.make_dir('/test_dir')

    @pytest.mark.asyncio
    async def test_remove_dir(self, manifest):
        # Working
        await manifest.make_dir('/test_dir')
        await manifest.remove_dir('/test_dir')
        # Not found
        with pytest.raises(UserManifestNotFound):
            await manifest.remove_dir('/test_dir')

    @pytest.mark.asyncio
    async def test_cant_remove_root_dir(self, manifest):
        with pytest.raises(UserManifestError):
            await manifest.remove_dir('/')

    @pytest.mark.asyncio
    async def test_remove_not_empty_dir(self, manifest):
        # Not empty
        await manifest.make_dir('/test_dir')
        await manifest.make_dir('/test_dir/test')
        with pytest.raises(UserManifestError):
            await manifest.remove_dir('/test_dir')
        # Empty
        await manifest.remove_dir('/test_dir/test')
        await manifest.remove_dir('/test_dir')

    @pytest.mark.asyncio
    async def test_remove_not_dir(self, manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await manifest.add_file('/test_dir', file_vlob)
        with pytest.raises(UserManifestError):
            await manifest.remove_dir('/test_dir')

    @pytest.mark.asyncio
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_show_dustbin(self, manifest, path):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        # Empty dustbin
        dustbin = await manifest.show_dustbin()
        assert dustbin == []
        await manifest.add_file('/foo', file_vlob)
        await manifest.delete_file('/foo')
        await manifest.make_dir('/test_dir')
        for i in [1, 2]:
            await manifest.add_file(path, file_vlob)
            await manifest.delete_file(path)
            # Global dustbin with one additional file
            dustbin = await manifest.show_dustbin()
            assert len(dustbin) == i + 1
            # File in dustbin
            dustbin = await manifest.show_dustbin(path)
            assert len(dustbin) == i
            # Not found
            with pytest.raises(UserManifestNotFound):
                await manifest.remove_dir('/unknown')

    @pytest.mark.asyncio
    async def test_check_consistency(self, file_svc, manifest):
        content = encodebytes('foo'.encode()).decode()
        good_vlob = await file_svc.create(content)
        bad_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        # With good vlobs only
        await manifest.add_file('/foo', good_vlob)
        await manifest.delete_file('/foo')
        await manifest.add_file('/bar', good_vlob)
        dump = await manifest.dumps()
        assert await manifest.check_consistency(json.loads(dump)) is True
        # With a bad vlob
        await manifest.add_file('/bad', bad_vlob)
        dump = await manifest.dumps()
        assert await manifest.check_consistency(json.loads(dump)) is False
        await manifest.delete_file('/bad')
        dump = await manifest.dumps()
        assert await manifest.check_consistency(json.loads(dump)) is False


class TestGroupManifest:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('payload', [
        {'id': 'i123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'},
        {'id': None, 'key': None, 'read_trust_seed': None, 'write_trust_seed': None}])
    async def test_init(self, user_manifest_svc, payload):
        manifest = GroupManifest(user_manifest_svc, **payload)
        assert await manifest.get_vlob() == payload

    @pytest.mark.asyncio
    async def test_update_vlob(self, group_manifest):
        new_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await group_manifest.update_vlob(new_vlob)
        assert await group_manifest.get_vlob() == new_vlob

    @pytest.mark.asyncio
    async def test_diff_versions(self, user_manifest_svc, group_manifest):
        # Old version (0) and new version (0) of non-saved manifest
        manifest = GroupManifest(group_manifest.service)
        diff = await manifest.diff_versions(0, 0)
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # No old version (use original) and no new version (dump current)
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await group_manifest.add_file('/foo', file_vlob)
        diff = await group_manifest.diff_versions()
        assert diff == {'entries': {'added': {'/foo': file_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # Old version (2) and no new version (dump current)
        await group_manifest.save()
        await group_manifest.add_file('/bar', file_vlob)
        diff = await group_manifest.diff_versions(2)
        assert diff == {'entries': {'added': {'/bar': file_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # Old version (3) and new version (5)
        await group_manifest.save()
        await group_manifest.add_file('/dir/foo', file_vlob)
        await group_manifest.save()
        await group_manifest.add_file('/dir/bar', file_vlob)
        await group_manifest.save()
        await group_manifest.add_file('/dir/last', file_vlob)
        diff = await group_manifest.diff_versions(3, 5)
        assert diff == {'entries': {'added': {'/dir/bar': file_vlob, '/dir/foo': file_vlob},
                                    'changed': {},
                                    'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # Old version (6) and new version (4)
        diff = await group_manifest.diff_versions(5, 3)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir/bar': file_vlob, '/dir/foo': file_vlob}},
                        'dustbin': {'removed': [], 'added': []}}
        # No old version (use original) and new version (4)
        diff = await group_manifest.diff_versions(None, 4)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir/bar': file_vlob}},
                        'dustbin': {'removed': [], 'added': []}}

    @pytest.mark.asyncio
    async def test_reload_not_saved_manifest(self, user_manifest_svc):
        group_manifest = GroupManifest(user_manifest_svc)
        with pytest.raises(UserManifestError):
            await group_manifest.reload()

    @pytest.mark.asyncio
    async def test_reload_not_consistent(self, user_manifest_svc, group_manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.save()
        vlob = await group_manifest.get_vlob()
        group_manifest_2 = GroupManifest(user_manifest_svc, **vlob)
        with pytest.raises(UserManifestError):
            await group_manifest_2.reload(reset=True)

    @pytest.mark.asyncio
    async def test_reload_with_reset_and_new_version(self,
                                                     user_manifest_svc,
                                                     file_svc,
                                                     group_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.save()
        vlob = await group_manifest.get_vlob()
        group_manifest_2 = GroupManifest(user_manifest_svc, **vlob)
        file_vlob_2 = await file_svc.create(content)
        await group_manifest_2.add_file('/bar', file_vlob_2)
        assert await group_manifest_2.get_version() == 0
        await group_manifest_2.reload(reset=True)
        assert await group_manifest_2.get_version() == 2
        diff = await group_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []}}
        manifest = await group_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.asyncio
    async def test_reload_with_reset_no_new_version(self,
                                                    file_svc,
                                                    group_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.save()
        file_vlob_2 = await file_svc.create(content)
        await group_manifest.add_file('/bar', file_vlob_2)
        assert await group_manifest.get_version() == 2
        await group_manifest.reload(reset=True)
        assert await group_manifest.get_version() == 2
        diff = await group_manifest.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []}}
        manifest = await group_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.asyncio
    async def test_reload_without_reset_and_new_version(self,
                                                        user_manifest_svc,
                                                        file_svc,
                                                        group_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.save()
        vlob = await group_manifest.get_vlob()
        group_manifest_2 = GroupManifest(user_manifest_svc, **vlob)
        file_vlob_2 = await file_svc.create(content)
        await group_manifest_2.add_file('/bar', file_vlob_2)
        assert await group_manifest_2.get_version() == 0
        await group_manifest_2.reload(reset=False)
        assert await group_manifest_2.get_version() == 2
        diff = await group_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []}}
        manifest = await group_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_reload_without_reset_and_no_new_version(self,
                                                           file_svc,
                                                           group_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.save()
        file_vlob_2 = await file_svc.create(content)
        await group_manifest.add_file('/bar', file_vlob_2)
        assert await group_manifest.get_version() == 2
        await group_manifest.reload(reset=False)
        assert await group_manifest.get_version() == 2
        diff = await group_manifest.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []}}
        manifest = await group_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_save(self, user_manifest_svc, file_svc):
        # Create group manifest
        group_manifest = GroupManifest(user_manifest_svc)
        # Save firt time
        await group_manifest.save()
        manifest_vlob = await group_manifest.get_vlob()
        assert manifest_vlob['id'] is not None
        assert manifest_vlob['key'] is not None
        assert manifest_vlob['read_trust_seed'] is not None
        assert manifest_vlob['write_trust_seed'] is not None
        # Modify and save
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await group_manifest.add_file('/foo', file_vlob)
        await group_manifest.save()
        assert await group_manifest.get_vlob() == manifest_vlob
        assert await group_manifest.get_version() == 2
        # Save without modifications
        await group_manifest.save()
        assert await group_manifest.get_version() == 2
        # TODO assert called methods

    @pytest.mark.asyncio
    async def test_reencrypt(self, user_manifest_svc, file_svc, group_manifest):
        content = encodebytes('foo'.encode()).decode()
        vlob = await file_svc.create(content)
        await group_manifest.add_file('/foo', vlob)
        await group_manifest.save()
        assert await group_manifest.get_version() == 2
        dump = await group_manifest.dumps()
        dump = json.loads(dump)
        old_id = group_manifest.id
        old_key = group_manifest.key
        old_read_trust_seed = group_manifest.read_trust_seed
        old_write_trust_seed = group_manifest.write_trust_seed
        await group_manifest.reencrypt()
        assert group_manifest.id != old_id
        assert group_manifest.key != old_key
        assert group_manifest.read_trust_seed != old_read_trust_seed
        assert group_manifest.write_trust_seed != old_write_trust_seed
        assert await group_manifest.get_version() == 1
        new_group_manifest = GroupManifest(user_manifest_svc,
                                           group_manifest.id,
                                           group_manifest.key,
                                           group_manifest.read_trust_seed,
                                           group_manifest.write_trust_seed)
        await new_group_manifest.reload(reset=True)
        assert await new_group_manifest.get_version() == 1
        new_dump = await new_group_manifest.dumps()
        new_dump = json.loads(new_dump)
        assert new_dump == dump

    @pytest.mark.asyncio
    async def test_restore_manifest(self, user_manifest_svc, file_svc, group_manifest):
        content = encodebytes('foo'.encode()).decode()
        vlob = await file_svc.create(content)
        # Restore dirty manifest with version 1
        await group_manifest.add_file('/tmp', vlob)
        assert await group_manifest.get_version() == 1
        await group_manifest.restore()
        assert await group_manifest.get_version() == 1
        children = await group_manifest.list_dir('/')
        assert sorted(children.keys()) == []
        # Restore previous version
        await group_manifest.add_file('/foo', vlob)
        await group_manifest.save()
        await group_manifest.add_file('/bar', vlob)
        await group_manifest.save()
        await group_manifest.add_file('/baz', vlob)
        await group_manifest.save()
        assert await group_manifest.get_version() == 4
        await group_manifest.restore()
        assert await group_manifest.get_version() == 5
        children = await group_manifest.list_dir('/')
        assert sorted(children.keys()) == ['bar', 'foo']
        # Restore old version
        await group_manifest.restore(4)
        assert await group_manifest.get_version() == 6
        children = await group_manifest.list_dir('/')
        assert sorted(children.keys()) == ['bar', 'baz', 'foo']
        # Bad version
        for version in [0, 10]:
            with pytest.raises(UserManifestError):
                await group_manifest.restore(version)
        # Restore not saved manifest
        new_group_manifest = GroupManifest(group_manifest.service)
        with pytest.raises(UserManifestError):
            await new_group_manifest.restore()
        # TODO restore file versions


class TestUserManifest:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('payload', [
        {'id': 'i123'},
        {'id': None}])
    async def test_init(self, user_manifest_svc, payload):
        manifest = UserManifest(user_manifest_svc, **payload)
        payload.update({'key': None, 'read_trust_seed': None, 'write_trust_seed': None})
        assert await manifest.get_vlob() == payload

    @pytest.mark.asyncio
    async def test_diff_versions(self, file_svc, user_manifest):
        # Old version (0) and new version (0) of non-saved manifest
        manifest = UserManifest(user_manifest.service, 'i123')
        diff = await manifest.diff_versions(0, 0)
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # No old version (use original) and no new version (dump current)
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest.add_file('/foo', file_vlob)
        group_manifest = GroupManifest(user_manifest.service)
        await group_manifest.save()
        group_vlob = await group_manifest.get_vlob()
        await user_manifest.import_group_vlob('share', group_vlob)
        diff = await user_manifest.diff_versions()
        await user_manifest.remove_group('share')
        assert diff == {'entries': {'added': {'/foo': file_vlob}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {'share': group_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # Old version (2) and no new version (dump current)
        await user_manifest.save()
        await user_manifest.add_file('/bar', file_vlob)
        diff = await user_manifest.diff_versions(2)
        assert diff == {'entries': {'added': {'/bar': file_vlob}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # Old version (3) and new version (5)
        await user_manifest.save()
        await user_manifest.add_file('/dir/foo', file_vlob)
        await user_manifest.save()
        await user_manifest.add_file('/dir/bar', file_vlob)
        await user_manifest.save()
        await user_manifest.add_file('/dir/last', file_vlob)
        diff = await user_manifest.diff_versions(3, 5)
        assert diff == {'entries': {'added': {'/dir/bar': file_vlob, '/dir/foo': file_vlob},
                                    'changed': {},
                                    'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # Old version (5) and new version (3)
        diff = await user_manifest.diff_versions(5, 3)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir/bar': file_vlob, '/dir/foo': file_vlob}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}
        # No old version (use original) and new version (4)
        diff = await user_manifest.diff_versions(None, 4)
        assert diff == {'entries': {'added': {},
                                    'changed': {},
                                    'removed': {'/dir/bar': file_vlob}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []}}

    @pytest.mark.asyncio
    async def test_dumps_current_manifest(self, file_svc, user_manifest_with_group):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest_with_group.add_file('/foo', file_vlob)
        dump = await user_manifest_with_group.dumps(original_manifest=False)
        dump = json.loads(dump)
        group_vlob = dump['groups']['foo_community']
        assert dump == {'entries': {'/': {'id': None,
                                          'key': None,
                                          'read_trust_seed': None,
                                          'write_trust_seed': None},
                                    '/foo': file_vlob},
                        'dustbin': [],
                        'groups': {'foo_community': group_vlob}}

    @pytest.mark.asyncio
    async def test_dumps_original_manifest(self, file_svc, user_manifest_with_group):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest_with_group.add_file('/foo', file_vlob)
        dump = await user_manifest_with_group.dumps(original_manifest=True)
        dump = json.loads(dump)
        group_vlob = dump['groups']['foo_community']
        assert dump == {'entries': {'/': {'id': None,
                                          'key': None,
                                          'read_trust_seed': None,
                                          'write_trust_seed': None}
                                    },
                        'dustbin': [],
                        'groups': {'foo_community': group_vlob}}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'share'])
    async def test_get_group_vlobs(self, user_manifest_with_group, group):
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
        with pytest.raises(UserManifestNotFound):
            await user_manifest_with_group.get_group_vlobs('unknown')

    @pytest.mark.asyncio
    async def test_get_group_manifest(self, user_manifest_with_group):
        group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        assert isinstance(group_manifest, GroupManifest)
        # Not found
        with pytest.raises(UserManifestNotFound):
            await user_manifest_with_group.get_group_manifest('unknown')

    @pytest.mark.asyncio
    async def test_reencrypt_group_manifest(self, user_manifest_with_group):
        group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        await user_manifest_with_group.reencrypt_group_manifest('foo_community')
        new_group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        assert group_manifest.get_vlob() != new_group_manifest.get_vlob()
        assert isinstance(group_manifest, GroupManifest)
        # Not found
        with pytest.raises(UserManifestNotFound):
            await user_manifest_with_group.reencrypt_group_manifest('unknown')

    @pytest.mark.asyncio
    async def test_create_group_manifest(self, user_manifest):
        with pytest.raises(UserManifestNotFound):
            await user_manifest.get_group_manifest('share')
        await user_manifest.create_group_manifest('share')
        group_manifest = await user_manifest.get_group_manifest('share')
        assert isinstance(group_manifest, GroupManifest)
        # Already exists
        with pytest.raises(UserManifestError):
            await user_manifest.create_group_manifest('share')

    @pytest.mark.asyncio
    async def test_import_group_vlob(self, user_manifest_svc, user_manifest):
        group_manifest = GroupManifest(user_manifest_svc)
        await group_manifest.save()
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
        with pytest.raises(UserManifestNotFound):
            await user_manifest_with_group.remove_group('foo_community')

    @pytest.mark.asyncio
    async def test_reload_not_exists(self, user_manifest_svc, file_svc):
        user_manifest = UserManifest(user_manifest_svc, '3C3FA85FB9736362497EB23DC0485AC10E6274C7')
        with pytest.raises(UserManifestError):
            await user_manifest.reload(reset=True)

    @pytest.mark.asyncio
    async def test_reload_not_consistent(self, user_manifest_svc, file_svc, user_manifest):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.save()
        vlob = await user_manifest.get_vlob()
        user_manifest_2 = UserManifest(user_manifest_svc, vlob['id'])
        with pytest.raises(UserManifestError):
            await user_manifest_2.reload(reset=True)

    @pytest.mark.asyncio
    async def test_reload_with_reset_and_new_version(self, user_manifest_svc, file_svc,
                                                     user_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.save()
        vlob = await user_manifest.get_vlob()
        user_manifest_2 = UserManifest(user_manifest_svc, vlob['id'])
        file_vlob_2 = await file_svc.create(content)
        await user_manifest_2.add_file('/bar', file_vlob_2)
        assert await user_manifest_2.get_version() == 0
        await user_manifest_2.reload(reset=True)
        assert await user_manifest_2.get_version() == 2
        diff = await user_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.asyncio
    async def test_reload_with_reset_no_new_version(self, file_svc, user_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.save()
        file_vlob_2 = await file_svc.create(content)
        await user_manifest.add_file('/bar', file_vlob_2)
        assert await user_manifest.get_version() == 2
        await user_manifest.reload(reset=True)
        assert await user_manifest.get_version() == 2
        diff = await user_manifest.diff_versions()
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' not in entries

    @pytest.mark.asyncio
    async def test_reload_without_reset_and_new_version(self,
                                                        user_manifest_svc,
                                                        file_svc,
                                                        user_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.save()
        vlob = await user_manifest.get_vlob()
        user_manifest_2 = UserManifest(user_manifest_svc, vlob['id'])
        file_vlob_2 = await file_svc.create(content)
        await user_manifest_2.add_file('/bar', file_vlob_2)
        assert await user_manifest_2.get_version() == 0
        await user_manifest_2.reload(reset=False)
        assert await user_manifest_2.get_version() == 2
        diff = await user_manifest_2.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest_2.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_reload_without_reset_and_no_new_version(self, file_svc, user_manifest):
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.save()
        file_vlob_2 = await file_svc.create(content)
        await user_manifest.add_file('/bar', file_vlob_2)
        assert await user_manifest.get_version() == 2
        await user_manifest.reload(reset=False)
        assert await user_manifest.get_version() == 2
        diff = await user_manifest.diff_versions()
        assert diff == {'entries': {'added': {'/bar': file_vlob_2}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [], 'removed': []},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}}}
        manifest = await user_manifest.dumps()
        manifest = json.loads(manifest)
        entries = manifest['entries']
        assert '/foo' in entries and entries['/foo'] == file_vlob
        assert '/bar' in entries and entries['/bar'] == file_vlob_2

    @pytest.mark.asyncio
    async def test_save(self, file_svc, user_manifest):
        manifest_vlob = await user_manifest.get_vlob()
        # Modify and save
        content = encodebytes('foo'.encode()).decode()
        file_vlob = await file_svc.create(content)
        await user_manifest.add_file('/foo', file_vlob)
        await user_manifest.save()
        assert await user_manifest.get_vlob() == manifest_vlob
        assert await user_manifest.get_version() == 2
        # Save without modifications
        await user_manifest.save()
        assert await user_manifest.get_version() == 2
        # TODO assert called methods

    @pytest.mark.asyncio
    async def test_restore_manifest(self, file_svc, user_manifest):
        content = encodebytes('foo'.encode()).decode()
        vlob = await file_svc.create(content)
        # Restore dirty manifest with version 1
        await user_manifest.add_file('/tmp', vlob)
        assert await user_manifest.get_version() == 1
        await user_manifest.restore()
        assert await user_manifest.get_version() == 1
        children = await user_manifest.list_dir('/')
        assert sorted(children.keys()) == []
        # Restore previous version
        await user_manifest.add_file('/foo', vlob)
        await user_manifest.save()
        await user_manifest.add_file('/bar', vlob)
        await user_manifest.save()
        await user_manifest.add_file('/baz', vlob)
        await user_manifest.save()
        assert await user_manifest.get_version() == 4
        await user_manifest.restore()
        assert await user_manifest.get_version() == 5
        children = await user_manifest.list_dir('/')
        assert sorted(children.keys()) == ['bar', 'foo']
        # Restore old version
        await user_manifest.restore(4)
        assert await user_manifest.get_version() == 6
        children = await user_manifest.list_dir('/')
        assert sorted(children.keys()) == ['bar', 'baz', 'foo']
        # Bad version
        for version in [0, 10]:
            with pytest.raises(UserManifestError):
                await user_manifest.restore(version)
        # Restore not saved manifest
        vlob = await user_manifest.get_vlob()
        new_user_manifest = UserManifest(user_manifest.service, vlob['id'])
        with pytest.raises(UserManifestError):
            await new_user_manifest.restore()
        # TODO restore file versions

    @pytest.mark.asyncio
    async def test_check_consistency(self, user_manifest_svc, file_svc, user_manifest):
        content = encodebytes('foo'.encode()).decode()
        good_vlob = await file_svc.create(content)
        bad_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        # With good vlobs only
        await user_manifest.add_file('/foo', good_vlob)
        await user_manifest.delete_file('/foo')
        await user_manifest.add_file('/bar', good_vlob)
        dump = await user_manifest.dumps()
        assert await user_manifest.check_consistency(json.loads(dump)) is True
        # With a bad vlob
        user_manifest.group_manifests['share'] = GroupManifest(user_manifest_svc, **bad_vlob)
        dump = await user_manifest.dumps()
        assert await user_manifest.check_consistency(json.loads(dump)) is False
        await user_manifest.remove_group('share')
        dump = await user_manifest.dumps()
        assert await user_manifest.check_consistency(json.loads(dump)) is True


class TestUserManifestService:

    @pytest.mark.asyncio
    async def test_create_group_manifest(self, user_manifest_svc, user_manifest):
        with pytest.raises(UserManifestNotFound):
            await user_manifest.get_group_manifest('share')
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_group_manifest',
                                                    'group': 'share'})
        assert ret == {'status': 'ok'}
        await user_manifest.reload(reset=True)
        group_manifest = await user_manifest.get_group_manifest('share')
        assert isinstance(group_manifest, GroupManifest)
        # Already exists
        with pytest.raises(UserManifestError):
            await user_manifest.create_group_manifest('share')

    @pytest.mark.asyncio
    async def test_reencrypt_group_manifest(self, user_manifest_svc, user_manifest_with_group):
        group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        await user_manifest_svc.reencrypt_group_manifest('foo_community')
        new_group_manifest = await user_manifest_with_group.get_group_manifest('foo_community')
        assert group_manifest.get_vlob() != new_group_manifest.get_vlob()
        assert isinstance(group_manifest, GroupManifest)
        # Not found
        with pytest.raises(UserManifestError):
            await user_manifest_svc.reencrypt_group_manifest('unknown')

    @pytest.mark.asyncio
    async def test_import_group_vlob(self, user_manifest_svc, user_manifest):
        group_manifest = GroupManifest(user_manifest_svc)
        await group_manifest.save()
        vlob = await group_manifest.get_vlob()
        await user_manifest_svc.import_group_vlob('share', vlob)
        await user_manifest.reload(reset=True)
        retrieved_manifest = await user_manifest.get_group_manifest('share')
        assert await retrieved_manifest.get_vlob() == vlob
        await group_manifest.reencrypt()
        new_vlob = await group_manifest.get_vlob()
        await user_manifest_svc.import_group_vlob('share', new_vlob)
        await user_manifest.reload(reset=True)
        retrieved_manifest = await user_manifest.get_group_manifest('share')
        assert await retrieved_manifest.get_vlob() == new_vlob

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_import_file_vlob(self,
                                    user_manifest_svc,
                                    file_svc,
                                    user_manifest_with_group,
                                    group):
        file_vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        await user_manifest_svc.import_file_vlob('/test', file_vlob, group)
        with pytest.raises(UserManifestError):
            await user_manifest_svc.import_file_vlob('/test', file_vlob, group)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_create_file(self, user_manifest_svc, user_manifest_with_group, group, path):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret['status'] == 'ok'
        assert ret['id'] is not None
        # Already exist
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret == {'status': 'already_exists', 'label': 'File already exists.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_rename_file(self, user_manifest_svc, user_manifest_with_group, group):
        await user_manifest_svc.create_file('/test', group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_rename_file',
                                                    'old_path': '/test',
                                                    'new_path': '/foo',
                                                    'group': group})
        assert ret['status'] == 'ok'
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.list_dir('/test', group=group)
        await user_manifest_svc.list_dir('/foo', group=group)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_rename_file_and_target_exist(self,
                                                user_manifest_svc,
                                                user_manifest_with_group,
                                                group):
        await user_manifest_svc.create_file('/test', group=group)
        await user_manifest_svc.create_file('/foo', group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_rename_file',
                                                    'old_path': '/test',
                                                    'new_path': '/foo',
                                                    'group': group})
        assert ret == {'status': 'already_exists', 'label': 'File already exists.'}
        await user_manifest_svc.list_dir('/test', group=group)
        await user_manifest_svc.list_dir('/foo', group=group)

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_delete_file(self, user_manifest_svc, user_manifest_with_group, group, path):
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
    async def test_delete_not_file(self, user_manifest_svc, user_manifest_with_group, group):
        await user_manifest_svc.make_dir('/test', group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_delete_file',
                                                    'path': '/test',
                                                    'group': group})
        assert ret == {'status': 'path_is_not_file', 'label': 'Path is not a file.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_restore_file(self, user_manifest_svc, user_manifest_with_group, group, path):
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
        # Restore path already used
        await user_manifest_svc.delete_file(path, group)
        await user_manifest_svc.create_file(path, group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore_file',
                                                    'vlob': vlob_id,
                                                    'group': group})
        assert ret == {'status': 'already_exists', 'label': 'Restore path already used.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_list_dir(self, user_manifest_svc, user_manifest_with_group, group):
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
                assert sorted(keys) == sorted(ret['current'].keys())
                assert sorted(keys) == sorted(ret['children'][name].keys())

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
    async def test_create_dir(self, user_manifest_svc, user_manifest_with_group, group):
        # Working
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_make_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret['status'] == 'ok'
        # Already exist
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_make_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'already_exists', 'label': 'Directory already exists.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_remove_dir(self, user_manifest_svc, user_manifest_with_group, group):
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
        assert ret == {'status': 'not_found', 'label': 'Directory not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_cant_remove_root_dir(self, user_manifest_svc, user_manifest_with_group, group):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/',
                                                    'group': group})
        assert ret == {'status': 'cannot_remove_root', 'label': 'Cannot remove root directory.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_remove_not_empty_dir(self, user_manifest_svc, user_manifest_with_group, group):
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
    async def test_remove_not_dir(self, user_manifest_svc, user_manifest_with_group, group):
        await user_manifest_svc.create_file('/test_dir', group=group)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_remove_dir',
                                                    'path': '/test_dir',
                                                    'group': group})
        assert ret == {'status': 'path_is_not_dir', 'label': 'Path is not a directory.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    async def test_show_dustbin(self, user_manifest_svc, user_manifest_with_group, group, path):
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
    async def test_load_user_manifest(self, user_manifest_svc, identity_svc):
        await user_manifest_svc.create_file('test')
        await user_manifest_svc.list_dir('test')
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_load'})
        assert ret == {'status': 'ok'}
        await user_manifest_svc.list_dir('test')
        identity = '3C3FA85FB9736362497EB23DC0485AC10E6274C7'
        manifest = await user_manifest_svc.get_manifest()
        assert manifest.id != identity
        await identity_svc.load_identity(identity)
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_load'})
        assert ret == {'status': 'ok'}
        manifest = await user_manifest_svc.get_manifest()
        assert manifest.id == identity
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.list_dir('test')

    @pytest.mark.asyncio
    async def test_get_manifest(self, user_manifest_svc, user_manifest_with_group):
        manifest = await user_manifest_svc.get_manifest()
        assert manifest.id == await user_manifest_svc.identity_service.get_identity()
        group_manifest = await user_manifest_svc.get_manifest('foo_community')
        assert group_manifest.id is not None
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.get_manifest('unknown')
        with pytest.raises(UserManifestNotFound):
            user_manifest_svc.user_manifest = None  # TODO too intrusive
            await user_manifest_svc.get_manifest()

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_get_properties(self, user_manifest_svc, user_manifest_with_group, group):
        foo_vlob = await user_manifest_svc.create_file('/foo', group=group)
        bar_vlob = await user_manifest_svc.create_file('/bar', group=group)
        await user_manifest_svc.delete_file('/bar', group)
        # Lookup group
        group_manifest = await user_manifest_svc.get_manifest(group='foo_community')
        group_vlob = await user_manifest_svc.get_properties(group='foo_community')
        assert await group_manifest.get_vlob() == group_vlob
        # Lookup foo by path
        vlob = await user_manifest_svc.get_properties(path='/foo', dustbin=False, group=group)
        assert vlob == foo_vlob
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.get_properties(path='/foo', dustbin=True, group=group)
        # Lookup bar by path
        vlob = await user_manifest_svc.get_properties(path='/bar', dustbin=True, group=group)
        vlob = deepcopy(vlob)  # TODO use deepcopy?
        del vlob['removed_date']
        del vlob['path']
        assert vlob == bar_vlob
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.get_properties(path='/bar', dustbin=False, group=group)
        # Lookup foo by id
        vlob = await user_manifest_svc.get_properties(id=foo_vlob['id'], dustbin=False, group=group)
        assert vlob == foo_vlob
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.get_properties(id=foo_vlob['id'], dustbin=True, group=group)
        # Lookup bar by id
        vlob = await user_manifest_svc.get_properties(id=bar_vlob['id'], dustbin=True, group=group)
        vlob = deepcopy(vlob)  # TODO use deepcopy?
        del vlob['removed_date']
        del vlob['path']
        assert vlob == bar_vlob
        with pytest.raises(UserManifestNotFound):
            await user_manifest_svc.get_properties(id=bar_vlob['id'], dustbin=False, group=group)

    @pytest.mark.asyncio
    async def test_user_manifest_history(self, user_manifest_svc):
        foo_vlob = await user_manifest_svc.create_file('/foo')
        bar_vlob = await user_manifest_svc.create_file('/bar')
        baz_vlob = await user_manifest_svc.create_file('/baz')
        # Full history
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history'})
        assert ret == {
            'status': 'ok',
            'detailed_history': [
                {
                    'version': 1,
                    'entries': {'added': {}, 'changed': {}, 'removed': {}},
                    'groups': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 2,
                    'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 3,
                    'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'removed': {}, 'added': {}, 'changed': {}},
                    'dustbin': {'removed': [], 'added': []}
                },
                {
                    'version': 4,
                    'entries': {'added': {'/baz': baz_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'removed': {}, 'added': {}, 'changed': {}},
                    'dustbin': {'removed': [], 'added': []}
                }
            ]
        }
        # Partial history starting at version 2
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'first_version': 2})
        assert ret == {
            'status': 'ok',
            'detailed_history': [
                {
                    'version': 2,
                    'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 3,
                    'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'removed': {}, 'added': {}, 'changed': {}},
                    'dustbin': {'removed': [], 'added': []}
                },
                {
                    'version': 4,
                    'entries': {'added': {'/baz': baz_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'removed': {}, 'added': {}, 'changed': {}},
                    'dustbin': {'removed': [], 'added': []}
                }
            ]
        }
        # Partial history ending at version 2
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'last_version': 2})
        assert ret == {
            'status': 'ok',
            'detailed_history': [
                {
                    'version': 1,
                    'entries': {'added': {}, 'changed': {}, 'removed': {}},
                    'groups': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 2,
                    'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                    'groups': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                }
            ]
        }
        # Summary of full history
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'summary': True})

        assert ret == {
            'status': 'ok',
            'summary_history': {
                'entries': {'added': {'/foo': foo_vlob,
                                      '/bar': bar_vlob,
                                      '/baz': baz_vlob},
                            'changed': {},
                            'removed': {}},
                'groups': {'added': {}, 'changed': {}, 'removed': {}},
                'dustbin': {'added': [], 'removed': []}
            }
        }
        # Summary of partial history
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'first_version': 2,
                                                    'last_version': 4,
                                                    'summary': True})
        assert ret == {
            'status': 'ok',
            'summary_history': {
                'entries': {'added': {'/bar': bar_vlob,
                                      '/baz': baz_vlob},
                            'changed': {},
                            'removed': {}},
                'groups': {'added': {}, 'changed': {}, 'removed': {}},
                'dustbin': {'added': [], 'removed': []}
            }
        }
        # First version > last version
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'first_version': 4,
                                                    'last_version': 2,
                                                    'summary': True})
        assert ret == {'status': 'bad_versions',
                       'label': 'First version number greater than second version number.'}

    @pytest.mark.asyncio
    async def test_group_manifest_history(self, user_manifest_svc, user_manifest_with_group):
        group = 'foo_community'
        foo_vlob = await user_manifest_svc.create_file('/foo', group=group)
        bar_vlob = await user_manifest_svc.create_file('/bar', group=group)
        baz_vlob = await user_manifest_svc.create_file('/baz', group=group)
        # Full history
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history', 'group': group})
        assert ret == {
            'status': 'ok',
            'detailed_history': [
                {
                    'version': 1,
                    'entries': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 2,
                    'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 3,
                    'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'removed': [], 'added': []}
                },
                {
                    'version': 4,
                    'entries': {'added': {'/baz': baz_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'removed': [], 'added': []}
                }
            ]
        }
        # Partial history starting at version 2
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'first_version': 2,
                                                    'group': group})
        assert ret == {
            'status': 'ok',
            'detailed_history': [
                {
                    'version': 2,
                    'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 3,
                    'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'removed': [], 'added': []}
                },
                {
                    'version': 4,
                    'entries': {'added': {'/baz': baz_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'removed': [], 'added': []}
                }
            ]
        }
        # Partial history ending at version 2
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'last_version': 2,
                                                    'group': group})
        assert ret == {
            'status': 'ok',
            'detailed_history': [
                {
                    'version': 1,
                    'entries': {'added': {}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                },
                {
                    'version': 2,
                    'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                    'dustbin': {'added': [], 'removed': []}
                }
            ]
        }
        # Summary of full history
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'summary': True,
                                                    'group': group})

        assert ret == {
            'status': 'ok',
            'summary_history': {
                'entries': {'added': {'/foo': foo_vlob,
                                      '/bar': bar_vlob,
                                      '/baz': baz_vlob},
                            'changed': {},
                            'removed': {}},
                'dustbin': {'added': [], 'removed': []}
            }
        }
        # Summary of partial history
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'first_version': 2,
                                                    'last_version': 4,
                                                    'summary': True,
                                                    'group': group})
        assert ret == {
            'status': 'ok',
            'summary_history': {
                'entries': {'added': {'/bar': bar_vlob,
                                      '/baz': baz_vlob},
                            'changed': {},
                            'removed': {}},
                'dustbin': {'added': [], 'removed': []}
            }
        }
        # First version > last version
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_history',
                                                    'first_version': 4,
                                                    'last_version': 2,
                                                    'summary': True,
                                                    'group': group})
        assert ret == {'status': 'bad_versions',
                       'label': 'First version number greater than second version number.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group', [None, 'foo_community'])
    async def test_restore_manifest(self, user_manifest_svc, user_manifest_with_group, group):
        group = 'foo_community'
        await user_manifest_svc.create_file('/foo', group=group)
        await user_manifest_svc.create_file('/bar', group=group)
        await user_manifest_svc.create_file('/baz', group=group)
        # Previous version
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore', 'group': group})
        assert ret == {'status': 'ok'}
        listing = await user_manifest_svc.list_dir('/', group)
        assert sorted(listing[1].keys()) == ['bar', 'foo']
        # Restore old version
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore',
                                                    'version': 2,
                                                    'group': group})
        assert ret == {'status': 'ok'}
        listing = await user_manifest_svc.list_dir('/', group)
        assert sorted(listing[1].keys()) == ['foo']
        # Bad version
        for version in [0, 10]:
            ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_restore',
                                                        'version': version,
                                                        'group': group})
            assert ret == {'status': 'bad_version', 'label': 'Bad version number.'}
        # TODO restore file versions
