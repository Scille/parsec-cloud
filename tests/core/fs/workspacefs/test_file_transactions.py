# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest
from pendulum import Pendulum
from hypothesis_trio.stateful import (
    initialize,
    rule,
    run_state_machine_as_test,
    TrioAsyncioRuleBasedStateMachine,
)
from hypothesis import strategies as st

from parsec.core.types import EntryID, BlockAccess, LocalFileManifest
from parsec.core.fs.workspacefs.file_transactions import FSInvalidFileDescriptor
from parsec.core.fs.exceptions import FSRemoteBlockNotFound

from tests.common import freeze_time


class File:
    def __init__(self, local_storage, entry_id):
        self.entry_id = entry_id
        self.local_storage = local_storage

    def ensure_manifest(self, **kwargs):
        manifest = self.local_storage.get_manifest(self.entry_id)
        for k, v in kwargs.items():
            assert getattr(manifest, k) == v

    def get_manifest(self):
        return self.local_storage.get_manifest(self.entry_id)

    def set_manifest(self, manifest):
        self.local_storage.set_manifest(self.entry_id, manifest)

    def open(self):
        return self.local_storage.create_file_descriptor(self.entry_id)


@pytest.fixture
def foo_txt(alice, file_transactions):
    local_storage = file_transactions.local_storage
    with freeze_time("2000-01-02"):
        entry_id = EntryID()
        placeholder = LocalFileManifest.make_placeholder(alice.device_id, EntryID())
        manifest = placeholder.to_remote().evolve(version=1)
        local_storage.set_base_manifest(entry_id, manifest)
    return File(local_storage, entry_id)


@pytest.mark.trio
async def test_close_unknown_fd(file_transactions):
    with pytest.raises(FSInvalidFileDescriptor):
        await file_transactions.fd_close(42)


@pytest.mark.trio
async def test_operations_on_file(file_transactions, foo_txt):
    fd = foo_txt.open()
    assert isinstance(fd, int)

    with freeze_time("2000-01-03"):
        await file_transactions.fd_write(fd, b"hello ", 0)
        await file_transactions.fd_write(fd, b"world !", -1)
        await file_transactions.fd_write(fd, b"H", 0)
        await file_transactions.fd_write(fd, b"", 0)

        fd2 = foo_txt.open()

        await file_transactions.fd_write(fd2, b"!!!", -1)
        data = await file_transactions.fd_read(fd2, 1, 0)
        assert data == b"H"

        await file_transactions.fd_close(fd2)

    data = await file_transactions.fd_read(fd, 5, 6)
    assert data == b"world"

    await file_transactions.fd_close(fd)

    fd2 = foo_txt.open()

    data = await file_transactions.fd_read(fd2, -1, 0)
    assert data == b"Hello world !!!!"

    await file_transactions.fd_close(fd2)

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
    fd = foo_txt.open()

    foo_txt.ensure_manifest(
        size=0,
        is_placeholder=False,
        need_sync=False,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 2),
    )

    with freeze_time("2000-01-03"):
        await file_transactions.fd_write(fd, b"hello ", 0)
        await file_transactions.fd_write(fd, b"world !", -1)

    foo_txt.ensure_manifest(
        size=13,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )

    await file_transactions.fd_flush(fd)
    await file_transactions.fd_close(fd)

    foo_txt.ensure_manifest(
        size=13,
        is_placeholder=False,
        need_sync=True,
        base_version=1,
        created=Pendulum(2000, 1, 2),
        updated=Pendulum(2000, 1, 3),
    )


@pytest.mark.trio
async def test_block_not_loaded_entry(file_transactions, foo_txt):
    foo_manifest = foo_txt.get_manifest()
    block1 = b"a" * 10
    block2 = b"b" * 5
    block1_access = BlockAccess.from_block(block1, 0)
    block2_access = BlockAccess.from_block(block2, 10)
    foo_manifest = foo_manifest.evolve(
        blocks=[*foo_manifest.blocks, block1_access, block2_access], size=15
    )
    foo_txt.set_manifest(foo_manifest)

    fd = foo_txt.open()
    with pytest.raises(FSRemoteBlockNotFound):
        await file_transactions.fd_read(fd, 14, 0)

    file_transactions.local_storage.set_dirty_block(block1_access.id, block1)
    file_transactions.local_storage.set_dirty_block(block2_access.id, block2)

    data = await file_transactions.fd_read(fd, 14, 0)
    assert data == block1 + block2[:4]


@pytest.mark.trio
async def test_load_block_from_remote(file_transactions, foo_txt):
    # Prepare the backend
    workspace_id = file_transactions.remote_loader.workspace_id
    await file_transactions.remote_loader.create_realm(workspace_id)

    foo_manifest = foo_txt.get_manifest()
    block1 = b"a" * 10
    block2 = b"b" * 5
    block1_access = BlockAccess.from_block(block1, 0)
    block2_access = BlockAccess.from_block(block2, 10)
    foo_manifest = foo_manifest.evolve(
        blocks=[*foo_manifest.blocks, block1_access, block2_access], size=15
    )
    foo_txt.set_manifest(foo_manifest)

    fd = foo_txt.open()
    await file_transactions.remote_loader.upload_block(block1_access, block1)
    await file_transactions.remote_loader.upload_block(block2_access, block2)
    file_transactions.local_storage.clear_block(block1_access.id)
    file_transactions.local_storage.clear_block(block2_access.id)

    data = await file_transactions.fd_read(fd, 14, 0)
    assert data == block1 + block2[:4]


size = st.integers(min_value=0, max_value=4 * 1024 ** 2)  # Between 0 and 4MB


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows file style not compatible with oracle")
def test_file_operations(
    tmpdir,
    hypothesis_settings,
    reset_testbed,
    local_storage_factory,
    file_transactions_factory,
    alice,
    alice_backend_cmds,
):
    tentative = 0

    class FileOperationsStateMachine(TrioAsyncioRuleBasedStateMachine):
        @initialize()
        async def init(self):
            nonlocal tentative
            tentative += 1
            await reset_testbed()

            self.device = alice
            self.local_storage = local_storage_factory(self.device)

            self.file_transactions = file_transactions_factory(
                self.device, self.local_storage, alice_backend_cmds
            )

            self.entry_id = EntryID()
            manifest = LocalFileManifest.make_placeholder(
                self.device.device_id, parent_id=EntryID()
            )
            self.local_storage.set_manifest(self.entry_id, manifest)

            self.fd = self.local_storage.create_file_descriptor(self.entry_id)
            self.file_oracle_path = tmpdir / f"oracle-test-{tentative}.txt"
            self.file_oracle_fd = os.open(self.file_oracle_path, os.O_RDWR | os.O_CREAT)

        async def teardown(self):
            await self.file_transactions.fd_close(self.fd)
            os.close(self.file_oracle_fd)

        @rule(size=size, offset=size)
        async def read(self, size, offset):
            data = await self.file_transactions.fd_read(self.fd, size, offset)
            os.lseek(self.file_oracle_fd, offset, os.SEEK_SET)
            expected = os.read(self.file_oracle_fd, size)
            assert data == expected

        @rule(content=st.binary(), offset=size)
        async def write(self, content, offset):
            await self.file_transactions.fd_write(self.fd, content, offset)
            os.lseek(self.file_oracle_fd, offset, os.SEEK_SET)
            os.write(self.file_oracle_fd, content)

        @rule(length=size)
        async def resize(self, length):
            await self.file_transactions.fd_resize(self.fd, length)
            os.ftruncate(self.file_oracle_fd, length)

        @rule()
        async def reopen(self):
            await self.file_transactions.fd_close(self.fd)
            self.fd = self.local_storage.create_file_descriptor(self.entry_id)
            os.close(self.file_oracle_fd)
            self.file_oracle_fd = os.open(self.file_oracle_path, os.O_RDWR)

    run_state_machine_as_test(FileOperationsStateMachine, settings=hypothesis_settings)
