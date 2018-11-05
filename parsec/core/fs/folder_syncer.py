from typing import Union, List, Optional, cast
from uuid import UUID

from parsec.core.fs.types import Access, Path, LocalFolderManifest, RemoteFolderManifest
from parsec.core.fs.utils import (
    is_user_manifest,
    is_folder_manifest,
    is_placeholder_manifest,
    local_to_remote_manifest,
    remote_to_local_manifest,
)
from parsec.core.fs.sync_base import SyncConcurrencyError, BaseSyncer
from parsec.core.fs.merge_folders import merge_local_folder_manifests, merge_remote_folder_manifests
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss


class FolderSyncerMixin(BaseSyncer):
    async def _sync_folder_sync_children(
        self,
        path: Path,
        access: Access,
        manifest: LocalFolderManifest,
        recursive: Union[bool, dict, list],
        notify_beacons: List[UUID],
    ) -> None:
        # Build a list of the children to synchronize. This children created
        # during the synchronization are ignored.
        if isinstance(recursive, (set, tuple, list)):
            to_sync = {k: v for k, v in manifest["children"].items() if k in recursive}
            determine_child_recursiveness = lambda x: True
        elif isinstance(recursive, dict):
            # Such overkill recursive system is needed when asking to
            determine_child_recursiveness = lambda x: cast(dict, recursive)[x]
            to_sync = {k: v for k, v in manifest["children"].items() if k in recursive.keys()}
        else:
            to_sync = manifest["children"]
            determine_child_recursiveness = lambda x: recursive

        # Synchronize the children.
        for child_name, child_access in to_sync.items():
            child_path = path / child_name
            child_recursive = determine_child_recursiveness(child_name)
            await self._sync_nolock(child_path, child_access, child_recursive, notify_beacons)

    async def _sync_folder_look_for_remote_changes(
        self, access: Access, manifest: LocalFolderManifest
    ) -> Optional[RemoteFolderManifest]:
        # Placeholder means we need synchro !
        assert not is_placeholder_manifest(manifest)
        # This folder hasn't been modified locally, just download
        # last version from the backend if any.
        target_remote_manifest = await self._backend_vlob_read(access)
        if (
            not target_remote_manifest
            or target_remote_manifest["version"] == manifest["base_version"]
        ):
            return None
        return target_remote_manifest

    async def _sync_folder_actual_sync(
        self, access: Access, manifest: LocalFolderManifest, notify_beacons: List[UUID]
    ) -> RemoteFolderManifest:
        to_sync_manifest = local_to_remote_manifest(manifest)
        to_sync_manifest["version"] += 1

        # Upload the folder manifest as new vlob version
        force_update = False
        while True:
            try:
                if is_placeholder_manifest(manifest) and not force_update:
                    await self._backend_vlob_create(access, to_sync_manifest, notify_beacons)
                else:
                    await self._backend_vlob_update(access, to_sync_manifest, notify_beacons)
                break

            except SyncConcurrencyError:
                if is_placeholder_manifest(manifest):
                    # By definition, placeholder shouldn't have remote version.
                    # However user manifest is a special case here given it
                    # access is shared between devices even if it is not yet
                    # synced.
                    # If such case occured, we just have to pretend we were
                    # trying to do an update and rely on the generic merge.
                    assert is_user_manifest(manifest)
                    base = None
                    force_update = True
                else:
                    base = await self._backend_vlob_read(access, to_sync_manifest["version"] - 1)

                # Do a 3-ways merge to fix the concurrency error, first we must
                # fetch the base version and the new one present in the backend
                # TODO: base should be available locally
                target = await self._backend_vlob_read(access)

                # 3-ways merge between base, modified and target versions
                to_sync_manifest, sync_needed = merge_remote_folder_manifests(
                    base, to_sync_manifest, target
                )
                if not sync_needed:
                    # It maybe possible the changes that cause the concurrency
                    # error were the same than the one we wanted to make in the
                    # first place (e.g. when removing the same file)
                    break
                to_sync_manifest["version"] = target["version"] + 1

        return to_sync_manifest

    async def _sync_folder_nolock(
        self,
        path: Path,
        access: Access,
        manifest: LocalFolderManifest,
        recursive: Union[bool, dict, list],
        notify_beacons: List[UUID],
    ) -> None:
        """
        Args:
            recursive: whether the folder's children must be synced before itself.
                Can be a boolean, list or dict to describe complex sync nesting
                useful when syncing an entry with placeholders as parent.
        """
        assert is_folder_manifest(manifest)

        # Synchronizing a folder is divided into three steps:
        # - first synchronizing it children
        # - then sychronize itself
        # - finally merge the synchronized version with the current one (that
        #   may have been updated in the meantime)

        # Synchronizing children
        if recursive:
            await self._sync_folder_sync_children(path, access, manifest, recursive, notify_beacons)

        # The trick here is to retreive the current version of the manifest
        # and remove it placeholders (those are the children created since
        # the start of our sync)
        base_manifest = self.local_folder_fs.get_manifest(access)
        assert is_folder_manifest(base_manifest)
        synced_children = {}
        for child_name, child_access in base_manifest["children"].items():
            try:
                child_manifest = self.local_folder_fs.get_manifest(child_access)
            except FSManifestLocalMiss:
                # Child not in local, cannot be a placeholder then !
                synced_children[child_name] = child_access
            else:
                if not is_placeholder_manifest(child_manifest):
                    synced_children[child_name] = child_access
        base_manifest["children"] = synced_children

        # Now we can synchronize the folder if needed
        if not base_manifest["need_sync"]:
            target_remote_manifest = await self._sync_folder_look_for_remote_changes(
                access, base_manifest
            )
            # Quick exit if nothing's new
            if not target_remote_manifest:
                return
        else:
            target_remote_manifest = await self._sync_folder_actual_sync(
                access, base_manifest, notify_beacons
            )
        assert is_folder_manifest(target_remote_manifest)

        # Finally merge with the current version of the manifest which may have
        # been modified in the meantime
        current_manifest = self.local_folder_fs.get_manifest(access)
        assert is_folder_manifest(current_manifest)

        target_manifest = remote_to_local_manifest(target_remote_manifest)
        final_manifest = merge_local_folder_manifests(
            base_manifest, current_manifest, target_manifest
        )
        self.local_folder_fs.set_manifest(access, final_manifest)
        self.event_bus.send("fs.entry.synced", path=str(path), id=access["id"])
