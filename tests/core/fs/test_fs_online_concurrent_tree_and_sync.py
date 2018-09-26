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
    oracle_fs_with_sync_factory,
    unused_tcp_addr,
    device_factory,
    backend_factory,
    server_factory,
    fs_factory,
    backend_addr,
):
    class FSOnlineConcurrentTreeAndSync(TrioRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        FSs = Bundle("fs")

        async def start_fs(self, device):
            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=device, backend_addr=backend_addr) as fs:
                    await started_cb(fs=fs)

            return await self.get_root_nursery().start(call_with_control, _fs_controlled_cb)

        async def start_backend(self, devices):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory(devices=devices) as backend:
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
            self.oracle_fs = oracle_fs_with_sync_factory()
            self.device1 = device_factory()
            self.device2 = device_factory(user_id=self.device1.user_id)

            self.backend_controller = await self.start_backend([self.device1, self.device2])
            self.fs1_controller = await self.start_fs(self.device1)
            self.fs2_controller = await self.start_fs(self.device2)

            return "/"

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
                await fs.file_create(path=path)
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
            # TODO: separate delete file from delete folder
            try:
                await fs.delete(path=path)
            except OSError:
                pass
            return path

        @rule(fs=FSs, path=Folders)
        async def delete_folder(self, fs, path):
            # TODO: separate delete file from delete folder
            try:
                await fs.delete(path=path)
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
            print("~~~ SYNC 1 ~~~")
            # Send two syncs in a row given file conflict results are not synced
            # once created
            await self.fs1.sync("/")
            await self.fs1.sync("/")
            print("~~~ SYNC 2 ~~~")
            await self.fs2.sync("/")
            await self.fs2.sync("/")
            print("~~~ SYNC 1 ~~~")
            await self.fs1.sync("/")

            fs_dump_1 = self.fs1._local_folder_fs.dump()
            fs_dump_2 = self.fs2._local_folder_fs.dump()
            compare_fs_dumps(fs_dump_1, fs_dump_2)

    run_state_machine_as_test(FSOnlineConcurrentTreeAndSync, settings=hypothesis_settings)
