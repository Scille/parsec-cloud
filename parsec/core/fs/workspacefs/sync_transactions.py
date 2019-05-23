# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional, List, Dict, Iterator


from parsec.event_bus import EventBus
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.local_storage import LocalStorage, LocalStorageMissingEntry
from parsec.core.types import AccessID, FolderManifest, Access, LocalFolderManifest, Manifest

from parsec.core.fs.utils import is_file_manifest

__all__ = "SyncTransactions"


# Helpers


def full_name(name: str, suffixes: List[str]):
    # No suffix
    if not suffixes:
        return name

    # No extension
    suffix_string = "".join(f" ({suffix})" for suffix in suffixes)
    if "." not in name[1:]:
        return name + suffix_string

    # Extension
    first_name, *ext = name.split(".")
    return ".".join([first_name + suffix_string, *ext])


def merge_folder_children(
    base_children: Dict[str, Access],
    local_children: Dict[str, Access],
    remote_children: Dict[str, Access],
    remote_device_name: str,
):
    # Prepare lookups
    base_reversed = {access.id: name for name, access in base_children.items()}
    local_reversed = {access.id: name for name, access in local_children.items()}
    remote_reversed = {access.id: name for name, access in remote_children.items()}

    # All ids that might remain
    ids = set(local_reversed) | set(remote_reversed)

    # First map all ids to their rightful name
    solved_local_children = {}
    solved_remote_children = {}
    for id in ids:
        base_name = base_reversed.get(id)
        local_name = local_reversed.get(id)
        remote_name = remote_reversed.get(id)

        # Added locally
        if base_name is None and local_name is not None:
            solved_local_children[local_name] = (local_children[local_name],)

        # Added remotely
        elif base_name is None and remote_name is not None:
            solved_remote_children[remote_name] = (remote_children[remote_name],)

        # Removed locally
        elif local_name is None:
            # Note that locally removed children might not be synchronized at this point
            pass

        # Removed remotely
        elif remote_name is None:
            # Note that we're blindly removing children just because the remote said so
            # This is OK as long as users have a way to recover their local changes
            pass

        # Preserved remotely and locally with the same naming
        elif local_name == remote_name:
            solved_local_children[local_name] = (local_children[local_name],)

        # Name changed locally
        elif base_name == remote_name:
            solved_local_children[local_name] = (local_children[local_name],)

        # Name changed remotely
        elif base_name == local_name:
            solved_remote_children[remote_name] = (remote_children[remote_name],)

        # Name changed both locally and remotely
        else:
            suffix = f"renamed by {remote_device_name}"
            solved_remote_children[remote_name] = remote_children[remote_name], suffix

    # Merge mappings and fix conflicting names
    children = {}
    for name, (access, *suffixes) in solved_remote_children.items():
        children[full_name(name, suffixes)] = access
    for name, (access, *suffixes) in solved_local_children.items():
        if name in children:
            suffixes = *suffixes, f"conflicting with {remote_device_name}"
        children[full_name(name, suffixes)] = access

    # Return
    return children


def merge_folder_manifests(
    local_manifest: LocalFolderManifest,
    base_manifest: Optional[FolderManifest] = None,
    remote_manifest: Optional[FolderManifest] = None,
):
    # Exctract versions
    local_version = local_manifest.base_version
    assert base_manifest is None or base_manifest.version == local_version
    remote_version = local_version if remote_manifest is None else remote_manifest.version
    base_manifest_children = {} if base_manifest is None else base_manifest.children

    # The remote hasn't changed
    if remote_version <= local_version:
        return local_manifest

    # Only the remote has changed
    if remote_version > local_version and not local_manifest.need_sync:
        return remote_manifest.to_local(local_manifest.author)

    # Both the remote and the local have changed
    assert remote_version > local_version and local_manifest.need_sync

    # All the local changes have been successfully uploaded
    if remote_manifest == local_manifest.to_remote().evolve(version=remote_version):
        return remote_manifest.to_local(local_manifest.author)

    # Some of the local changes have been updated by our device
    if remote_manifest.author == local_manifest.author:
        return local_manifest.evolve(is_placeholder=False, base_version=remote_version)

    # The remote has been updated by some other device
    new_children = merge_folder_children(
        base_manifest_children,
        local_manifest.children,
        remote_manifest.children,
        remote_manifest.author,
    )
    return local_manifest.evolve_and_mark_updated(
        is_placeholder=False, base_version=remote_version, children=new_children
    )


class SyncTransactions:
    def __init__(
        self,
        workspace_id: AccessID,
        local_storage: LocalStorage,
        remote_loader: RemoteLoader,
        event_bus: EventBus,
    ):
        self.workspace_id = workspace_id
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus

    # Event helper

    def _send_event(self, event, **kwargs):
        self.event_bus.send(event, workspace_id=self.workspace_id, **kwargs)

    # Public read-only helpers

    async def placeholder_children(self, remote_manifest: Manifest) -> Iterator[Access]:
        # Check children placeholder
        for child_access in remote_manifest.children.values():
            try:
                child_manifest = self.local_storage.get_manifest(child_access)
            except LocalStorageMissingEntry:
                continue
            if child_manifest.is_placeholder:
                yield child_access

    async def minimal_sync(self, access: Access) -> Optional[Manifest]:
        manifest = self.local_storage.get_manifest(access)
        if not manifest.is_placeholder:
            return None
        if is_file_manifest(manifest):
            new_manifest = manifest.evolve(updated=manifest.created, size=0, blocks=[])
        else:
            new_manifest = manifest.evolve(updated=manifest.created, children={})
        return new_manifest.to_remote().evolve(version=1)

    # Atomic transactions

    async def folder_sync(
        self, access: Access, remote_manifest: Optional[FolderManifest] = None
    ) -> Optional[FolderManifest]:

        # Fetch and lock
        async with self.local_storage.lock_manifest(access) as local_manifest:

            # Get base manifest
            if local_manifest.is_placeholder:
                base_manifest = None
            else:
                base_manifest = self.local_storage.get_base_manifest(access)

            # Merge manifests
            new_local_manifest = merge_folder_manifests(
                local_manifest, base_manifest, remote_manifest
            )

            # Set the new base manifest
            if remote_manifest is not None and remote_manifest != base_manifest:
                self.local_storage.set_base_manifest(access, remote_manifest)

            # Set the new local manifest
            if new_local_manifest.need_sync:
                self.local_storage.set_manifest(access, new_local_manifest)

            # Send synced event
            if local_manifest.need_sync and not new_local_manifest.need_sync:
                self._send_event("fs.entry.synced", id=access.id)

            # Nothing new to upload
            if not new_local_manifest.need_sync:
                return None

            # Produce the new remote manifest to upload
            new_remote_manifest = new_local_manifest.to_remote()
            return new_remote_manifest.evolve(version=new_remote_manifest.version + 1)
