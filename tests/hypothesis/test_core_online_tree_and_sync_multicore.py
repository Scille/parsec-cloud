import os
import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle, invariant

from parsec.utils import to_jsonb64

from tests.common import connect_core, core_factory, backend_factory, run_app
from tests.hypothesis.common import rule, rule_once


async def get_tree_from_core(core):

    async def get_tree_from_folder_entry(entry):
        tree = {}
        for k, v in entry._children.items():
            if not v.is_loaded:
                v = await v.load()
            if isinstance(v, core.fs._file_entry_cls):
                tree[k] = v._access.dump()
            else:
                tree[k] = await get_tree_from_folder_entry(v)
        return tree

    return await get_tree_from_folder_entry(core.fs.root)


@pytest.mark.slow
@pytest.mark.trio
async def test_online_core_tree_and_sync_multicore(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    tcp_stream_spy,
    backend_addr,
    tmpdir,
    alice,
    alice2,
):

    st_entry_name = st.text(min_size=1).filter(lambda x: "/" not in x)
    st_core = st.sampled_from(["core_1", "core_2"])

    class MultiCoreTreeAndSync(TrioDriverRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()

            type(self).count += 1
            backend_config = {"blockstore_postgresql": True}
            core_config = {
                "base_settings_path": tmpdir.mkdir("try-%s" % self.count).strpath,
                "backend_addr": backend_addr,
            }

            self.sys_cmd = lambda x, y: self.communicator.send(("sys", x, y))
            self.core_cmd = lambda x, y: self.communicator.send(("core", x, y))

            async with backend_factory(**backend_config) as backend:
                await backend.user.create(
                    author="<backend-fixture>",
                    user_id=alice.user_id,
                    broadcast_key=alice.user_pubkey.encode(),
                    devices=[
                        (alice.device_name, alice.device_verifykey.encode()),
                        (alice2.device_name, alice2.device_verifykey.encode()),
                    ],
                )

                async with run_app(backend) as backend_connection_factory:
                    tcp_stream_spy.install_hook(backend_addr, backend_connection_factory)

                    try:
                        async with core_factory(**core_config) as self.core_1, core_factory(
                            **core_config
                        ) as self.core_2:

                            await self.core_1.login(alice)
                            await self.core_2.login(alice2)

                            async with connect_core(self.core_1) as sock_1, connect_core(
                                self.core_2
                            ) as sock_2:
                                task_status.started()

                                sockets = {"core_1": sock_1, "core_2": sock_2}

                                while True:
                                    target, core, msg = await self.communicator.trio_recv()
                                    if target == "core":
                                        await sockets[core].send(msg)
                                        rep = await sockets[core].recv()
                                        await self.communicator.trio_respond(rep)
                                    elif msg == "get_tree_from_core":
                                        core = self.core_1 if core == "core_1" else self.core_2
                                        await self.communicator.trio_respond(
                                            await get_tree_from_core(core)
                                        )

                    finally:
                        tcp_stream_spy.install_hook(backend_addr, None)

        @rule_once(target=Folders)
        def get_root(self):
            return "/"

        @rule(target=Files, core=st_core, parent=Folders, name=st_entry_name)
        def create_file(self, core, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd(core, {"cmd": "file_create", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return path

        @rule(core=st_core, path=Files)
        def update_file(self, core, path):
            b64content = to_jsonb64(b"a")
            rep = self.core_cmd(
                core, {"cmd": "file_write", "path": path, "offset": 0, "content": b64content}
            )
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")

        @rule(target=Folders, core=st_core, parent=Folders, name=st_entry_name)
        def create_folder(self, core, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd(core, {"cmd": "folder_create", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return path

        @rule(path=Files, core=st_core)
        def delete_file(self, core, path):
            rep = self.core_cmd(core, {"cmd": "delete", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")

        @rule(path=Folders, core=st_core)
        def delete_folder(self, core, path):
            rep = self.core_cmd(core, {"cmd": "delete", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")

        @rule(target=Files, core=st_core, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move_file(self, core, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd(core, {"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return dst

        @rule(target=Folders, core=st_core, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        def move_folder(self, core, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd(core, {"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return dst

        @rule()
        def sync_all_the_files(self):
            print("~~~ SYNC 1 ~~~")
            rep1 = self.core_cmd("core_1", {"cmd": "synchronize", "path": "/"})
            assert rep1["status"] == "ok"
            print("~~~ SYNC 2 ~~~")
            rep2 = self.core_cmd("core_2", {"cmd": "synchronize", "path": "/"})
            assert rep2["status"] == "ok"
            print("~~~ SYNC 1 ~~~")
            rep3 = self.core_cmd("core_1", {"cmd": "synchronize", "path": "/"})
            assert rep3["status"] == "ok"
            note((rep1, rep2, rep3))

            note("core_1 fs %r" % self.core_1.fs.root._children)
            note("core_2 fs %r" % self.core_2.fs.root._children)
            synced_tree_1 = self.sys_cmd("core_1", "get_tree_from_core")
            synced_tree_2 = self.sys_cmd("core_2", "get_tree_from_core")
            note((synced_tree_1, synced_tree_2))

            assert not self.core_1.fs.root.need_sync
            assert not self.core_2.fs.root.need_sync
            assert self.core_1.fs.root.base_version == self.core_2.fs.root.base_version
            assert synced_tree_1 == synced_tree_2

    await MultiCoreTreeAndSync.run_test()
