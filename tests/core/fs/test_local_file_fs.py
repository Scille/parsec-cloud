import os
import pytest
from pendulum import Pendulum
from hypothesis.stateful import RuleBasedStateMachine, initialize, rule, run_state_machine_as_test
from hypothesis import strategies as st

from parsec.core.fs.local_file_fs import FSInvalidFileDescriptor, FSBlocksLocalMiss
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss
from parsec.core.fs.utils import new_block_access, new_access, new_local_file_manifest

from tests.common import freeze_time


class File:
    def __init__(self, local_folder_fs, access):
        self.access = access
        self.local_folder_fs = local_folder_fs

    def ensure_manifest(self, **kwargs):
        manifest = self.local_folder_fs.get_manifest(self.access)
        for k, v in kwargs.items():
            assert manifest[k] == v


@pytest.fixture
def foo_txt(alice, local_folder_fs):
    access = new_access()
    with freeze_time("2000-01-02"):
        manifest = new_local_file_manifest(alice.id)
        manifest["is_placeholder"] = False
        manifest["need_sync"] = False
        manifest["base_version"] = 1
    local_folder_fs.set_manifest(access, manifest)
    return File(local_folder_fs, access)


def test_open_unknown_file(local_file_fs):
    dummy_access = new_access()
    with pytest.raises(FSManifestLocalMiss):
        local_file_fs.open(dummy_access)


def test_close_unknown_fd(local_file_fs):
    with pytest.raises(FSInvalidFileDescriptor):
        local_file_fs.close(42)


def test_operations_on_file(local_file_fs, foo_txt):
    fd = local_file_fs.open(foo_txt.access)
    assert isinstance(fd, int)

    local_file_fs.write(fd, b"hello ")
    local_file_fs.write(fd, b"world !")

    local_file_fs.seek(fd, 0)
    local_file_fs.write(fd, b"H")

    fd2 = local_file_fs.open(foo_txt.access)

    local_file_fs.seek(fd2, -1)
    local_file_fs.write(fd2, b"!!!")

    local_file_fs.seek(fd2, 0)
    data = local_file_fs.read(fd2, 1)
    assert data == b"H"

    with freeze_time("2000-01-03"):
        local_file_fs.close(fd2)

    local_file_fs.seek(fd, 6)
    data = local_file_fs.read(fd, 5)
    assert data == b"world"

    local_file_fs.seek(fd, 0)
    local_file_fs.close(fd)

    fd2 = local_file_fs.open(foo_txt.access)

    data = local_file_fs.read(fd2)
    assert data == b"Hello world !!!!"

    local_file_fs.close(fd2)

    foo_txt.ensure_manifest(
        size=16,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )


def test_flush_file(local_file_fs, foo_txt):
    fd = local_file_fs.open(foo_txt.access)

    local_file_fs.write(fd, b"hello ")
    local_file_fs.write(fd, b"world !")

    foo_txt.ensure_manifest(
        size=0,
        is_placeholder=False,
        need_sync=False,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 2),
    )

    with freeze_time("2000-01-03"):
        local_file_fs.flush(fd)

    foo_txt.ensure_manifest(
        size=13,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )

    local_file_fs.close(fd)

    foo_txt.ensure_manifest(
        size=13,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )


def test_block_not_loaded_entry(local_folder_fs, local_file_fs, foo_txt):
    foo_manifest = local_folder_fs.get_manifest(foo_txt.access)
    block1 = b"a" * 10
    block2 = b"b" * 5
    block1_access = new_block_access(block1, 0)
    block2_access = new_block_access(block2, 10)
    foo_manifest["blocks"].append(block1_access)
    foo_manifest["blocks"].append(block2_access)
    foo_manifest["size"] = 15
    local_folder_fs.set_manifest(foo_txt.access, foo_manifest)

    fd = local_file_fs.open(foo_txt.access)
    with pytest.raises(FSBlocksLocalMiss) as exc:
        local_file_fs.read(fd, 14)
    assert exc.value.accesses == [block1_access, block2_access]

    local_file_fs.set_block(block1_access, block1)
    local_file_fs.set_block(block2_access, block2)

    data = local_file_fs.read(fd, 14)
    assert data == block1 + block2[:4]


@pytest.mark.slow
def test_file_operations(
    tmpdir, hypothesis_settings, signal_ns, device_factory, local_file_fs_factory
):
    tentative = 0

    class FileOperationsStateMachine(RuleBasedStateMachine):
        @initialize()
        def init(self):
            nonlocal tentative
            tentative += 1

            self.device = device_factory()
            self.local_file_fs = local_file_fs_factory(self.device)
            self.access = new_access()
            manifest = new_local_file_manifest(self.device.id)
            self.local_file_fs.local_folder_fs.set_manifest(self.access, manifest)

            self.fd = self.local_file_fs.open(self.access)
            self.file_oracle_path = tmpdir / f"oracle-test-{tentative}.txt"
            self.file_oracle_fd = os.open(self.file_oracle_path, os.O_RDWR | os.O_CREAT)

        def teardown(self):
            os.close(self.file_oracle_fd)

        @rule(size=st.integers(min_value=0))
        def read(self, size):
            data = self.local_file_fs.read(self.fd, size)
            expected = os.read(self.file_oracle_fd, size)
            assert data == expected

        @rule(content=st.binary())
        def write(self, content):
            self.local_file_fs.write(self.fd, content)
            os.write(self.file_oracle_fd, content)

        @rule(length=st.integers(min_value=0))
        def seek(self, length):
            self.local_file_fs.seek(self.fd, length)
            os.lseek(self.file_oracle_fd, length, os.SEEK_SET)

        @rule(length=st.integers(min_value=0))
        def truncate(self, length):
            self.local_file_fs.truncate(self.fd, length)
            os.ftruncate(self.file_oracle_fd, length)

        @rule()
        def reopen(self):
            self.local_file_fs.close(self.fd)
            self.fd = self.local_file_fs.open(self.access)
            os.close(self.file_oracle_fd)
            self.file_oracle_fd = os.open(self.file_oracle_path, os.O_RDWR)

    run_state_machine_as_test(FileOperationsStateMachine, settings=hypothesis_settings)
