# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    Bundle,
    initialize,
    rule,
    run_state_machine_as_test,
    TrioRuleBasedStateMachine,
)

from tests.common import call_with_control

# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)
st_fs = st.sampled_from(["fs_1", "fs_2"])


def compare_fs_dumps(entry_1, entry_2):
    def cook_entry(entry):
        if "children" in entry:
            return {**entry, "children": {k: v["access"] for k, v in entry["children"].items()}}
        else:
            return entry

    assert not entry_1.get("need_sync", False)
    assert not entry_2.get("need_sync", False)

    if "need_sync" not in entry_1 or "need_sync" not in entry_2:
        # One of the entry is not loaded
        return

    assert cook_entry(entry_1) == cook_entry(entry_2)
    if "children" in entry_1:
        for key, child_for_entry_1 in entry_1["children"].items():
            child_for_entry_2 = entry_2["children"][key]
            compare_fs_dumps(child_for_entry_1, child_for_entry_2)


@pytest.mark.slow
def test_fs_online_concurrent_tree_and_sync(
    hypothesis_settings,
    backend_addr,
    backend_factory,
    server_factory,
    local_storage_factory,
    fs_factory,
    alice,
    alice2,
):
    class FSOnlineConcurrentTreeAndSync(TrioRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        FSs = Bundle("fs")

        async def start_fs(self, device, local_storage):
            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=device, local_storage=local_storage) as fs:
                    await started_cb(fs=fs)

            return await self.get_root_nursery().start(call_with_control, _fs_controlled_cb)

        async def start_backend(self, devices):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory() as backend:
                    async with server_factory(backend.handle_client, backend_addr) as server:
                        await started_cb(backend=backend, server=server)

            return await self.get_root_nursery().start(call_with_control, _backend_controlled_cb)

        @property
        def fs1(self):
            return self.fs1_controller.fs

        @property
        def fs2(self):
            return self.fs2_controller.fs

        @initialize(target=Folders)
        async def init(self):
            self.device1 = alice
            self.device2 = alice2
            self.local_storage1 = local_storage_factory(self.device1)
            self.local_storage2 = local_storage_factory(self.device2)

            self.backend_controller = await self.start_backend([self.device1, self.device2])
            self.fs1_controller = await self.start_fs(self.device1, self.local_storage1)
            self.fs2_controller = await self.start_fs(self.device2, self.local_storage2)

            await self.fs1.workspace_create("/w")
            await self.fs1.sync("/")
            await self.fs2.sync("/")

            return "/w"

        @initialize(target=FSs, force_after_init=Folders)
        async def register_fs1(self, force_after_init):
            return self.fs1

        @initialize(target=FSs, force_after_init=Folders)
        async def register_fs2(self, force_after_init):
            return self.fs2

        @rule(target=Files, fs=FSs, parent=Folders, name=st_entry_name)
        async def create_file(self, fs, parent, name):
            path = os.path.join(parent, name)
            try:
                await fs.touch(path=path)
            except OSError:
                pass
            return path

        @rule(target=Folders, fs=FSs, parent=Folders, name=st_entry_name)
        async def create_folder(self, fs, parent, name):
            path = os.path.join(parent, name)
            try:
                await fs.folder_create(path=path)
            except OSError:
                pass
            return path

        @rule(fs=FSs, path=Files)
        async def update_file(self, fs, path):
            try:
                await fs.file_write(path, offset=0, content=b"a")
            except OSError:
                pass

        @rule(fs=FSs, path=Files)
        async def delete_file(self, fs, path):
            try:
                await fs.file_delete(path=path)
            except OSError:
                pass
            return path

        @rule(fs=FSs, path=Folders)
        async def delete_folder(self, fs, path):
            try:
                await fs.folder_delete(path=path)
            except OSError:
                pass
            return path

        @rule(target=Files, fs=FSs, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        async def move_file(self, fs, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            try:
                await fs.move(src, dst)
            except OSError:
                pass
            return dst

        @rule(target=Folders, fs=FSs, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        async def move_folder(self, fs, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            try:
                await fs.move(src, dst)
            except OSError:
                pass
            return dst

        @rule()
        async def sync_all_the_files(self):
            # Less than 4 retries causes the following test case to fail:
            # ```python
            # state = FSOnlineConcurrentTreeAndSync()
            # async def steps():
            #     v1 = await state.init()
            #     v2 = await state.register_fs1(force_after_init=v1)
            #     v3 = await state.register_fs2(force_after_init=v1)
            #     v4 = await state.create_file(fs=v3, name='b', parent=v1)
            #     await state.sync_all_the_files()
            #     await state.update_file(fs=v3, path=v4)
            #     await state.update_file(fs=v2, path=v4)
            #     v5 = await state.create_file(fs=v3, name='a', parent=v1)
            #     v6 = await state.create_file(fs=v3, name='a', parent=v1)
            #     v7 = await state.move_file(dst_name='a', dst_parent=v1, fs=v2, src=v4)
            #     await state.sync_all_the_files()
            #     await state.teardown()
            # state.trio_run(steps)
            # ```
            retries = 4
            for _ in range(retries):
                await self.fs1.sync("/")
                await self.fs2.sync("/")

            fs_dump_1 = self.fs1._local_folder_fs.dump()
            fs_dump_2 = self.fs2._local_folder_fs.dump()
            compare_fs_dumps(fs_dump_1, fs_dump_2)

    run_state_machine_as_test(FSOnlineConcurrentTreeAndSync, settings=hypothesis_settings)
