# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from hypothesis.stateful import (
    RuleBasedStateMachine,
    Bundle,
    initialize,
    invariant,
    rule,
    run_state_machine_as_test,
)
from hypothesis import strategies as st

from parsec.core.local_db import LocalDB, LocalDBMissingEntry
from parsec.core.local_db import DEFAULT_BLOCK_SIZE as block_size
from parsec.core.types import ManifestAccess


@pytest.fixture
def local_db(tmpdir):
    with LocalDB(tmpdir, max_cache_size=128 * block_size) as db:
        yield db


def test_local_db_path(tmpdir, local_db):
    assert local_db.path == tmpdir
    assert local_db.max_cache_size == 128 * block_size
    assert local_db.block_limit == 128


def test_local_db_cache_size(local_db):
    access = ManifestAccess()
    assert local_db.get_cache_size() == 0

    local_db.set_dirty_block(access, b"data")
    assert local_db.get_cache_size() == 0

    local_db.set_clean_block(access, b"data")
    assert local_db.get_cache_size() > 4


@pytest.mark.parametrize("dtype", ["block", "manifest"])
@pytest.mark.parametrize("sensitivity", ["dirty", "clean"])
def test_local_db_set_get_clear(local_db, dtype, sensitivity):
    get_method = getattr(local_db, f"get_{sensitivity}_{dtype}")
    set_method = getattr(local_db, f"set_{sensitivity}_{dtype}")
    clear_method = getattr(local_db, f"clear_{sensitivity}_{dtype}")

    access = ManifestAccess()
    set_method(access, b"data")

    data = get_method(access)
    assert data == b"data"

    clear_method(access)

    with pytest.raises(LocalDBMissingEntry):
        clear_method(access)

    with pytest.raises(LocalDBMissingEntry):
        get_method(access)


def test_local_db_on_disk(tmpdir, local_db):
    vlob_access = ManifestAccess()
    local_db.set_clean_manifest(vlob_access, b"vlob_data")
    block_access = ManifestAccess()
    local_db.set_clean_block(block_access, b"block_data")
    local_db.close()

    with LocalDB(tmpdir, max_cache_size=128 * block_size) as local_db_copy:
        local_db_copy.memory_cache = local_db.memory_cache
        vlob_data = local_db_copy.get_clean_manifest(vlob_access)
        block_data = local_db_copy.get_clean_block(block_access)

    assert vlob_data == b"vlob_data"
    assert block_data == b"block_data"


def test_local_manual_run_block_garbage_collector(local_db):
    access_precious = ManifestAccess()
    local_db.set_dirty_block(access_precious, b"precious_data")

    access_deletable = ManifestAccess()
    local_db.set_clean_block(access_deletable, b"deletable_data")

    local_db.run_block_garbage_collector()
    local_db.get_dirty_block(access_precious) == b"precious_data"
    with pytest.raises(LocalDBMissingEntry):
        local_db.get_clean_block(access_deletable)


def test_local_automatic_run_garbage_collector(local_db):
    local_db.max_cache_size = 1 * block_size

    access_a = ManifestAccess()
    local_db.set_dirty_block(access_a, b"a" * 10)

    access_b = ManifestAccess()
    local_db.set_clean_block(access_b, b"b" * 5)

    data_b = local_db.get_clean_block(access_b)
    assert data_b == b"b" * 5

    access_c = ManifestAccess()
    local_db.set_clean_block(access_c, b"c" * 5)

    data_a = local_db.get_dirty_block(access_a)
    assert data_a == b"a" * 10

    with pytest.raises(LocalDBMissingEntry):
        local_db.get_clean_block(access_b)

    data_c = local_db.get_clean_block(access_c)
    assert data_c == b"c" * 5


@pytest.mark.slow
def test_local_db_stateful(tmpdir, hypothesis_settings):
    tentative = 0

    class LocalDBStateMachine(RuleBasedStateMachine):
        PreciousEntry = Bundle("precious_entry")
        DeletableEntry = Bundle("deletable_entry")

        @initialize()
        def init(self):
            nonlocal tentative
            tentative += 1

            self.cleared_precious_data = set()

            self.local_db = LocalDB(
                tmpdir / f"local-db-{tentative}", max_cache_size=128 * block_size
            )
            self.local_db.connect()

            # Monkey patch to simplify test
            self.local_db._encrypt_with_symkey = lambda key, data: data
            self.local_db._decrypt_with_symkey = lambda key, data: data

        def teardown(self):
            self.local_db.close()

        @rule(entry=PreciousEntry)
        def get_precious_data(self, entry):
            access, expected_data = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalDBMissingEntry):
                    self.local_db.get_dirty_block(access)
            else:
                data = self.local_db.get_dirty_block(access)
                assert data == expected_data

        @rule(entry=DeletableEntry)
        def get_deletable_data(self, entry):
            access, expected_data = entry
            try:
                data = self.local_db.get_clean_block(access)
                assert data == expected_data
            except LocalDBMissingEntry:
                pass

        @rule(target=DeletableEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_deletable_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.local_db.set_clean_block(access, data)
            return access, data

        @rule(target=PreciousEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_precious_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.local_db.set_dirty_block(access, data)
            return access, data

        @rule(entry=PreciousEntry)
        def clear_precious_data(self, entry):
            access, _ = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalDBMissingEntry):
                    self.local_db.clear_dirty_block(access)
            else:
                self.local_db.clear_dirty_block(access)
                self.cleared_precious_data.add(access.id)

        @rule(entry=DeletableEntry)
        def clear_deletable_data(self, entry):
            access, _ = entry
            try:
                self.local_db.clear_clean_block(access)
            except LocalDBMissingEntry:
                pass

        @rule()
        def gc(self):
            self.local_db.run_block_garbage_collector()

        @invariant()
        def check(self):
            if not hasattr(self, "local_db"):
                return
            assert self.local_db.get_cache_size() <= self.local_db.max_cache_size

    run_state_machine_as_test(LocalDBStateMachine, settings=hypothesis_settings)
