from copy import deepcopy

from effect.testing import perform_sequence
from freezegun import freeze_time
import pytest

from parsec.core.file import File
from parsec.core.manifest import GroupManifest, Manifest, UserManifest
from parsec.core.synchronizer import (
    EUserVlobSynchronize, EUserVlobRead, EUserVlobUpdate, EVlobCreate, EVlobList, EVlobRead,
    EVlobUpdate, EVlobDelete, EVlobSynchronize, EBlockCreate, EBlockStat, EBlockDelete)
from parsec.crypto import generate_sym_key
from parsec.exceptions import BlockNotFound, ManifestError, ManifestNotFound, VlobNotFound
from parsec.tools import to_jsonb64, ejson_loads, ejson_dumps, digest

from tests.test_crypto import mock_crypto_passthrough, ALICE_PRIVATE_RSA


@pytest.fixture
def group_manifest(mock_crypto_passthrough):
    vlob = {'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = to_jsonb64(b'{"dustbin": [], "entries": {"/": null}, "versions": {}}')
    sequence = [
        (EVlobCreate(),
            lambda _: vlob),
        (EVlobUpdate(vlob['id'], vlob['write_trust_seed'], 1, blob),
            lambda _: vlob),
    ]
    return perform_sequence(sequence, GroupManifest.create())


@pytest.fixture
def user_manifest(mock_crypto_passthrough):
        blob = {'entries': {'/': None}, 'groups': {}, 'dustbin': [], 'versions': {}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': '', 'version': 1}),
            (EUserVlobUpdate(1, blob),
                lambda _: None)
        ]
        return perform_sequence(sequence, UserManifest.load(ALICE_PRIVATE_RSA))


@pytest.fixture
def user_manifest_with_group(user_manifest, group_manifest):
    user_manifest.group_manifests['share'] = group_manifest
    return user_manifest


def raise_(ex):
    raise ex


class TestManifest:

    @pytest.mark.parametrize('id', ['i123', None])
    def test_init(self, id):
        manifest = Manifest(id)
        assert manifest.id == id

    @freeze_time("2012-01-01")
    def test_is_dirty(self, mock_crypto_passthrough):
        manifest = Manifest()
        sequence = [
        ]
        ret = perform_sequence(sequence, manifest.is_dirty())
        assert ret is False
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        vlob = file.get_vlob()
        manifest.add_file('/foo', vlob)
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, manifest.is_dirty())
        assert ret is True
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobList(),
                lambda _: [vlob_id]),
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EBlockDelete(block_id),
                lambda _: None),
            (EVlobDelete(vlob_id),
                lambda _: None)
        ]
        perform_sequence(sequence, manifest.delete_file('/foo'))
        ret = perform_sequence([], manifest.is_dirty())
        assert ret is False

    @freeze_time("2012-01-01")
    def test_diff(self):
        manifest = Manifest()
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
        diff = manifest.diff(
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
        diff = manifest.diff(
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

    def test_patch(self):
        manifest = Manifest()
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
        sequence = [
            (EVlobRead('vlob_5', 'rts'),
                lambda _: {'id': 'vlob_5', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_4', 'rts'),
                lambda _: {'id': 'vlob_4', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_2', 'rts'),
                lambda _: {'id': 'vlob_2', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_3', 'rts'),
                lambda _: {'id': 'vlob_3', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_6', 'rts'),
                lambda _: {'id': 'vlob_6', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_7', 'rts'),
                lambda _: {'id': 'vlob_7', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_9', 'rts'),
                lambda _: {'id': 'vlob_9', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_6', 'rts'),
                lambda _: {'id': 'vlob_6', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_7', 'rts'),
                lambda _: {'id': 'vlob_7', 'version': 1, 'blob': ''}),
            (EVlobRead('vlob_8', 'rts'),
                lambda _: {'id': 'vlob_8', 'version': 1, 'blob': ''}),
        ]
        dump = perform_sequence(sequence, manifest.dumps())
        new_manifest = ejson_loads(dump)
        diff = manifest.diff(backup_original, new_manifest)
        patched_manifest = manifest.patch(backup_original, diff)
        assert backup_original == manifest.original_manifest
        assert patched_manifest['entries'] == manifest.entries
        assert patched_manifest['dustbin'] == manifest.dustbin
        # Reapply patch on already patched manifest
        patched_manifest_2 = manifest.patch(patched_manifest, diff)
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
        patched_manifest = manifest.patch(new_manifest, diff)
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

    def test_history(self, group_manifest):
        manifest = Manifest()
        with pytest.raises(NotImplementedError):
            perform_sequence([], manifest.history(2, 5))
        with pytest.raises(ManifestError):
            perform_sequence([], group_manifest.history(2, 1))
        foo_vlob = {'id': '123', 'key': '123', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        bar_vlob = {'id': '234', 'key': '123', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        old_blob = {'entries': {'/': None, '/foo': foo_vlob},
                    'dustbin': [],
                    'versions': {'123': 1}}
        new_blob = {'entries': {'/': None, '/foo': foo_vlob, '/bar': bar_vlob},
                    'dustbin': [],
                    'versions': {'123': 1, '234': 2}}
        old_blob = ejson_dumps(old_blob).encode()
        new_blob = ejson_dumps(new_blob).encode()
        old_blob = to_jsonb64(old_blob)
        new_blob = to_jsonb64(new_blob)
        # Detailed without last version
        group_manifest.version = 5
        sequence = [
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 1}),
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 1}),
            (EVlobRead('1234', '42', 2),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 2}),
            (EVlobRead('1234', '42', 2),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 2}),
            (EVlobRead('1234', '42', 3),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 3}),
            (EVlobRead('1234', '42', 3),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 3}),
            (EVlobRead('1234', '42', 4),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 4}),
            (EVlobRead('1234', '42', 4),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 4}),
            (EVlobRead('1234', '42', 5),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 5})
        ]
        history = perform_sequence(sequence, group_manifest.history(1))
        assert history == {
            'detailed_history': [
                {'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {'123': 1}, 'changed': {}, 'removed': {}},
                 'version': 1},
                {'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {'234': 2}, 'changed': {}, 'removed': {}},
                 'version': 2},
                {'entries': {'added': {}, 'changed': {}, 'removed': {'/bar': bar_vlob}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {}, 'changed': {}, 'removed': {'234': 2}},
                 'version': 3},
                {'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {'234': 2}, 'changed': {}, 'removed': {}},
                 'version': 4},
                {'entries': {'added': {}, 'changed': {}, 'removed': {'/bar': bar_vlob}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {}, 'changed': {}, 'removed': {'234': 2}},
                 'version': 5}]}
        # Detailed with last version
        sequence = [
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 1}),
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 1}),
            (EVlobRead('1234', '42', 2),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 2}),
            (EVlobRead('1234', '42', 2),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 2}),
            (EVlobRead('1234', '42', 3),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 3}),
            (EVlobRead('1234', '42', 3),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 3}),
            (EVlobRead('1234', '42', 4),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 4})
        ]
        history = perform_sequence(sequence, group_manifest.history(1, 4))
        assert history == {
            'detailed_history': [
                {'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {'123': 1}, 'changed': {}, 'removed': {}},
                 'version': 1},
                {'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {'234': 2}, 'changed': {}, 'removed': {}},
                 'version': 2},
                {'entries': {'added': {}, 'changed': {}, 'removed': {'/bar': bar_vlob}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {}, 'changed': {}, 'removed': {'234': 2}},
                 'version': 3},
                {'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                 'dustbin': {'added': [], 'removed': []},
                 'versions': {'added': {'234': 2}, 'changed': {}, 'removed': {}},
                 'version': 4}]}
        # Summary
        sequence = [
            (EVlobRead('1234', '42', 2),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 2}),
            (EVlobRead('1234', '42', 4),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 4})
        ]
        history = perform_sequence(sequence, group_manifest.history(2, 4, summary=True))
        assert history == {
            'summary_history': {
                'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                'dustbin': {'added': [], 'removed': []},
                'versions': {'added': {'234': 2}, 'changed': {}, 'removed': {}}
            }
        }

    def test_get_version(self):
        manifest = Manifest()
        sequence = [
        ]
        ret = perform_sequence(sequence, manifest.get_version())
        assert ret == 0
        manifest.add_file('/foo', {'id': 'vlob_1',
                                   'key': 'key',
                                   'read_trust_seed': 'rts',
                                   'write_trust_seed': 'wts'})
        sequence = [
            (EVlobRead('vlob_1', 'rts'),
                lambda _: {'id': 'vlob_1', 'version': 1, 'blob': ''}),
        ]
        ret = perform_sequence(sequence, manifest.get_version())
        assert ret == 1
        # TODO check after synchronization

    def test_get_vlobs_versions(self):
        manifest = Manifest()
        manifest.make_folder('/dir')
        manifest.add_file('/dir/foo', {'id': 'vlob_1',
                                       'key': 'key',
                                       'read_trust_seed': 'rts',
                                       'write_trust_seed': 'wts'})
        manifest.add_file('/bar', {'id': 'vlob_2',
                                   'key': 'key',
                                   'read_trust_seed': 'rts',
                                   'write_trust_seed': 'wts'})
        manifest.dustbin.append({'path': '/baz',
                                 'removed_date': 'foo',
                                 'id': 'vlob_3',
                                 'key': 'key',
                                 'read_trust_seed': 'rts',
                                 'write_trust_seed': 'wts'})  # TODO too intrusive ?
        sequence = [
            (EVlobRead('vlob_2', 'rts'),
                lambda _: raise_(VlobNotFound('Vlob not found.'))),
            (EVlobRead('vlob_1', 'rts'),
                lambda _: {'id': 'vlob_1', 'version': 2, 'blob': ''}),
            (EVlobRead('vlob_3', 'rts'),
                lambda _: {'id': 'vlob_2', 'version': 1, 'blob': ''})
        ]
        vlobs_versions = perform_sequence(sequence, manifest.get_vlobs_versions())
        assert vlobs_versions == {'vlob_1': 2, 'vlob_2': None, 'vlob_3': 1}

    def test_dumps_current_manifest(self):
        vlob = {'id': 'vlob_1', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest = Manifest()
        manifest.add_file('/foo', vlob)
        sequence = [
            (EVlobRead('vlob_1', 'rts'),
                lambda _: {'id': vlob['id'], 'version': 2, 'blob': ''}),
        ]
        dump = perform_sequence(sequence, manifest.dumps(original_manifest=False))
        dump = ejson_loads(dump)
        assert dump == {'entries': {'/': None, '/foo': vlob},
                        'dustbin': [],
                        'versions': {vlob['id']: 2}}

    def test_dumps_original_manifest(self):
        vlob = {'id': 'vlob_1', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest = Manifest()
        manifest.add_file('/foo', vlob)
        dump = perform_sequence([], manifest.dumps(original_manifest=True))
        dump = ejson_loads(dump)
        assert dump == {'entries': {'/': None},
                        'dustbin': [],
                        'versions': {}}

    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_add_file(self, final_slash):
        vlob = {'id': 'vlob_1', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest = Manifest()
        manifest.add_file('/test' + final_slash, vlob)
        # Already exists
        with pytest.raises(ManifestError):
            manifest.add_file('/test', vlob)
        # Parent not found
        with pytest.raises(ManifestNotFound):
            manifest.add_file('/test_dir/test', vlob)
        # Parent found
        manifest.make_folder('/test_dir')
        manifest.add_file('/test_dir/test', vlob)

    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_rename_file(self, final_slash):
        vlob = {'id': 'vlob_1', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest = Manifest()
        manifest.make_folder('/test')
        manifest.add_file('/test/test', vlob)
        # Rename file
        manifest.rename_file('/test/test' + final_slash, '/test/foo' + final_slash)
        with pytest.raises(KeyError):
            manifest.entries['/test/test']
        assert manifest.entries['/test/foo']
        # Rename dir
        manifest.rename_file('/test' + final_slash, '/foo' + final_slash)
        with pytest.raises(KeyError):
            manifest.entries['/test']
        with pytest.raises(KeyError):
            manifest.entries['/test/foo']
        assert manifest.entries['/foo'] is None
        assert manifest.entries['/foo/foo']
        # Rename parent and parent not found
        with pytest.raises(ManifestNotFound):
            manifest.rename_file('/foo/foo' + final_slash, '/test/test' + final_slash)
        assert manifest.entries['/foo'] is None
        assert manifest.entries['/foo/foo']
        # Rename parent and parent found
        manifest.make_folder('/test')
        manifest.rename_file('/foo/foo' + final_slash, '/test/test' + final_slash)
        assert manifest.entries['/test'] is None
        assert manifest.entries['/test/test']

    def test_rename_file_and_source_not_exists(self):
        manifest = Manifest()
        with pytest.raises(ManifestNotFound):
            manifest.rename_file('/test', '/foo')

    def test_rename_file_and_target_exists(self):
        vlob = {'id': 'vlob_1', 'key': 'key', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest = Manifest()
        manifest.add_file('/test', vlob)
        manifest.add_file('/foo', vlob)
        with pytest.raises(ManifestError):
            manifest.rename_file('/test', '/foo')
        manifest.stat('/test')
        manifest.stat('/foo')

    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    @pytest.mark.parametrize('synchronize', [True, False])
    def test_delete_file(self, mock_crypto_passthrough, path, final_slash, synchronize):
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        vlob = file.get_vlob()
        persistent_vlob = {'id': 'vlob_1',
                           'key': 'key',
                           'read_trust_seed': 'rts',
                           'write_trust_seed': 'wts'}
        manifest = Manifest()
        manifest.make_folder('/test_dir')
        for persistent_path in ['/persistent', '/test_dir/persistent']:
            manifest.add_file(persistent_path, persistent_vlob)
        for i in range(1):
            manifest.add_file(path, vlob)
            sequence = [
                (EVlobRead(vlob_id, '42'),
                    lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
                (EVlobList(),
                    lambda _: [] if synchronize else [vlob_id]),
                (EVlobRead(vlob_id, '42', 1),
                    lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
                (EBlockDelete(id='4567'),
                    lambda _: raise_(BlockNotFound('Block not found.')) if synchronize else None),
                (EVlobDelete(id='1234'),
                    lambda _: raise_(VlobNotFound('Vlob not found.')) if synchronize else None)
            ]
            ret = perform_sequence(sequence, manifest.delete_file(path + final_slash))
            assert ret is None
            # File not found
            with pytest.raises(ManifestNotFound):
                perform_sequence([], manifest.delete_file(path + final_slash))
            # Persistent files
            for persistent_path in ['/persistent', '/test_dir/persistent']:
                assert manifest.entries[persistent_path]
        if synchronize:
            assert len(manifest.dustbin) == 1
        else:
            assert len(manifest.dustbin) == 0

    def test_delete_not_file(self):
        manifest = Manifest()
        manifest.make_folder('/test')
        with pytest.raises(ManifestError):  # TODO InvalidPath
            perform_sequence([], manifest.delete_file('/test'))

    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    def test_undelete_file(self, mock_crypto_passthrough, path):
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        vlob = file.get_vlob()
        manifest = Manifest()
        # Working
        manifest.make_folder('/test_dir')
        manifest.add_file(path, vlob)
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EBlockDelete(id='4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EVlobDelete(id='1234'),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        perform_sequence(sequence, manifest.delete_file(path))
        manifest.remove_folder('/test_dir')
        # Working
        manifest.undelete_file(vlob['id'])
        assert manifest.entries[path]
        if path.startswith('/test_dir'):
            assert manifest.entries['/test_dir'] is None
        # Not found
        with pytest.raises(ManifestNotFound):
            manifest.undelete_file(vlob['id'])
        # Restore path already used
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EBlockDelete(id='4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EVlobDelete(id='1234'),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        perform_sequence(sequence, manifest.delete_file(path))
        manifest.add_file(path, vlob)
        with pytest.raises(ManifestError):
            manifest.undelete_file(vlob['id'])

    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_reencrypt_file(self, mock_crypto_passthrough, path, final_slash):
        manifest = Manifest()
        file_vlob = {'id': '123',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': 'rts',
                     'write_trust_seed': 'wts'}
        manifest.make_folder('/test_dir')
        manifest.add_file(path, file_vlob)
        sequence = [
            (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed']),
                lambda _: {'id': file_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed'], 1),
                lambda _: {'id': file_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobCreate(to_jsonb64(b'foo')),
                lambda _: {'id': '234',
                           'read_trust_seed': 'rtsnew',
                           'write_trust_seed': 'wtsnew',
                           'version': 1})
        ]
        ret = perform_sequence(sequence, manifest.reencrypt_file(path + final_slash))
        assert ret is None
        assert manifest.entries[path]['id'] == '234'
        assert manifest.entries[path]['read_trust_seed'] == 'rtsnew'
        assert manifest.entries[path]['write_trust_seed'] == 'wtsnew'
        with pytest.raises(ManifestNotFound):
            perform_sequence([], manifest.reencrypt_file('/unknown'))

    def test_stat(self, mock_crypto_passthrough):
        manifest = Manifest()
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        vlob = file.get_vlob()
        with freeze_time('2012-01-01') as frozen_datetime:
            # Create folders
            manifest.make_folder('/countries')
            manifest.make_folder('/countries/France')
            manifest.make_folder('/countries/France/cities')
            manifest.make_folder('/countries/Belgium')
            manifest.make_folder('/countries/Belgium/cities')
            # Create multiple files
            manifest.add_file('/.root', vlob)
            manifest.add_file('/countries/index', vlob)
            manifest.add_file('/countries/France/info', vlob)
            manifest.add_file('/countries/Belgium/info', vlob)
            ret = perform_sequence([], manifest.stat('/'))
            assert ret == {'type': 'folder', 'items': ['.root', 'countries']}
            for final_slash in ['', '/']:
                ret = perform_sequence([], manifest.stat('/countries' + final_slash))
                assert ret == {'type': 'folder', 'items': ['Belgium', 'France', 'index']}
                ret = perform_sequence([], manifest.stat('/countries/France/cities' + final_slash))
                assert ret == {'type': 'folder', 'items': []}
                sequence = [
                    (EVlobRead(vlob_id, '42'),
                        lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
                    (EVlobList(),
                        lambda _: []),
                    (EVlobRead(vlob_id, '42', 1),
                        lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
                    (EBlockStat('4567'),
                        lambda _: {'creation_date': '2012-01-01T00:00:00'})
                ]
                ret = perform_sequence(sequence,
                                       manifest.stat('/countries/France/info' + final_slash))
                assert ret == {'id': vlob['id'],
                               'type': 'file',
                               'created': frozen_datetime().isoformat(),
                               'updated': frozen_datetime().isoformat(),
                               'size': 0,
                               'version': 1}

        # Test bad list as well
        with pytest.raises(ManifestNotFound):
            perform_sequence([], manifest.stat('/dummy'))
            perform_sequence([], manifest.stat('/countries/dummy'))

    @pytest.mark.parametrize('parents', ['/', '/parent_1/', '/parent_1/parent_2/'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    @pytest.mark.parametrize('create_parents', [False, True])
    def test_make_folder(self, parents, final_slash, create_parents):
        manifest = Manifest()
        complete_path = parents + 'test_dir' + final_slash
        # Working
        if parents == '/' or create_parents:
            manifest.make_folder(complete_path, parents=create_parents)
        else:
            # Parents not found
            with pytest.raises(ManifestNotFound):
                manifest.make_folder(complete_path, parents=create_parents)
        # Already exist
        if create_parents:
            manifest.make_folder(complete_path, parents=create_parents)
        else:
            with pytest.raises((ManifestError, ManifestNotFound)):
                manifest.make_folder(complete_path, parents=create_parents)

    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_remove_folder(self, final_slash):
        manifest = Manifest()
        # Working
        manifest.make_folder('/test_dir')
        manifest.remove_folder('/test_dir' + final_slash)
        # Not found
        with pytest.raises(ManifestNotFound):
            manifest.remove_folder('/test_dir')
        with pytest.raises(ManifestNotFound):
            manifest.remove_folder('/test_dir/')

    def test_cant_remove_root_dir(self):
        manifest = Manifest()
        with pytest.raises(ManifestError):
            manifest.remove_folder('/')

    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_remove_not_empty_dir(self, final_slash):
        manifest = Manifest()
        # Not empty
        manifest.make_folder('/test_dir')
        manifest.make_folder('/test_dir/test')
        with pytest.raises(ManifestError):
            manifest.remove_folder('/test_dir' + final_slash)
        # Empty
        manifest.remove_folder('/test_dir/test' + final_slash)
        manifest.remove_folder('/test_dir' + final_slash)

    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_remove_not_dir(self, final_slash):
        vlob = {'id': '123', 'key': '123', 'read_trust_seed': '123', 'write_trust_seed': '123'}
        manifest = Manifest()
        manifest.add_file('/test_dir' + final_slash, vlob)
        with pytest.raises(ManifestError):
            manifest.remove_folder('/test_dir')

    @pytest.mark.parametrize('path', ['/test', '/test_dir/test'])
    @pytest.mark.parametrize('final_slash', ['', '/'])
    def test_show_dustbin(self, mock_crypto_passthrough, path, final_slash):
        manifest = Manifest()
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        vlob = file.get_vlob()
        # Empty dustbin
        dustbin = manifest.show_dustbin()
        assert dustbin == []
        manifest.add_file('/foo', vlob)
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EBlockDelete(id='4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EVlobDelete(id='1234'),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        perform_sequence(sequence, manifest.delete_file('/foo'))
        manifest.make_folder('/test_dir')
        for i in [1, 2]:
            manifest.add_file(path, vlob)
            sequence = [
                (EVlobRead(vlob_id, '42'),
                    lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
                (EVlobList(),
                    lambda _: []),
                (EVlobRead(vlob_id, '42', 1),
                    lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
                (EBlockDelete(id='4567'),
                    lambda _: raise_(BlockNotFound('Block not found.'))),
                (EVlobDelete(id='1234'),
                    lambda _: raise_(VlobNotFound('Vlob not found.')))
            ]
            perform_sequence(sequence, manifest.delete_file(path))
            # Global dustbin with one additional file
            dustbin = manifest.show_dustbin()
            assert len(dustbin) == i + 1
            # File in dustbin
            dustbin = manifest.show_dustbin(path + final_slash)
            assert len(dustbin) == i
            # Not found
            with pytest.raises(ManifestNotFound):
                manifest.show_dustbin('/unknown')

    def test_check_consistency(self, mock_crypto_passthrough):
        manifest = Manifest()
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        good_vlob = file.get_vlob()
        bad_vlob = {'id': '123',
                    'key': good_vlob['key'],
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        # With good vlobs only
        manifest.add_file('/foo', good_vlob)
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, manifest.dumps())
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, manifest.check_consistency(ejson_loads(dump)))
        assert ret is True
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EBlockDelete('4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EVlobDelete('1234'),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        perform_sequence(sequence, manifest.delete_file('/foo'))
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, manifest.dumps())
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, manifest.check_consistency(ejson_loads(dump)))
        assert ret is True
        # With a bad vlob
        manifest.add_file('/bad', bad_vlob)
        sequence = [
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed']),
                lambda _: raise_(VlobNotFound('Vlob not found.'))),
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, manifest.dumps())
        sequence = [
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed']),
                lambda _: raise_(VlobNotFound('Vlob not found.'))),
        ]
        ret = perform_sequence(sequence, manifest.check_consistency(ejson_loads(dump)))
        assert ret is False
        sequence = [
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed']),
                lambda _: {'id': bad_vlob['id'], 'blob': blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed'], 1),
                lambda _: {'id': bad_vlob['id'], 'blob': blob, 'version': 1}),
            (EBlockDelete('4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EVlobDelete(bad_vlob['id']),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        perform_sequence(sequence, manifest.delete_file('/bad'))
        sequence = [
            (EVlobRead(vlob_id, '42'),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed'], None),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        dump = perform_sequence(sequence, manifest.dumps())
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                lambda _: {'id': vlob_id, 'blob': blob, 'version': 1}),
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed']),
                lambda _: raise_(VlobNotFound('Vlob not found.'))),
        ]
        ret = perform_sequence(sequence, manifest.check_consistency(ejson_loads(dump)))
        assert ret is False


class TestGroupManifest:

    def test_create(self, group_manifest):
        ret = group_manifest.get_vlob()
        assert ret == {'id': '1234',
                       'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                       'read_trust_seed': '42',
                       'write_trust_seed': '43'}
        ret = perform_sequence([], group_manifest.get_version())
        assert ret == 0

    def test_load(self, group_manifest):
        vlob = group_manifest.get_vlob()
        blob = {'entries': {'/': None}, 'dustbin': [], 'versions': {}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1}),
        ]
        manifest = perform_sequence(sequence, GroupManifest.load(**vlob))
        version = perform_sequence([], manifest.get_version())
        assert version == 1

    def test_get_vlob(self, group_manifest):
        ret = group_manifest.get_vlob()
        assert ret == {'id': '1234',
                       'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                       'read_trust_seed': '42',
                       'write_trust_seed': '43'}

    def test_update_vlob(self, group_manifest):
        new_vlob = {
            'id': 'i123',
            'key': 'DOXnMEmt34+PdAnCLMVgXmKmk3SFjfWxDgx3zeKpbyU=\n',
            'read_trust_seed': 'rts',
            'write_trust_seed': 'wts'
        }
        group_manifest.update_vlob(new_vlob)
        assert group_manifest.get_vlob() == new_vlob

    def test_diff_versions(self, group_manifest):
        foo_vlob = {'id': '123', 'key': '123', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        bar_vlob = {'id': '234', 'key': '123', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        # Old version (0) and new version (0) of non-commited manifest
        diff = perform_sequence([], group_manifest.diff_versions(0, 0))
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # No old version (use original) and no new version (dump current)
        group_manifest.add_file('/foo', foo_vlob)
        sequence = [
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed']),
                lambda _: {'id': foo_vlob['id'], 'blob': '', 'version': 1})
        ]
        diff = perform_sequence(sequence, group_manifest.diff_versions())
        assert diff == {'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {foo_vlob['id']: 1},
                                     'changed': {},
                                     'removed': {}}}
        # Old version (2) and new version (4)
        old_blob = {'entries': {'/': None, '/foo': foo_vlob},
                    'dustbin': [],
                    'versions': {'123': 1}}
        old_blob = ejson_dumps(old_blob).encode()
        old_blob = to_jsonb64(old_blob)
        new_blob = {'entries': {'/': None, '/foo': foo_vlob, '/bar': bar_vlob},
                    'dustbin': [],
                    'versions': {'123': 1, '234': 2}}
        new_blob = ejson_dumps(new_blob).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EVlobRead('1234', '42', 2),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 2}),
            (EVlobRead('1234', '42', 4),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 4})
        ]
        diff = perform_sequence(sequence, group_manifest.diff_versions(2, 4))
        assert diff == {'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [],
                                    'removed': []},
                        'versions': {'added': {bar_vlob['id']: 2}, 'changed': {}, 'removed': {}}}

    def test_reload_not_consistent(self, group_manifest):
        file_vlob = {'id': '123',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '123',
                     'write_trust_seed': '123'}
        blob = {'entries': {'/': None, '/foo': file_vlob},
                'dustbin': [],
                'versions': {'123': 1}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        with pytest.raises(ManifestError):
            sequence = [
                (EVlobRead('1234', '42'),
                    lambda _: {'id': '1234', 'blob': blob, 'version': 2}),
                (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed'], 1),
                    lambda _: raise_(VlobNotFound('Vlob not found.'))),
            ]
            perform_sequence(sequence, group_manifest.reload(reset=True))

    def test_reload_with_reset_and_new_version(self, group_manifest):
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        group_manifest.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 2}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed'], 1),
                lambda _: {'id': foo_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed'], 1),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobRead('123', '123'),
                lambda _: {'id': '123', 'blob': new_blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed']),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobList(),
                lambda _: [])
        ]
        ret = perform_sequence(sequence, group_manifest.reload(reset=True))
        assert ret is None
        assert group_manifest.version == 2
        assert group_manifest.original_manifest == new_blob_dict
        assert group_manifest.entries['/foo'] == foo_vlob
        assert '/bar' not in group_manifest.entries

    def test_reload_with_reset_no_new_version(self, group_manifest):
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        group_manifest.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 1}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed'], 1),
                lambda _: {'id': foo_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed'], 1),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobRead('123', '123'),
                lambda _: {'id': '123', 'blob': new_blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed']),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobList(),
                lambda _: [])
        ]
        ret = perform_sequence(sequence, group_manifest.reload(reset=True))
        assert ret is None
        assert group_manifest.version == 1
        assert group_manifest.original_manifest == new_blob_dict
        assert group_manifest.entries['/foo'] == foo_vlob
        assert '/bar' not in group_manifest.entries

    def test_reload_without_reset_and_new_version(self, group_manifest):
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        group_manifest.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 2}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed'], 1),
                lambda _: {'id': foo_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed'], 1),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobRead(bar_vlob['id'], bar_vlob['read_trust_seed']),
                lambda _: {'id': bar_vlob['id'], 'blob': to_jsonb64(b'bar'), 'version': 1}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed']),
                lambda _: {'id': foo_vlob['id'], 'blob': new_blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed']),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobList(),
                lambda _: [])
        ]
        ret = perform_sequence(sequence, group_manifest.reload(reset=False))
        assert ret is None
        assert group_manifest.version == 2
        assert group_manifest.original_manifest == new_blob_dict
        assert group_manifest.entries['/bar'] == bar_vlob
        assert group_manifest.entries['/foo'] == foo_vlob

    def test_reload_without_reset_and_no_new_version(self, group_manifest):
        group_manifest.version = 1
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        group_manifest.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, group_manifest.reload(reset=False))
        assert ret is None
        assert group_manifest.version == 1
        assert '/foo' not in group_manifest.entries
        assert group_manifest.entries['/bar'] == bar_vlob

    def test_commit(self, group_manifest):
        # Save first time
        manifest_vlob_id = '1234'
        manifest_new_vlob = {'id': '2345', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        manifest_blob = {'entries': {'/': None}, 'dustbin': [], 'versions': {}}
        manifest_blob = ejson_dumps(manifest_blob).encode()
        manifest_blob = to_jsonb64(manifest_blob)
        sequence = [
            (EVlobUpdate(manifest_vlob_id, '43', 1, manifest_blob),
                lambda _: None),
            (EVlobSynchronize(manifest_vlob_id),
                lambda _: manifest_new_vlob)
        ]
        ret = perform_sequence(sequence, group_manifest.commit())
        manifest_new_vlob['key'] = to_jsonb64(b'<dummy-key-00000000000000000001>')
        assert ret == manifest_new_vlob
        version = perform_sequence([], group_manifest.get_version())
        assert version == 1
        assert group_manifest.version == 1
        # Modify and commit
        file_vlob_id = '3456'
        block_id = '4567'
        file_blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                      'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        file_blob = ejson_dumps(file_blob).encode()
        file_blob = to_jsonb64(file_blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(file_blob),
                lambda _: {'id': file_vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        file_vlob = file.get_vlob()
        group_manifest.add_file('/foo', file_vlob)

        manifest_blob = {'entries': {'/': None, '/foo': file_vlob},
                         'dustbin': [],
                         'versions': {file_vlob_id: 1}}
        manifest_blob = ejson_dumps(manifest_blob).encode()
        manifest_blob = to_jsonb64(manifest_blob)
        sequence = [
            (EVlobRead(file_vlob_id, '42'),
                lambda _: {'id': file_vlob_id, 'blob': file_blob, 'version': 1}),
            (EVlobRead(file_vlob_id, '42'),
                lambda _: {'id': file_vlob_id, 'blob': file_blob, 'version': 1}),
            (EVlobUpdate(manifest_new_vlob['id'],
                         manifest_new_vlob['write_trust_seed'],
                         2,
                         manifest_blob),
                lambda _: None),
            (EVlobSynchronize(manifest_new_vlob['id']),
                lambda _: True)
        ]
        ret = perform_sequence(sequence, group_manifest.commit())
        assert group_manifest.get_vlob() == manifest_new_vlob
        sequence = [
            (EVlobRead(file_vlob_id, '42'),
                lambda _: {'id': file_vlob_id, 'blob': file_blob, 'version': 1})
        ]
        version = perform_sequence(sequence, group_manifest.get_version())
        assert version == 2
        assert group_manifest.version == 2
        # Save without modifications
        sequence = [
            (EVlobRead(file_vlob_id, '42'),
                lambda _: {'id': file_vlob_id, 'blob': file_blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, group_manifest.commit())
        sequence = [
            (EVlobRead(file_vlob_id, '42'),
                lambda _: {'id': file_vlob_id, 'blob': file_blob, 'version': 1})
        ]
        version = perform_sequence(sequence, group_manifest.get_version())
        assert version == 2
        assert group_manifest.version == 2

    def test_reencrypt(self, group_manifest):
        old_id = group_manifest.id
        old_key = group_manifest.encryptor.key
        old_read_trust_seed = group_manifest.read_trust_seed
        old_write_trust_seed = group_manifest.write_trust_seed
        assert group_manifest.version == 0
        group_manifest.entries['/foo'] = {'id': '2345',
                                          'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                                          'read_trust_seed': 'rts',
                                          'write_trust_seed': 'wts'}
        group_manifest.dustbin = [{'path': '/bar',
                                   'removed_date': '2012-01-01T00:00:00',
                                   'id': '3456',
                                   'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                                   'read_trust_seed': 'rts',
                                   'write_trust_seed': 'wts'}]
        new_blob = {'entries': {'/': None,
                                '/foo': {'id': '2345new',
                                         'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                                         'read_trust_seed': 'rtsnew',
                                         'write_trust_seed': 'wtsnew'}},
                    'dustbin': [{'path': '/bar',
                                 'removed_date': '2012-01-01T00:00:00',
                                 'id': '3456new',
                                 'key': to_jsonb64(b'<dummy-key-00000000000000000003>'),
                                 'read_trust_seed': 'rtsnew',
                                 'write_trust_seed': 'wtsnew'}],
                    'versions': {'2345new': 1, '3456new': 1}}
        new_blob = ejson_dumps(new_blob).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EVlobRead('2345', 'rts'),
                lambda _: {'id': '2345', 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead('2345', 'rts', 1),
                lambda _: {'id': '2345', 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobCreate(to_jsonb64(b'foo')),
                lambda _: {'id': '2345new',
                           'read_trust_seed': 'rtsnew',
                           'write_trust_seed': 'wtsnew'}),
            (EVlobRead('3456', 'rts'),
                lambda _: {'id': '3456', 'blob': to_jsonb64(b'bar'), 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead('3456', 'rts', 1),
                lambda _: {'id': '3456', 'blob': to_jsonb64(b'bar'), 'version': 1}),
            (EVlobCreate(to_jsonb64(b'bar')),
                lambda _: {'id': '3456new',
                           'read_trust_seed': 'rtsnew',
                           'write_trust_seed': 'wtsnew'}),
            (EVlobRead('2345new', 'rtsnew'),
                lambda _: {'id': '2345', 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead('3456new', 'rtsnew'),
                lambda _: {'id': '3456new', 'blob': to_jsonb64(b'bar'), 'version': 1}),
            (EVlobCreate(new_blob),
                lambda _: {'id': '1234new',
                           'read_trust_seed': 'rtsnew',
                           'write_trust_seed': 'wtsnew'}),
        ]
        ret = perform_sequence(sequence, group_manifest.reencrypt())
        assert ret is None
        assert group_manifest.id != old_id
        assert group_manifest.encryptor.key != old_key
        assert group_manifest.read_trust_seed != old_read_trust_seed
        assert group_manifest.write_trust_seed != old_write_trust_seed
        assert group_manifest.version == 0

    def test_restore_manifest(self, group_manifest):
        block_id = '4567'
        file_blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                      'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        file_blob = ejson_dumps(file_blob).encode()
        file_blob = to_jsonb64(file_blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(file_blob),
                lambda _: {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        file_vlob = file.get_vlob()
        # Restore dirty manifest with version 1
        group_manifest.add_file('/tmp', {'id': '123', 'read_trust_seed': 'rts'})
        group_manifest.version = 1
        blob = ejson_dumps(group_manifest.original_manifest).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, group_manifest.restore())
        assert ret is None
        assert group_manifest.version == 1
        assert '/tmp' not in group_manifest.entries
        # Restore previous version
        group_manifest.version = 6
        blob_dict = {'entries': {'/': None, '/foo': file_vlob},
                     'dustbin': [],
                     'versions': {'2345': 2}}
        blob = ejson_dumps(blob_dict).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead('1234', '42', 5),
                lambda _: {'id': '1234', 'blob': blob, 'version': 5}),
            (EVlobUpdate('1234', '43', 6, blob),
                lambda _: None),
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 6}),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobRead('2345', '42'),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead('2345', '42', 3),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EBlockDelete('4567'),
                lambda _: None),
            (EVlobDelete('2345'),
                lambda _: None),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobUpdate('2345', '43', 4, file_blob),  # TODO modify file_blob
                lambda _: None),
        ]
        perform_sequence(sequence, group_manifest.restore())
        assert group_manifest.version == 6
        # Restore old version
        group_manifest.version = 6
        blob_dict = {'entries': {'/': None, '/foo': file_vlob},
                     'dustbin': [],
                     'versions': {'2345': 2}}
        blob = ejson_dumps(blob_dict).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead('1234', '42', 3),
                lambda _: {'id': '1234', 'blob': blob, 'version': 3}),
            (EVlobUpdate('1234', '43', 6, blob),
                lambda _: None),
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 6}),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobRead('2345', '42'),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead('2345', '42', 3),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EBlockDelete('4567'),
                lambda _: None),
            (EVlobDelete('2345'),
                lambda _: None),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobUpdate('2345', '43', 4, file_blob),  # TODO modify file_blob
                lambda _: None),
        ]
        perform_sequence(sequence, group_manifest.restore(3))
        assert group_manifest.version == 6
        # Bad version
        with pytest.raises(ManifestError):
            perform_sequence([], group_manifest.restore(10))
        # Restore not commited manifest
        group_manifest.version = 0
        with pytest.raises(ManifestError):
            perform_sequence([], group_manifest.restore())


class TestUserManifest:

    def test_load(self, user_manifest):
        # Create
        version = perform_sequence([], user_manifest.get_version())
        assert version == 0
        # Load
        blob = {'entries': {'/': None}, 'groups': {}, 'dustbin': [], 'versions': {}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': blob, 'version': 1})
        ]
        user_manifest = perform_sequence(sequence, UserManifest.load(ALICE_PRIVATE_RSA))
        version = perform_sequence([], user_manifest.get_version())
        assert version == 1

    def test_diff_versions(self, user_manifest):
        foo_vlob = {'id': '123', 'key': '123', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        bar_vlob = {'id': '234', 'key': '123', 'read_trust_seed': 'rts', 'write_trust_seed': 'wts'}
        # Old version (0) and new version (0) of non-commited manifest
        diff = perform_sequence([], user_manifest.diff_versions(0, 0))
        assert diff == {'entries': {'added': {}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {}, 'changed': {}, 'removed': {}}}
        # No old version (use original) and no new version (dump current)
        user_manifest.add_file('/foo', foo_vlob)
        group_vlob = {'id': '345',
                      'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                      'read_trust_seed': 'rts',
                      'write_trust_seed': 'wts'}
        blob = {'entries': {'/': None}, 'dustbin': [], 'versions': {}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead(group_vlob['id'], group_vlob['read_trust_seed']),
                lambda _: {'blob': blob, 'version': 1})
        ]
        perform_sequence(sequence, user_manifest.import_group_vlob('share', group_vlob))
        sequence = [
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed']),
                lambda _: {'id': foo_vlob['id'], 'blob': blob, 'version': 1})
        ]
        diff = perform_sequence(sequence, user_manifest.diff_versions())
        assert diff == {'entries': {'added': {'/foo': foo_vlob}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {'share': group_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'removed': [], 'added': []},
                        'versions': {'added': {foo_vlob['id']: 1},
                                     'changed': {},
                                     'removed': {}}}
        # Old version (2) and new version (4)
        old_blob = {'entries': {'/': None, '/foo': foo_vlob},
                    'groups': {},
                    'dustbin': [],
                    'versions': {'123': 1}}
        old_blob = ejson_dumps(old_blob).encode()
        old_blob = to_jsonb64(old_blob)
        new_blob = {'entries': {'/': None, '/foo': foo_vlob, '/bar': bar_vlob},
                    'groups': {'share': group_vlob},
                    'dustbin': [],
                    'versions': {'123': 1, '234': 2}}
        new_blob = ejson_dumps(new_blob).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EUserVlobRead(2),
                lambda _: {'id': '1234', 'blob': old_blob, 'version': 2}),
            (EUserVlobRead(4),
                lambda _: {'id': '1234', 'blob': new_blob, 'version': 4})
        ]
        diff = perform_sequence(sequence, user_manifest.diff_versions(2, 4))
        assert diff == {'entries': {'added': {'/bar': bar_vlob}, 'changed': {}, 'removed': {}},
                        'groups': {'added': {'share': group_vlob}, 'changed': {}, 'removed': {}},
                        'dustbin': {'added': [],
                                    'removed': []},
                        'versions': {'added': {bar_vlob['id']: 2}, 'changed': {}, 'removed': {}}}

    def test_dumps_current_manifest(self, user_manifest_with_group):
        block_id = '4567'
        file_blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                      'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        file_blob = ejson_dumps(file_blob).encode()
        file_blob = to_jsonb64(file_blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(file_blob),
                lambda _: {'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        file_vlob = file.get_vlob()
        user_manifest_with_group.add_file('/foo', file_vlob)
        sequence = [
            (EVlobRead(file_vlob['id'], '42'),
                lambda _: {'id': file_vlob['id'], 'blob': file_blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, user_manifest_with_group.dumps(original_manifest=False))
        dump = ejson_loads(dump)
        group_vlob = dump['groups']['share']
        assert dump == {'entries': {'/': None,
                                    '/foo': file_vlob},
                        'dustbin': [],
                        'groups': {'share': group_vlob},
                        'versions': {file_vlob['id']: 1}}

    def test_dumps_original_manifest(self, user_manifest_with_group):
        block_id = '4567'
        file_blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                      'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        file_blob = ejson_dumps(file_blob).encode()
        file_blob = to_jsonb64(file_blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(file_blob),
                lambda _: {'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        file_vlob = file.get_vlob()
        user_manifest_with_group.add_file('/foo', file_vlob)
        dump = perform_sequence([], user_manifest_with_group.dumps(original_manifest=True))
        dump = ejson_loads(dump)
        group_vlob = dump['groups']['share']
        assert dump == {'entries': {'/': None},
                        'dustbin': [],
                        'groups': {'share': group_vlob},
                        'versions': {}}

    @pytest.mark.parametrize('group', [None, 'share2'])
    def test_get_group_vlobs(self, user_manifest_with_group, group_manifest, group):
        group_vlob = {'id': '456',
                      'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                      'read_trust_seed': 'rts',
                      'write_trust_seed': 'wts'}
        group_blob = {'entries': {'/': None}, 'dustbin': [], 'versions': {}}
        group_blob = ejson_dumps(group_blob).encode()
        group_blob = to_jsonb64(group_blob)
        sequence = [
            (EVlobCreate(''),
                lambda _: {'id': group_vlob['id'],
                           'read_trust_seed': group_vlob['read_trust_seed'],
                           'write_trust_seed': group_vlob['write_trust_seed']}),
            (EVlobUpdate(group_vlob['id'], group_vlob['write_trust_seed'], 1, group_blob),
                lambda _: None)
        ]
        perform_sequence(sequence, user_manifest_with_group.create_group_manifest('share2'))
        group_vlobs = user_manifest_with_group.get_group_vlobs(group)
        if group:
            assert group_vlobs == {group: group_vlob}
        else:
            assert group_vlobs == {'share': group_manifest.get_vlob(), 'share2': group_vlob}
        # Not found
        with pytest.raises(ManifestNotFound):
            user_manifest_with_group.get_group_vlobs('unknown')

    def test_get_group_manifest(self, user_manifest_with_group, group_manifest):
        retrieved_group_manifest = user_manifest_with_group.get_group_manifest('share')
        assert isinstance(retrieved_group_manifest, GroupManifest)
        assert retrieved_group_manifest.get_vlob() == group_manifest.get_vlob()
        # Not found
        with pytest.raises(ManifestNotFound):
            user_manifest_with_group.get_group_manifest('unknown')

    def test_reencrypt_group_manifest(self, user_manifest_with_group):
        file_vlob = {'id': '123',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                     'read_trust_seed': 'rts',
                     'write_trust_seed': 'wts'}
        group_manifest = user_manifest_with_group.get_group_manifest('share')
        group_manifest.make_folder('/test_dir')
        group_manifest.add_file('/foo', file_vlob)
        group_manifest_vlob = group_manifest.get_vlob()
        group_blob_dict = {'entries': {'/': None, '/foo': file_vlob}, 'dustbin': [], 'versions': {}}
        group_blob = ejson_dumps(group_blob_dict).encode()
        group_blob = to_jsonb64(group_blob)
        new_file_vlob = {'id': '234',
                         'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                         'read_trust_seed': 'rtsnew',
                         'write_trust_seed': 'wtsnew'}
        new_group_blob_dict = {'entries': {'/': None,
                                           '/test_dir': None,
                                           '/foo': new_file_vlob},
                               'dustbin': [],
                               'versions': {'234': 1}}
        new_group_blob = ejson_dumps(new_group_blob_dict).encode()
        new_group_blob = to_jsonb64(new_group_blob)
        sequence = [
            (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed']),
                lambda _: {'id': file_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed'], 1),
                lambda _: {'id': file_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobCreate(to_jsonb64(b'foo')),
                lambda _: {'id': new_file_vlob['id'],
                           'read_trust_seed': new_file_vlob['read_trust_seed'],
                           'write_trust_seed': new_file_vlob['write_trust_seed'],
                           'version': 1}),
            (EVlobRead('234', 'rtsnew'),
                lambda _: {'id': '234', 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobCreate(new_group_blob),
                lambda _: {'id': '2345',
                           'read_trust_seed': 'rtsnew',
                           'write_trust_seed': 'wtsnew',
                           'version': 1})
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.reencrypt_group_manifest('share'))
        assert ret is None
        new_group_manifest = user_manifest_with_group.get_group_manifest('share')
        assert group_manifest_vlob != new_group_manifest.get_vlob()
        assert isinstance(group_manifest, GroupManifest)
        # Not found
        with pytest.raises(ManifestNotFound):
            perform_sequence([], user_manifest_with_group.reencrypt_group_manifest('unknown'))

    def test_create_group_manifest(self, user_manifest):
        with pytest.raises(ManifestNotFound):
            user_manifest.get_group_manifest('share')
        vlob = {'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'}
        blob = to_jsonb64(b'{"dustbin": [], "entries": {"/": null}, "versions": {}}')
        sequence = [
            (EVlobCreate(),
                lambda _: vlob),
            (EVlobUpdate(vlob['id'], vlob['write_trust_seed'], 1, blob),
                lambda _: vlob)
        ]
        ret = perform_sequence(sequence, user_manifest.create_group_manifest('share'))
        assert ret is None
        group_manifest = user_manifest.get_group_manifest('share')
        assert isinstance(group_manifest, GroupManifest)
        # Already exists
        with pytest.raises(ManifestError):
            perform_sequence([], user_manifest.create_group_manifest('share'))

    def test_import_group_vlob(self, user_manifest, group_manifest):
        blob = ejson_dumps(group_manifest.original_manifest).encode()
        blob = to_jsonb64(blob)
        vlob = group_manifest.get_vlob()
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, user_manifest.import_group_vlob('share', vlob))
        assert ret is None
        retrieved_manifest = user_manifest.get_group_manifest('share')
        assert retrieved_manifest.get_vlob() == vlob
        new_vlob = {'id': '2345',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': 'rts',
                    'write_trust_seed': 'wts'}
        sequence = [
            (EVlobRead('2345', 'rts'),
                lambda _: {'id': '2345', 'blob': blob, 'version': 1}),
            (EVlobRead('2345', 'rts'),
                lambda _: {'id': '2345', 'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, user_manifest.import_group_vlob('share', new_vlob))
        retrieved_manifest = user_manifest.get_group_manifest('share')
        assert retrieved_manifest.get_vlob() == new_vlob

    def test_remove_group(self, user_manifest_with_group):
        user_manifest_with_group.remove_group('share')
        with pytest.raises(ManifestNotFound):
            user_manifest_with_group.remove_group('share')

    def test_reload_not_consistent(self, user_manifest):
        # File not consistent
        file_vlob = {'id': '123',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '123',
                     'write_trust_seed': '123'}
        blob = {'entries': {'/': None, '/foo': file_vlob},
                'groups': {},
                'dustbin': [],
                'versions': {'123': 1}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        with pytest.raises(ManifestError):
            sequence = [
                (EUserVlobRead(),
                    lambda _: {'blob': blob, 'version': 2}),
                (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed'], 1),
                    lambda _: raise_(VlobNotFound('Vlob not found.'))),
            ]
            perform_sequence(sequence, user_manifest.reload(reset=True))
        # Group (vlob and not content) not consistent
        group_vlob = {'id': '123',
                      'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                      'read_trust_seed': '123',
                      'write_trust_seed': '123'}
        blob = {'entries': {'/': None},
                'groups': {'share': group_vlob},
                'dustbin': [],
                'versions': {'123': 1}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        with pytest.raises(ManifestError):
            sequence = [
                (EUserVlobRead(),
                    lambda _: {'blob': blob, 'version': 2}),
                (EVlobRead(group_vlob['id'], group_vlob['read_trust_seed']),
                    lambda _: raise_(VlobNotFound('Vlob not found.'))),
            ]
            perform_sequence(sequence, user_manifest.reload(reset=True))

    def test_reload_with_reset_and_new_version(self, user_manifest_with_group, group_manifest):
        user_manifest_with_group.entries['/bar'] = {'id': '234'}
        file_vlob = {'id': '123',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '123',
                     'write_trust_seed': '123'}
        file_vlob_2 = {'id': '234',
                       'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                       'read_trust_seed': '234',
                       'write_trust_seed': '234'}
        dustbin_entry = deepcopy(file_vlob_2)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        new_blob_dict = {'entries': {'/': None, '/foo': file_vlob},
                         'groups': {'share': group_manifest.get_vlob()},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        group_blob = to_jsonb64(b'{"dustbin": [], "entries": {"/": null}, "versions": {}}')
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': new_blob, 'version': 2}),
            (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed'], 1),
                lambda _: {'id': file_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead(file_vlob_2['id'], file_vlob_2['read_trust_seed'], 1),
                lambda _: {'id': file_vlob_2['id'], 'blob': to_jsonb64(b'bar'), 'version': 1}),
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': group_blob, 'version': 1}),
            (EVlobRead(file_vlob['id'], file_vlob['read_trust_seed']),
                lambda _: {'id': file_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(file_vlob_2['id'], file_vlob_2['read_trust_seed']),
                lambda _: {'id': file_vlob_2['id'], 'blob': to_jsonb64(b'bar'), 'version': 1}),
            (EVlobList(),
                lambda _: [])
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.reload(reset=True))
        assert ret is None
        assert user_manifest_with_group.version == 2
        assert user_manifest_with_group.original_manifest == new_blob_dict
        assert user_manifest_with_group.entries['/foo'] == file_vlob
        assert '/bar' not in user_manifest_with_group.entries

    def test_reload_with_reset_no_new_version(self, user_manifest_with_group, group_manifest):
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        user_manifest_with_group.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'groups': {'share': group_manifest.get_vlob()},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        group_blob = to_jsonb64(b'{"dustbin": [], "entries": {"/": null}, "versions": {}}')
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': new_blob, 'version': 1}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed'], 1),
                lambda _: {'id': foo_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed'], 1),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': group_blob, 'version': 1}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed']),
                lambda _: {'id': foo_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed']),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobList(),
                lambda _: [])
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.reload(reset=True))
        assert ret is None
        assert user_manifest_with_group.version == 1
        assert user_manifest_with_group.original_manifest == new_blob_dict
        assert user_manifest_with_group.entries['/foo'] == foo_vlob
        assert '/bar' not in user_manifest_with_group.entries

    def test_reload_without_reset_and_new_version(self, user_manifest_with_group, group_manifest):
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        user_manifest_with_group.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'groups': {'share': group_manifest.get_vlob()},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1, '234': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        group_blob = to_jsonb64(b'{"dustbin": [], "entries": {"/": null}, "versions": {}}')
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': new_blob, 'version': 2}),
            (EVlobRead(foo_vlob['id'], foo_vlob['read_trust_seed'], 1),
                lambda _: {'id': foo_vlob['id'], 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed'], 1),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': group_blob, 'version': 1}),
            (EVlobRead('234', '234'),
                lambda _: {'id': '234', 'blob': to_jsonb64(b'foo'), 'version': 1}),
            (EVlobRead('123', '123'),
                lambda _: {'id': '123', 'blob': new_blob, 'version': 1}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead(dust_vlob['id'], dust_vlob['read_trust_seed']),
                lambda _: {'id': dust_vlob['id'], 'blob': to_jsonb64(b'dust'), 'version': 1}),
            (EVlobList(),
                lambda _: [])
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.reload(reset=False))
        assert ret is None
        assert user_manifest_with_group.version == 2
        assert user_manifest_with_group.original_manifest == new_blob_dict
        assert user_manifest_with_group.entries['/bar'] == bar_vlob
        assert user_manifest_with_group.entries['/foo'] == foo_vlob

    def test_reload_without_reset_and_no_new_version(self,
                                                     user_manifest_with_group,
                                                     group_manifest):
        user_manifest_with_group.version = 1
        bar_vlob = {'id': '234',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '234',
                    'write_trust_seed': '234'}
        foo_vlob = {'id': '123',
                    'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        dust_vlob = {'id': '234',
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>'),
                     'read_trust_seed': '234',
                     'write_trust_seed': '234'}
        dustbin_entry = deepcopy(dust_vlob)
        dustbin_entry['path'] = '/bar'
        dustbin_entry['removed_date'] = '2012-01-01T00:00:00'
        user_manifest_with_group.entries['/bar'] = bar_vlob
        new_blob_dict = {'entries': {'/': None, '/foo': foo_vlob},
                         'groups': {'share': group_manifest.get_vlob()},
                         'dustbin': [dustbin_entry],
                         'versions': {'123': 1}}
        new_blob = ejson_dumps(new_blob_dict).encode()
        new_blob = to_jsonb64(new_blob)
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': new_blob, 'version': 1}),
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.reload(reset=False))
        assert ret is None
        assert user_manifest_with_group.version == 1
        assert '/foo' not in user_manifest_with_group.entries
        assert user_manifest_with_group.entries['/bar'] == bar_vlob

    def test_commit(self, user_manifest_with_group):
        # Modify and save
        block_id = '4567'
        file_blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                      'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        file_blob = ejson_dumps(file_blob).encode()
        file_blob = to_jsonb64(file_blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(file_blob),
                lambda _: {'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        file_vlob = file.get_vlob()
        user_manifest_with_group.add_file('/foo', file_vlob)
        blob = {'entries': {'/': None,
                            '/foo': {'id': '1234',
                                     'key': to_jsonb64(b'<dummy-key-00000000000000000003>'),
                                     'read_trust_seed': '42', 'write_trust_seed': '43'}},
                'groups': {'share': user_manifest_with_group.group_manifests['share'].get_vlob()},
                'dustbin': [],
                'versions': {'1234': 1}}
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        group_blob = {'entries': {'/': None}, 'dustbin': [], 'versions': {}}
        group_blob = ejson_dumps(group_blob).encode()
        group_blob = to_jsonb64(group_blob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': file_vlob, 'version': 1}),
            (EVlobUpdate('1234', '43', 1, group_blob),
                lambda _: None),
            (EVlobSynchronize('1234'),
                lambda _: True),
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': file_vlob, 'version': 1}),
            (EUserVlobUpdate(1, blob),
                lambda _: None),
            (EUserVlobSynchronize(),
                lambda _: True)
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.commit())
        assert ret is None
        assert user_manifest_with_group.version == 1
        # Save without modifications
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': file_vlob, 'version': 1})
        ]
        ret = perform_sequence(sequence, user_manifest_with_group.commit())
        assert user_manifest_with_group.version == 1
        # TODO assert called methods

    def test_restore_manifest(self, user_manifest):
        block_id = '4567'
        file_blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                      'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        file_blob = ejson_dumps(file_blob).encode()
        file_blob = to_jsonb64(file_blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(file_blob),
                lambda _: {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        file_vlob = file.get_vlob()
        # Restore dirty manifest with version 1
        user_manifest.add_file('/tmp', {'id': '123', 'read_trust_seed': 'rts'})
        user_manifest.version = 1
        blob = ejson_dumps(user_manifest.original_manifest).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EUserVlobRead(),
                lambda _: {'blob': blob, 'version': 1})
        ]
        ret = perform_sequence(sequence, user_manifest.restore())
        assert ret is None
        assert user_manifest.version == 1
        assert '/tmp' not in user_manifest.entries
        # Restore previous version
        user_manifest.version = 6
        blob_dict = {'entries': {'/': None, '/foo': file_vlob},
                     'groups': {},
                     'dustbin': [],
                     'versions': {'2345': 2}}
        blob = ejson_dumps(blob_dict).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EUserVlobRead(5),
                lambda _: {'blob': blob, 'version': 5}),
            (EUserVlobUpdate(6, blob),
                lambda _: None),
            (EUserVlobRead(),
                lambda _: {'blob': blob, 'version': 6}),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobRead('2345', '42'),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead('2345', '42', 3),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EBlockDelete('4567'),
                lambda _: None),
            (EVlobDelete('2345'),
                lambda _: None),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobUpdate('2345', '43', 4, file_blob),  # TODO modify file_blob
                lambda _: None),
        ]
        perform_sequence(sequence, user_manifest.restore())
        assert user_manifest.version == 6
        # Restore old version
        user_manifest.version = 6
        blob_dict = {'entries': {'/': None, '/foo': file_vlob},
                     'groups': {},
                     'dustbin': [],
                     'versions': {'2345': 2}}
        blob = ejson_dumps(blob_dict).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EUserVlobRead(3),
                lambda _: {'blob': blob, 'version': 3}),
            (EUserVlobUpdate(6, blob),
                lambda _: None),
            (EUserVlobRead(),
                lambda _: {'blob': blob, 'version': 6}),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobRead('2345', '42'),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EVlobList(),
                lambda _: []),
            (EVlobRead('2345', '42', 3),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 3}),
            (EBlockDelete('4567'),
                lambda _: None),
            (EVlobDelete('2345'),
                lambda _: None),
            (EVlobRead('2345', '42', 2),
                lambda _: {'id': '2345', 'blob': file_blob, 'version': 2}),
            (EVlobUpdate('2345', '43', 4, file_blob),  # TODO modify file_blob
                lambda _: None),
        ]
        perform_sequence(sequence, user_manifest.restore(3))
        assert user_manifest.version == 6
        # Bad version
        with pytest.raises(ManifestError):
            perform_sequence([], user_manifest.restore(10))
        # Restore not commited manifest
        user_manifest.version = 0
        with pytest.raises(ManifestError):
            perform_sequence([], user_manifest.restore())

    def test_check_consistency(self, user_manifest_with_group, group_manifest):
        vlob_id = '1234'
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                lambda _: block_id),
            (EVlobCreate(blob),
                lambda _: {'id': vlob_id, 'read_trust_seed': '42', 'write_trust_seed': '43'}),
        ]
        file = perform_sequence(sequence, File.create())
        good_vlob = file.get_vlob()
        encryptor = generate_sym_key()
        bad_vlob = {'id': '123',
                    'key': to_jsonb64(encryptor.key),
                    'read_trust_seed': '123',
                    'write_trust_seed': '123'}
        # With good vlobs only
        user_manifest_with_group.add_file('/foo', good_vlob)
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, user_manifest_with_group.dumps())
        group_blob = {'entries': {'/': None}, 'dustbin': [], 'versions': {}}
        group_blob = ejson_dumps(group_blob).encode()
        group_blob = to_jsonb64(group_blob)
        sequence = [
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1}),
            (EVlobRead('1234', '42', None),
                lambda _: {'id': '1234', 'blob': group_blob, 'version': 1}),
        ]
        consistency = perform_sequence(
            sequence, user_manifest_with_group.check_consistency(ejson_loads(dump)))
        assert consistency is True
        # With a bad vlob
        group_manifest.update_vlob(bad_vlob)
        user_manifest_with_group.group_manifests['share'] = group_manifest
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, user_manifest_with_group.dumps())
        sequence = [
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1}),
            (EVlobRead(bad_vlob['id'], bad_vlob['read_trust_seed']),
                lambda _: raise_(VlobNotFound('Vlob not found.')))
        ]
        consistency = perform_sequence(
            sequence, user_manifest_with_group.check_consistency(ejson_loads(dump)))
        assert consistency is False
        user_manifest_with_group.remove_group('share')
        sequence = [
            (EVlobRead('1234', '42'),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1})
        ]
        dump = perform_sequence(sequence, user_manifest_with_group.dumps())
        sequence = [
            (EVlobRead('1234', '42', 1),
                lambda _: {'id': '1234', 'blob': blob, 'version': 1})
        ]
        consistency = perform_sequence(
            sequence, user_manifest_with_group.check_consistency(ejson_loads(dump)))
        assert consistency is True
