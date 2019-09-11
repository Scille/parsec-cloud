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
from uuid import uuid4

from parsec.crypto import SecretKey
from parsec.api.protocol import DeviceID
from parsec.core.types import (
    EntryID,
    BlockID,
    LocalUserManifest,
    LocalWorkspaceManifest,
    LocalFolderManifest,
    LocalFileManifest,
    DEFAULT_BLOCK_SIZE as block_size,
)
from parsec.core.fs import FSLocalMissError
from parsec.core.fs.persistent_storage import PersistentStorage

from tests.common import freeze_time


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
    entry_id = EntryID("00000000000000000000000000000001")

    assert persistent_storage.get_cache_size() == 0

    persistent_storage.set_dirty_chunk(entry_id, b"data")
    assert persistent_storage.get_cache_size() == 0

    persistent_storage.set_clean_block(entry_id, b"data")
    assert persistent_storage.get_cache_size() > 4

    persistent_storage.clear_clean_block(entry_id)
    assert persistent_storage.get_cache_size() == 0


def test_persistent_storage_set_get_clear_manifest(persistent_storage):
    entry_id = EntryID("00000000000000000000000000000001")

    expected_manifest = LocalWorkspaceManifest.new_placeholder()
    persistent_storage.set_manifest(entry_id, expected_manifest)

    manifest = persistent_storage.get_manifest(entry_id)
    assert manifest == expected_manifest

    persistent_storage.clear_manifest(entry_id)

    with pytest.raises(FSLocalMissError):
        persistent_storage.clear_manifest(entry_id)

    with pytest.raises(FSLocalMissError):
        persistent_storage.get_manifest(entry_id)


@pytest.mark.parametrize("dtype", ["dirty_chunk", "clean_block"])
def test_persistent_storage_set_get_clear_chunk(persistent_storage, dtype):
    entry_id = EntryID("00000000000000000000000000000001")

    get_method = getattr(persistent_storage, f"get_{dtype}")
    set_method = getattr(persistent_storage, f"set_{dtype}")
    clear_method = getattr(persistent_storage, f"clear_{dtype}")

    set_method(entry_id, b"data")

    data = get_method(entry_id)
    assert data == b"data"

    clear_method(entry_id)

    with pytest.raises(FSLocalMissError):
        clear_method(entry_id)

    with pytest.raises(FSLocalMissError):
        get_method(entry_id)


def test_persistent_storage_on_disk(tmpdir, persistent_storage):
    entry_id = EntryID("00000000000000000000000000000001")
    block_id = BlockID("0000000000000000000000000000000A")

    expected_manifest = LocalWorkspaceManifest.new_placeholder(id=entry_id)
    persistent_storage.set_manifest(entry_id, expected_manifest)
    persistent_storage.set_clean_block(block_id, b"block_data")
    persistent_storage.close()

    with PersistentStorage(
        persistent_storage.local_symkey, tmpdir, max_cache_size=128 * block_size
    ) as persistent_storage_copy:
        manifest = persistent_storage_copy.get_manifest(entry_id)
        block_data = persistent_storage_copy.get_clean_block(block_id)

    assert manifest == expected_manifest
    assert block_data == b"block_data"


def test_local_manual_run_block_garbage_collector(persistent_storage):
    block_id_precious = BlockID()
    persistent_storage.set_dirty_chunk(block_id_precious, b"precious_data")

    block_id_deletable = BlockID()
    persistent_storage.set_clean_block(block_id_deletable, b"deletable_data")

    persistent_storage.run_block_garbage_collector()
    persistent_storage.get_dirty_chunk(block_id_precious) == b"precious_data"
    with pytest.raises(FSLocalMissError):
        persistent_storage.get_clean_block(block_id_deletable)


def test_local_manual_run_block_garbage_collector_with_limit(persistent_storage):
    block_id_precious = BlockID()
    # No matter how old, shouldn't be deleted
    with freeze_time("2000-01-01"):
        persistent_storage.set_dirty_chunk(block_id_precious, b"precious_data")

    block_id_deletable1 = BlockID()
    block_id_deletable2 = BlockID()
    block_id_deletable3 = BlockID()
    block_id_deletable4 = BlockID()
    with freeze_time("2000-01-01"):
        persistent_storage.set_clean_block(block_id_deletable1, b"deletable_data")
        persistent_storage.set_clean_block(block_id_deletable2, b"deletable_data")
        persistent_storage.set_clean_block(block_id_deletable3, b"deletable_data")

    with freeze_time("2000-01-02"):
        persistent_storage.get_clean_block(block_id_deletable2)

    with freeze_time("2000-01-03"):
        persistent_storage.get_clean_block(block_id_deletable3)
        persistent_storage.set_clean_block(block_id_deletable4, b"deletable_data")

    # Blocks 1 and 2 are the oldest
    persistent_storage.clear_clean_blocks(limit=2)

    persistent_storage.get_dirty_chunk(block_id_precious) == b"precious_data"
    persistent_storage.get_clean_block(block_id_deletable3) == b"deletable_data"
    persistent_storage.get_clean_block(block_id_deletable4) == b"deletable_data"
    with pytest.raises(FSLocalMissError):
        persistent_storage.get_clean_block(block_id_deletable1)
    with pytest.raises(FSLocalMissError):
        persistent_storage.get_clean_block(block_id_deletable2)


def test_local_automatic_run_garbage_collector(persistent_storage):
    persistent_storage.max_cache_size = 1 * block_size

    block_id_a = BlockID()
    persistent_storage.set_dirty_chunk(block_id_a, b"a" * 10)

    block_id_b = BlockID()
    persistent_storage.set_clean_block(block_id_b, b"b" * 5)

    data_b = persistent_storage.get_clean_block(block_id_b)
    assert data_b == b"b" * 5

    block_id_c = BlockID()
    persistent_storage.set_clean_block(block_id_c, b"c" * 5)

    data_a = persistent_storage.get_dirty_chunk(block_id_a)
    assert data_a == b"a" * 10

    with pytest.raises(FSLocalMissError):
        persistent_storage.get_clean_block(block_id_b)

    data_c = persistent_storage.get_clean_block(block_id_c)
    assert data_c == b"c" * 5


def test_persistent_storage_get_need_sync_and_checkpoint_lazy_defined(persistent_storage):
    need_sync_local, need_sync_remote = persistent_storage.get_need_sync_entries()
    assert need_sync_local == set()
    assert need_sync_remote == set()

    checkpoint = persistent_storage.get_realm_checkpoint()
    assert checkpoint == 0


def test_persistent_storage_local_need_sync(persistent_storage):
    author = DeviceID("a@a")
    e1 = EntryID("00000000000000000000000000000001")
    e2 = EntryID("00000000000000000000000000000002")
    e3 = EntryID("00000000000000000000000000000003")
    e4 = EntryID("00000000000000000000000000000004")

    # m1&m2 need sync, m3&m4 doesn't
    m1 = LocalUserManifest.new_placeholder(id=e1)
    m2 = LocalWorkspaceManifest.new_placeholder(id=e2)
    base_m3 = LocalFolderManifest.new_placeholder(id=e3, parent=e2).to_remote(author=author)
    m3 = LocalFolderManifest.from_remote(base_m3)
    base_m4 = LocalFileManifest.new_placeholder(id=e4, parent=e2).to_remote(author=author)
    m4 = LocalFileManifest.from_remote(base_m4)

    persistent_storage.set_manifest(e1, m1)
    persistent_storage.set_manifest(e2, m2)
    persistent_storage.set_manifest(e3, m3)
    persistent_storage.set_manifest(e4, m4)

    # m2 no longer need sync, m3 now need sync
    m22 = LocalWorkspaceManifest.from_remote(m2.to_remote(author=author))
    m32 = m3.evolve_and_mark_updated()

    persistent_storage.set_manifest(e2, m22)
    persistent_storage.set_manifest(e3, m32)

    need_sync_local, need_sync_remote = persistent_storage.get_need_sync_entries()
    assert need_sync_local == {e1, e3}
    assert need_sync_remote == set()


def test_persistent_storage_remote_need_sync(persistent_storage):
    author = DeviceID("a@a")
    e1 = EntryID("00000000000000000000000000000001")
    e2 = EntryID("00000000000000000000000000000002")
    e3 = EntryID("00000000000000000000000000000003")
    e4 = EntryID("00000000000000000000000000000004")

    # m1, m2 and m3 doesn't need sync
    base_m1 = LocalUserManifest.new_placeholder(id=e1).to_remote(author=author)
    m1 = LocalUserManifest.from_remote(base_m1)
    m2 = LocalUserManifest.from_remote(base_m1.evolve(id=e2))
    m3 = LocalUserManifest.from_remote(base_m1.evolve(id=e3))

    persistent_storage.set_manifest(e1, m1)
    persistent_storage.set_manifest(e2, m2)
    persistent_storage.set_manifest(e3, m3)

    persistent_storage.update_realm_checkpoint(41, {e1: 1, e2: 2, e3: 2, e4: 2})
    m22 = m2.evolve(base=m2.base.evolve(version=2))
    persistent_storage.set_manifest(e2, m22)
    persistent_storage.update_realm_checkpoint(42, {e2: 2})

    need_sync_local, need_sync_remote = persistent_storage.get_need_sync_entries()
    assert need_sync_local == set()
    assert need_sync_remote == {e3}

    checkpoint = persistent_storage.get_realm_checkpoint()
    assert checkpoint == 42


@pytest.mark.slow
def test_persistent_storage_stateful(tmpdir, hypothesis_settings):
    tentative = 0

    class PersistentStorageStateMachine(RuleBasedStateMachine):
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
                with pytest.raises(FSLocalMissError):
                    self.persistent_storage.get_dirty_chunk(block_id)
            else:
                data = self.persistent_storage.get_dirty_chunk(block_id)
                assert data == expected_data

        @rule(entry=DeletableEntry)
        def get_deletable_data(self, entry):
            block_id, expected_data = entry
            try:
                data = self.persistent_storage.get_clean_block(block_id)
                assert data == expected_data
            except FSLocalMissError:
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
            self.persistent_storage.set_dirty_chunk(block_id, data)
            return block_id, data

        @rule(entry=PreciousEntry)
        def clear_precious_data(self, entry):
            block_id, _ = entry
            if block_id in self.cleared_precious_data:
                with pytest.raises(FSLocalMissError):
                    self.persistent_storage.clear_dirty_chunk(block_id)
            else:
                self.persistent_storage.clear_dirty_chunk(block_id)
                self.cleared_precious_data.add(block_id)

        @rule(entry=DeletableEntry)
        def clear_deletable_data(self, entry):
            block_id, _ = entry
            try:
                self.persistent_storage.clear_clean_block(block_id)
            except FSLocalMissError:
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

    run_state_machine_as_test(PersistentStorageStateMachine, settings=hypothesis_settings)
