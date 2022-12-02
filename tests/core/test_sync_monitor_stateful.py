# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    Bundle,
    TrioAsyncioRuleBasedStateMachine,
    initialize,
    multiple,
    rule,
    run_state_machine_as_test,
)

from parsec.api.data import EntryName
from parsec.core.fs import FSWorkspaceNoAccess, FSWorkspaceNotFoundError
from parsec.core.types import WorkspaceRole
from tests.common import call_with_control


MISSING = object()
TO_COMPARE_FIELDS = ("id", "created", "updated", "need_sync", "base_version", "is_placeholder")


def recursive_compare_fs_dumps(alice_dump, bob_dump, ignore_need_sync=False):
    if ignore_need_sync and alice_dump.get("need_sync", False):
        return

    alice_stripped_dump = {}
    bob_stripped_dump = {}
    for field in TO_COMPARE_FIELDS:
        alice_value = alice_dump.get(field, MISSING)
        bob_value = bob_dump.get(field, MISSING)
        if alice_value is not MISSING and bob_value is not MISSING:
            alice_stripped_dump[field] = alice_value
            bob_stripped_dump[field] = bob_value
    assert alice_stripped_dump == bob_stripped_dump

    alice_children = alice_dump.get("children", MISSING)
    bob_children = bob_dump.get("children", MISSING)
    if alice_children is not MISSING and bob_children is not MISSING:
        assert alice_children.keys() == bob_children.keys()
        for key, alice_value in alice_children.items():
            bob_value = bob_children[key]
            recursive_compare_fs_dumps(alice_value, bob_value, ignore_need_sync=ignore_need_sync)


@pytest.mark.slow
@pytest.mark.flaky(reruns=2)
def test_sync_monitor_stateful(
    hypothesis_settings,
    reset_testbed,
    backend_factory,
    running_backend_factory,
    core_factory,
    user_fs_factory,
    alice,
    bob,
    monkeypatch,
):
    monkeypatch.setattr("parsec.utils.BALLPARK_ALWAYS_OK", True)

    class SyncMonitorStateful(TrioAsyncioRuleBasedStateMachine):
        SharedWorkspaces = Bundle("shared_workspace")
        SyncedFiles = Bundle("synced_files")

        def __init__(self):
            super().__init__()
            self.file_count = 0
            self.data_count = 0
            self.workspace_count = 0

        def get_next_file_path(self):
            self.file_count = self.file_count + 1
            return f"/file-{self.file_count}.txt"

        def get_next_data(self):
            self.data_count = self.data_count + 1
            return f"data {self.data_count}".encode()

        def get_next_workspace_name(self):
            self.workspace_count = self.workspace_count + 1
            return EntryName(f"w{self.workspace_count}")

        def get_workspace(self, device_id, wid):
            return self.user_fs_per_device[device_id].get_workspace(wid)

        async def start_alice_core(self):
            async def _core_controlled_cb(started_cb):
                async with core_factory(self.alice) as core:
                    await started_cb(core=core)

            self.alice_core_controller = await self.get_root_nursery().start(
                call_with_control, _core_controlled_cb
            )
            return self.alice_core_controller.core

        async def start_bob_user_fs(self):
            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(device=self.bob) as user_fs:
                    await started_cb(user_fs=user_fs)

            self.bob_user_fs_controller = await self.get_root_nursery().start(
                call_with_control, _user_fs_controlled_cb
            )
            return self.bob_user_fs_controller.user_fs

        async def start_backend(self):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory() as backend:
                    async with running_backend_factory(backend) as server:
                        await started_cb(backend=backend, server=server)

            self.backend_controller = await self.get_root_nursery().start(
                call_with_control, _backend_controlled_cb
            )

        @initialize()
        async def init(self):
            await reset_testbed()

            await self.start_backend()
            self.alice = self.backend_controller.server.correct_addr(alice)
            self.bob = self.backend_controller.server.correct_addr(bob)
            # Align alice and bob by using the same time provider
            self.bob.time_provider = self.alice.time_provider
            self.bob_user_fs = await self.start_bob_user_fs()
            self.alice_core = await self.start_alice_core()
            self.user_fs_per_device = {
                alice.device_id: self.alice_core.user_fs,
                bob.device_id: self.bob_user_fs,
            }
            self.synced_files = set()
            self.alice_workspaces_role = {}

        @rule(
            target=SharedWorkspaces,
            role=st.one_of(st.just(WorkspaceRole.CONTRIBUTOR), st.just(WorkspaceRole.READER)),
        )
        async def create_sharing(self, role):
            wname = self.get_next_workspace_name()
            wid = await self.bob_user_fs.workspace_create(wname)
            await self.bob_user_fs.workspace_share(wid, alice.user_id, role)
            self.alice_workspaces_role[wid] = role
            return wid

        @rule(
            wid=SharedWorkspaces,
            new_role=st.one_of(
                st.just(WorkspaceRole.CONTRIBUTOR), st.just(WorkspaceRole.READER), st.just(None)
            ),
        )
        async def update_sharing(self, wid, new_role):
            await self.bob_user_fs.workspace_share(wid, alice.user_id, new_role)
            self.alice_workspaces_role[wid] = new_role

        @rule(
            author=st.one_of(st.just(alice.device_id), st.just(bob.device_id)), wid=SharedWorkspaces
        )
        async def create_file(self, author, wid):
            file_path = self.get_next_file_path()
            if author == bob.device_id:
                await self._bob_update_file(wid, file_path, create_file=True)
            else:
                await self._alice_update_file(wid, file_path, create_file=True)

        @rule(author=st.one_of(st.just(alice.device_id), st.just(bob.device_id)), file=SyncedFiles)
        async def update_file(self, author, file):
            wid, file_path = file
            if author == bob.device_id:
                await self._bob_update_file(wid, file_path)
            else:
                await self._alice_update_file(wid, file_path)

        async def _bob_update_file(self, wid, file_path, create_file=False):
            wfs = self.get_workspace(bob.device_id, wid)
            if create_file:
                await wfs.touch(file_path)
            else:
                data = self.get_next_data()
                await wfs.write_bytes(file_path, data)
            await wfs.sync()

        async def _alice_update_file(self, wid, file_path, create_file=False):
            try:
                wfs = self.get_workspace(alice.device_id, wid)
            except FSWorkspaceNotFoundError:
                return
            if create_file:
                try:
                    await wfs.touch(file_path)
                except (FSWorkspaceNoAccess, OSError):
                    return
            else:
                data = self.get_next_data()
                try:
                    if await wfs.exists(file_path):
                        await wfs.write_bytes(file_path, data)
                except (FSWorkspaceNoAccess, OSError):
                    return

        @rule(target=SyncedFiles)
        async def let_core_monitors_process_changes(self):
            # Set a high clock speed
            self.alice.time_provider.mock_time(speed=1000.0)
            try:
                # Loop over 100 attemps (at least 100 seconds)
                attempts = 100
                while True:
                    # Bob get back alice's changes
                    await self.bob_user_fs.sync()
                    for bob_workspace_entry in self.bob_user_fs.get_user_manifest().workspaces:
                        bob_w = self.bob_user_fs.get_workspace(bob_workspace_entry.id)
                        await bob_w.sync()
                    # Alice get back possible changes from bob's sync
                    await self.alice.time_provider.sleep(1)
                    # See if alice and bob are in sync
                    try:
                        new_synced_files = await self.assert_alice_and_bob_in_sync()
                    # Not yet in sync, wait and try again
                    except AssertionError:
                        attempts -= 1
                        if attempts == 0:
                            raise
                        await self.alice.time_provider.sleep(1)
                        continue
                    # Success !
                    else:
                        self.synced_files.update(new_synced_files)
                        return multiple(*(sorted(new_synced_files)))
            # Reset clock speed
            finally:
                self.alice.time_provider.mock_time(speed=1.0)

        async def assert_alice_and_bob_in_sync(self):
            new_synced_files = []

            # Now alice and bob should have agreed on the data
            for alice_workspace_entry in self.alice_core.user_fs.get_user_manifest().workspaces:
                alice_w = self.alice_core.user_fs.get_workspace(alice_workspace_entry.id)
                bob_w = self.bob_user_fs.get_workspace(alice_workspace_entry.id)

                if alice_workspace_entry.role is None:
                    # No access, workspace can only diverge from bob's
                    continue

                bob_dump = await bob_w.dump()
                alice_dump = await alice_w.dump()
                if self.alice_workspaces_role[alice_workspace_entry.id] == WorkspaceRole.READER:
                    # Synced with bob, but we can have local changes that cannot be synced
                    recursive_compare_fs_dumps(alice_dump, bob_dump, ignore_need_sync=True)
                else:
                    # Fully synced with bob
                    recursive_compare_fs_dumps(alice_dump, bob_dump, ignore_need_sync=False)

                for child_name in bob_dump["children"].keys():
                    key = (alice_workspace_entry.id, f"/{child_name}")
                    if key not in self.synced_files:
                        new_synced_files.append(key)

            return new_synced_files

    run_state_machine_as_test(SyncMonitorStateful, settings=hypothesis_settings)
