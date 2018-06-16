import os
import pytest
import pprint
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle, invariant, initialize

from parsec.utils import to_jsonb64

from tests.hypothesis.common import rule


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


def check_fs_dump(entry, is_root=True):
    assert not entry["need_sync"]
    assert entry["base_version"] == 2 if is_root else 1
    if "children" in entry:
        for k, v in entry["children"].items():
            check_fs_dump(v, is_root=False)


@pytest.mark.xfail
@pytest.mark.slow
@pytest.mark.trio
async def test_online_core_idempotent_sync(
    TrioDriverRuleBasedStateMachine,
    server_factory,
    backend_factory,
    core_factory_cm,
    core_sock_factory,
    device_factory,
):

    st_entry_type = st.sampled_from(["file", "folder"])

    class OnlineCoreIdempotentSync(TrioDriverRuleBasedStateMachine):
        BadPath = Bundle("bad_path")
        GoodPath = Bundle("good_path")

        async def trio_runner(self, task_status):
            self.core_cmd = self.communicator.send

            self.device = device_factory()
            self.backend = await backend_factory(devices=[self.device])
            server = server_factory(self.backend.handle_client)
            async with core_factory_cm(
                devices=[self.device], config={"backend_addr": server.addr, "auto_sync": False}
            ) as self.core:
                await self.core.login(self.device)

                await self.core.fs.file_create("/good_file.txt")
                await self.core.fs.folder_create("/good_folder")
                await self.core.fs.file_create("/good_folder/good_sub_file.txt")
                await self.core.fs.sync("/")

                sock = core_sock_factory(self.core)
                task_status.started()

                while True:
                    msg = await self.communicator.trio_recv()
                    await sock.send(msg)
                    rep = await sock.recv()
                    await self.communicator.trio_respond(rep)

        @initialize(target=BadPath)
        def init_bad_path(self):
            return "/dummy"

        @rule(
            target=GoodPath,
            path=st.sampled_from(
                ["/", "/good_file.txt", "/good_folder/", "/good_folder/good_sub_file.txt"]
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

        @rule(src=GoodPath, dst=GoodPath.filter(lambda x: x.endswith("/")))
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
            fs_dump = self.core.fs._local_folder_fs.dump()
            note("core fs dump: " + pprint.pformat(fs_dump))
            check_fs_dump(fs_dump)

    await OnlineCoreIdempotentSync.run_test()
