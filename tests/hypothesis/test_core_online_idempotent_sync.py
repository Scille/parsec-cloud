import os
import pytest
import pprint
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle, invariant

from parsec.utils import to_jsonb64

from tests.common import connect_core, core_factory, backend_factory, run_app
from tests.hypothesis.common import rule, rule_once


def check_fs_dump(entry):
    assert not entry["need_sync"]
    assert entry["base_version"] == 1
    if "children" in entry:
        for k, v in entry["children"].items():
            check_fs_dump(v)


@pytest.mark.slow
@pytest.mark.trio
async def test_online_core_idempotent_sync(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    tcp_stream_spy,
    backend_addr,
    tmpdir,
    alice,
):

    # The point is not to find breaking filenames here, so keep it simple
    st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)
    st_entry_type = st.sampled_from(["file", "folder"])

    class MultiCoreTreeAndSync(TrioDriverRuleBasedStateMachine):
        BadPath = Bundle("bad_path")
        GoodPath = Bundle("good_path")
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()

            type(self).count += 1
            backend_config = {"blockstore_postgresql": True}
            core_config = {
                "base_settings_path": tmpdir.mkdir("try-%s" % self.count).strpath,
                "backend_addr": backend_addr,
            }

            self.core_cmd = self.communicator.send

            async with backend_factory(**backend_config) as backend:
                await backend.user.create(
                    author="<backend-fixture>",
                    user_id=alice.user_id,
                    broadcast_key=alice.user_pubkey.encode(),
                    devices=[(alice.device_name, alice.device_verifykey.encode())],
                )

                async with run_app(backend) as backend_connection_factory:
                    tcp_stream_spy.install_hook(backend_addr, backend_connection_factory)

                    try:
                        async with core_factory(**core_config) as self.core:

                            await self.core.login(alice)

                            await self.core.fs.file_create("/good_file.txt")
                            await self.core.fs.folder_create("/good_folder")
                            await self.core.fs.file_create("/good_folder/good_sub_file.txt")
                            await self.core.fs.sync("/")

                            async with connect_core(self.core) as sock:
                                task_status.started()

                                while True:
                                    msg = await self.communicator.trio_recv()
                                    await sock.send(msg)
                                    rep = await sock.recv()
                                    await self.communicator.trio_respond(rep)

                    finally:
                        tcp_stream_spy.install_hook(backend_addr, None)

        @rule_once(target=BadPath)
        def init_bad_path(self):
            return "/dummy"

        @rule(
            target=GoodPath,
            path=st.sampled_from(
                ["/", "/good_file.txt", "/good_folder", "/good_folder/good_sub_file.txt"]
            ),
        )
        def init_good_path(self, path):
            return path

        @rule(target=BadPath, type=st_entry_type, bad_parent=BadPath, name=st_entry_name)
        def try_to_create_bad_path(self, type, bad_parent, name):
            path = os.path.join(bad_parent, name)
            rep = self.core_cmd({"cmd": "%s_create" % type, "path": path})
            note(rep)
            assert rep["status"] == "invalid_path"

            return path

        @rule(type=st_entry_type, path=GoodPath)
        def try_to_create_already_exists(self, type, path):
            rep = self.core_cmd({"cmd": "%s_create" % type, "path": path})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(path=BadPath)
        def try_to_update_file(self, path):
            b64content = to_jsonb64(b"a")
            rep = self.core_cmd(
                {"cmd": "file_write", "path": path, "offset": 0, "content": b64content}
            )
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(path=BadPath)
        def try_to_delete(self, path):
            rep = self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(src=BadPath, dst_name=st_entry_name)
        def try_to_move_bad_src(self, src, dst_name):
            dst = "/%s" % dst_name
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(src=GoodPath, dst=GoodPath)
        def try_to_move_bad_dst(self, src, dst):
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(path=st.one_of(GoodPath))
        def sync(self, path):
            rep = self.core_cmd({"cmd": "synchronize", "path": path})
            assert rep["status"] == "ok"
            note("sync: %r" % rep)

        @invariant()
        def check_fs(self):
            fs_dump = self.core.fs._local_tree.dump()
            note("core fs dump: " + pprint.pformat(fs_dump))
            pprint.pprint(fs_dump)
            check_fs_dump(fs_dump)

    await MultiCoreTreeAndSync.run_test()
