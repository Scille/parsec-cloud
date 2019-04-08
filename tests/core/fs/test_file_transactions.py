# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest
from pendulum import Pendulum
from hypothesis_trio.stateful import (
    initialize,
    rule,
    run_state_machine_as_test,
    TrioRuleBasedStateMachine,
)
from hypothesis import strategies as st

from parsec.core.types import ManifestAccess, BlockAccess, LocalFileManifest
from parsec.core.fs.file_transactions import FSInvalidFileDescriptor
from parsec.core.backend_connection.exceptions import BackendCmdsNotFound

from tests.common import freeze_time


class File:
    def __init__(self, local_folder_fs, access):
        self.access = access
        self.local_folder_fs = local_folder_fs

    def ensure_manifest(self, **kwargs):
        manifest = self.local_folder_fs.get_manifest(self.access)
        for k, v in kwargs.items():
            assert getattr(manifest, k) == v


@pytest.fixture
def foo_txt(alice, local_folder_fs):
    access = ManifestAccess()
    with freeze_time("2000-01-02"):
        manifest = LocalFileManifest(
            author=alice.device_id, is_placeholder=False, need_sync=False, base_version=1
        )
    local_folder_fs.set_clean_manifest(access, manifest)
    return File(local_folder_fs, access)


@pytest.mark.trio
async def test_open_unknown_file(file_transactions):
    dummy_access = ManifestAccess()
    fd = file_transactions.open(dummy_access)
    assert fd == 1


@pytest.mark.trio
async def test_close_unknown_fd(file_transactions):
    with pytest.raises(FSInvalidFileDescriptor):
        await file_transactions.close(42)


@pytest.mark.trio
async def test_operations_on_file(file_transactions, foo_txt):
    fd = file_transactions.open(foo_txt.access)
    assert isinstance(fd, int)

    with freeze_time("2000-01-03"):
        await file_transactions.write(fd, b"hello ")
        await file_transactions.write(fd, b"world !")

        await file_transactions.seek(fd, 0)
        await file_transactions.write(fd, b"H")

        fd2 = file_transactions.open(foo_txt.access)

        await file_transactions.seek(fd2, -1)
        await file_transactions.write(fd2, b"!!!")

        await file_transactions.seek(fd2, 0)
        data = await file_transactions.read(fd2, 1)
        assert data == b"H"

        await file_transactions.close(fd2)

    await file_transactions.seek(fd, 6)
    data = await file_transactions.read(fd, 5)
    assert data == b"world"

    await file_transactions.seek(fd, 0)
    await file_transactions.close(fd)

    fd2 = file_transactions.open(foo_txt.access)

    data = await file_transactions.read(fd2)
    assert data == b"Hello world !!!!"

    await file_transactions.close(fd2)

    foo_txt.ensure_manifest(
        size=16,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )


@pytest.mark.trio
async def test_flush_file(file_transactions, foo_txt):
    fd = file_transactions.open(foo_txt.access)

    foo_txt.ensure_manifest(
        size=0,
        is_placeholder=False,
        need_sync=False,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 2),
    )

    with freeze_time("2000-01-03"):
        await file_transactions.write(fd, b"hello ")
        await file_transactions.write(fd, b"world !")

    foo_txt.ensure_manifest(
        size=13,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )

    await file_transactions.flush(fd)
    await file_transactions.close(fd)

    foo_txt.ensure_manifest(
        size=13,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )


@pytest.mark.trio
async def test_block_not_loaded_entry(local_folder_fs, file_transactions, foo_txt):
    foo_manifest = local_folder_fs.get_manifest(foo_txt.access)
    block1 = b"a" * 10
    block2 = b"b" * 5
    block1_access = BlockAccess.from_block(block1, 0)
    block2_access = BlockAccess.from_block(block2, 10)
    foo_manifest = foo_manifest.evolve(
        blocks=[*foo_manifest.blocks, block1_access, block2_access], size=15
    )
    local_folder_fs.set_clean_manifest(foo_txt.access, foo_manifest)

    fd = file_transactions.open(foo_txt.access)
    with pytest.raises(BackendCmdsNotFound):
        await file_transactions.read(fd, 14)

    file_transactions.local_storage.set_dirty_block(block1_access, block1)
    file_transactions.local_storage.set_dirty_block(block2_access, block2)

    data = await file_transactions.read(fd, 14)
    assert data == block1 + block2[:4]


size = st.integers(min_value=0, max_value=4 * 1024 ** 2)  # Between 0 and 4MB


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows file style not compatible with oracle")
def test_file_operations(
    tmpdir,
    hypothesis_settings,
    local_storage_factory,
    file_transactions_factory,
    alice,
    alice_backend_cmds,
):
    tentative = 0

    class FileOperationsStateMachine(TrioRuleBasedStateMachine):
        @initialize()
        async def init(self):
            nonlocal tentative
            tentative += 1

            self.device = alice
            self.local_storage = local_storage_factory(self.device)

            self.file_transactions = file_transactions_factory(
                self.device, self.local_storage, alice_backend_cmds
            )
            self.access = ManifestAccess()
            manifest = LocalFileManifest(self.device.device_id, need_sync=True)
            self.local_storage.set_dirty_manifest(self.access, manifest)

            self.fd = self.file_transactions.open(self.access)
            self.file_oracle_path = tmpdir / f"oracle-test-{tentative}.txt"
            self.file_oracle_fd = os.open(self.file_oracle_path, os.O_RDWR | os.O_CREAT)

        async def teardown(self):
            await self.file_transactions.close(self.fd)
            os.close(self.file_oracle_fd)

        @rule(size=size)
        async def read(self, size):
            data = await self.file_transactions.read(self.fd, size)
            expected = os.read(self.file_oracle_fd, size)
            assert data == expected

        @rule(content=st.binary())
        async def write(self, content):
            await self.file_transactions.write(self.fd, content)
            os.write(self.file_oracle_fd, content)

        @rule(length=size)
        async def seek(self, length):
            await self.file_transactions.seek(self.fd, length)
            os.lseek(self.file_oracle_fd, length, os.SEEK_SET)

        @rule(length=size)
        async def truncate(self, length):
            await self.file_transactions.truncate(self.fd, length)
            os.ftruncate(self.file_oracle_fd, length)

        @rule()
        async def reopen(self):
            await self.file_transactions.close(self.fd)
            self.fd = self.file_transactions.open(self.access)
            os.close(self.file_oracle_fd)
            self.file_oracle_fd = os.open(self.file_oracle_path, os.O_RDWR)

    run_state_machine_as_test(FileOperationsStateMachine, settings=hypothesis_settings)
