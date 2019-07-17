# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import uuid4

import pytest
from hypothesis import strategies as st
from hypothesis.stateful import (
    Bundle,
    RuleBasedStateMachine,
    initialize,
    invariant,
    rule,
    run_state_machine_as_test,
)

from parsec.core.persistent_storage import DEFAULT_BLOCK_SIZE as block_size
from parsec.core.persistent_storage import LocalStorageMissingError, PersistentStorage
from parsec.core.types import BlockID, EntryID
from parsec.crypto import SecretKey

ENTRY_ID = EntryID("00000000000000000000000000000001")
BLOCK_ID = BlockID("0000000000000000000000000000000A")


@pytest.fixture
def persistent_storage(tmpdir):
    key = SecretKey.generate()
    with PersistentStorage(key, tmpdir, max_cache_size=128 * block_size) as db:
        yield db


def test_persistent_storage_path(tmpdir, persistent_storage):
    assert persistent_storage.path == tmpdir
    assert persistent_storage.max_cache_size == 128 * block_size
    assert persistent_storage.block_limit == 128


def test_persistent_storage_cache_size(persistent_storage):
    assert persistent_storage.get_cache_size() == 0

    persistent_storage.set_dirty_block(ENTRY_ID, b"data")
    assert persistent_storage.get_cache_size() == 0

    persistent_storage.set_clean_block(ENTRY_ID, b"data")
    assert persistent_storage.get_cache_size() > 4


@pytest.mark.parametrize("dtype", ["block", "manifest"])
@pytest.mark.parametrize("sensitivity", ["dirty", "clean"])
def test_persistent_storage_set_get_clear(persistent_storage, dtype, sensitivity):
    get_method = getattr(persistent_storage, f"get_{sensitivity}_{dtype}")
    set_method = getattr(persistent_storage, f"set_{sensitivity}_{dtype}")
    clear_method = getattr(persistent_storage, f"clear_{sensitivity}_{dtype}")

    set_method(ENTRY_ID, b"data")

    data = get_method(ENTRY_ID)
    assert data == b"data"

    clear_method(ENTRY_ID)

    with pytest.raises(LocalStorageMissingError):
        clear_method(ENTRY_ID)

    with pytest.raises(LocalStorageMissingError):
        get_method(ENTRY_ID)


def test_persistent_storage_on_disk(tmpdir, persistent_storage):
    persistent_storage.set_clean_manifest(ENTRY_ID, b"vlob_data")
    persistent_storage.set_clean_block(BLOCK_ID, b"block_data")
    persistent_storage.close()

    with PersistentStorage(
        persistent_storage.local_symkey, tmpdir, max_cache_size=128 * block_size
    ) as persistent_storage_copy:
        vlob_data = persistent_storage_copy.get_clean_manifest(ENTRY_ID)
        block_data = persistent_storage_copy.get_clean_block(BLOCK_ID)

    assert vlob_data == b"vlob_data"
    assert block_data == b"block_data"


def test_local_manual_run_block_garbage_collector(persistent_storage):
    block_id_precious = BlockID()
    persistent_storage.set_dirty_block(block_id_precious, b"precious_data")

    block_id_deletable = BlockID()
    persistent_storage.set_clean_block(block_id_deletable, b"deletable_data")

    persistent_storage.run_block_garbage_collector()
    persistent_storage.get_dirty_block(block_id_precious) == b"precious_data"
    with pytest.raises(LocalStorageMissingError):
        persistent_storage.get_clean_block(block_id_deletable)


def test_local_automatic_run_garbage_collector(persistent_storage):
    persistent_storage.max_cache_size = 1 * block_size

    block_id_a = BlockID()
    persistent_storage.set_dirty_block(block_id_a, b"a" * 10)

    block_id_b = BlockID()
    persistent_storage.set_clean_block(block_id_b, b"b" * 5)

    data_b = persistent_storage.get_clean_block(block_id_b)
    assert data_b == b"b" * 5

    block_id_c = BlockID()
    persistent_storage.set_clean_block(block_id_c, b"c" * 5)

    data_a = persistent_storage.get_dirty_block(block_id_a)
    assert data_a == b"a" * 10

    with pytest.raises(LocalStorageMissingError):
        persistent_storage.get_clean_block(block_id_b)

    data_c = persistent_storage.get_clean_block(block_id_c)
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

            self.local_symkey = SecretKey.generate()
            self.cleared_precious_data = set()

            self.persistent_storage = PersistentStorage(
                self.local_symkey, tmpdir / f"local-db-{tentative}", max_cache_size=128 * block_size
            )
            self.persistent_storage.connect()

            # Monkey patch to simplify test
            self.persistent_storage._encrypt_with_symkey = lambda key, data: data
            self.persistent_storage._decrypt_with_symkey = lambda key, data: data

        def teardown(self):
            if hasattr(self, "persistent_storage"):
                self.persistent_storage.close()

        @rule(entry=PreciousEntry)
        def get_precious_data(self, entry):
            block_id, expected_data = entry
            if block_id in self.cleared_precious_data:
                with pytest.raises(LocalStorageMissingError):
                    self.persistent_storage.get_dirty_block(block_id)
            else:
                data = self.persistent_storage.get_dirty_block(block_id)
                assert data == expected_data

        @rule(entry=DeletableEntry)
        def get_deletable_data(self, entry):
            block_id, expected_data = entry
            try:
                data = self.persistent_storage.get_clean_block(block_id)
                assert data == expected_data
            except LocalStorageMissingError:
                pass

        @rule(target=DeletableEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_deletable_data(self, data_size):
            block_id = BlockID(uuid4().hex)
            data = b"x" * data_size
            self.persistent_storage.set_clean_block(block_id, data)
            return block_id, data

        @rule(target=PreciousEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_precious_data(self, data_size):
            block_id = BlockID(uuid4().hex)
            data = b"x" * data_size
            self.persistent_storage.set_dirty_block(block_id, data)
            return block_id, data

        @rule(entry=PreciousEntry)
        def clear_precious_data(self, entry):
            block_id, _ = entry
            if block_id in self.cleared_precious_data:
                with pytest.raises(LocalStorageMissingError):
                    self.persistent_storage.clear_dirty_block(block_id)
            else:
                self.persistent_storage.clear_dirty_block(block_id)
                self.cleared_precious_data.add(block_id)

        @rule(entry=DeletableEntry)
        def clear_deletable_data(self, entry):
            block_id, _ = entry
            try:
                self.persistent_storage.clear_clean_block(block_id)
            except LocalStorageMissingError:
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
