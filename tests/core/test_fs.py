import asyncio
import pytest
from effect2 import do, Effect
from effect2.testing import const, conste, noop, perform_sequence
from freezegun import freeze_time
from unittest.mock import Mock

from parsec.core.file import File
from parsec.core.fs import (FSComponent, ERegisterMountpoint, EUnregisterMountpoint, ESynchronize,
                            EGroupCreate, EDustbinShow, EManifestHistory, EManifestRestore,
                            EFileCreate, EFileRead, EFileWrite, EFileTruncate, EFileHistory,
                            EFileRestore, EFolderCreate, EStat, EMove, EDelete, EUndelete)
from parsec.core.identity import EIdentityGet, IdentityComponent, Identity
from parsec.core.synchronizer import (
    EUserVlobSynchronize, EUserVlobRead, EUserVlobUpdate, EVlobCreate, EVlobList, EVlobRead,
    EVlobUpdate, EVlobDelete, EBlockCreate, EBlockDelete, SynchronizerComponent)
from parsec.exceptions import (
    ManifestError, BlockNotFound, VlobNotFound)
from parsec.tools import ejson_dumps, to_jsonb64, digest


@pytest.fixture
def app(mock_crypto_passthrough, alice_identity):
    # app = FSComponent()
    # identity_component = IdentityComponent()
    fs_component = FSComponent()
    # synchronizer_component = SynchronizerComponent()
    # identity_component = IdentityComponent()
    # app = app_factory(
    #     fs_component.get_dispatcher(),
    #     synchronizer_component.get_dispatcher(),
    #     identity_component.get_dispatcher()
    # )
    blob = {'dustbin': [], 'entries': {'/': None}, 'groups': {}, 'versions': {}}
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EUserVlobRead(),
            const({'blob': '', 'version': 0})),
        (EUserVlobUpdate(1, blob),
            noop)
    ]
    perform_sequence(sequence, fs_component._get_manifest())
    return fs_component


@pytest.fixture
def file(app, alice_identity, mock_crypto_passthrough):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    block_id = '4567'
    blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_create(EFileCreate('/foo'))
    sequence = [
        (EBlockCreate(''), const(block_id)),
        (EVlobCreate(blob), const(vlob)),
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
    File.files = {}


def test_perform_register_mountpoint(app):
    assert len(app.mountpoint) == 0
    ret = app.perform_register_mountpoint(ERegisterMountpoint('/mnt/parsec'))
    assert ret is None
    assert len(app.mountpoint) == 1


def test_perform_unregister_mountpoint(app):
    # Register mountpoint
    ret = app.perform_register_mountpoint(ERegisterMountpoint('/mnt/parsec'))
    # Unregister mountpoint
    assert len(app.mountpoint) == 1
    ret = app.perform_unregister_mountpoint(EUnregisterMountpoint('/mnt/parsec'))
    assert ret is None
    assert len(app.mountpoint) == 0


def test_perform_synchronize(app, alice_identity):
    blob = {'dustbin': [], 'entries': {'/': None}, 'groups': {}, 'versions': {}}
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_synchronize(ESynchronize())
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EVlobList(), const([])),
        (EUserVlobUpdate(1, blob), noop),
        (EUserVlobSynchronize(), noop)
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_group_create(app, alice_identity):
    blob = {'dustbin': [], 'entries': {'/': None}, 'versions': {}}
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_group_create(EGroupCreate('share'))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EVlobCreate(),
            const({'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'})),
        (EVlobUpdate('1234', '43', 1, blob),
            noop)
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_dustbin_show(app, alice_identity, file):
    with freeze_time('2012-01-01') as frozen_datetime:
        vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
        blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        eff = app.perform_delete(EDelete('/foo'))
        sequence = [
            (EIdentityGet(), const(alice_identity)),
            (EVlobRead(vlob['id'], vlob['read_trust_seed']),
                const({'id': vlob['id'], 'blob': blob, 'version': 1})),
            (EVlobList(), const([])),
            (EVlobRead(vlob['id'], vlob['read_trust_seed'], 1),
                const({'id': vlob['id'], 'blob': blob, 'version': 1})),
            (EBlockDelete('4567'),
                conste(BlockNotFound('Block not found.'))),
            (EVlobDelete('2345'),
                conste(VlobNotFound('Vlob not found.'))),
        ]
        perform_sequence(sequence, eff)
        eff = app.perform_dustbin_show(EDustbinShow())
        sequence = [
            (EIdentityGet(), const(alice_identity))
        ]
        dustbin = perform_sequence(sequence, eff)
        vlob['path'] = '/foo'
        vlob['removed_date'] = frozen_datetime().isoformat()
        vlob['key'] = to_jsonb64(b'<dummy-key-00000000000000000002>')
        assert dustbin == [vlob]


def test_perform_manifest_history(app, alice_identity):
    eff = app.perform_manifest_history(EManifestHistory())
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    history = perform_sequence(sequence, eff)
    assert history == {'detailed_history': []}


def test_perform_manifest_restore(app, alice_identity):
    eff = app.perform_manifest_restore(EManifestRestore())
    with pytest.raises(ManifestError):
        sequence = [
            (EIdentityGet(), const(alice_identity))
        ]
        perform_sequence(sequence, eff)


def test_perform_file_create(app, alice_identity, file):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    block_id = '4567'
    # Already exist
    blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000003>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_create(EFileCreate('/foo'))
    sequence = [
        (EBlockCreate(''), const(block_id)),
        (EVlobCreate(blob), const(vlob)),
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed'], 1),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EBlockDelete(block_id), noop),
        (EVlobDelete(vlob['id']), noop),
    ]
    with pytest.raises(ManifestError):
        perform_sequence(sequence, eff)


def test_perform_file_read(app, file, alice_identity):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_read(EFileRead('/foo'))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed']),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EVlobList(),
            const([vlob['id']])),
        (EVlobRead(vlob['id'], vlob['read_trust_seed'], 1),
            const({'id': vlob['id'], 'blob': blob, 'version': 1}))
    ]
    file = perform_sequence(sequence, eff)
    assert file == b''


def test_perform_file_write(app, file, alice_identity):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_write(EFileWrite('/foo', b'foo', 0))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed']),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EVlobList(),
            const([vlob['id']]))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_file_truncate(app, file, alice_identity):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_truncate(EFileTruncate('/foo', 0))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed']),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EVlobList(),
            const([vlob['id']]))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


@pytest.mark.xfail
def test_perform_file_history(app, file, alice_identity):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_history(EFileHistory('/foo', 1, 1))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed']),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EVlobList(),
            const([])),
    ]
    perform_sequence(sequence, eff)


def test_perform_file_restore(app, file, alice_identity):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_file_restore(EFileRestore('/foo'))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed']),
            const({'id': vlob['id'], 'blob': blob, 'version': 2})),
        (EVlobList(),
            const([])),
        (EVlobRead(vlob['id'], vlob['read_trust_seed'], 2),
            const({'id': vlob['id'], 'blob': blob, 'version': 2})),
        (EBlockDelete('4567'),
            noop),
        (EVlobDelete(vlob['id']),
            noop),
        (EVlobRead(vlob['id'], vlob['read_trust_seed'], 1),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EVlobUpdate(vlob['id'], vlob['write_trust_seed'], 3, blob),
            noop),
    ]
    perform_sequence(sequence, eff)


def test_perform_folder_create(app, alice_identity):
    eff = app.perform_folder_create(EFolderCreate('/dir'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_stat(app, alice_identity, file):
    eff = app.perform_folder_create(EFolderCreate('/dir'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    eff = app.perform_stat(EStat('/dir'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == {'children': [], 'type': 'folder', 'mountpoint': ''}


def test_perform_move(app, alice_identity):
    eff = app.perform_folder_create(EFolderCreate('/dir'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    eff = app.perform_move(EMove('/dir', '/dir2'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_delete(app, alice_identity):
    eff = app.perform_folder_create(EFolderCreate('/dir'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    eff = app.perform_delete(EDelete('/dir'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_undelete(app, alice_identity, file):
    vlob = {'id': '2345', 'read_trust_seed': '42', 'write_trust_seed': '43'}
    blob = [{'blocks': [{'block': '4567', 'digest': digest(b''), 'size': 0}],
             'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
    blob = ejson_dumps(blob).encode()
    blob = to_jsonb64(blob)
    eff = app.perform_delete(EDelete('/foo'))
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EVlobRead(vlob['id'], vlob['read_trust_seed']),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EVlobList(),
            const([])),
        (EVlobRead(vlob['id'], vlob['read_trust_seed'], 1),
            const({'id': vlob['id'], 'blob': blob, 'version': 1})),
        (EBlockDelete('4567'),
            conste(BlockNotFound('Block not found.'))),
        (EVlobDelete('2345'),
            conste(VlobNotFound('Vlob not found.')))
    ]
    ret = perform_sequence(sequence, eff)
    eff = app.perform_undelete(EUndelete('2345'))
    sequence = [
        (EIdentityGet(), const(alice_identity))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
