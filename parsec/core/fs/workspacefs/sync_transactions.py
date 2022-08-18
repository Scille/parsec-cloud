# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from itertools import count
from typing import Optional, Dict, AsyncIterator, Union, Pattern, Iterable

from parsec._parsec import DateTime

from parsec.api.protocol import DeviceID
from parsec.core.core_events import CoreEvent
from parsec.api.data import EntryNameTooLongError, AnyManifest as RemoteAnyManifest
from parsec.core.types import (
    Chunk,
    EntryID,
    EntryName,
    BaseLocalManifest,
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalFolderishManifests,
    RemoteFolderishManifests,
)

from parsec.core.fs.workspacefs.entry_transactions import EntryTransactions
from parsec.core.fs.exceptions import (
    FSFileConflictError,
    FSReshapingRequiredError,
    FSLocalMissError,
    FSIsADirectoryError,
    FSNotADirectoryError,
)
from parsec.core.types.manifest import LocalManifestTypeVar

__all__ = "SyncTransactions"

DEFAULT_BLOCK_SIZE = 512 * 1024  # 512Ko
FILENAME_CONFLICT_KEY = "FILENAME_CONFLICT"
FILE_CONTENT_CONFLICT_KEY = "FILE_CONTENT_CONFLICT"
TRANSLATIONS = {
    "en": {
        "FILENAME_CONFLICT": "Parsec - name conflict",
        "FILE_CONTENT_CONFLICT": "Parsec - content conflict",
    },
    "fr": {
        "FILENAME_CONFLICT": "Parsec - Conflit de nom",
        "FILE_CONTENT_CONFLICT": "Parsec - Conflit de contenu",
    },
}


# Helpers


def get_translated_message(preferred_language: str, key: str) -> str:
    try:
        translations = TRANSLATIONS[preferred_language]
    except KeyError:  # Default to english
        translations = TRANSLATIONS["en"]

    return translations.get(key, key)


def get_filename(manifest: LocalFolderishManifests, entry_id: EntryID) -> Optional[EntryName]:
    gen = (name for name, child_id in manifest.children.items() if child_id == entry_id)
    return next(gen, None)


def get_conflict_filename(
    filename: EntryName,
    filenames: Iterable[EntryName],
    suffix_key: str,
    preferred_language: str = "en",
) -> EntryName:
    counter = count(2)

    suffix = get_translated_message(preferred_language, suffix_key)
    new_filename = full_name(filename, suffix)
    filename_set = set(filenames)
    while new_filename in filename_set:
        new_filename = full_name(filename, f"{suffix} ({next(counter)})")
    return new_filename


def full_name(name: EntryName, suffix: str) -> EntryName:
    # Separate file name from the extensions (if any)
    name_parts = name.str.split(".")
    non_empty_indexes = (i for i, part in enumerate(name_parts) if part)
    first_non_empty_index = next(non_empty_indexes, len(name_parts) - 1)
    original_base_name = ".".join(name_parts[: first_non_empty_index + 1])
    original_extensions = name_parts[first_non_empty_index + 1 :]
    # Loop over attempts, in case the produced entry name is too long
    base_name = original_base_name
    extensions = original_extensions
    while True:
        # Convert to EntryName
        try:
            return EntryName(".".join([f"{base_name} ({suffix})", *extensions]))
        # Entry name too long
        except EntryNameTooLongError:
            # Simply strip 10 characters from the first name then try again
            if len(base_name) > 10:
                base_name = base_name[:-10]
            # Very rare case where the extensions are very long
            else:
                # This assert should only fail when the suffix is longer than 200 bytes,
                # which should not happen
                assert extensions
                # Pop the left most extension and restore the original base name
                extensions = extensions[1:]
                base_name = original_base_name


# Merging helpers


def merge_folder_children(
    base_children: Dict[EntryName, EntryID],
    local_children: Dict[EntryName, EntryID],
    remote_children: Dict[EntryName, EntryID],
    preferred_language: str = "en",
) -> Dict[EntryName, EntryID]:
    # Prepare lookups
    base_reversed = {entry_id: name for name, entry_id in base_children.items()}
    local_reversed = {entry_id: name for name, entry_id in local_children.items()}
    remote_reversed = {entry_id: name for name, entry_id in remote_children.items()}

    # All ids that might remain
    ids = set(local_reversed) | set(remote_reversed)

    # First map all ids to their rightful name
    solved_local_children: Dict[EntryName, EntryID] = {}
    solved_remote_children: Dict[EntryName, EntryID] = {}
    for id in ids:
        base_name = base_reversed.get(id)
        local_name = local_reversed.get(id)
        remote_name = remote_reversed.get(id)

        # Added locally
        if base_name is None and local_name is not None:
            solved_local_children[local_name] = local_children[local_name]

        # Added remotely
        elif base_name is None and remote_name is not None:
            solved_remote_children[remote_name] = remote_children[remote_name]

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
            solved_remote_children[remote_name] = remote_children[remote_name]

        # Name changed locally
        elif base_name == remote_name:
            solved_local_children[local_name] = local_children[local_name]

        # Name changed remotely
        elif base_name == local_name:
            solved_remote_children[remote_name] = remote_children[remote_name]

        # Name changed both locally and remotely
        else:
            # In this case, we simply decide that the remote is right since it means
            # another user managed to upload their change first. Tough luck for the
            # local device!
            solved_remote_children[remote_name] = remote_children[remote_name]

    # Merge mappings and fix conflicting names
    children = {}
    for name, entry_id in solved_remote_children.items():
        children[name] = entry_id
    for name, entry_id in solved_local_children.items():
        if name in children:
            name = get_conflict_filename(
                filename=name,
                filenames=children.keys(),
                suffix_key=FILENAME_CONFLICT_KEY,
                preferred_language=preferred_language,
            )
        children[name] = entry_id

    # Return
    return children


def merge_manifests(
    local_author: DeviceID,
    timestamp: DateTime,
    prevent_sync_pattern: Pattern[str],
    local_manifest: LocalManifestTypeVar,
    remote_manifest: Optional[RemoteAnyManifest] = None,
    force_apply_pattern: Optional[bool] = False,
    preferred_language: str = "en",
) -> LocalManifestTypeVar:
    # Start by re-applying pattern (idempotent)
    if force_apply_pattern and isinstance(
        local_manifest, (LocalFolderManifest, LocalWorkspaceManifest)
    ):
        local_manifest = local_manifest.apply_prevent_sync_pattern(prevent_sync_pattern, timestamp)

    # The remote hasn't changed
    if remote_manifest is None or remote_manifest.version <= local_manifest.base_version:
        return local_manifest
    assert remote_manifest is not None

    # Extract versions
    remote_version = remote_manifest.version
    local_version = local_manifest.base_version
    local_from_remote = BaseLocalManifest.from_remote_with_local_context(
        remote_manifest, prevent_sync_pattern, local_manifest, timestamp
    )

    # Only the remote has changed
    if not local_manifest.need_sync:
        return local_from_remote

    # Both the remote and the local have changed
    assert remote_version > local_version and local_manifest.need_sync

    # All the local changes have been successfully uploaded
    if local_manifest.match_remote(remote_manifest):  # type: ignore[arg-type]
        return local_from_remote

    # The remote changes are ours (our current local changes occurs while
    # we were uploading previous local changes that became the remote changes),
    # simply acknowledge them remote changes and keep our local changes
    #
    # However speculative manifest can lead to a funny behavior:
    # 1) alice has access to the workspace
    # 2) alice upload a new remote workspace manifest
    # 3) alice gets it local storage removed
    # So next time alice tries to access this workspace she will
    # creates a speculative workspace manifest.
    # This speculative manifest will eventually be synced against
    # the previous remote remote manifest which appears to be remote
    # changes we know about (given we are the author of it !).
    # If the speculative flag is not taken into account, we would
    # consider we have  willingly removed all entries from the remote,
    # hence uploading a new expunged remote manifest.
    #
    # Of course removing local storage is an unlikely situation, but:
    # - it cannot be ruled out and would produce rare&exotic behavior
    #   that would be considered as bug :/
    # - the fixtures and backend data binder system used in the tests
    #   makes it much more likely
    speculative = isinstance(local_manifest, LocalWorkspaceManifest) and local_manifest.speculative
    if remote_manifest.author == local_author and not speculative:
        return local_manifest.evolve(base=remote_manifest)  # type: ignore[arg-type]

    # The remote has been updated by some other device
    assert remote_manifest.author != local_author or speculative is True

    # Cannot solve a file conflict directly
    if isinstance(local_manifest, LocalFileManifest):
        raise FSFileConflictError(local_manifest, remote_manifest)

    assert isinstance(local_manifest, (LocalFolderManifest, LocalWorkspaceManifest))
    # Solve the folder conflict
    new_children = merge_folder_children(
        base_children=local_manifest.base.children,
        local_children=local_manifest.children,
        remote_children=local_from_remote.children,  # type: ignore[union-attr]
        preferred_language=preferred_language,
    )

    # Children merge can end up with nothing to sync.
    #
    # This is typically the case when we sync for the first time a workspace
    # shared with us that we didn't modify:
    # - the workspace manifest is a speculative placeholder (with arbitrary update&create dates)
    # - on sync the update date is different than in the remote, so a merge occurs
    # - given we didn't modify the workspace, the children merge is trivial
    # So without this check each each user we share the workspace with would
    # sync a new workspace manifest version with only it updated date changing :/
    #
    # Another case where this happen:
    # - we have local change on our workspace manifest for removing an entry
    # - we rely on a base workspace manifest in version N
    # - remote workspace manifest is in version N+1 and already integrate the removal
    #
    # /!\ Extra attention should be payed here if we want to add new fields
    # /!\ with they own sync logic, as this optimization may shadow them !

    if new_children == local_from_remote.children:  # type: ignore[union-attr]
        return local_from_remote
    else:
        return local_from_remote.evolve_and_mark_updated(children=new_children, timestamp=timestamp)


class SyncTransactions(EntryTransactions):

    # Public read-only helpers

    async def get_placeholder_children(
        self, remote_manifest: RemoteFolderishManifests
    ) -> AsyncIterator[EntryID]:
        # Check children placeholder
        for child_entry_id in remote_manifest.children.values():
            try:
                child_manifest = await self.local_storage.get_manifest(child_entry_id)
            except FSLocalMissError:
                continue
            if child_manifest.is_placeholder:
                yield child_entry_id

    async def get_minimal_remote_manifest(self, entry_id: EntryID) -> Optional[RemoteAnyManifest]:
        manifest = await self.local_storage.get_manifest(entry_id)
        if not manifest.is_placeholder:
            return None
        timestamp = self.device.timestamp()
        return manifest.base.evolve(author=self.local_author, timestamp=timestamp, version=1)

    # Atomic transactions

    async def apply_prevent_sync_pattern(
        self, entry_id: EntryID, prevent_sync_pattern: Pattern[str]
    ) -> None:
        # Fetch and lock
        async with self.local_storage.lock_manifest(entry_id) as local_manifest:

            # Not a folderish manifest
            if not isinstance(local_manifest, (LocalFolderManifest, LocalWorkspaceManifest)):
                return

            # Craft new local manifest
            timestamp = self.device.timestamp()
            new_local_manifest = local_manifest.apply_prevent_sync_pattern(
                prevent_sync_pattern, timestamp
            )

            # Set the new base manifest
            if new_local_manifest != local_manifest:  # type: ignore[operator]
                await self.local_storage.set_manifest(entry_id, new_local_manifest)

    async def synchronization_step(
        self,
        entry_id: EntryID,
        remote_manifest: Optional[RemoteAnyManifest] = None,
        final: bool = False,
    ) -> Optional[RemoteAnyManifest]:
        """Perform a synchronization step.

        This step is meant to be called several times until the right state is reached.
        It takes the current remote manifest as an argument and returns the new remote
        manifest to upload. When the manifest is successfully uploaded, this method has
        to be called once again with the new remote manifest as an argument. When there
        is no more changes to upload, this method returns None. The `final` argument can
        be set to true to indicate that the caller has no intention to upload a new
        manifest. This also causes the method to return None.
        """

        # Check for confinement
        confinement_point = await self._get_confinement_point(entry_id)
        if confinement_point is not None:
            # Send entry confined event for the sync monitor
            self._send_event(
                CoreEvent.FS_ENTRY_CONFINED, entry_id=entry_id, cause_id=confinement_point
            )
            # Do not perform the synchronization
            return None

        # Fetch and lock
        async with self.local_storage.lock_manifest(entry_id) as local_manifest:

            # Sync cannot be performed yet
            if (
                not final
                and isinstance(local_manifest, LocalFileManifest)
                and not local_manifest.is_reshaped()
            ):

                # Try a quick reshape (without downloading any block)
                missing = await self._manifest_reshape(local_manifest)

                # Downloading block is necessary for this reshape
                if missing:
                    raise FSReshapingRequiredError(entry_id)

                # The manifest should be reshaped by now
                local_manifest = await self.local_storage.get_manifest(entry_id)
                assert isinstance(local_manifest, LocalFileManifest)
                assert local_manifest.is_reshaped()

            # Merge manifests
            timestamp = self.device.timestamp()
            prevent_sync_pattern = self.local_storage.get_prevent_sync_pattern()
            force_apply_pattern = not self.local_storage.get_prevent_sync_pattern_fully_applied()
            new_local_manifest = merge_manifests(
                local_author=self.local_author,
                timestamp=timestamp,
                prevent_sync_pattern=prevent_sync_pattern,
                local_manifest=local_manifest,
                remote_manifest=remote_manifest,
                force_apply_pattern=force_apply_pattern,
                preferred_language=self.preferred_language,
            )

            # Extract authors
            base_author = local_manifest.base.author
            remote_author = base_author if remote_manifest is None else remote_manifest.author

            # Extract versions
            base_version = local_manifest.base_version
            new_base_version = new_local_manifest.base_version

            # Set the new base manifest
            if new_local_manifest != local_manifest:  # type: ignore[operator]
                await self.local_storage.set_manifest(entry_id, new_local_manifest)

            # Send downsynced event
            if base_version != new_base_version and remote_author != self.local_author:
                self._send_event(CoreEvent.FS_ENTRY_DOWNSYNCED, id=entry_id)

            # Send synced event
            if local_manifest.need_sync and not new_local_manifest.need_sync:
                self._send_event(CoreEvent.FS_ENTRY_SYNCED, id=entry_id)

            # Nothing new to upload
            if final or not new_local_manifest.need_sync:
                return None

            # Produce the new remote manifest to upload
            timestamp = self.device.timestamp()
            return new_local_manifest.to_remote(self.local_author, timestamp)

    async def file_reshape(self, entry_id: EntryID) -> None:

        # Loop over attempts
        while True:

            # Fetch and lock
            async with self.local_storage.lock_manifest(entry_id) as manifest:

                # Not a file manifest
                if not isinstance(manifest, LocalFileManifest):
                    raise FSIsADirectoryError(entry_id)

                # Normalize
                missing = await self._manifest_reshape(manifest)

            # Done
            if not missing:
                return

            # Load missing blocks
            await self.remote_loader.load_blocks(missing)

    async def file_conflict(
        self,
        entry_id: EntryID,
        local_manifest: Union[LocalFolderManifest, LocalFileManifest],
        remote_manifest: RemoteAnyManifest,
    ) -> None:
        # This is the only transaction that affects more than one manifests
        # That's because the local version of the file has to be registered in the
        # parent as a new child while the remote version has to be set as the actual
        # version. In practice, this should not be an issue.

        # Lock parent then child
        parent_id = local_manifest.parent
        async with self.local_storage.lock_manifest(parent_id) as parent_manifest:

            # Not a folderish manifest
            if not isinstance(parent_manifest, (LocalFolderManifest, LocalWorkspaceManifest)):
                raise FSNotADirectoryError(parent_id)

            async with self.local_storage.lock_manifest(entry_id) as current_manifest:

                # Not a file manifest
                if not isinstance(current_manifest, LocalFileManifest):
                    raise FSIsADirectoryError(entry_id)

                # Make sure the file still exists
                filename = get_filename(parent_manifest, entry_id)
                if filename is None:
                    return

                # Copy blocks
                new_blocks = []
                for chunks in current_manifest.blocks:
                    new_chunks = []
                    for chunk in chunks:
                        data = await self.local_storage.get_chunk(chunk.id)
                        new_chunk = Chunk.new(chunk.start, chunk.stop)
                        await self.local_storage.set_chunk(new_chunk.id, data)
                        if len(chunks) == 1:
                            new_chunk = new_chunk.evolve_as_block(data)
                        new_chunks.append(chunk)
                    new_blocks.append(tuple(new_chunks))

                # Prepare
                timestamp = self.device.timestamp()
                prevent_sync_pattern = self.local_storage.get_prevent_sync_pattern()
                new_name = get_conflict_filename(
                    filename=filename,
                    filenames=parent_manifest.children.keys(),
                    suffix_key=FILE_CONTENT_CONFLICT_KEY,
                    preferred_language=self.preferred_language,
                )
                new_manifest = LocalFileManifest.new_placeholder(
                    self.local_author, parent=parent_id, timestamp=timestamp
                ).evolve(size=current_manifest.size, blocks=tuple(new_blocks))
                timestamp = self.device.timestamp()
                new_parent_manifest = parent_manifest.evolve_children_and_mark_updated(
                    {new_name: new_manifest.id},
                    prevent_sync_pattern=prevent_sync_pattern,
                    timestamp=timestamp,
                )
                other_manifest = BaseLocalManifest.from_remote(
                    remote_manifest, prevent_sync_pattern=prevent_sync_pattern
                )

                # Set manifests
                await self.local_storage.set_manifest(
                    new_manifest.id, new_manifest, check_lock_status=False
                )
                await self.local_storage.set_manifest(parent_id, new_parent_manifest)
                await self.local_storage.set_manifest(entry_id, other_manifest)

                self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=new_manifest.id)
                self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=parent_id)
                self._send_event(
                    CoreEvent.FS_ENTRY_FILE_CONFLICT_RESOLVED,
                    id=entry_id,
                    backup_id=new_manifest.id,
                )
