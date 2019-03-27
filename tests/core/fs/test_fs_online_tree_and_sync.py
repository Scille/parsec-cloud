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


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows path style not compatible with oracle")
def test_fs_online_tree_and_sync(
    hypothesis_settings,
    backend_addr,
    backend_factory,
    server_factory,
    oracle_fs_with_sync_factory,
    local_storage_factory,
    fs_factory,
    alice,
):
    class FSOnlineTreeAndSync(TrioRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        async def restart_fs(self, device, local_storage):
            try:
                await self.fs_controller.stop()
            except AttributeError:
                pass

            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=device, local_storage=local_storage) as fs:
                    await started_cb(fs=fs)

            self.fs_controller = await self.get_root_nursery().start(
                call_with_control, _fs_controlled_cb
            )

        async def start_backend(self):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory() as backend:
                    async with server_factory(backend.handle_client, backend_addr) as server:
                        await started_cb(backend=backend, server=server)

            self.backend_controller = await self.get_root_nursery().start(
                call_with_control, _backend_controlled_cb
            )

        @property
        def fs(self):
            return self.fs_controller.fs

        @property
        def backend(self):
            return self.backend_controller.backend

        @initialize(target=Folders)
        async def init(self):
            self.oracle_fs = oracle_fs_with_sync_factory()
            self.oracle_fs.create_workspace("/w")
            self.device = alice
            self.local_storage = local_storage_factory(self.device)

            await self.start_backend()
            await self.restart_fs(self.device, self.local_storage)
            await self.fs.workspace_create("/w")

            return "/w"

        @rule()
        async def restart(self):
            await self.restart_fs(self.device, self.local_storage)

        @rule()
        async def reset(self):
            # TODO: would be cleaner to recreate a new device...
            self.local_storage = local_storage_factory(self.device, force=True)
            await self.restart_fs(self.device, self.local_storage)
            await self.fs.sync("/")
            self.oracle_fs.reset()

        @rule()
        async def sync_root(self):
            await self.fs.sync("/")
            self.oracle_fs.sync("/")

        # TODO: really complex to implement...
        #         @rule(path=st.one_of(Folders, Files))
        #         def sync(self, path):
        #             rep = await self.core_cmd({"cmd": "synchronize", "path": path})
        #             note(rep)
        #             expected_status = self.oracle_fs.sync(path)
        #             assert rep["status"] == expected_status

        @rule(target=Files, parent=Folders, name=st_entry_name)
        async def create_file(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_file(path)
            if expected_status == "ok":
                await self.fs.file_create(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.file_create(path=path)
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        async def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_folder(path)
            if expected_status == "ok":
                await self.fs.folder_create(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.folder_create(path=path)
            return path

        @rule(path=Files)
        async def delete_file(self, path):
            # TODO: separate delete file from delete folder
            expected_status = self.oracle_fs.delete(path)
            if expected_status == "ok":
                await self.fs.delete(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.delete(path=path)
            return path

        @rule(path=Folders)
        async def delete_folder(self, path):
            # TODO: separate delete file from delete folder
            expected_status = self.oracle_fs.delete(path)
            if expected_status == "ok":
                await self.fs.delete(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.delete(path=path)
            return path

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        async def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.fs.move(src, dst)
            else:
                with pytest.raises(OSError):
                    await self.fs.move(src, dst)
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        async def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.fs.move(src, dst)
            else:
                with pytest.raises(OSError):
                    await self.fs.move(src, dst)
            return dst

        async def _stat(self, path):
            expected = self.oracle_fs.stat(path)
            if expected["status"] != "ok":
                with pytest.raises(OSError):
                    await self.fs.stat(path)
            else:
                stat = await self.fs.stat(path)
                assert stat["type"] == expected["type"]
                # TODO: oracle's `base_version` is broken (synchronization
                # strategy with parent placeholder make it complex to get right)
                # assert stat["base_version"] == expected["base_version"]
                if not stat["need_sync"]:
                    assert stat["base_version"] > 0
                assert stat["is_placeholder"] == expected["is_placeholder"]
                assert stat["need_sync"] == expected["need_sync"]

        @rule(path=Files)
        async def stat_file(self, path):
            await self._stat(path)

        @rule(path=Folders)
        async def stat_folder(self, path):
            await self._stat(path)

    run_state_machine_as_test(FSOnlineTreeAndSync, settings=hypothesis_settings)
