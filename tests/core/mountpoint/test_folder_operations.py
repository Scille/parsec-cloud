# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import errno
from pathlib import Path
from string import ascii_lowercase

import attr
import pytest
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
    def __init__(self, *expected_excs):
        try:
            self.expected_exc, *self.fallback_excs = expected_excs
        except ValueError:
            self.expected_exc = None
            self.fallback_excs = ()

    def __enter__(self):
        __tracebackhide__ = True
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        __tracebackhide__ = True

        if not self.expected_exc:
            return False

        if not exc_type:
            if not self.fallback_excs:
                raise AssertionError(f"DID NOT RAISED {self.expected_exc!r}")

            else:
                raise AssertionError(
                    f"DID NOT RAISED {self.expected_exc!r}"
                    f" OR {' OR '.join(exc) for exc in self.fallback_excs}"
                )

        if self.fallback_excs:
            allowed = (type(self.expected_exc), *map(type, self.fallback_excs))
        else:
            allowed = type(self.expected_exc)

        if not isinstance(exc_value, allowed):
            if not self.fallback_excs:
                raise AssertionError(
                    f"RAISED {exc_value!r} BUT EXPECTED {self.expected_exc!r}"
                ) from exc_value

            else:
                raise AssertionError(
                    f"RAISED {exc_value!r} BUT EXPECTED {self.expected_exc!r}"
                    f" OR {' OR '.join(exc) for exc in self.fallback_excs}"
                ) from exc_value

        return True


def oracle_rename(src, dst):
    """The oracle must behave differently than `src.rename`, as the
    workspace file system does not support cross-directory renaming.
    """
    if src.parent != dst.parent:
        raise OSError(errno.EXDEV, os.strerror(errno.EXDEV))
    return src.rename(str(dst))


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
        return self.parsec_root / self.absolute_path[1:]

    def __truediv__(self, path):
        assert isinstance(path, str) and path[0] != "/"
        absolute_path = f"/{path}" if self.absolute_path == "/" else f"{self.absolute_path}/{path}"
        return PathElement(absolute_path, self.parsec_root, self.oracle_root)


def translate_exception_into_compatible_ones(exc, is_touch_operation=False):
    if os.name != "nt":
        return (exc,)

    fallback_exc = None

    if is_touch_operation:
        # WinFSP raises `OSError(22, 'Invalid argument')` instead
        # of `FileNotFoundError(2, 'No such file or directory')`
        if isinstance(exc, FileNotFoundError):
            fallback_exc = OSError(22, "Invalid argument")

    else:
        # WinFSP raises `NotADirectoryError(20, 'The directory name is invalid')`
        # instead of `FileNotFoundError(2, 'The system cannot find the path specified')`
        if isinstance(exc, FileNotFoundError):
            fallback_exc = NotADirectoryError(20, "The directory name is invalid")

    return (exc, fallback_exc) if fallback_exc else (exc,)


@pytest.mark.slow
@pytest.mark.mountpoint
def test_folder_operations(tmpdir, hypothesis_settings, mountpoint_service):

    tentative = 0

    class FolderOperationsStateMachine(RuleBasedStateMachine):
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
            oracle_root.chmod(0o500)  # Also protect workspace from deletion

            return PathElement(
                f"/{mountpoint_service.default_workspace_name}",
                mountpoint_service.base_mountpoint,
                oracle_root,
            )

        def teardown(self):
            mountpoint_service.stop()

        @rule(target=Files, parent=Folders, name=st_entry_name)
        def touch(self, parent, name):
            path = parent / name

            expected_excs = ()
            try:
                path.to_oracle().touch(exist_ok=False)
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(
                    exc, is_touch_operation=True
                )

            with expect_raises(*expected_excs):
                path.to_parsec().touch(exist_ok=False)

            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        def mkdir(self, parent, name):
            path = parent / name

            expected_excs = ()
            try:
                path.to_oracle().mkdir(exist_ok=False)
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(exc)

            with expect_raises(*expected_excs):
                path.to_parsec().mkdir(exist_ok=False)

            return path

        @rule(path=Files)
        def unlink(self, path):
            expected_excs = ()
            try:
                path.to_oracle().unlink()
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(exc)

            with expect_raises(*expected_excs):
                path.to_parsec().unlink()

        @rule(path=Files, length=st.integers(min_value=0, max_value=16))
        def resize(self, path, length):
            expected_excs = ()
            try:
                os.truncate(path.to_oracle(), length)
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(exc)

            with expect_raises(*expected_excs):
                os.truncate(path.to_parsec(), length)

        @rule(path=NonRootFolder)
        def rmdir(self, path):
            expected_excs = ()
            try:
                path.to_oracle().rmdir()
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(exc)

            with expect_raises(*expected_excs):
                path.to_parsec().rmdir()

        def _move(self, src, dst_parent, dst_name):
            dst = dst_parent / dst_name

            expected_excs = ()
            try:
                oracle_rename(src.to_oracle(), dst.to_oracle())
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(exc)

            with expect_raises(*expected_excs):
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
            expected_excs = ()
            try:
                expected_children = {x.name for x in path.to_oracle().iterdir()}
            except OSError as exc:
                expected_excs = translate_exception_into_compatible_ones(exc)

            with expect_raises(*expected_excs):
                children = {x.name for x in path.to_parsec().iterdir()}

            if not expected_excs:
                assert children == expected_children

    run_state_machine_as_test(FolderOperationsStateMachine, settings=hypothesis_settings)
