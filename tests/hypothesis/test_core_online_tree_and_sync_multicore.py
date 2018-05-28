import os
import pytest
import pprint
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle

from parsec.utils import to_jsonb64

from tests.common import connect_core, core_factory, backend_factory, run_app
from tests.hypothesis.common import rule, rule_once, failure_reproducer, reproduce_rule


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

    # The point is not to find breaking filenames here, so keep it simple
    st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)
    st_core = st.sampled_from(["core_1", "core_2"])

    @failure_reproducer(
        """
import pytest
import os

from parsec.utils import to_jsonb64

from tests.hypothesis.test_core_online_tree_and_sync_multicore import compare_fs_dumps


@pytest.mark.trio
async def test_reproduce(running_backend, core, alice_core_sock, core2, alice2_core2_sock):
    socks = {{'core_1': alice_core_sock, 'core_2': alice2_core2_sock}}
    {body}
"""
    )
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

            self.core_cmd = lambda x, y: self.communicator.send((x, y))

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
                                    core, msg = await self.communicator.trio_recv()
                                    await sockets[core].send(msg)
                                    rep = await sockets[core].recv()
                                    await self.communicator.trio_respond(rep)

                    finally:
                        tcp_stream_spy.install_hook(backend_addr, None)

        @rule_once(target=Folders)
        def get_root(self):
            return "/"

        @rule(target=Files, core=st_core, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
path = os.path.join({parent}, {name})
await socks[{core}].send({{"cmd": "file_create", "path": path}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
"""
        )
        def create_file(self, core, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd(core, {"cmd": "file_create", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return path

        @rule(core=st_core, path=Files, with_flush=st.booleans())
        @reproduce_rule(
            """
await socks[{core}].send({{
    "cmd": "file_write", "path": {path}, "offset": 0, "content": to_jsonb64(b"a")
}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
if {with_flush} and rep['status'] == 'ok':
    await socks[{core}].send({{"cmd": "flush", "path": {path}}})
    rep2 = await socks[{core}].recv()
    assert rep2["status"] == "ok"
"""
        )
        def update_file(self, core, path, with_flush):
            b64content = to_jsonb64(b"a")
            rep = self.core_cmd(
                core, {"cmd": "file_write", "path": path, "offset": 0, "content": b64content}
            )
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            if with_flush and rep["status"] == "ok":
                rep2 = self.core_cmd(core, {"cmd": "flush", "path": path})
                assert rep2["status"] == "ok"

        @rule(target=Folders, core=st_core, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
path = os.path.join({parent}, {name})
await socks[{core}].send({{"cmd": "folder_create", "path": path}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
"""
        )
        def create_folder(self, core, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd(core, {"cmd": "folder_create", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return path

        @rule(path=Files, core=st_core)
        @reproduce_rule(
            """
await socks[{core}].send({{"cmd": "delete", "path": {path}}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
"""
        )
        def delete_file(self, core, path):
            rep = self.core_cmd(core, {"cmd": "delete", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")

        @rule(path=Folders, core=st_core)
        @reproduce_rule(
            """
await socks[{core}].send({{"cmd": "delete", "path": {path}}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
"""
        )
        def delete_folder(self, core, path):
            rep = self.core_cmd(core, {"cmd": "delete", "path": path})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")

        @rule(target=Files, core=st_core, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        @reproduce_rule(
            """
dst = os.path.join({dst_parent}, {dst_name})
await socks[{core}].send({{"cmd": "move", "src": {src}, "dst": dst}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
"""
        )
        def move_file(self, core, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd(core, {"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return dst

        @rule(target=Folders, core=st_core, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        @reproduce_rule(
            """
dst = os.path.join({dst_parent}, {dst_name})
await socks[{core}].send({{"cmd": "move", "src": {src}, "dst": dst}})
rep = await socks[{core}].recv()
assert rep["status"] in ("ok", "invalid_path")
"""
        )
        def move_folder(self, core, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd(core, {"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] in ("ok", "invalid_path")
            return dst

        @rule()
        @reproduce_rule(
            """
await socks["core_1"].send({{"cmd": "synchronize", "path": "/"}})
rep = await socks["core_1"].recv()
assert rep["status"] == "ok"
await socks["core_2"].send({{"cmd": "synchronize", "path": "/"}})
rep = await socks["core_2"].recv()
assert rep["status"] == "ok"
await socks["core_1"].send({{"cmd": "synchronize", "path": "/"}})
rep = await socks["core_1"].recv()
assert rep["status"] == "ok"

fs_dump_1 = core.fs._local_tree.dump()
fs_dump_2 = core2.fs._local_tree.dump()
compare_fs_dumps(fs_dump_1, fs_dump_2)
"""
        )
        def sync_all_the_files(self):
            print("~~~ SYNC 1 ~~~")
            rep1 = self.core_cmd("core_1", {"cmd": "synchronize", "path": "/"})
            assert rep1["status"] == "ok"
            note("sync 1: %r" % rep1)
            print("~~~ SYNC 2 ~~~")
            rep2 = self.core_cmd("core_2", {"cmd": "synchronize", "path": "/"})
            assert rep2["status"] == "ok"
            note("sync 2: %r" % rep2)
            print("~~~ SYNC 1 ~~~")
            rep3 = self.core_cmd("core_1", {"cmd": "synchronize", "path": "/"})
            assert rep3["status"] == "ok"
            note("sync 1: %r" % rep3)

            fs_dump_1 = self.core_1.fs._local_tree.dump()
            fs_dump_2 = self.core_2.fs._local_tree.dump()
            note("core_1 fs dump: " + pprint.pformat(fs_dump_1))
            note("core_2 fs dump: " + pprint.pformat(fs_dump_2))
            compare_fs_dumps(fs_dump_1, fs_dump_2)

    await MultiCoreTreeAndSync.run_test()
