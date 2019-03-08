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

from parsec.core.local_storage import LocalStorage, LocalStorageMissingEntry
from parsec.core.local_storage import DEFAULT_BLOCK_SIZE as block_size
from parsec.core.types import ManifestAccess


@pytest.fixture
def local_storage(tmpdir):
    with LocalStorage(tmpdir, max_cache_size=128 * block_size) as db:
        yield db


def test_local_storage_path(tmpdir, local_storage):
    assert local_storage.path == tmpdir
    assert local_storage.max_cache_size == 128 * block_size
    assert local_storage.block_limit == 128


def test_local_storage_cache_size(local_storage):
    access = ManifestAccess()
    assert local_storage.get_cache_size() == 0

    local_storage.set_dirty_block(access, b"data")
    assert local_storage.get_cache_size() == 0

    local_storage.set_clean_block(access, b"data")
    assert local_storage.get_cache_size() > 4


@pytest.mark.parametrize("dtype", ["block", "manifest"])
@pytest.mark.parametrize("sensitivity", ["dirty", "clean"])
def test_local_storage_set_get_clear(local_storage, dtype, sensitivity):
    get_method = getattr(local_storage, f"get_{sensitivity}_{dtype}")
    set_method = getattr(local_storage, f"set_{sensitivity}_{dtype}")
    clear_method = getattr(local_storage, f"clear_{sensitivity}_{dtype}")

    access = ManifestAccess()
    set_method(access, b"data")

    data = get_method(access)
    assert data == b"data"

    clear_method(access)

    with pytest.raises(LocalStorageMissingEntry):
        clear_method(access)

    with pytest.raises(LocalStorageMissingEntry):
        get_method(access)


def test_local_storage_on_disk(tmpdir, local_storage):
    vlob_access = ManifestAccess()
    local_storage.set_clean_manifest(vlob_access, b"vlob_data")
    block_access = ManifestAccess()
    local_storage.set_clean_block(block_access, b"block_data")
    local_storage.close()

    with LocalStorage(tmpdir, max_cache_size=128 * block_size) as local_storage_copy:
        vlob_data = local_storage_copy.get_clean_manifest(vlob_access)
        block_data = local_storage_copy.get_clean_block(block_access)

    assert vlob_data == b"vlob_data"
    assert block_data == b"block_data"


def test_local_manual_run_block_garbage_collector(local_storage):
    access_precious = ManifestAccess()
    local_storage.set_dirty_block(access_precious, b"precious_data")

    access_deletable = ManifestAccess()
    local_storage.set_clean_block(access_deletable, b"deletable_data")

    local_storage.run_block_garbage_collector()
    local_storage.get_dirty_block(access_precious) == b"precious_data"
    with pytest.raises(LocalStorageMissingEntry):
        local_storage.get_clean_block(access_deletable)


def test_local_automatic_run_garbage_collector(local_storage):
    local_storage.max_cache_size = 1 * block_size

    access_a = ManifestAccess()
    local_storage.set_dirty_block(access_a, b"a" * 10)

    access_b = ManifestAccess()
    local_storage.set_clean_block(access_b, b"b" * 5)

    data_b = local_storage.get_clean_block(access_b)
    assert data_b == b"b" * 5

    access_c = ManifestAccess()
    local_storage.set_clean_block(access_c, b"c" * 5)

    data_a = local_storage.get_dirty_block(access_a)
    assert data_a == b"a" * 10

    with pytest.raises(LocalStorageMissingEntry):
        local_storage.get_clean_block(access_b)

    data_c = local_storage.get_clean_block(access_c)
    assert data_c == b"c" * 5


@pytest.mark.slow
def test_local_storage_stateful(tmpdir, hypothesis_settings):
    tentative = 0

    class LocalStorageStateMachine(RuleBasedStateMachine):
        PreciousEntry = Bundle("precious_entry")
        DeletableEntry = Bundle("deletable_entry")

        @initialize()
        def init(self):
            nonlocal tentative
            tentative += 1

            self.cleared_precious_data = set()

            self.local_storage = LocalStorage(
                tmpdir / f"local-db-{tentative}", max_cache_size=128 * block_size
            )
            self.local_storage.connect()

            # Monkey patch to simplify test
            self.local_storage._encrypt_with_symkey = lambda key, data: data
            self.local_storage._decrypt_with_symkey = lambda key, data: data

        def teardown(self):
            self.local_storage.close()

        @rule(entry=PreciousEntry)
        def get_precious_data(self, entry):
            access, expected_data = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalStorageMissingEntry):
                    self.local_storage.get_dirty_block(access)
            else:
                data = self.local_storage.get_dirty_block(access)
                assert data == expected_data

        @rule(entry=DeletableEntry)
        def get_deletable_data(self, entry):
            access, expected_data = entry
            try:
                data = self.local_storage.get_clean_block(access)
                assert data == expected_data
            except LocalStorageMissingEntry:
                pass

        @rule(target=DeletableEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_deletable_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.local_storage.set_clean_block(access, data)
            return access, data

        @rule(target=PreciousEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_precious_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.local_storage.set_dirty_block(access, data)
            return access, data

        @rule(entry=PreciousEntry)
        def clear_precious_data(self, entry):
            access, _ = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalStorageMissingEntry):
                    self.local_storage.clear_dirty_block(access)
            else:
                self.local_storage.clear_dirty_block(access)
                self.cleared_precious_data.add(access.id)

        @rule(entry=DeletableEntry)
        def clear_deletable_data(self, entry):
            access, _ = entry
            try:
                self.local_storage.clear_clean_block(access)
            except LocalStorageMissingEntry:
                pass

        @rule()
        def gc(self):
            self.local_storage.run_block_garbage_collector()

        @invariant()
        def check(self):
            if not hasattr(self, "local_storage"):
                return
            assert self.local_storage.get_cache_size() <= self.local_storage.max_cache_size

    run_state_machine_as_test(LocalStorageStateMachine, settings=hypothesis_settings)
