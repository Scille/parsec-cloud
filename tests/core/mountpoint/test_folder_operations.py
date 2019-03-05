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
from hypothesis import strategies as st, reproduce_failure


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


class expect_raises:
    def __init__(self, expected_exc, fallback_exc=None):
        self.expected_exc = expected_exc
        self.fallback_exc = fallback_exc

    def __enter__(self):
        __tracebackhide__ = True
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        __tracebackhide__ = True

        if not self.expected_exc:
            return False

        if not exc_type:
            raise AssertionError(f"DID NOT RAISED {self.expected_exc!r}")

        if self.fallback_exc:
            allowed = (type(self.expected_exc), type(self.fallback_exc))
        else:
            allowed = type(self.expected_exc)
        if not isinstance(exc_value, allowed):
            raise AssertionError(
                f"RAISED {exc_value!r} BUT EXPECTED {self.expected_exc!r}"
            ) from exc_value

        return True


@attr.s
class PathElement:
    absolute_path = attr.ib()
    parsec_root = attr.ib()
    oracle_root = attr.ib()

    def is_workspace(self):
        return len(Path(self.absolute_path).parts) == 2

    def to_oracle(self):
        return self.oracle_root / self.absolute_path[1:]

    def to_parsec(self):
        root, workspace, *tail = Path(self.absolute_path).parts

        # Do not allow to go outside the inital workspace
        assert str(self.parsec_root).endswith("-" + workspace)

        return self.parsec_root / Path(*tail)

    def __truediv__(self, path):
        return PathElement(
            os.path.join(self.absolute_path, path), self.parsec_root, self.oracle_root
        )


@pytest.mark.slow
@pytest.mark.mountpoint
def test_fuse_folder_operations(tmpdir, hypothesis_settings, mountpoint_service):

    tentative = 0

    class FuseFolderOperationsStateMachine(RuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        # Moving mountpoint
        NonRootFolder = Folders.filter(lambda x: not x.is_workspace())

        @initialize(target=Folders)
        def init(self):
            nonlocal tentative
            tentative += 1

            mountpoint_service.start()

            self.folder_oracle = Path(tmpdir / f"oracle-test-{tentative}")
            self.folder_oracle.mkdir()
            oracle_root = self.folder_oracle / "root"
            oracle_root.mkdir()
            self.folder_oracle.chmod(0o500)  # Root oracle can no longer be removed this way
            (oracle_root / mountpoint_service.default_workspace_name).mkdir()

            return PathElement(
                f"/{mountpoint_service.default_workspace_name}",
                mountpoint_service.get_default_workspace_mountpoint(),
                oracle_root,
            )

        def teardown(self):
            mountpoint_service.stop()

        @rule(target=Files, parent=Folders, name=st_entry_name)
        def touch(self, parent, name):
            path = parent / name

            expected_exc = None
            fallback_exc = None
            try:
                path.to_oracle().touch(exist_ok=False)
            except OSError as exc:
                expected_exc = exc
                # WinFSP raises `OSError(22, 'Invalid argument')` instead
                # of `FileNotFoundError(2, 'No such file or directory')`
                if os.name == "nt" and isinstance(exc, FileNotFoundError):
                    fallback_exc = OSError(22, "Invalid argument")

            with expect_raises(expected_exc, fallback_exc):
                path.to_parsec().touch(exist_ok=False)

            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        def mkdir(self, parent, name):
            path = parent / name

            expected_exc = None
            fallback_exc = None
            try:
                path.to_oracle().mkdir(exist_ok=False)
            except OSError as exc:
                expected_exc = exc
                # WinFSP raises `NotADirectoryError(20, 'The directory name is invalid')`
                # instead of `FileNotFoundError(2, 'The system cannot find the path specified')`
                if os.name == "nt" and isinstance(exc, FileNotFoundError):
                    fallback_exc = NotADirectoryError(20, "The directory name is invalid")

            with expect_raises(expected_exc, fallback_exc):
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

        @rule(path=NonRootFolder)
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

        @rule(target=Folders, src=NonRootFolder, dst_parent=Folders, dst_name=st_entry_name)
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
