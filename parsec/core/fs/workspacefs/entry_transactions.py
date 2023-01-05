# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncIterator, Dict, List, NamedTuple, Tuple, cast

from parsec._parsec import CoreEvent
from parsec.api.data import BlockAccess
from parsec.core.fs.exceptions import (
    FSCrossDeviceError,
    FSDirectoryNotEmptyError,
    FSFileExistsError,
    FSFileNotFoundError,
    FSIsADirectoryError,
    FSLocalMissError,
    FSNoAccessError,
    FSNotADirectoryError,
    FSPermissionError,
    FSReadOnlyError,
)
from parsec.core.fs.path import FsPath
from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.types import (
    AnyLocalManifest,
    EntryID,
    FileDescriptor,
    LocalFileManifest,
    LocalFolderishManifests,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    WorkspaceRole,
    local_manifest_from_remote,
)

WRITE_RIGHT_ROLES = (WorkspaceRole.OWNER, WorkspaceRole.MANAGER, WorkspaceRole.CONTRIBUTOR)


class BlockInfo(NamedTuple):
    local_and_remote_blocks: List[BlockAccess | None]
    local_only_blocks: List[BlockAccess | None]
    remote_only_blocks: List[BlockAccess | None]
    file_size: int
    proper_blocks_size: int
    pending_chunks_size: int


class EntryTransactions(FileTransactions):

    # Right management helper

    def check_read_rights(self, path: FsPath) -> None:
        if self.get_workspace_entry().role is None:
            raise FSNoAccessError(filename=path)

    def check_write_rights(self, path: FsPath) -> None:
        self.check_read_rights(path)
        if self.get_workspace_entry().role not in WRITE_RIGHT_ROLES:
            raise FSReadOnlyError(filename=path)

    # Look-up helpers

    async def _get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        try:
            return await self.local_storage.get_manifest(entry_id)
        except FSLocalMissError as exc:
            remote_manifest = await self.remote_loader.load_manifest(cast(EntryID, exc.id))
            return local_manifest_from_remote(
                remote_manifest, prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern()
            )

    @asynccontextmanager
    async def _load_and_lock_manifest(self, entry_id: EntryID) -> AsyncIterator[AnyLocalManifest]:
        async with self.local_storage.lock_entry_id(entry_id):
            try:
                local_manifest = await self.local_storage.get_manifest(entry_id)
            except FSLocalMissError as exc:
                remote_manifest = await self.remote_loader.load_manifest(cast(EntryID, exc.id))
                local_manifest = local_manifest_from_remote(
                    remote_manifest,
                    prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern(),
                )
                await self.local_storage.set_manifest(entry_id, local_manifest)
            yield local_manifest

    async def _load_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        async with self._load_and_lock_manifest(entry_id) as manifest:
            return manifest

    async def _entry_id_from_path(self, path: FsPath) -> Tuple[EntryID, EntryID | None]:
        """Returns a tuple (entry_id, confinement_point).

        The confinement point corresponds to the entry id of the folderish manifest
        that contains a child with a confined name in the corresponding path.

        If the entry is not confined, the confinement point is `None`.
        """
        # Root entry_id and manifest
        entry_id = self.workspace_id
        confinement_point = None

        # Follow the path
        for name in path.parts:
            manifest = await self._load_manifest(entry_id)
            if not isinstance(manifest, (LocalFolderManifest, LocalWorkspaceManifest)):
                raise FSNotADirectoryError(filename=path)
            try:
                entry_id = manifest.children[name]
            except (AttributeError, KeyError):
                raise FSFileNotFoundError(filename=path)
            if entry_id in manifest.local_confinement_points:
                confinement_point = manifest.id

        # Return both entry_id and confined status
        return entry_id, confinement_point

    @asynccontextmanager
    async def _lock_manifest_from_path(self, path: FsPath) -> AsyncIterator[AnyLocalManifest]:
        entry_id, _ = await self._entry_id_from_path(path)
        async with self._load_and_lock_manifest(entry_id) as manifest:
            yield manifest

    async def _get_manifest_from_path(
        self, path: FsPath
    ) -> Tuple[AnyLocalManifest, EntryID | None]:
        """Returns a tuple (manifest, confinement_point).

        The confinement point corresponds to the entry id of the folderish manifest
        that contains a child with a confined name in the corresponding path.

        If the entry is not confined, the confinement point is `None`.
        """
        entry_id, confined = await self._entry_id_from_path(path)
        manifest = await self._load_manifest(entry_id)
        return manifest, confined

    @asynccontextmanager
    async def _lock_parent_manifest_from_path(
        self, path: FsPath
    ) -> AsyncIterator[Tuple[LocalFolderishManifests, AnyLocalManifest | None]]:
        # This is the most complicated locking scenario.
        # It requires locking the parent of the given entry and the entry itself
        # if it exists.

        # This is done in a two step process:
        # - 1. Lock the parent (it must exist). While the parent is locked, no
        #   children can be added, renamed or removed.
        # - 2. Lock the children if exists. It it doesn't, there is nothing to lock
        #   since the parent lock guarantees that it is not going to be added while
        #   using the context.

        # This double locking is only required for a single use case: the overwriting
        # of empty directory during a move. We have to make sure that no one adds
        # something to the directory while it is being overwritten.
        # If read/write locks were to be implemented, the parent would be write locked
        # and the child read locked. This means that despite locking two entries, only
        # a single entry is modified at a time.

        # Source is root
        if path.is_root():
            raise FSPermissionError(filename=str(path))

        # Loop over attempts
        while True:

            # Lock parent first
            async with self._lock_manifest_from_path(path.parent) as parent:

                # Parent is not a directory
                if not isinstance(parent, (LocalFolderManifest, LocalWorkspaceManifest)):
                    raise FSNotADirectoryError(filename=path.parent)

                # Child doesn't exist
                if path.name not in parent.children:
                    yield parent, None
                    return

                # Child exists
                entry_id = parent.children[path.name]
                try:
                    async with self.local_storage.lock_manifest(entry_id) as manifest:
                        yield parent, manifest
                        return

                # Child is not available
                except FSLocalMissError as exc:
                    assert exc.id == entry_id

            # Release the lock and download the child manifest
            await self._load_manifest(entry_id)

    # Transactions
    async def entry_get_blocks_by_type(self, path: FsPath, limit: int) -> BlockInfo:

        manifest, confinement_point = await self._get_manifest_from_path(path)
        manifest: LocalFileManifest
        block_dict = {}
        total_size = 0
        for manifest_blocks in manifest.blocks:
            for chunk in manifest_blocks:
                if limit < (total_size + chunk.raw_size):
                    break
                if chunk.access:
                    block_dict[chunk.id] = chunk
                    total_size += chunk.raw_size

        block_ids = set(block_dict)

        file_size = manifest.size
        proper_blocks_size = sum(
            chunk.raw_size for chunks in manifest.blocks for chunk in chunks if chunk.is_block()
        )
        pending_chunks_size = sum(
            chunk.raw_size for chunks in manifest.blocks for chunk in chunks if not chunk.is_block()
        )

        # To avoid concurrency problems block storage is called first
        local_and_remote_block_ids = set(
            await self.local_storage.get_local_block_ids(list(block_ids))
        )
        local_only_block_ids = set(await self.local_storage.get_local_chunk_ids(list(block_ids)))
        remote_only_block_ids = block_ids - local_and_remote_block_ids - local_only_block_ids

        local_only_blocks = [block_dict[block_id].access for block_id in local_only_block_ids]
        remote_only_blocks = [block_dict[block_id].access for block_id in remote_only_block_ids]
        local_and_remote_blocks = [
            block_dict[block_id].access for block_id in local_and_remote_block_ids
        ]

        return BlockInfo(
            local_and_remote_blocks,
            local_only_blocks,
            remote_only_blocks,
            file_size,
            proper_blocks_size,
            pending_chunks_size,
        )

    async def entry_info(self, path: FsPath) -> Dict[str, object]:
        # Check read rights
        self.check_read_rights(path)

        # Fetch data
        manifest, confinement_point = await self._get_manifest_from_path(path)
        stats = manifest.to_stats()
        stats["confinement_point"] = confinement_point
        return stats

    async def entry_rename(
        self, source: FsPath, destination: FsPath, overwrite: bool = True
    ) -> EntryID | None:
        # Check write rights
        self.check_write_rights(source)

        # Source is root
        if source.is_root():
            raise FSPermissionError(filename=source)

        # Destination is root
        if destination.is_root():
            raise FSPermissionError(filename=destination)

        # Cross-directory renaming is not supported
        if source.parent != destination.parent:
            raise FSCrossDeviceError(filename=source, filename2=destination)

        # Pre-fetch the source if necessary
        if overwrite:
            await self._get_manifest_from_path(source)

        # Fetch and lock
        async with self._lock_parent_manifest_from_path(destination) as (parent, child):

            # Source does not exist
            if source.name not in parent.children:
                raise FSFileNotFoundError(filename=source)
            source_entry_id = parent.children[source.name]

            # Source and destination are the same
            if source.name == destination.name:
                return None

            # Destination already exists
            if not overwrite and child is not None:
                raise FSFileExistsError(filename=destination)

            # Overwrite logic
            if overwrite and child is not None:
                source_manifest = await self._get_manifest(source_entry_id)

                # Overwrite a file
                if isinstance(source_manifest, LocalFileManifest):

                    # Destination is a folder
                    if isinstance(child, LocalFolderManifest):
                        raise FSIsADirectoryError(filename=destination)

                # Overwrite a folder
                if isinstance(source_manifest, LocalFolderManifest):

                    # Destination is not a folder
                    if not isinstance(child, LocalFolderManifest):
                        raise FSNotADirectoryError(filename=destination)

                    # Destination is not empty
                    if child.children:
                        raise FSDirectoryNotEmptyError(filename=destination)

            # Create new manifest
            new_parent = parent.evolve_children_and_mark_updated(
                {destination.name: source_entry_id, source.name: None},
                prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern(),
                timestamp=self.device.timestamp(),
            )

            # Atomic change
            await self.local_storage.set_manifest(parent.id, new_parent)

        # Send event
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=parent.id)

        # Return the entry id of the renamed entry
        return parent.children[source.name]

    async def folder_delete(self, path: FsPath) -> EntryID:
        # Check write rights
        self.check_write_rights(path)

        # Fetch and lock
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Entry doesn't exist
            if child is None:
                raise FSFileNotFoundError(filename=path)

            # Not a directory
            if not isinstance(child, (LocalFolderManifest, LocalWorkspaceManifest)):
                raise FSNotADirectoryError(filename=path)

            # Directory not empty
            if child.children:
                raise FSDirectoryNotEmptyError(filename=path)

            # Create new manifest
            new_parent = parent.evolve_children_and_mark_updated(
                {path.name: None},
                prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern(),
                timestamp=self.device.timestamp(),
            )

            # Atomic change
            await self.local_storage.set_manifest(parent.id, new_parent)

        # Send event
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=parent.id)

        # Return the entry id of the removed folder
        return child.id

    async def file_delete(self, path: FsPath) -> EntryID:
        # Check write rights
        self.check_write_rights(path)

        # Fetch and lock
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Entry doesn't exist
            if child is None:
                raise FSFileNotFoundError(filename=path)

            # Not a file
            if not isinstance(child, LocalFileManifest):
                raise FSIsADirectoryError(filename=path)

            # Create new manifest
            new_parent = parent.evolve_children_and_mark_updated(
                {path.name: None},
                prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern(),
                timestamp=self.device.timestamp(),
            )

            # Atomic change
            await self.local_storage.set_manifest(parent.id, new_parent)

        # Send event
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=parent.id)

        # Return the entry id of the deleted file
        return child.id

    async def folder_create(self, path: FsPath) -> EntryID:
        # Check write rights
        self.check_write_rights(path)

        # Lock parent and child
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise FSFileExistsError(filename=path)

            # Create folder
            timestamp = self.device.timestamp()
            child = LocalFolderManifest.new_placeholder(
                self.local_author, parent=parent.id, timestamp=timestamp
            )

            # New parent manifest
            new_parent = parent.evolve_children_and_mark_updated(
                {path.name: child.id},
                prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern(),
                timestamp=self.device.timestamp(),
            )

            # ~ Atomic change
            await self.local_storage.set_manifest(child.id, child, check_lock_status=False)
            await self.local_storage.set_manifest(parent.id, new_parent)

        # Send events
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=parent.id)
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=child.id)

        # Return the entry id of the created folder
        return child.id

    async def file_create(
        self, path: FsPath, open: bool = True
    ) -> Tuple[EntryID, FileDescriptor | None]:
        # Check write rights
        self.check_write_rights(path)

        # Lock parent in write mode
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise FSFileExistsError(filename=path)

            # Create file
            timestamp = self.device.timestamp()
            child = LocalFileManifest.new_placeholder(
                self.local_author, parent=parent.id, timestamp=timestamp
            )

            # New parent manifest
            new_parent = parent.evolve_children_and_mark_updated(
                {path.name: child.id},
                prevent_sync_pattern=self.local_storage.get_prevent_sync_pattern(),
                timestamp=self.device.timestamp(),
            )

            # ~ Atomic change
            await self.local_storage.set_manifest(child.id, child, check_lock_status=False)
            await self.local_storage.set_manifest(parent.id, new_parent)
            fd = self.local_storage.create_file_descriptor(child) if open else None

        # Send events
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=parent.id)
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=child.id)

        # Return the entry id of the created file and the file descriptor
        return child.id, fd

    async def file_open(self, path: FsPath, write_mode: bool) -> Tuple[EntryID, FileDescriptor]:
        # Check read and write rights
        if write_mode:
            self.check_write_rights(path)
        else:
            self.check_read_rights(path)

        # Lock path in read mode
        async with self._lock_manifest_from_path(path) as manifest:

            # Not a file
            if not isinstance(manifest, LocalFileManifest):
                raise FSIsADirectoryError(filename=path)

            # Return the entry id of the open file and the file descriptor
            return manifest.id, self.local_storage.create_file_descriptor(manifest)

    async def file_resize(self, path: FsPath, length: int) -> EntryID:
        # Check write rights
        self.check_write_rights(path)

        # Lock manifest
        async with self._lock_manifest_from_path(path) as manifest:

            # Not a file
            if not isinstance(manifest, LocalFileManifest):
                raise FSIsADirectoryError(filename=path)

            # Perform resize
            await self._manifest_resize(manifest, length)
            self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=manifest.id)
            # Return entry id
            return manifest.id
