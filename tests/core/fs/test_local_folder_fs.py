import os
import attr
import pytest
from pendulum import Pendulum
from pathlib import Path
from string import ascii_lowercase
from hypothesis.stateful import (
    RuleBasedStateMachine,
    initialize,
    rule,
    run_state_machine_as_test,
    Bundle,
)
from hypothesis import strategies as st

from parsec.core.fs.local_folder_fs import FSManifestLocalMiss
from parsec.core.fs.data import new_access, new_local_file_manifest

from tests.common import freeze_time


def test_stat_root(local_folder_fs):
    stat = local_folder_fs.stat("/")
    assert stat == {
        "type": "folder",
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(2000, 1, 1),
        "updated": Pendulum(2000, 1, 1),
        "children": [],
    }


def test_file_create(local_folder_fs):
    with freeze_time("2000-01-02"):
        local_folder_fs.touch("/foo.txt")

    root_stat = local_folder_fs.stat("/")
    assert root_stat == {
        "type": "folder",
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": True,
        "created": Pendulum(2000, 1, 1),
        "updated": Pendulum(2000, 1, 2),
        "children": ["foo.txt"],
    }

    foo_stat = local_folder_fs.stat("/foo.txt")
    assert foo_stat == {
        "type": "file",
        "base_version": 0,
        "is_placeholder": True,
        "need_sync": True,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "size": 0,
    }


def test_access_not_loaded_entry(alice, local_folder_fs):
    user_manifest = local_folder_fs.get_manifest(alice.user_manifest_access)
    with freeze_time("2000-01-02"):
        foo_access = new_access()
        foo_manifest = new_local_file_manifest("bob@test")
        user_manifest["children"]["foo.txt"] = foo_access
        local_folder_fs.set_manifest(alice.user_manifest_access, user_manifest)

    with pytest.raises(FSManifestLocalMiss):
        local_folder_fs.stat("/foo.txt")

    local_folder_fs.set_manifest(foo_access, foo_manifest)
    stat = local_folder_fs.stat("/foo.txt")
    assert stat == {
        "type": "file",
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "base_version": 0,
        "is_placeholder": True,
        "need_sync": True,
        "size": 0,
    }


def test_access_unknown_entry(local_folder_fs):
    with pytest.raises(FileNotFoundError):
        local_folder_fs.stat("/dummy")


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
        return self.absolute_path

    def __truediv__(self, path):
        return PathElement(os.path.join(self.absolute_path, path), self.oracle_root)


@pytest.mark.slow
def test_folder_operations(
    tmpdir, hypothesis_settings, signal_ns, device_factory, local_folder_fs_factory
):
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

            self.device = device_factory()
            self.local_folder_fs = local_folder_fs_factory(self.device)
            self.folder_oracle = Path(tmpdir / f"oracle-test-{tentative}")
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

        @rule(target=Folders | Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move(self, src, dst_parent, dst_name):
            dst = dst_parent / dst_name

            expected_exc = None
            try:
                src.to_oracle().rename(str(dst.to_oracle()))
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                self.local_folder_fs.move(src.to_parsec(), dst.to_parsec())

            return dst

    run_state_machine_as_test(FileOperationsStateMachine, settings=hypothesis_settings)
