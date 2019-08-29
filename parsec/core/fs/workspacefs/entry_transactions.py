# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import errno
from typing import Tuple, Dict
from async_generator import asynccontextmanager

from pendulum import Pendulum

from parsec.api.protocol import DeviceID
from parsec.core.types import (
    EntryID,
    FsPath,
    WorkspaceRole,
    LocalManifest,
    LocalFileManifest,
    LocalFolderManifest,
    FileDescriptor,
)


from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.fs.exceptions import FSEntryNotFound, FSLocalMissError
from parsec.core.fs.utils import (
    is_file_manifest,
    is_folder_manifest,
    is_workspace_manifest,
    is_folderish_manifest,
)


WRITE_RIGHT_ROLES = (WorkspaceRole.OWNER, WorkspaceRole.MANAGER, WorkspaceRole.CONTRIBUTOR)


def from_errno(errno, message=None, filename=None, filename2=None):
    if message is None:
        message = os.strerror(errno)
    return OSError(errno, message, filename, None, filename2)


class EntryTransactions(FileTransactions):

    # Right management helper

    def _check_write_rights(self, path: FsPath):
        if self.get_workspace_entry().role not in WRITE_RIGHT_ROLES:
            raise from_errno(errno.EACCES, str(path))

    # Look-up helpers

    async def _get_manifest(self, entry_id: EntryID) -> LocalManifest:
        try:
            return self.local_storage.get_manifest(entry_id)
        except FSLocalMissError as exc:
            remote_manifest = await self.remote_loader.load_manifest(exc.id)
            return LocalManifest.from_remote(remote_manifest)

    @asynccontextmanager
    async def _load_and_lock_manifest(self, entry_id: EntryID):
        async with self.local_storage.lock_entry_id(entry_id):
            try:
                local_manifest = self.local_storage.get_manifest(entry_id)
            except FSLocalMissError as exc:
                remote_manifest = await self.remote_loader.load_manifest(exc.id)
                local_manifest = LocalManifest.from_remote(remote_manifest)
                self.local_storage.set_manifest(entry_id, local_manifest)
            yield local_manifest

    async def _load_manifest(self, entry_id: EntryID) -> LocalManifest:
        async with self._load_and_lock_manifest(entry_id) as manifest:
            return manifest

    @asynccontextmanager
    async def _lock_manifest_from_path(self, path: FsPath) -> LocalManifest:
        # Root entry_id and manifest
        assert path.parts[0] == "/"
        entry_id = self.workspace_id

        # Follow the path
        for name in path.parts[1:]:
            manifest = await self._load_manifest(entry_id)
            if is_file_manifest(manifest):
                raise from_errno(errno.ENOTDIR, filename=str(path))
            try:
                entry_id = manifest.children[name]
            except (AttributeError, KeyError):
                raise from_errno(errno.ENOENT, filename=str(path))

        # Lock entry
        async with self._load_and_lock_manifest(entry_id) as manifest:
            yield manifest

    async def _get_manifest_from_path(self, path: FsPath) -> LocalManifest:
        async with self._lock_manifest_from_path(path) as manifest:
            return manifest

    @asynccontextmanager
    async def _lock_parent_manifest_from_path(
        self, path: FsPath
    ) -> Tuple[LocalManifest, LocalManifest]:
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
            async with self._lock_manifest_from_path(path.parent) as parent:

                # Parent is not a directory
                if not is_folderish_manifest(parent):
                    raise from_errno(errno.ENOTDIR, filename=str(path.parent))

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

    # Reverse lookup logic

    async def get_entry_path(self, entry_id: EntryID) -> FsPath:

        # Get first manifest
        try:
            current_id = entry_id
            current_manifest = self.local_storage.get_manifest(current_id)
        except FSLocalMissError:
            raise FSEntryNotFound(entry_id)

        # Loop over parts
        parts = []
        while not is_workspace_manifest(current_manifest):

            # Get the manifest
            try:
                parent_manifest = self.local_storage.get_manifest(current_manifest.parent)
            except FSLocalMissError:
                raise FSEntryNotFound(entry_id)

            # Find the child name
            try:
                name = next(
                    name
                    for name, child_id in parent_manifest.children.items()
                    if child_id == current_id
                )
            except StopIteration:
                raise FSEntryNotFound(entry_id)
            else:
                parts.append(name)

            # Continue until root is found
            current_id = current_manifest.parent
            current_manifest = parent_manifest

        # Return the path
        return FsPath("/" + "/".join(reversed(parts)))

    # Transactions

    async def entry_info(self, path: FsPath) -> dict:

        # Fetch data
        manifest = await self._get_manifest_from_path(path)

        # General stats
        stats = {
            "id": manifest.id,
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

    async def entry_versions(self, path: FsPath) -> Dict[int, Tuple[Pendulum, DeviceID]]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
        """
        manifest = await self._get_manifest_from_path(path)
        return await self.remote_loader.list_versions(manifest.id)

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
            await self._get_manifest_from_path(source)

        # Fetch and lock
        async with self._lock_parent_manifest_from_path(destination) as (parent, child):

            # Source does not exist
            if source.name not in parent.children:
                raise from_errno(errno.ENOENT, filename=str(source))
            source_entry_id = parent.children[source.name]

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
                    if is_folder_manifest(child):
                        raise from_errno(errno.EISDIR, str(destination))

                # Overwrite a folder
                if is_folder_manifest(source_manifest):

                    # Destination is not a folder
                    if is_file_manifest(child):
                        raise from_errno(errno.ENOTDIR, str(destination))

                    # Destination is not empty
                    if child.children:
                        raise from_errno(errno.ENOTEMPTY, str(destination))

            # Create new manifest
            new_parent = parent.evolve_children_and_mark_updated(
                {destination.name: source_entry_id, source.name: None}
            )

            # Atomic change
            self.local_storage.set_manifest(parent.id, new_parent)

        # Send event
        self._send_event("fs.entry.updated", id=parent.id)

        # Return the entry id of the renamed entry
        return parent.children[source.name]

    async def folder_delete(self, path: FsPath) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Fetch and lock
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Entry doesn't exist
            if child is None:
                raise from_errno(errno.ENOENT, filename=str(path))

            # Not a directory
            if not is_folderish_manifest(child):
                raise from_errno(errno.ENOTDIR, str(path))

            # Directory not empty
            if child.children:
                raise from_errno(errno.ENOTEMPTY, str(path))

            # Create new manifest
            new_parent = parent.evolve_children_and_mark_updated({path.name: None})

            # Atomic change
            self.local_storage.set_manifest(parent.id, new_parent)

        # Send event
        self._send_event("fs.entry.updated", id=parent.id)

        # Return the entry id of the removed folder
        return child.id

    async def file_delete(self, path: FsPath) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Fetch and lock
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Entry doesn't exist
            if child is None:
                raise from_errno(errno.ENOENT, filename=str(path))

            # Not a file
            if not is_file_manifest(child):
                raise from_errno(errno.EISDIR, str(path))

            # Create new manifest
            new_parent = parent.evolve_children_and_mark_updated({path.name: None})

            # Atomic change
            self.local_storage.set_manifest(parent.id, new_parent)

        # Send event
        self._send_event("fs.entry.updated", id=parent.id)

        # Return the entry id of the deleted file
        return child.id

    async def folder_create(self, path: FsPath) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Lock parent and child
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise from_errno(errno.EEXIST, filename=str(path))

            # Create folder
            child = LocalFolderManifest.new_placeholder(parent=parent.id)

            # New parent manifest
            new_parent = parent.evolve_children_and_mark_updated({path.name: child.id})

            # ~ Atomic change
            self.local_storage.set_manifest(child.id, child, check_lock_status=False)
            self.local_storage.set_manifest(parent.id, new_parent)

        # Send events
        self._send_event("fs.entry.updated", id=parent.id)
        self._send_event("fs.entry.updated", id=child.id)

        # Return the entry id of the created folder
        return child.id

    async def file_create(self, path: FsPath, open=True) -> Tuple[EntryID, FileDescriptor]:
        # Check write rights
        self._check_write_rights(path)

        # Lock parent in write mode
        async with self._lock_parent_manifest_from_path(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise from_errno(errno.EEXIST, filename=str(path))

            # Create file
            child = LocalFileManifest.new_placeholder(parent=parent.id)

            # New parent manifest
            new_parent = parent.evolve_children_and_mark_updated({path.name: child.id})

            # ~ Atomic change
            self.local_storage.set_manifest(child.id, child, check_lock_status=False)
            self.local_storage.set_manifest(parent.id, new_parent)
            fd = self.local_storage.create_file_descriptor(child.id) if open else None

        # Send events
        self._send_event("fs.entry.updated", id=parent.id)
        self._send_event("fs.entry.updated", id=child.id)

        # Return the entry id of the created file and the file descriptor
        return child.id, fd

    async def file_open(self, path: FsPath, mode="rw") -> Tuple[EntryID, FileDescriptor]:
        # Check write rights
        if "w" in mode:
            self._check_write_rights(path)

        # Lock path in read mode
        async with self._lock_manifest_from_path(path) as manifest:

            # Not a file
            if not is_file_manifest(manifest):
                raise from_errno(errno.EISDIR, str(path))

            # Return the entry id of the open file and the file descriptor
            return manifest.id, self.local_storage.create_file_descriptor(manifest.id)

    async def file_resize(self, path: FsPath, length: int) -> EntryID:
        # Check write rights
        self._check_write_rights(path)

        # Lock manifest
        async with self._lock_manifest_from_path(path) as manifest:

            # Not a file
            if not is_file_manifest(manifest):
                raise from_errno(errno.EISDIR, str(path))

            # Perform resize
            self._manifest_resize(manifest, length)

            # Return entry id
            return manifest.id
