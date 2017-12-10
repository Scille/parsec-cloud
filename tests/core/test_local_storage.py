import pytest

from parsec.core.local_storage import LocalStorage

from tests.common import InMemoryLocalStorage


@pytest.fixture(params=[InMemoryLocalStorage, LocalStorage])
def local_storage(request):
    if request.param is LocalStorage:
        pytest.skip('`LocalStorage` not implemented yet !')
    return request.param()


def test_fetch_user_manifest_not_available(local_storage):
    blob = local_storage.fetch_user_manifest()
    assert blob is None


def test_flush_and_fetch_user_manifest(local_storage):
    local_storage.flush_user_manifest(b'<user manifest>')
    blob = local_storage.fetch_user_manifest()
    assert blob == b'<user manifest>'


def test_fetch_manifest_not_available(local_storage):
    blob = local_storage.fetch_manifest('<unknown_id>')
    assert blob is None


def test_flush_and_fetch_manifest(local_storage):
    data = [('<id#%s>' % i, ('<manifest#%s>' % i).encode()) for i in range(3)]
    for id, blob in data:
        local_storage.flush_manifest(id, blob)
    for id, expected_blob in data:
        blob = local_storage.fetch_manifest(id)
        assert blob == expected_blob


def test_move_manifest_manifest(local_storage):
    local_storage.flush_manifest('<id#1>', b'<manifest>')
    local_storage.move_manifest('<id#1>', '<id#2>')
    old = local_storage.fetch_manifest('<id#1>')
    assert old is None
    new = local_storage.fetch_manifest('<id#2>')
    assert new == b'<manifest>'
