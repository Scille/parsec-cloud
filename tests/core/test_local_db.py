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
from parsec.core.types import ManifestAccess


@pytest.mark.trio
async def test_local_db_path(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)
    assert local_db.path == tmpdir
    assert local_db.max_cache_size == 128


@pytest.mark.trio
async def test_local_db_cache_size(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)

    access = ManifestAccess()
    assert local_db.get_cache_size() == 0

    local_db.set(access, b"data", False)
    assert local_db.get_cache_size() == 0

    local_db.set(access, b"data")
    assert local_db.get_cache_size() > 4


@pytest.mark.trio
async def test_local_db_set_get_clear(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)

    access = ManifestAccess()
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
    access = ManifestAccess()
    local_db.set(access, b"data")

    local_db_cpy = LocalDB(tmpdir, max_cache_size=128)
    data = local_db_cpy.get(access)
    assert data == b"data"


@pytest.mark.trio
async def test_local_manual_run_garbage_collector(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=128)

    access_precious = ManifestAccess()
    local_db.set(access_precious, b"precious_data", False)

    access_deletable = ManifestAccess()
    local_db.set(access_deletable, b"deletable_data")

    local_db.run_garbage_collector()
    local_db.get(access_precious) == b"precious_data"
    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access_deletable)


@pytest.mark.trio
async def test_local_automatic_run_garbage_collector(tmpdir):
    local_db = LocalDB(tmpdir, max_cache_size=16)

    access_a = ManifestAccess()
    local_db.set(access_a, b"a" * 10)

    access_b = ManifestAccess()
    local_db.set(access_b, b"b" * 5)

    data_b = local_db.get(access_b)
    assert data_b == b"b" * 5

    access_c = ManifestAccess()
    local_db.set(access_c, b"c" * 5)

    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access_a)

    with pytest.raises(LocalDBMissingEntry):
        local_db.get(access_b)

    data_c = local_db.get(access_c)
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

            self.local_db = LocalDB(tmpdir / f"local-db-{tentative}", max_cache_size=128)
            # Monkey patch to simplify test
            self.local_db._encrypt_with_symkey = lambda key, data: data
            self.local_db._decrypt_with_symkey = lambda key, data: data

        def teardown(self):
            pass

        @rule(entry=PreciousEntry)
        def get_precious_data(self, entry):
            access, expected_data = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalDBMissingEntry):
                    self.local_db.get(access)
            else:
                data = self.local_db.get(access)
                assert data == expected_data

        @rule(entry=DeletableEntry)
        def get_deletable_data(self, entry):
            access, expected_data = entry
            try:
                data = self.local_db.get(access)
                assert data == expected_data
            except LocalDBMissingEntry:
                pass

        @rule(target=DeletableEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_deletable_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.local_db.set(access, data, deletable=True)
            return access, data

        @rule(target=PreciousEntry, data_size=st.integers(min_value=0, max_value=64))
        def set_precious_data(self, data_size):
            access = ManifestAccess()
            data = b"x" * data_size
            self.local_db.set(access, data, deletable=False)
            return access, data

        @rule(entry=PreciousEntry)
        def clear_precious_data(self, entry):
            access, _ = entry
            if access.id in self.cleared_precious_data:
                with pytest.raises(LocalDBMissingEntry):
                    self.local_db.clear(access)
            else:
                self.local_db.clear(access)
                self.cleared_precious_data.add(access.id)

        @rule(entry=DeletableEntry)
        def clear_deletable_data(self, entry):
            access, _ = entry
            try:
                self.local_db.clear(access)
            except LocalDBMissingEntry:
                pass

        @rule()
        def gc(self):
            self.local_db.run_garbage_collector()

        @invariant()
        def check(self):
            if not hasattr(self, "local_db"):
                return
            assert self.local_db.get_cache_size() <= self.local_db.max_cache_size

    run_state_machine_as_test(LocalDBStateMachine, settings=hypothesis_settings)
