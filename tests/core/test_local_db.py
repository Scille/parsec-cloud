import pytest
from unittest.mock import PropertyMock
from uuid import UUID

from parsec.core.fs.types import Path
from parsec.core.local_db import LocalDB, LocalDBMissingEntry


@pytest.mark.trio
async def test_local_db_path(tmpdir):
    local_db = LocalDB(tmpdir)
    assert local_db.path == tmpdir


@pytest.mark.trio
async def test_local_db_cache_size(alice, tmpdir):
    local_db = LocalDB(tmpdir)
    assert local_db.cache_size == 0
    local_db.set(alice.user_manifest_access, b"data", False)
    assert local_db.cache_size == 0
    local_db.set(alice.user_manifest_access, b"data")
    assert local_db.cache_size > 4


@pytest.mark.trio
async def test_local_db_set_get_clear(alice, tmpdir):
    local_db = LocalDB(tmpdir)
    local_db.set(alice.user_manifest_access, b"data")
    assert local_db.get(alice.user_manifest_access) == b"data"
    local_db.clear(alice.user_manifest_access)
    with pytest.raises(LocalDBMissingEntry):
        local_db.clear(alice.user_manifest_access)
    with pytest.raises(LocalDBMissingEntry):
        local_db.get(alice.user_manifest_access)


@pytest.mark.trio
async def test_local_db_find(alice, tmpdir):
    local_db = LocalDB(tmpdir)
    local_db.set(alice.user_manifest_access, b"data")
    assert local_db._find(alice.user_manifest_access) == Path(
        str(tmpdir) + "/cache/" + str(alice.user_manifest_access["id"])
    )
    assert local_db._find({"id": "123"}) is None


@pytest.mark.trio
async def test_local_manual_run_garbage_collector(alice, tmpdir):
    local_db = LocalDB(tmpdir)
    local_db.set(alice.user_manifest_access, b"data", False)
    new_access = {**alice.user_manifest_access, "id": UUID("12345678123456781234567812345678")}
    local_db.set(new_access, b"deletable_data")
    local_db.run_garbage_collector()
    local_db.get(alice.user_manifest_access) == b"data"
    with pytest.raises(LocalDBMissingEntry):
        local_db.get(new_access)


@pytest.mark.trio
async def test_local_automatic_run_garbage_collector(alice, tmpdir):
    local_db = LocalDB(tmpdir)
    local_db.set(alice.user_manifest_access, b"data")
    type(local_db).cache_size = PropertyMock(return_value=2097152 + 1)
    new_access = {**alice.user_manifest_access, "id": UUID("12345678123456781234567812345678")}
    local_db.set(new_access, b"data2")
    with pytest.raises(LocalDBMissingEntry):
        local_db.get(alice.user_manifest_access)
    assert local_db.get(new_access) == b"data2"
