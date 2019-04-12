# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    Bundle,
    initialize,
    rule,
    invariant,
    run_state_machine_as_test,
    TrioRuleBasedStateMachine,
    multiple,
)

from tests.common import call_with_control


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)
st_entry_type = st.sampled_from(["file", "folder"])


def check_fs_dump(entry, is_root=True):
    assert not entry["need_sync"]
    assert entry["base_version"] == 2 if is_root else 1
    if "children" in entry:
        for k, v in entry["children"].items():
            check_fs_dump(v, is_root=False)


@pytest.mark.slow
def test_fs_online_idempotent_sync(
    hypothesis_settings, backend_addr, backend_factory, server_factory, fs_factory, alice
):
    class FSOnlineIdempotentSync(TrioRuleBasedStateMachine):
        BadPath = Bundle("bad_path")
        GoodFilePath = Bundle("good_file_path")
        GoodFolderPath = Bundle("good_folder_path")
        GoodPath = st.one_of(GoodFilePath, GoodFolderPath)

        async def start_fs(self, device):
            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=device) as fs:
                    await started_cb(fs=fs)

            return await self.get_root_nursery().start(call_with_control, _fs_controlled_cb)

        async def start_backend(self):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory() as backend:
                    async with server_factory(backend.handle_client, backend_addr) as server:
                        await started_cb(backend=backend, server=server)

            return await self.get_root_nursery().start(call_with_control, _backend_controlled_cb)

        @property
        def fs(self):
            return self.fs_controller.fs

        @initialize(target=BadPath)
        async def init(self):
            self.backend_controller = await self.start_backend()
            self.device = alice
            self.fs_controller = await self.start_fs(alice)

            await self.fs.workspace_create("/w")
            await self.fs.touch("/w/good_file.txt")
            await self.fs.folder_create("/w/good_folder")
            await self.fs.touch("/w/good_folder/good_sub_file.txt")
            await self.fs.sync("/")

            self.initial_fs_dump = self.fs._local_folder_fs.dump()
            check_fs_dump(self.initial_fs_dump)

            return "/w/dummy"

        @initialize(target=GoodFolderPath)
        async def init_good_folder_pathes(self):
            return multiple("/w/", "/w/good_folder/")

        @initialize(target=GoodFilePath)
        async def init_good_file_pathes(self):
            return multiple("/w/good_file.txt", "/w/good_folder/good_sub_file.txt")

        @rule(target=BadPath, type=st_entry_type, bad_parent=BadPath, name=st_entry_name)
        async def try_to_create_bad_path(self, type, bad_parent, name):
            path = os.path.join(bad_parent, name)
            with pytest.raises(OSError):
                if type == "file":
                    await self.fs.touch(path)
                else:
                    await self.fs.folder_create(path)
            return path

        @rule(type=st_entry_type, path=GoodPath)
        async def try_to_create_already_exists(self, type, path):
            with pytest.raises(OSError):
                if type == "file":
                    await self.fs.touch(path)
                else:
                    await self.fs.folder_create(path)

        @rule(path=BadPath)
        async def try_to_update_file(self, path):
            with pytest.raises(OSError):
                await self.fs.file_write(path, offset=0, content=b"a")

        @rule(path=BadPath)
        async def try_to_delete(self, path):
            with pytest.raises(OSError):
                await self.fs.delete(path)

        @rule(src=BadPath, dst_name=st_entry_name)
        async def try_to_move_bad_src(self, src, dst_name):
            dst = "/%s" % dst_name
            with pytest.raises(OSError):
                await self.fs.move(src, dst)

        @rule(src=GoodPath, dst=GoodFolderPath)
        async def try_to_move_bad_dst(self, src, dst):
            # TODO: why so much special cases ?
            if src == dst and src not in ("/", "/w/"):
                await self.fs.move(src, dst)
            else:
                with pytest.raises(OSError):
                    await self.fs.move(src, dst)

        @rule(path=GoodPath)
        async def sync(self, path):
            await self.fs.sync(path)

        @invariant()
        async def check_fs(self):
            try:
                fs_dump = self.fs._local_folder_fs.dump()
            except AttributeError:
                # FS not yet initialized
                pass
            else:
                assert fs_dump == self.initial_fs_dump

    run_state_machine_as_test(FSOnlineIdempotentSync, settings=hypothesis_settings)
