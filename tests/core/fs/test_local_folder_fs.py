import os
import attr
import pytest
from pendulum import Pendulum
import pathlib
from string import ascii_lowercase
from hypothesis.stateful import (
    RuleBasedStateMachine,
    initialize,
    invariant,
    rule,
    run_state_machine_as_test,
    Bundle,
)
from hypothesis import strategies as st

from parsec.core.fs.local_folder_fs import FSManifestLocalMiss, Path, is_folder_manifest
from parsec.core.fs.utils import new_access, new_local_file_manifest

from tests.common import freeze_time


def test_stat_root(local_folder_fs):
    stat = local_folder_fs.stat(Path("/"))
    assert stat == {
        "type": "root",
        "is_folder": True,
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(2000, 1, 1),
        "updated": Pendulum(2000, 1, 1),
        "children": [],
    }


def test_workspace_create(local_folder_fs):
    with freeze_time("2000-01-02"):
        local_folder_fs.workspace_create(Path("/foo"))

    root_stat = local_folder_fs.stat(Path("/"))
    assert root_stat == {
        "type": "root",
        "is_folder": True,
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": True,
        "created": Pendulum(2000, 1, 1),
        "updated": Pendulum(2000, 1, 2),
        "children": ["foo"],
    }

    stat = local_folder_fs.stat(Path("/foo"))
    assert stat == {
        "type": "workspace",
        "is_folder": True,
        "base_version": 0,
        "is_placeholder": True,
        "need_sync": True,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "children": [],
        "creator": "alice",
        "participants": ["alice"],
    }


def test_file_create(local_folder_fs):
    with freeze_time("2000-01-02"):
        local_folder_fs.touch(Path("/foo.txt"))

    root_stat = local_folder_fs.stat(Path("/"))
    assert root_stat == {
        "type": "root",
        "is_folder": True,
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": True,
        "created": Pendulum(2000, 1, 1),
        "updated": Pendulum(2000, 1, 2),
        "children": ["foo.txt"],
    }

    foo_stat = local_folder_fs.stat(Path("/foo.txt"))
    assert foo_stat == {
        "type": "file",
        "is_folder": False,
        "base_version": 0,
        "is_placeholder": True,
        "need_sync": True,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "size": 0,
    }


@pytest.mark.parametrize("type", ["copy", "move"])
def test_copy_folders_create_new_accesses(local_folder_fs, type):
    with freeze_time("2000-01-02"):
        local_folder_fs.mkdir(Path("/foo"))
        local_folder_fs.mkdir(Path("/foo/bar"))
        local_folder_fs.mkdir(Path("/foo/bar/zob"))
        local_folder_fs.mkdir(Path("/foo/bar/zob/fizz.txt"))
        local_folder_fs.touch(Path("/foo/spam.txt"))

        existing_ids = set()
        pathes = ("", "bar", "bar/zob", "bar/zob/fizz.txt", "spam.txt")
        for path in pathes:
            access, _ = local_folder_fs.get_entry(Path("/foo/" + path))
            existing_ids.add(access["id"])

    if type == "copy":
        local_folder_fs.copy(Path("/foo"), Path("/foo2"))
        stat = local_folder_fs.stat(Path("/"))
        assert stat["children"] == ["foo", "foo2"]
    else:
        local_folder_fs.move(Path("/foo"), Path("/foo2"))
        stat = local_folder_fs.stat(Path("/"))
        assert stat["children"] == ["foo2"]

        for path in pathes:
            access, _ = local_folder_fs.get_entry(Path("/foo2/" + path))
            assert access["id"] not in existing_ids


def test_rename_but_cannot_move_workspace(local_folder_fs):
    local_folder_fs.workspace_create(Path("/spam"))
    # Workpsace children should not have they access modified
    local_folder_fs.mkdir(Path("/spam/bar"))

    # Test bad move/copy

    with pytest.raises(PermissionError):
        local_folder_fs.move(Path("/spam"), Path("/foo"))
    with pytest.raises(PermissionError):
        local_folder_fs.copy(Path("/spam"), Path("/foo"))

    # Now test the good rename

    old_name_stat = local_folder_fs.stat(Path("/spam"))

    local_folder_fs.workspace_rename(Path("/spam"), Path("/foo"))

    stat = local_folder_fs.stat(Path("/"))
    assert stat["children"] == ["foo"]

    new_name_stat = local_folder_fs.stat(Path("/foo"))
    assert old_name_stat == new_name_stat


def test_folder_file_outside_workpace_not_ok(local_folder_fs_factory, alice):
    # Well folder/file created inside directly as root children are everywhere
    # in the tests (because it's very convenient...), but in real life we
    # don't want this.
    local_folder_fs = local_folder_fs_factory(alice, allow_non_workpace_in_root=False)

    with pytest.raises(PermissionError):
        local_folder_fs.touch(Path("/foo"))
    with pytest.raises(PermissionError):
        local_folder_fs.mkdir(Path("/bar"))

    # Make sure it is ok inside workspace
    local_folder_fs.workspace_create(Path("/spam"))
    local_folder_fs.touch(Path("/spam/foo"))
    local_folder_fs.mkdir(Path("/spam/bar"))


def test_not_root_child_bad_workspace_create(local_folder_fs):
    local_folder_fs.mkdir(Path("/foo"))
    with pytest.raises(PermissionError):
        local_folder_fs.workspace_create(Path("/foo/bar"))
    local_folder_fs.workspace_create(Path("/spam"))


def test_cannot_replace_root(local_folder_fs):
    with pytest.raises(FileExistsError):
        local_folder_fs.touch(Path("/"))
    with pytest.raises(FileExistsError):
        local_folder_fs.mkdir(Path("/"))

    local_folder_fs.mkdir(Path("/foo"))
    with pytest.raises(PermissionError):
        local_folder_fs.move(Path("/"), Path("/foo"))
    with pytest.raises(PermissionError):
        local_folder_fs.move(Path("/foo"), Path("/"))


def test_access_not_loaded_entry(alice, local_folder_fs):
    user_manifest = local_folder_fs.get_manifest(alice.user_manifest_access)
    with freeze_time("2000-01-02"):
        foo_access = new_access()
        foo_manifest = new_local_file_manifest("bob@test")
        user_manifest["children"]["foo.txt"] = foo_access
        local_folder_fs.set_manifest(alice.user_manifest_access, user_manifest)

    with pytest.raises(FSManifestLocalMiss):
        local_folder_fs.stat(Path("/foo.txt"))

    local_folder_fs.set_manifest(foo_access, foo_manifest)
    stat = local_folder_fs.stat(Path("/foo.txt"))
    assert stat == {
        "type": "file",
        "is_folder": False,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "base_version": 0,
        "is_placeholder": True,
        "need_sync": True,
        "size": 0,
    }


def test_access_unknown_entry(local_folder_fs):
    with pytest.raises(FileNotFoundError):
        local_folder_fs.stat(Path("/dummy"))


class expect_raises:
    def __init__(self, expected_exc):
        self.expected_exc = expected_exc

    def __enter__(self):
        __tracebackhide__ = True
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        __tracebackhide__ = True

        if self.expected_exc is None:
            return False

        if not exc_type:
            raise AssertionError(f"DID NOT RAISED {self.expected_exc!r}")

        if not isinstance(exc_value, type(self.expected_exc)):
            raise AssertionError(
                f"RAISED {exc_value!r} BUT EXPECTED {self.expected_exc!r}"
            ) from exc_value

        return True


@attr.s
class PathElement:
    absolute_path = attr.ib()
    oracle_root = attr.ib()

    def to_oracle(self):
        return self.oracle_root / self.absolute_path[1:]

    def to_parsec(self):
        return Path(self.absolute_path)

    def __truediv__(self, path):
        return PathElement(os.path.join(self.absolute_path, path), self.oracle_root)


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows path style not compatible with oracle")
def test_folder_operations(tmpdir, hypothesis_settings, device_factory, local_folder_fs_factory):
    tentative = 0

    # The point is not to find breaking filenames here, so keep it simple
    st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)

    class FileOperationsStateMachine(RuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        @initialize(target=Folders)
        def init_root(self):
            nonlocal tentative
            tentative += 1

            self.last_step_id_to_path = set()
            self.device = device_factory()
            self.local_folder_fs = local_folder_fs_factory(self.device)
            self.folder_oracle = pathlib.Path(tmpdir / f"oracle-test-{tentative}")
            self.folder_oracle.mkdir()
            oracle_root = self.folder_oracle / "root"
            oracle_root.mkdir()
            self.folder_oracle.chmod(0o500)  # Root oracle can no longer be removed this way

            return PathElement("/", oracle_root)

        @rule(target=Files, parent=Folders, name=st_entry_name)
        def touch(self, parent, name):
            path = parent / name

            expected_exc = None
            try:
                path.to_oracle().touch(exist_ok=False)
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                self.local_folder_fs.touch(path.to_parsec())

            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        def mkdir(self, parent, name):
            path = parent / name

            expected_exc = None
            try:
                path.to_oracle().mkdir(exist_ok=False)
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                self.local_folder_fs.mkdir(path.to_parsec())

            return path

        @rule(path=Files)
        def unlink(self, path):
            expected_exc = None
            try:
                path.to_oracle().unlink()
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                self.local_folder_fs.unlink(path.to_parsec())

        @rule(path=Folders)
        def rmdir(self, path):
            expected_exc = None
            try:
                path.to_oracle().rmdir()
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                self.local_folder_fs.rmdir(path.to_parsec())

        def _move(self, src, dst_parent, dst_name):
            dst = dst_parent / dst_name

            expected_exc = None
            try:
                src.to_oracle().rename(str(dst.to_oracle()))
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                self.local_folder_fs.move(src.to_parsec(), dst.to_parsec())

            return dst

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move_file(self, src, dst_parent, dst_name):
            return self._move(src, dst_parent, dst_name)

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        def move_folder(self, src, dst_parent, dst_name):
            return self._move(src, dst_parent, dst_name)

        @invariant()
        def check_access_to_path_unicity(self):
            try:
                local_folder_fs = self.local_folder_fs
            except AttributeError:
                return

            new_id_to_path = set()

            def _recursive_build_id_to_path(access, path):
                new_id_to_path.add((access["id"], path))
                try:
                    manifest = local_folder_fs.get_manifest(access)
                except FSManifestLocalMiss:
                    return
                if is_folder_manifest(manifest):
                    for child_name, child_access in manifest["children"].items():
                        _recursive_build_id_to_path(child_access, f"{path}/{child_name}")

            _recursive_build_id_to_path(local_folder_fs.root_access, "/")

            added_items = new_id_to_path - self.last_step_id_to_path
            for added_id, added_path in added_items:
                for old_id, old_path in self.last_step_id_to_path:
                    if old_id == added_id:
                        raise AssertionError(
                            f"Same id ({old_id}) but different path: {old_path} -> {added_path}"
                        )

            self.last_step_id_to_path = new_id_to_path

    run_state_machine_as_test(FileOperationsStateMachine, settings=hypothesis_settings)
