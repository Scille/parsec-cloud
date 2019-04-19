# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import errno
from typing import Tuple
from collections import namedtuple
from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.core.types import (
    FsPath,
    Access,
    WorkspaceEntry,
    LocalDevice,
    LocalManifest,
    ManifestAccess,
    LocalFileManifest,
    LocalFolderManifest,
)
from parsec.core.local_storage import LocalStorage, LocalStorageMissingEntry
from parsec.core.fs.utils import (
    is_file_manifest,
    is_folder_manifest,
    is_folderish_manifest,
    is_workspace_manifest,
)
from parsec.core.fs.remote_loader import RemoteLoader


Entry = namedtuple("Entry", "access manifest")


def from_errno(errno, message=None, filename=None, filename2=None):
    if message is None:
        message = os.strerror(errno)
    return OSError(errno, message, filename, None, filename2)


class EntryTransactions:
    def __init__(
        self,
        device: LocalDevice,
        workspace_entry: WorkspaceEntry,
        local_storage: LocalStorage,
        remote_loader: RemoteLoader,
        event_bus: EventBus,
    ):
        self.local_author = device.device_id
        self.workspace_entry = workspace_entry
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus

    # Right management helper

    def _check_write_rights(self, path: FsPath):
        if not self.workspace_entry.write_right:
            raise from_errno(errno.EACCES, str(path))

    # Look-up helpers

    async def _get_manifest(self, access: Access) -> LocalManifest:
        try:
            return self.local_storage.get_manifest(access)
        except LocalStorageMissingEntry as exc:
            return await self.remote_loader.load_manifest(exc.access)

    async def _get_entry(self, path: FsPath) -> Tuple[Access, LocalManifest]:
        # Root access and manifest
        assert path.parts[0] == "/"
        access = self.workspace_entry.access
        manifest = await self._get_manifest(access)
        assert is_workspace_manifest(manifest)

        # Follow the path
        for name in path.parts[1:]:
            if is_file_manifest(manifest):
                raise from_errno(errno.ENOTDIR, filename=str(path))
            try:
                access = manifest.children[name]
            except (AttributeError, KeyError):
                raise from_errno(errno.ENOENT, filename=str(path))
            manifest = await self._get_manifest(access)

        # Return entry
        return Entry(access, manifest)

    # Locking helpers

    # This logic should move to the local storage along with
    # the remote loader. It would then be up to the local storage
    # to download the missing manifests. This should simplify the
    # code and help gather all the sensitive methods in the same
    # module.

    @asynccontextmanager
    async def _lock_entry(self, path: FsPath):
        # Load the entry
        access, _ = await self._get_entry(path)

        # Try to lock the corresponding access
        try:
            async with self.local_storage.lock_manifest(access) as manifest:
                yield Entry(access, manifest)

        # The entry has been deleted while we were waiting for the lock
        except LocalStorageMissingEntry:
            raise from_errno(errno.ENOENT, filename=str(path))

    @asynccontextmanager
    async def _lock_parent_entry(self, path):
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
                access = parent.manifest.children[path.name]
                try:
                    async with self.local_storage.lock_manifest(access) as manifest:
                        yield parent, Entry(access, manifest)
                        return

                # Child is not available
                except LocalStorageMissingEntry as exc:
                    assert exc.access == access

            # Release the lock and download the child manifest
            await self.remote_loader.load_manifest(access)

    # Reverse lookup logic

    async def _get_path(self, access: Access) -> FsPath:
        # TODO
        pass

    # Transactions

    async def entry_info(self, path: FsPath):

        # Fetch data
        access, manifest = await self._get_entry(path)

        # General stats
        stats = {
            "id": access.id,
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

    async def entry_rename(self, source: FsPath, destination: FsPath, overwrite: bool = True):
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
            source_access = parent.manifest.children[source.name]

            # Source and destination are the same
            if source.name == destination.name:
                return

            # Destination already exists
            if not overwrite and child is not None:
                raise from_errno(errno.EEXIST, filename=str(destination))

            # Overwrite logic
            if overwrite and child is not None:
                source_manifest = await self._get_manifest(source_access)

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
                {destination.name: source_access, source.name: None}
            )

            # Atomic change
            self.local_storage.set_dirty_manifest(parent.access, new_parent_manifest)

        # Send event
        self.event_bus.send("fs.entry.updated", id=parent.access.id)

    async def folder_delete(self, path: FsPath):
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
            self.local_storage.set_dirty_manifest(parent.access, new_parent_manifest)

            # Clean up
            self.local_storage.clear_manifest(child.access)

        # Send event
        self.event_bus.send("fs.entry.updated", id=parent.access.id)

    async def file_delete(self, path: FsPath):
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
            self.local_storage.set_dirty_manifest(parent.access, new_parent_manifest)

            # Clean up
            self.local_storage.remove_file_reference(child.access, child.manifest)

        # Send event
        self.event_bus.send("fs.entry.updated", id=parent.access.id)

    async def folder_create(self, path: FsPath):
        # Check write rights
        self._check_write_rights(path)

        # Lock parent and child
        async with self._lock_parent_entry(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise from_errno(errno.EEXIST, filename=str(path))

            # Create folder
            child_access = ManifestAccess()
            child_manifest = LocalFolderManifest(self.local_author)

            # New parent manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {path.name: child_access}
            )

            # ~ Atomic change
            self.local_storage.set_dirty_manifest(child_access, child_manifest)
            self.local_storage.set_dirty_manifest(parent.access, new_parent_manifest)

        # Send events
        self.event_bus.send("fs.entry.updated", id=parent.access.id)
        self.event_bus.send("fs.entry.updated", id=child_access.id)

    async def file_create(self, path: FsPath):
        # Check write rights
        self._check_write_rights(path)

        # Lock parent in write mode
        async with self._lock_parent_entry(path) as (parent, child):

            # Destination already exists
            if child is not None:
                raise from_errno(errno.EEXIST, filename=str(path))

            # Create file
            child_access = ManifestAccess()
            child_manifest = LocalFileManifest(self.local_author)

            # New parent manifest
            new_parent_manifest = parent.manifest.evolve_children_and_mark_updated(
                {path.name: child_access}
            )

            # ~ Atomic change
            self.local_storage.set_dirty_manifest(child_access, child_manifest)
            self.local_storage.set_dirty_manifest(parent.access, new_parent_manifest)
            fd = self.local_storage.create_cursor(child_access)

        # Send events
        self.event_bus.send("fs.entry.updated", id=parent.access.id)
        self.event_bus.send("fs.entry.updated", id=child_access.id)

        # Return file descriptor
        return fd

    async def file_open(self, path: FsPath, mode="rw"):
        # Check write rights
        if "w" in mode:
            self._check_write_rights(path)

        # Lock path in read mode
        async with self._lock_entry(path) as entry:

            # Not a file
            if not is_file_manifest(entry.manifest):
                raise from_errno(errno.EISDIR, str(path))

            # Return file descriptor
            return self.local_storage.create_cursor(entry.access)
