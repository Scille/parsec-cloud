import pytest
from effect2.testing import perform_sequence

from parsec.core.core_api import execute_cmd
from parsec.core.fs import (
    ESynchronize, EGroupCreate, EDustbinShow, EManifestHistory, EManifestRestore,
    EFileCreate, EFileRead, EFileWrite, EFileTruncate, EFileHistory, EFileRestore, EFolderCreate,
    EStat, EMove, EDelete, EUndelete
)
from parsec.tools import to_jsonb64


def test_api_synchronize():
    eff = execute_cmd('synchronize', {})
    sequence = [
        (ESynchronize(),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_group_create():
    eff = execute_cmd('group_create', {'group': 'share'})
    sequence = [
        (EGroupCreate('share'),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_dustbin_show():
    # Specific file in dustbin
    eff = execute_cmd('dustbin_show', {'path': '/foo'})
    sequence = [
        (EDustbinShow('/foo'),
            lambda _: [{'path': '/foo'}]),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'dustbin': [{'path': '/foo'}]}
    # All files in dustbin
    eff = execute_cmd('dustbin_show', {})
    sequence = [
        (EDustbinShow(),
            lambda _: [{'path': '/foo'}, {'path': '/bar'}]),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'dustbin': [{'path': '/foo'}, {'path': '/bar'}]}


def test_api_manifest_history():
    summary_history = {
        'summary_history': {
            'entries': {'added': {}, 'changed': {}, 'removed': {}},
            'dustbin': {'added': [], 'removed': []},
            'versions': {'added': {}, 'changed': {}, 'removed': {}}
        }
    }
    eff = execute_cmd('history', {})
    sequence = [
        (EManifestHistory(1, None, False),
            lambda _: summary_history),
    ]
    resp = perform_sequence(sequence, eff)
    summary_history['status'] = 'ok'
    assert resp == summary_history


def test_api_manifest_restore():
    eff = execute_cmd('restore', {})
    sequence = [
        (EManifestRestore(None),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_file_create():
    eff = execute_cmd('file_create', {'path': '/foo'})
    sequence = [
        (EFileCreate('/foo'),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_file_read():
    eff = execute_cmd('file_read', {'path': '/foo'})
    sequence = [
        (EFileRead('/foo', 0, None),
            lambda _: 'foo'),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'content': 'foo'}


def test_api_file_write():
    eff = execute_cmd('file_write', {'path': '/foo', 'content': to_jsonb64(b'foo'), 'offset': 0})
    sequence = [
        (EFileWrite('/foo', b'foo', 0),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_file_truncate():
    eff = execute_cmd('file_truncate', {'path': '/foo', 'length': 5})
    sequence = [
        (EFileTruncate('/foo', 5),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.xfail
def test_api_file_history():
    raise NotImplementedError()


@pytest.mark.xfail
def test_api_file_restore():
    raise NotImplementedError()


def test_api_folder_create():
    eff = execute_cmd('folder_create', {'path': '/dir'})
    sequence = [
        (EFolderCreate('/dir'),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_stat():
    stat = {'type': 'file', 'id': '123'}
    eff = execute_cmd('stat', {'path': '/foo'})
    sequence = [
        (EStat('/foo'),
            lambda _: stat),
    ]
    resp = perform_sequence(sequence, eff)
    stat['status'] = 'ok'
    assert resp == stat


def test_api_move():
    eff = execute_cmd('move', {'src': '/foo', 'dst': '/bar'})
    sequence = [
        (EMove('/foo', '/bar'),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_delete():
    eff = execute_cmd('delete', {'path': '/foo'})
    sequence = [
        (EDelete('/foo'),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_undelete():
    eff = execute_cmd('undelete', {'vlob': '123'})
    sequence = [
        (EUndelete('123'),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}
