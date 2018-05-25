import attr
import pendulum
from uuid import uuid4
from copy import deepcopy
from async_generator import asynccontextmanager

from parsec.core.fs2.base import FSBase
from parsec.core.fs2.utils import (
    normalize_path,
    is_placeholder_access,
    is_folder_manifest,
    new_placeholder_access,
    copy_manifest,
    convert_to_remote_manifest,
    convert_to_local_manifest,
)
from parsec.core.fs2.opened_file import (
    OpenedFile,
    fast_forward_file_manifest,
    OpenedFilesManager,
)
from parsec.core.fs2.merge_folders import (
    merge_remote_folder_manifests,
    merge_local_folder_manifests,
)
from parsec.core.backend_storage import BackendConcurrencyError


def find_conflicting_name_for_child_entry(parent_manifest, original_name):
    try:
        base_name, extension = original_name.rsplit(".")
    except ValueError:
        extension = None

    now = pendulum.now()
    for tentative in count():
        tentative_str = "" if not tentative else " n°%s" % (tentative + 1)
        # TODO: Also add device id in the naming ?
        diverged_name = "%s (conflict%s %s)" % (
            base_name,
            tentative_str,
            now.to_datetime_string(),
        )
        if extension:
            diverged_name += ".%s" % extension

        if diverged_name not in parent_manifest["children"]:
            return diverged_name
            break


class SyncConcurrencyError(Exception):
    access = attr.ib()
    manifest = attr.ib()


class FSSyncMixin(FSBase):

    async def sync(self, path: str):
        # TODO: handle arbitrary path sync
        assert path == "/"
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        await self._sync(access, manifest)

    async def _sync(self, access, manifest):
        if is_folder_manifest(manifest):
            return await self._sync_folder(access, manifest)
        else:
            return await self._sync_file(access, manifest)

    async def _sync_folder(self, access, manifest, recursive=False):
        # Synchronizing a folder is divided into three steps:
        # - first synchronizing it children
        # - then sychronize itself
        # - finally merge the synchronized version with the current one (that
        #   may have been updated in the meantime)
        # The tricky part is given this function is asynchronous, a concurrent
        # update can occurs at anytime.

        # Synchronizing children
        if recursive:
            # Build a list of the children to synchronize. This children created
            # during the synchronization are ignored.
            if isinstance(recursive, (set, tuple, list)):
                to_sync = {
                    k: v for k, v in manifest["children"].items() if k in recursive
                }
            else:
                to_sync = manifest["children"].copy()

            # Synchronize the children.
            for child_name, child_access in to_sync.items():
                async with self._rename_entry_on_concurrency_error(access):
                    if is_placeholder_access(child_access):
                        await self._sync_placeholder(
                            access, child_access, child_manifest
                        )
                    else:
                        await self._sync(child_access, child_manifest)

        base_manifest = self._local_tree.retrieve_entry_by_access(access)
        # Now we can synchronize the folder if needed
        if not base_manifest["need_sync"]:
            # This folder hasn't been modified locally, just download
            # last version from the backend if any.
            if not access:
                target_remote_manifest = (
                    await self._manifests_manager.fetch_user_manifest_from_backend()
                )
            else:
                # Placeholder means we need synchro !
                assert access["type"] != "placeholder"
                target_remote_manifest = await self._manifests_manager.fetch_from_backend(
                    access["id"], access["rts"], access["key"]
                )
            if (
                not target_remote_manifest
                or target_remote_manifest["version"] == manifest["base_version"]
            ):
                return access
        else:
            access, target_remote_manifest = await self._sync_folder_actual_sync(
                access, base_manifest
            )

        # Finally merge with the current version of the manifest which may have
        # been modified in the meantime
        current_manifest = self._local_tree.retrieve_entry_by_access(access)
        target_manifest = convert_to_local_manifest(target_remote_manifest)
        final_manifest = merge_local_folder_manifests(
            base_manifest, current_manifest, target_manifest
        )
        self._local_tree.overwrite_entry(access, final_manifest)

        return access

    async def _sync_folder_actual_sync(self, access, manifest):
        # The trick here is to retreive the current version of the manifest
        # and remove it placeholders (those are the children created since
        # the start of our sync)
        to_sync_manifest = convert_to_remote_manifest(manifest)
        to_sync_manifest["children"] = {
            k: v
            for k, v in manifest["children"].items()
            if not is_placeholder_access(v)
        }
        # Upload the file manifest as new vlob version
        while True:
            try:
                if not access:
                    await self._manifests_manager.sync_user_manifest_with_backend(
                        to_sync_manifest
                    )
                elif is_placeholder_access(access):
                    id, rts, wts = await self._manifests_manager.sync_new_entry_with_backend(
                        access["key"], to_sync_manifest
                    )
                    access = {
                        "key": access["key"],
                        "id": id,
                        "rts": rts,
                        "wts": wts,
                        "type": "vlob",
                    }
                else:
                    await self._manifests_manager.sync_with_backend(
                        access["id"], access["wts"], access["key"], to_sync_manifest
                    )
                break

            except BackendConcurrencyError:
                # Do a 3-ways merge to fix the concurrency error, first we must
                # fetch the base version and the new one present in the backend
                # TODO: base should be available locally
                if not access:
                    base = await self._manifests_manager.fetch_user_manifest_from_backend(
                        version=to_sync_manifest["version"] - 1
                    )
                    target = (
                        await self._manifests_manager.fetch_user_manifest_from_backend()
                    )
                else:
                    base = await self._manifests_manager.fetch_from_backend(
                        access["id"],
                        access["rts"],
                        access["key"],
                        version=to_sync_manifest["version"] - 1,
                    )
                    target = await self._manifests_manager.fetch_from_backend(
                        access["id"], access["rts"], access["key"]
                    )
                # 3-ways merge between base, modified and target versions
                to_sync_manifest = merge_remote_folder_manifests(
                    base, to_sync_manifest, target
                )
                to_sync_manifest["version"] = target["version"] + 1

        return access, to_sync_manifest

    async def _sync_placeholder(self, parent_access, access, manifest):
        assert is_placeholder_access(access)
        new_access = await self._sync(access, manifest)

        parent_manifest = self._local_tree.retrieve_entry_by_access(parent_access)
        assert is_folder_manifest(parent_manifest)

        parent_manifest["children"][new_name] = new_access
        self._local_tree.update_entry(parent_access, parent_manifest)

    @asynccontextmanager
    async def _rename_entry_on_concurrency_error(parent_access):
        try:
            yield

        except SyncConcurrencyError as exc:

            # First, we save the conflicting version as a new entry
            new_access = new_placeholder_access()
            id, rts, wts = await self._manifests_manager.sync_new_entry_with_backend(
                new_access["key"], exc.manifest
            )
            new_access.update(id=id, rts=rts, wts=wts, type="vlob")

            # Next we have to save this new entry in the parent manifest.

            # Start by retreiving it current version.
            parent_manifest = self._local_tree.retrieve_entry_by_access(parent_access)
            assert is_folder_manifest(parent_manifest)

            # Retreive original name of the conflicting child.
            for candidate_name, candidate_access in parent_manifest["children"].items():
                if exc.access["id"] == candidate_access["id"]:
                    original_name = candidate_name
                    break
            else:
                # The original entry has been remove in the meantime, there
                # is no point in trying to keep the conflicting data anymore !
                return

            # Find a new cool name and save the parent manifest in local storage.
            new_name = find_conflicting_name_for_child_entry(
                parent_manifest, original_name
            )
            parent_manifest["children"][new_name] = new_access
            self._local_tree.update_entry(parent_access, parent_manifest)

            # Note here we no longer risk to lose our conflicting data.

            # It's possible the conflicting data have been updated since we
            # tried synchronizing them. To avoid another concurrency error,
            # we simply move those changes to our new entry.
            self._local_tree.move_local_manifest(exc.access, new_access)

    async def _sync_file(self, access, manifest):
        if not manifest["need_sync"]:
            # Placeholder means we need synchro !
            assert access["type"] != "placeholder"

            # This file hasn't been modified locally,
            # just download last version from the backend if any.
            target_manifest = await self._manifests_manager.fetch_from_backend(
                access["id"], access["rts"], access["key"]
            )
            if target_manifest["version"] != manifest["base_version"]:
                if manifest["need_sync"]:
                    raise SyncConcurrencyError(access, manifest)
                else:
                    target_manifest = convert_to_local_manifest(target_manifest)
                    self._manifests_cache[access["id"]] = target_manifest
                    self._flush_entry(access, target_manifest)

        else:
            # The file has been locally modified, time to sync it with the backend !
            # TODO
            # fd = self._open_file(access, manifest)
            # ...
            snapshot_manifest = convert_to_remote_manifest(manifest)
            try:
                if access["type"] == "placeholder":
                    id, rts, wts = await self._manifests_manager.sync_new_entry_with_backend(
                        access["key"], snapshot_manifest
                    )
                    access.update(id=id, rts=rts, wts=wts, type="vlob")
                else:
                    await self._manifests_manager.sync_with_backend(
                        access["id"], access["wts"], access["key"], snapshot_manifest
                    )
            except BackendConcurrencyError as exc:
                raise SyncConcurrencyError(access, manifest) from exc

            # Finally fast forward the manifest with it new base version
            # TODO
            manifest = fast_forward_file_manifest(snapshot_manifest, manifest)
            # Note the manifest is still kept in the cache with it
            # previous placeholder id
            self._manifests_cache[access["id"]] = manifest
            self._flush_entry(access, manifest)

        return access
