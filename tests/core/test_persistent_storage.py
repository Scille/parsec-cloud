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

from parsec.core.persistent_storage import PersistentStorage, LocalStorageMissingEntry
from parsec.core.persistent_storage import DEFAULT_BLOCK_SIZE as block_size
from parsec.core.types import ManifestAccess


@pytest.fixture
def persistent_storage(tmpdir):
    with PersistentStorage(tmpdir, max_cache_size=128 * block_size) as db:
        yield db


def test_persistent_storage_path(tmpdir, persistent_storage):
    assert persistent_storage.path == tmpdir
    assert persistent_storage.max_cache_size == 128 * block_size
    assert persistent_storage.block_limit == 128


def test_persistent_storage_cache_size(persistent_storage):
    access = ManifestAccess()
    assert persistent_storage.get_cache_size() == 0

    persistent_storage.set_dirty_block(access, b"data")
    assert persistent_storage.get_cache_size() == 0

    persistent_storage.set_clean_block(access, b"data")
    assert persistent_storage.get_cache_size() > 4


@pytest.mark.parametrize("dtype", ["block", "manifest"])
@pytest.mark.parametrize("sensitivity", ["dirty", "clean"])
def test_persistent_storage_set_get_clear(persistent_storage, dtype, sensitivity):
    get_method = getattr(persistent_storage, f"get_{sensitivity}_{dtype}")
    set_method = getattr(persistent_storage, f"set_{sensitivity}_{dtype}")
    clear_method = getattr(persistent_storage, f"clear_{sensitivity}_{dtype}")

    access = ManifestAccess()
    set_method(access, b"data")

    data = get_method(access)
    assert data == b"data"

    clear_method(access)

    with pytest.raises(LocalStorageMissingEntry):
        clear_method(access)

    with pytest.raises(LocalStorageMissingEntry):
        get_method(access)


def test_persistent_storage_on_disk(tmpdir, persistent_storage):
    vlob_access = ManifestAccess()
    persistent_storage.set_clean_manifest(vlob_access, b"vlob_data")
    block_access = ManifestAccess()
    persistent_storage.set_clean_block(block_access, b"block_data")
    persistent_storage.close()

    with PersistentStorage(tmpdir, max_cache_size=128 * block_size) as persistent_storage_copy:
        vlob_data = persistent_storage_copy.get_clean_manifest(vlob_access)
        block_data = persistent_storage_copy.get_clean_block(block_access)

    assert vlob_data == b"vlob_data"
    assert block_data == b"block_data"


def test_local_manual_run_block_garbage_collector(persistent_storage):
    access_precious = ManifestAccess()
    persistent_storage.set_dirty_block(access_precious, b"precious_data")

    access_deletable = ManifestAccess()
    persistent_storage.set_clean_block(access_deletable, b"deletable_data")

    persistent_storage.run_block_garbage_collector()
    persistent_storage.get_dirty_block(access_precious) == b"precious_data"
    with pytest.raises(LocalStorageMissingEntry):
        persistent_storage.get_clean_block(access_deletable)


def test_local_automatic_run_garbage_collector(persistent_storage):
    persistent_storage.max_cache_size = 1 * block_size

    access_a = ManifestAccess()
    persistent_storage.set_dirty_block(access_a, b"a" * 10)

    access_b = ManifestAccess()
    persistent_storage.set_clean_block(access_b, b"b" * 5)

    data_b = persistent_storage.get_clean_block(access_b)
    assert data_b == b"b" * 5

    access_c = ManifestAccess()
    persistent_storage.set_clean_block(access_c, b"c" * 5)

    data_a = persistent_storage.get_dirty_block(access_a)
    assert data_a == b"a" * 10

    with pytest.raises(LocalStorageMissingEntry):
        persistent_storage.get_clean_block(access_b)

    data_c = persistent_storage.get_clean_block(access_c)
    assert data_c == b"c" * 5


@pytest.mark.slow
def test_persistent_storage_stateful(tmpdir, hypothesis_settings):
    tentative = 0

    class LocalStorageStateMachine(RuleBasedStateMachine):
        PreciousEntry = Bundle("precious_entry")
        DeletableEntry = Bundle("deletable_entry")

        @initialize()
        def init(self):
            nonlocal tentative
            tentative += 1

            self.cleared_precious_data = set()

            self.persistent_storage = PersistentStorage(
                tmpdir / f"local-db-{tentative}", max_cache_size=128 * block_size
            )
            self.persistent_storage.connect()

            # Monkey patch to simplify test
            self.persistent_storage._encrypt_with_symkey = lambda key, data: data
            self.persistent_storage._decrypt_with_symkey = lambda key, data: data

        def teardown(self):
            self.persistent_storage.close()

        @rule(entry=PreciousEntry)
        def get_precious_data(self, entry):
            access, expected_data = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalStorageMissingEntry):
                    self.persistent_storage.get_dirty_block(access)
            else:
                data = self.persistent_storage.get_dirty_block(access)
                assert data == expected_data

        @rule(entry=DeletableEntry)
        def get_deletable_data(self, entry):
            access, expected_data = entry
            try:
                data = self.persistent_storage.get_clean_block(access)
                assert data == expected_data
            except LocalStorageMissingEntry:
                pass

        @rule(target=DeletableEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_deletable_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.persistent_storage.set_clean_block(access, data)
            return access, data

        @rule(target=PreciousEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_precious_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.persistent_storage.set_dirty_block(access, data)
            return access, data

        @rule(entry=PreciousEntry)
        def clear_precious_data(self, entry):
            access, _ = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalStorageMissingEntry):
                    self.persistent_storage.clear_dirty_block(access)
            else:
                self.persistent_storage.clear_dirty_block(access)
                self.cleared_precious_data.add(access.id)

        @rule(entry=DeletableEntry)
        def clear_deletable_data(self, entry):
            access, _ = entry
            try:
                self.persistent_storage.clear_clean_block(access)
            except LocalStorageMissingEntry:
                pass

        @rule()
        def gc(self):
            self.persistent_storage.run_block_garbage_collector()

        @invariant()
        def check(self):
            if not hasattr(self, "persistent_storage"):
                return
            assert (
                self.persistent_storage.get_cache_size() <= self.persistent_storage.max_cache_size
            )

    run_state_machine_as_test(LocalStorageStateMachine, settings=hypothesis_settings)
