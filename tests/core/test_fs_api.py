import pytest
from effect2.testing import const, noop, perform_sequence

from parsec.core.core_api import execute_cmd
from parsec.core.fs import (
    EFSFileCreate, EFSFileRead, EFSFileWrite, EFSFileTruncate, EFSFolderCreate,
    EFSStat, EFSMove, EFSDelete
)
from parsec.tools import to_jsonb64


@pytest.mark.xfail(reason='not implemented yet')
def test_api_group_create():
    eff = execute_cmd('group_create', {'group': 'share'})
    sequence = [
        (EFSGroupCreate('share'), noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.xfail(reason='not implemented yet')
def test_api_dustbin_show():
    # Specific file in dustbin
    eff = execute_cmd('dustbin_show', {'path': '/foo'})
    sequence = [
        (EFSDustbinShow('/foo'),
            const([{'path': '/foo'}])),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'dustbin': [{'path': '/foo'}]}
    # All files in dustbin
    eff = execute_cmd('dustbin_show', {})
    sequence = [
        (EFSDustbinShow(),
            const([{'path': '/foo'}, {'path': '/bar'}])),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'dustbin': [{'path': '/foo'}, {'path': '/bar'}]}


@pytest.mark.xfail(reason='not implemented yet')
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
        (EFSManifestHistory(1, None, False),
            const(summary_history)),
    ]
    resp = perform_sequence(sequence, eff)
    summary_history['status'] = 'ok'
    assert resp == summary_history


@pytest.mark.xfail(reason='not implemented yet')
def test_api_manifest_restore():
    eff = execute_cmd('restore', {})
    sequence = [
        (EFSManifestRestore(None),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_file_create():
    eff = execute_cmd('file_create', {'path': '/foo'})
    sequence = [
        (EFSFileCreate('/foo'),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_file_read():
    eff = execute_cmd('file_read', {'path': '/foo'})
    sequence = [
        (EFSFileRead('/foo', 0, None),
            const('foo')),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'content': 'foo'}


def test_api_file_write():
    eff = execute_cmd('file_write', {'path': '/foo', 'content': to_jsonb64(b'foo'), 'offset': 0})
    sequence = [
        (EFSFileWrite('/foo', b'foo', 0),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_file_truncate():
    eff = execute_cmd('file_truncate', {'path': '/foo', 'length': 5})
    sequence = [
        (EFSFileTruncate('/foo', 5),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.xfail(reason='not implemented yet')
def test_api_file_history():
    raise NotImplementedError()


@pytest.mark.xfail(reason='not implemented yet')
def test_api_file_restore():
    raise NotImplementedError()


def test_api_folder_create():
    eff = execute_cmd('folder_create', {'path': '/dir'})
    sequence = [
        (EFSFolderCreate('/dir'),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_stat():
    stat = {'type': 'file', 'id': '123'}
    eff = execute_cmd('stat', {'path': '/foo'})
    sequence = [
        (EFSStat('/foo'),
            const(stat)),
    ]
    resp = perform_sequence(sequence, eff)
    stat['status'] = 'ok'
    assert resp == stat


def test_api_move():
    eff = execute_cmd('move', {'src': '/foo', 'dst': '/bar'})
    sequence = [
        (EFSMove('/foo', '/bar'),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_api_delete():
    eff = execute_cmd('delete', {'path': '/foo'})
    sequence = [
        (EFSDelete('/foo'),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.xfail(reason='not implemented yet')
def test_api_undelete():
    eff = execute_cmd('undelete', {'vlob': '123'})
    sequence = [
        (EFSUndelete('123'),
            noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}
