# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.types import FsPath
from parsec.core.fs.workspacefs.file_transactions import FSInvalidFileDescriptor
from parsec.core.persistent_storage import LocalStorageError


@pytest.mark.trio
async def test_close_unknown_fd(alice_workspace_t4):
    with pytest.raises(FSInvalidFileDescriptor):
        await alice_workspace_t4.file_transactions.fd_close(42)


@pytest.mark.trio
async def test_operations_on_file(alice_workspace_t4, alice_workspace_t5):
    _, fd4 = await alice_workspace_t4.entry_transactions.file_open(FsPath("/files/content"), "r")
    assert isinstance(fd4, int)
    file_transactions_t4 = alice_workspace_t4.file_transactions
    _, fd5 = await alice_workspace_t5.entry_transactions.file_open(FsPath("/files/content"), "r")
    assert isinstance(fd5, int)
    file_transactions_t5 = alice_workspace_t5.file_transactions

    data = await file_transactions_t4.fd_read(fd4, 1)
    assert data == b"a"
    data = await file_transactions_t4.fd_read(fd4, 3)
    assert data == b"bcd"
    data = await file_transactions_t4.fd_read(fd4, 100)
    assert data == b"e"

    await file_transactions_t4.fd_seek(fd4, 0)
    data = await file_transactions_t4.fd_read(fd4, 4)
    assert data == b"abcd"
    await file_transactions_t4.fd_seek(fd4, 1)
    data = await file_transactions_t4.fd_read(fd4, 1)
    assert data == b"b"

    with pytest.raises(LocalStorageError):  # if removed from local_storage, no write right error?..
        await alice_workspace_t4.file_transactions.fd_write(fd4, b"hello ")

    data = await file_transactions_t5.fd_read(fd5, 100)
    assert data == b"fghij"
    await file_transactions_t5.fd_seek(fd5, 1)
    data = await file_transactions_t5.fd_read(fd5, 1)
    assert data == b"g"

    data = await file_transactions_t4.fd_read(fd4, 1)
    assert data == b"c"

    await file_transactions_t5.fd_close(fd5)
    with pytest.raises(FSInvalidFileDescriptor):
        data = await file_transactions_t5.fd_read(fd5, 1)
    data = await file_transactions_t4.fd_read(fd4, 1)
    assert data == b"d"

    _, fd5 = await alice_workspace_t5.entry_transactions.file_open(FsPath("/files/content"), "r")
    data = await file_transactions_t5.fd_read(fd5, 3)
    assert data == b"fgh"


@pytest.mark.trio
async def test_flush_file(alice_workspace_t4):
    _, fd4 = await alice_workspace_t4.entry_transactions.file_open(FsPath("/files/content"), "r")
    assert isinstance(fd4, int)
    file_transactions_t4 = alice_workspace_t4.file_transactions

    await file_transactions_t4.fd_flush(fd4)
