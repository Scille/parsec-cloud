# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
import errno
from typing import Iterator
import pendulum

from parsec.core.types import FsPath, LocalFileManifest, FileCursor

# Needed if we want to completly bypass local_storage for blocks :  BlockAccess

from parsec.core.fs.workspacefs.entry_transactions import from_errno
from parsec.core.fs.workspacefs.file_transactions import normalize_offset, merge_buffers
from parsec.core.fs.workspacefs.workspacefs import AnyPath
import parsec.core.fs.workspacefs as workspacefs

from parsec.core.fs.utils import is_file_manifest

from parsec.core.local_storage import LocalStorageMissingError


class WorkspaceFSTimestamped(workspacefs.WorkspaceFS):
    def _throw_permission_error(*e, **ke):
        raise from_errno(errno.EACCES, "Not available for timestamped workspaces.")

    rename = _throw_permission_error
    mkdir = _throw_permission_error
    rmdir = _throw_permission_error
    touch = _throw_permission_error
    unlink = _throw_permission_error
    truncate = _throw_permission_error
    write_bytes = _throw_permission_error
    move = _throw_permission_error
    copytree = _throw_permission_error
    copyfile = _throw_permission_error
    rmtree = _throw_permission_error
    minimal_sync = _throw_permission_error
    sync_by_id = _throw_permission_error
    sync = _throw_permission_error

    def __init__(self, workspacefs: workspacefs.WorkspaceFS, timestamp: pendulum.Pendulum):
        self.workspace_id = workspacefs.workspace_id
        self.get_workspace_entry = workspacefs.get_workspace_entry
        self.device = workspacefs.device
        self.local_storage = workspacefs.local_storage
        self.backend_cmds = workspacefs.backend_cmds
        self.event_bus = workspacefs.event_bus
        self.remote_device_manager = workspacefs.remote_device_manager

        self.timestamp = timestamp

        self.remote_loader = workspacefs.remote_loader
        self.file_transactions = workspacefs.file_transactions
        self.entry_transactions = workspacefs.entry_transactions

    # Information

    async def path_info(self, path: AnyPath) -> dict:
        return await self.entry_transactions.entry_info(FsPath(path), self.timestamp)

    async def path_id(self, path: AnyPath) -> UUID:
        info = await self.entry_transactions.entry_info(FsPath(path), self.timestamp)
        return info["id"]

    # Pathlib-like interface

    async def exists(self, path: AnyPath) -> bool:
        path = FsPath(path)
        try:
            if await self.entry_transactions.entry_info(path, self.timestamp):
                return True
        except FileNotFoundError:
            return False
        return False

    async def is_dir(self, path: AnyPath) -> bool:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(path, self.timestamp)
        return info["type"] == "folder"

    async def is_file(self, path: AnyPath) -> bool:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(FsPath(path), self.timestamp)
        return info["type"] == "file"

    async def iterdir(self, path: AnyPath) -> Iterator[FsPath]:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(path, self.timestamp)
        if "children" not in info:
            raise NotADirectoryError(str(path))
        for child in info["children"]:
            yield path / child

    # Naive read_bytes not implementing caching, and private helpers for bypassing local_storage

    # # Needed only if we want to bypass local_storage for blocks
    # async def _load_block_no_storage(self, access: BlockAccess) -> bytes:
    #     """
    #     Raises:
    #         BackendConnectionError
    #         CryptoError
    #     """
    #     ciphered_block = await self.backend_cmds.block_read(access.id)
    #     block = decrypt_raw_with_secret_key(access.key, ciphered_block)
    #     assert sha256(block).hexdigest() == access.digest, access
    #     return block

    def _attempt_read(self, manifest: LocalFileManifest, start: int, end: int):
        missing = []
        data = bytearray(end - start)
        merged = merge_buffers(manifest, start, end)
        for cs in merged.spaces:
            for bs in cs.buffers:
                block_access = bs.buffer.access
                try:
                    buff = self.local_storage.get_block(block_access.id)
                except LocalStorageMissingError:
                    missing.append(block_access)
                    continue

                data[bs.start - cs.start : bs.end - cs.start] = buff[
                    bs.buffer_slice_start : bs.buffer_slice_end
                ]

        return data, missing

    async def read_bytes(self, path: AnyPath, size: int = -1, offset: int = 0) -> bytes:
        path = FsPath(path)

        # Loop over attemps
        missing = []

        entry_id = await self.path_id(path)
        remote_manifest = await self.remote_loader.load_manifest(entry_id, timestamp=self.timestamp)
        manifest = remote_manifest.to_local(self.local_storage.device_id)
        if not is_file_manifest(manifest):
            raise from_errno(errno.EISDIR, str(path))
        cursor = FileCursor(entry_id)

        while True:
            # for block_access in missing:
            #    blocsk[block_access.id] = await _load_block_no_storage(block_access)

            # No-op
            offset = normalize_offset(offset, cursor, manifest)
            if offset is not None and offset > manifest.size:
                return b""

            # Prepare
            start = offset
            size = manifest.size if size < 0 else size
            end = min(offset + size, manifest.size)
            data, missing = self._attempt_read(manifest, start, end)

            # Retry
            if missing:
                continue

            # Atomic change
            cursor.offset = end

            return data
