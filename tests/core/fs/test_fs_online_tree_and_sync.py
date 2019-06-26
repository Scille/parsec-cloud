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
    TrioAsyncioRuleBasedStateMachine,
)

from tests.common import call_with_control


def get_path(path):
    return path[2:] if path[2:] else "/"


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows path style not compatible with oracle")
def test_fs_online_tree_and_sync(
    hypothesis_settings,
    reset_testbed,
    backend_addr,
    backend_factory,
    server_factory,
    oracle_fs_with_sync_factory,
    local_storage_factory,
    user_fs_factory,
    alice,
):
    class FSOnlineTreeAndSync(TrioAsyncioRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        async def restart_user_fs(self, device, local_storage):
            try:
                await self.user_fs_controller.stop()
            except AttributeError:
                pass

            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(device=device, local_storage=local_storage) as user_fs:
                    await started_cb(user_fs=user_fs)

            self.user_fs_controller = await self.get_root_nursery().start(
                call_with_control, _user_fs_controlled_cb
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
        def user_fs(self):
            return self.user_fs_controller.user_fs

        @property
        def backend(self):
            return self.backend_controller.backend

        @property
        def workspace(self):
            return self.user_fs.get_workspace(self.wid)

        @initialize(target=Folders)
        async def init(self):
            await reset_testbed()
            self.oracle_fs = oracle_fs_with_sync_factory()
            self.oracle_fs.create_workspace("/w")
            self.device = alice
            self.local_storage = local_storage_factory(self.device)

            await self.start_backend()
            await self.restart_user_fs(self.device, self.local_storage)
            self.wid = await self.user_fs.workspace_create("w")
            workspace = self.user_fs.get_workspace(self.wid)
            await workspace.sync("/")
            await self.user_fs.sync()

            return "/w"

        @rule()
        async def restart(self):
            await self.restart_user_fs(self.device, self.local_storage)

        @rule()
        async def reset(self):
            # TODO: would be cleaner to recreate a new device...
            self.local_storage = local_storage_factory(self.device, force=True)
            await self.restart_user_fs(self.device, self.local_storage)
            self.oracle_fs.reset()
            self.oracle_fs.create_workspace("/w")
            await self.user_fs.sync()

        @rule()
        async def sync_root(self):
            await self.workspace.sync("/")
            await self.user_fs.sync()
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
                await self.workspace.touch(path=get_path(path), exist_ok=False)
            else:
                with pytest.raises((FileExistsError, FileNotFoundError, NotADirectoryError)):
                    await self.workspace.touch(path=get_path(path), exist_ok=False)
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        async def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_folder(path)
            if expected_status == "ok":
                await self.workspace.mkdir(path=get_path(path), exist_ok=False)
            else:
                with pytest.raises((FileExistsError, FileNotFoundError, NotADirectoryError)):
                    await self.workspace.mkdir(path=get_path(path), exist_ok=False)
            return path

        @rule(path=Files)
        async def delete_file(self, path):
            expected_status = self.oracle_fs.unlink(path)
            if expected_status == "ok":
                await self.workspace.unlink(path=get_path(path))
            else:
                with pytest.raises(OSError):
                    await self.workspace.unlink(path=get_path(path))
            return path

        @rule(path=Folders)
        async def delete_folder(self, path):
            expected_status = self.oracle_fs.rmdir(path)
            if expected_status == "ok":
                await self.workspace.rmdir(path=get_path(path))
            else:
                with pytest.raises(OSError):
                    await self.workspace.rmdir(path=get_path(path))
            return path

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        async def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.workspace.rename(get_path(src), get_path(dst))
            else:
                with pytest.raises(OSError):
                    await self.workspace.rename(get_path(src), get_path(dst))
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        async def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.workspace.rename(get_path(src), get_path(dst))
            else:
                with pytest.raises(OSError):
                    await self.workspace.rename(get_path(src), get_path(dst))
            return dst

        async def _stat(self, path):
            expected = self.oracle_fs.stat(path)
            if expected["status"] != "ok":
                if path == "/w":
                    await self.workspace.path_info(get_path(path))
                else:
                    with pytest.raises(OSError):
                        await self.workspace.path_info(get_path(path))
            else:
                path_info = await self.workspace.path_info(get_path(path))
                assert path_info["type"] == expected["type"]
                # TODO: oracle's `base_version` is broken (synchronization
                # strategy with parent placeholder make it complex to get right)
                # assert stat["base_version"] == expected["base_version"]
                if not path_info["need_sync"]:
                    assert path_info["base_version"] > 0
                    if path == "/w":
                        assert not path_info["is_placeholder"]
                    else:
                        assert path_info["is_placeholder"] == expected["is_placeholder"]

        @rule(path=Files)
        async def stat_file(self, path):
            await self._stat(path)

        @rule(path=Folders)
        async def stat_folder(self, path):
            await self._stat(path)

    run_state_machine_as_test(FSOnlineTreeAndSync, settings=hypothesis_settings)
