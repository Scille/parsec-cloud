# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import attr
import pytest
from pathlib import Path
from string import ascii_lowercase
from hypothesis.stateful import (
    RuleBasedStateMachine,
    Bundle,
    initialize,
    rule,
    run_state_machine_as_test,
)
from hypothesis import strategies as st


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


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
    parsec_root = attr.ib()
    oracle_root = attr.ib()

    def to_oracle(self):
        return self.oracle_root / self.absolute_path[1:]

    def to_parsec(self):
        return self.parsec_root / self.absolute_path[1:]

    def __truediv__(self, path):
        return PathElement(
            os.path.join(self.absolute_path, path), self.parsec_root, self.oracle_root
        )


@pytest.mark.slow
@pytest.mark.fuse
@pytest.mark.skipif(os.name == "nt", reason="TODO: fix this ASAP !!!")
def test_fuse_folder_operations(tmpdir, hypothesis_settings, fuse_service):

    tentative = 0

    class FuseFolderOperationsStateMachine(RuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        @initialize(target=Folders)
        def init(self):
            nonlocal tentative
            tentative += 1

            fuse_service.start()

            self.folder_oracle = Path(tmpdir / f"oracle-test-{tentative}")
            self.folder_oracle.mkdir()
            oracle_root = self.folder_oracle / "root"
            oracle_root.mkdir()
            self.folder_oracle.chmod(0o500)  # Root oracle can no longer be removed this way
            (oracle_root / fuse_service.default_workspace_name).mkdir()

            return PathElement(
                f"/{fuse_service.default_workspace_name}",
                Path(str(fuse_service.mountpoint)),
                oracle_root,
            )

        def teardown(self):
            fuse_service.stop()

        @rule(target=Files, parent=Folders, name=st_entry_name)
        def touch(self, parent, name):
            path = parent / name

            expected_exc = None
            try:
                path.to_oracle().touch(exist_ok=False)
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().touch(exist_ok=False)

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
                path.to_parsec().mkdir(exist_ok=False)

            return path

        @rule(path=Files)
        def unlink(self, path):
            expected_exc = None
            try:
                path.to_oracle().unlink()
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().unlink()

        # TODO: trying to remove root gives a DeviceBusy error instead of PermissionError...
        @rule(path=Folders.filter(lambda x: x.absolute_path != "/"))
        def rmdir(self, path):
            expected_exc = None
            try:
                path.to_oracle().rmdir()
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().rmdir()

        def _move(self, src, dst_parent, dst_name):
            dst = dst_parent / dst_name

            expected_exc = None
            try:
                src.to_oracle().rename(str(dst.to_oracle()))
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                src.to_parsec().rename(str(dst.to_parsec()))

            return dst

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move_file(self, src, dst_parent, dst_name):
            return self._move(src, dst_parent, dst_name)

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        def move_folder(self, src, dst_parent, dst_name):
            return self._move(src, dst_parent, dst_name)

        @rule(path=Folders)
        def iterdir(self, path):
            expected_exc = None
            try:
                expected_children = {x.name for x in path.to_oracle().iterdir()}
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                children = {x.name for x in path.to_parsec().iterdir()}

            if not expected_exc:
                assert children == expected_children

    run_state_machine_as_test(FuseFolderOperationsStateMachine, settings=hypothesis_settings)
