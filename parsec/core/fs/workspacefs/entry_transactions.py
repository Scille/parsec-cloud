# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import errno
from typing import Tuple, Callable
from collections import namedtuple
from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.core.types import (
    EntryID,
    FsPath,
    WorkspaceRole,
    LocalDevice,
    LocalManifest,
    LocalFileManifest,
    LocalFolderManifest,
    FileDescriptor,
)
from parsec.core.local_storage import LocalStorage, LocalStorageMissingError
from parsec.core.fs.utils import (
    is_file_manifest,
    is_folder_manifest,
    is_folderish_manifest,
    is_workspace_manifest,
)
from parsec.core.fs.remote_loader import RemoteLoader


Entry = namedtuple("Entry", "id manifest")
WRITE_RIGHT_ROLES = (WorkspaceRole.OWNER, WorkspaceRole.MANAGER, WorkspaceRole.CONTRIBUTOR)


def from_errno(errno, message=None, filename=None, filename2=None):
    if message is None:
        message = os.strerror(errno)
    return OSError(errno, message, filename, None, filename2)


class EntryTransactions:
    def __init__(
        self,
        workspace_id: EntryID,
        get_workspace_entry: Callable,
        device: LocalDevice,
        local_storage: LocalStorage,
        remote_loader: RemoteLoader,
        event_bus: EventBus,
    ):
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.local_author = device.device_id
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus

    # Right management helper

    def _check_write_rights(self, path: FsPath):
        if self.get_workspace_entry().role not in WRITE_RIGHT_ROLES:
            raise from_errno(errno.EACCES, str(path))

    # Event helper

    def _send_event(self, event, **kwargs):
        self.event_bus.send(event, workspace_id=self.workspace_id, **kwargs)

    # Look-up helpers

    async def _get_manifest(self, entry_id: EntryID) -> LocalManifest:
        try:
            return self.local_storage.get_manifest(entry_id)
        except LocalStorageMissingError as exc:
            return await self.remote_loader.load_manifest(exc.id)

    async def _get_entry(self, path: FsPath) -> Tuple[EntryID, LocalManifest]:
        # Root entry_id and manifest
        assert path.parts[0] == "/"
        entry_id = self.get_workspace_entry().id
        manifest = await self._get_manifest(entry_id)
        assert is_workspace_manifest(manifest)

        # Follow the path
        for name in path.parts[1:]:
            if is_file_manifest(manifest):
                raise from_errno(errno.ENOTDIR, filename=str(path))
            try:
                entry_id = manifest.children[name]
            except (AttributeError, KeyError):
                raise from_errno(errno.ENOENT, filename=str(path))
            manifest = await self._get_manifest(entry_id)

        # Return entry
        return Entry(entry_id, manifest)

    # Locking helpers

    # This logic should move to the local storage along with
    # the remote loader. It would then be up to the local storage
    # to download the missing manifests. This should simplify the
    # code and help gather all the sensitive methods in the same
    # module.

    @asynccontextmanager
    async def _lock_entry(self, path: FsPath):
        # Load the entry
        entry_id, _ = await self._get_entry(path)

        # Try to lock the corresponding entry
        try:
            async with self.local_storage.lock_manifest(entry_id) as manifest:
                yield Entry(entry_id, manifest)

        # The entry has been deleted while we were waiting for the lock
        except LocalStorageMissingError:
            raise from_errno(errno.ENOENT, filename=str(path))

    @asynccontextmanager
    async def _lock_parent_entry(self, path):
        # This is the most complicated locking scenario.
        # It requires locking the parent of the given entry and the entry itself
        # if it exists.

        # This is done in a two step process:
        # - 1. Lock the parent (it must exist). While the parent is locked, no
        #   children can be added, renamed or removed.
        # - 2. Lock the children if exists. It it doesn't, there is nothing to lock
        #   since the parent lock guarentees that it is not going to be added while
        #   using the context.

        # This double locking is only required for a single use case: the overwriting
        # of empty directory during a move. We have to make sure that no one adds
        # something to the directory while it is being overwritten.
        # If read/write locks were to be implemented, the parent would be write locked
        # and the child read locked. This means that despite locking two entries, only
        # a single entry is modified at a time.

        # Source is root
        if path.is_root():
            raise from_errno(errno.EACCES, filename=str(path))

        # Loop over attempts
        while True:

            # Lock parent first
            async with self._lock_entry(path.parent) as parent:

                # Parent is not a directory
                if not is_folderish_manifest(parent.manifest):
                    raise from_errno(errno.ENOTDIR, filename=str(path.parent))

                # Child doesn't exist
                if path.name not in parent.manifest.children:
                    yield parent, None
                    return

                # Child exists
                entry_id = parent.manifest.children[path.name]
                try:
                    async with self.local_storage.lock_manifest(entry_id) as manifest:
                        yield parent, Entry(entry_id, manifest)
                        return

                # Child is not available
                except LocalStorageMissingError as exc:
                    assert exc.id == entry_id

            # Release the lock and download the child manifest
            await self.remote_loader.load_manifest(entry_id)

    # Reverse lookup logic

    async def _get_path(self, id: EntryID) -> FsPath:
        # TODO
        pass

    # Transactions

    async def entry_info(self, path: FsPath) -> dict:

        # Fetch data
        entry_id, manifest = await self._get_entry(path)

        # General stats
        stats = {
            "id": entry_id,
            "created": manifest.created,
            "updated": manifest.updated,
            "base_version": manifest.base_version,
            "is_placeholder": manifest.is_placeholder,
            "need_sync": manifest.need_sync,
        }

        # File/folder specific stats
        if is_file_manifest(manifest):
            stats["type"] = "file"
            stats["size"] = manifest.size
        else:
            stats["type"] = "folder"
            stats["children"] = sorted(manifest.children.keys())

        return stats

    async def entry_rename(
        self, source: FsPath, destination: FsPath, overwrite: bool = True
    ) -> EntryID:
        # Check write rights
        self._check_write_rights(source)

        # Source is root
        if source.is_root():
            raise from_errno(errno.EACCES, filename=str(source))

        # Destination is root
        if destination.is_root():
            raise from_errno(errno.EACCES, filename=str(destination))

        # Cross-directory renaming is not supported
        if source.parent != destination.parent:
            raise from_errno(errno.EXDEV, filename=str(source), filename2=str(destination))

        # Pre-fetch the source if necessary
        if overwrite:
            await self._get_entry(source)

        # Fetch and lock
        async with self._lock_parent_entry(destination) as (parent, child):

            # Source does not exist
            if source.name not in parent.manifest.children:
                raise from_errno(errno.ENOENT, filename=str(source))
            source_entry_id = parent.manifest.children[source.name]

            # Source and destination are the same
            if source.name == destination.name:
                return

            # Destination already exists
            if not overwrite and child is not None:
                raise from_errno(errno.EEXIST, filename=str(destination))

            # Overwrite logic
            if overwrite and child is not None:
                source_manifest = await self._get_manifest(source_entry_id)

                # Overwrite a file
                if is_file_manifest(source_manifest):

                    # Destination is a folder
                    if is_folder_manifest(child.manifest):
                        raise from_errno(errno.EISDIR, str(destination))

                # Overwrite a folder
                if is_folder_manifest(source_manifest):

                    # Destination is not a folder
                    if is_file_manifest(child.manifest):
                        raise from_errno(errno.ENOTDIR, str(destination))

                    # Destination is not empty
                    if child.manifest.children:
                        raise from_errno(errno.ENOTEMPTY, str(destination))

            # Create new manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {destination.name: source_entry_id, source.name: None}
            )

            # Atomic change
            self.local_storage.set_manifest(parent.id, new_parent_manifest)

        # Send event
        self._send_event("fs.entry.updated", id=parent.id)

        # Return the entry id of the renamed entry
        return parent.manifest.children[source.name]

    async def folder_delete(self, path: FsPath) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Fetch and lock
        async with self._lock_parent_entry(path) as (parent, child):

            # Entry doesn't exist
            if child is None:
                raise from_errno(errno.ENOENT, filename=str(path))

            # Not a directory
            if not is_folderish_manifest(child.manifest):
                raise from_errno(errno.ENOTDIR, str(path))

            # Directory not empty
            if child.manifest.children:
                raise from_errno(errno.ENOTEMPTY, str(path))

            # Create new manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {path.name: None}
            )

            # Atomic change
            self.local_storage.set_manifest(parent.id, new_parent_manifest)

            # Clean up
            # TODO: actually we can't do that: this child my require synchronization
            self.local_storage.clear_manifest(child.id)

        # Send event
        self._send_event("fs.entry.updated", id=parent.id)

        # Return the entry id of the removed folder
        return child.id

    async def file_delete(self, path: FsPath) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Fetch and lock
        async with self._lock_parent_entry(path) as (parent, child):

            # Entry doesn't exist
            if child is None:
                raise from_errno(errno.ENOENT, filename=str(path))

            # Not a file
            if not is_file_manifest(child.manifest):
                raise from_errno(errno.EISDIR, str(path))

            # Create new manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {path.name: None}
            )

            # Atomic change
            self.local_storage.set_manifest(parent.id, new_parent_manifest)

            # Clean up
            self.local_storage.remove_file_reference(child.id, child.manifest)

        # Send event
        self._send_event("fs.entry.updated", id=parent.id)

        # Return the entry id of the deleted file
        return child.id

    async def folder_create(self, path: FsPath) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Lock parent and child
        async with self._lock_parent_entry(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise from_errno(errno.EEXIST, filename=str(path))

            # Create folder
            child_entry_id = EntryID()
            child_manifest = LocalFolderManifest(self.local_author)

            # New parent manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {path.name: child_entry_id}
            )

            # ~ Atomic change
            self.local_storage.set_manifest(child_entry_id, child_manifest)
            self.local_storage.set_manifest(parent.id, new_parent_manifest)

        # Send events
        self._send_event("fs.entry.updated", id=parent.id)
        self._send_event("fs.entry.updated", id=child_entry_id)

        # Return the entry id of the created folder
        return child_entry_id

    async def file_create(self, path: FsPath, open=True) -> Tuple[EntryID, FileDescriptor]:
        # Check write rights
        self._check_write_rights(path)

        # Lock parent in write mode
        async with self._lock_parent_entry(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise from_errno(errno.EEXIST, filename=str(path))

            # Create file
            child_entry_id = EntryID()
            child_manifest = LocalFileManifest(self.local_author)

            # New parent manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {path.name: child_entry_id}
            )

            # ~ Atomic change
            self.local_storage.set_manifest(child_entry_id, child_manifest)
            self.local_storage.set_manifest(parent.id, new_parent_manifest)
            fd = self.local_storage.create_cursor(child_entry_id) if open else None

        # Send events
        self._send_event("fs.entry.updated", id=parent.id)
        self._send_event("fs.entry.updated", id=child_entry_id)

        # Return the entry id of the created file and the file descriptor
        return child_entry_id, fd

    async def file_open(self, path: FsPath, mode="rw") -> Tuple[EntryID, FileDescriptor]:
        # Check write rights
        if "w" in mode:
            self._check_write_rights(path)

        # Lock path in read mode
        async with self._lock_entry(path) as entry:

            # Not a file
            if not is_file_manifest(entry.manifest):
                raise from_errno(errno.EISDIR, str(path))

            # Return the entry id of the open file and the file descriptor
            return entry.id, self.local_storage.create_cursor(entry.id)
