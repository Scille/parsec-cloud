import pytest

from parsec.core.local_db import LocalDB, LocalDBMissingEntry
from parsec.core.fs.utils import new_access


@pytest.mark.trio
async def test_local_db_path(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)
    assert local_db.path == tmpdir
    assert local_db.max_cache_size == 128


@pytest.mark.trio
async def test_local_db_cache_size(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)

    access = new_access()
    assert local_db.cache_size == 0

    local_db.set(access, b"data", False)
    assert local_db.cache_size == 0

    local_db.set(access, b"data")
    assert local_db.cache_size > 4


@pytest.mark.trio
async def test_local_db_set_get_clear(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)

    access = new_access()
    local_db.set(access, b"data")

    data = local_db.get(access)
    assert data == b"data"

    local_db.clear(access)

    with pytest.raises(LocalDBMissingEntry):
        local_db.clear(access)

    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access)


@pytest.mark.trio
async def test_local_db_on_disk(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)
    access = new_access()
    local_db.set(access, b"data")

    local_db_cpy = LocalDB(tmpdir, max_cache_size=128)
    data = local_db_cpy.get(access)
    assert data == b"data"


@pytest.mark.trio
async def test_local_manual_run_garbage_collector(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)

    access_precious = new_access()
    local_db.set(access_precious, b"precious_data", False)

    access_deletable = new_access()
    local_db.set(access_deletable, b"deletable_data")

    local_db.run_garbage_collector()
    local_db.get(access_precious) == b"precious_data"
    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access_deletable)


@pytest.mark.trio
async def test_local_automatic_run_garbage_collector(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=16)

    access_a = new_access()
    local_db.set(access_a, b"a" * 10)

    access_b = new_access()
    local_db.set(access_b, b"b" * 5)

    data_b = local_db.get(access_b)
    assert data_b == b"b" * 5

    access_c = new_access()
    local_db.set(access_c, b"c" * 5)

    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access_a)

    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access_b)

    data_c = local_db.get(access_c)
    assert data_c == b"c" * 5
