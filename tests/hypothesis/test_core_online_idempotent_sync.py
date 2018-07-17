import os
import pytest
import pprint
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle, invariant
from hypothesis_trio.stateful import run_state_machine_as_test

from parsec.utils import to_jsonb64

from tests.hypothesis.common import initialize, rule


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


def check_fs_dump(entry, is_root=True):
    assert not entry["need_sync"]
    assert entry["base_version"] == 2 if is_root else 1
    if "children" in entry:
        for k, v in entry["children"].items():
            check_fs_dump(v, is_root=False)


@pytest.mark.slow
def test_online_core_idempotent_sync(BaseCoreWithBackendStateMachine, hypothesis_settings):

    st_entry_type = st.sampled_from(["file", "folder"])

    class OnlineCoreIdempotentSync(BaseCoreWithBackendStateMachine):
        BadPath = Bundle("bad_path")
        GoodPath = Bundle("good_path")

        @initialize(target=BadPath, core=BaseCoreWithBackendStateMachine.Cores)
        async def init_fs(self, core):
            await core.fs.file_create("/good_file.txt")
            await core.fs.folder_create("/good_folder")
            await core.fs.file_create("/good_folder/good_sub_file.txt")
            await core.fs.sync("/")
            self.init_done = True
            self.initial_fs_dump = self.core_info.core.fs._local_folder_fs.dump()
            check_fs_dump(self.initial_fs_dump)
            return "/dummy"

        @rule(
            target=GoodPath,
            path=st.sampled_from(
                ["/", "/good_file.txt", "/good_folder/", "/good_folder/good_sub_file.txt"]
            ),
        )
        async def init_good_path(self, path):
            return path

        @rule(target=BadPath, type=st_entry_type, bad_parent=BadPath, name=st_entry_name)
        async def try_to_create_bad_path(self, type, bad_parent, name):
            path = os.path.join(bad_parent, name)
            rep = await self.core_cmd({"cmd": "%s_create" % type, "path": path})
            note(rep)
            assert rep["status"] == "invalid_path"

            return path

        @rule(type=st_entry_type, path=GoodPath)
        async def try_to_create_already_exists(self, type, path):
            rep = await self.core_cmd({"cmd": "%s_create" % type, "path": path})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(path=BadPath)
        async def try_to_update_file(self, path):
            b64content = to_jsonb64(b"a")
            rep = await self.core_cmd(
                {"cmd": "file_write", "path": path, "offset": 0, "content": b64content}
            )
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(path=BadPath)
        async def try_to_delete(self, path):
            rep = await self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(src=BadPath, dst_name=st_entry_name)
        async def try_to_move_bad_src(self, src, dst_name):
            dst = "/%s" % dst_name
            rep = await self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] == "invalid_path"

        @rule(src=GoodPath, dst=GoodPath.filter(lambda x: x.endswith("/")))
        async def try_to_move_bad_dst(self, src, dst):
            rep = await self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            assert rep["status"] == "invalid_path" if src != dst else "ok"

        @rule(path=st.one_of(GoodPath))
        async def sync(self, path):
            rep = await self.core_cmd({"cmd": "synchronize", "path": path})
            assert rep["status"] == "ok"
            note("sync: %r" % rep)

        @invariant()
        async def check_fs(self):
            if not getattr(self, "init_done", False):
                return
            fs_dump = self.core_info.core.fs._local_folder_fs.dump()
            note("core fs dump: " + pprint.pformat(fs_dump))
            assert fs_dump == self.initial_fs_dump

    run_state_machine_as_test(OnlineCoreIdempotentSync, settings=hypothesis_settings)
