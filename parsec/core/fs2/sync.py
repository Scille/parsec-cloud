import attr
import pendulum
from uuid import uuid4
from copy import deepcopy
from async_generator import asynccontextmanager
from itertools import count
from huepy import good, bad, run, info, que

from parsec.core.fs2.base import FSBase
from parsec.core.fs2.utils import (
    normalize_path,
    is_placeholder_access,
    is_folder_manifest,
    new_placeholder_access,
    new_block_access,
    copy_manifest,
    convert_to_remote_manifest,
    convert_to_local_manifest,
)
from parsec.core.fs2.opened_file import OpenedFile, fast_forward_file_manifest, OpenedFilesManager
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
        base_name = original_name

    now = pendulum.now()
    for tentative in count():
        tentative_str = "" if not tentative else " n°%s" % (tentative + 1)
        # TODO: Also add device id in the naming ?
        diverged_name = "%s (conflict%s %s)" % (base_name, tentative_str, now.to_datetime_string())
        if extension:
            diverged_name += ".%s" % extension

        if diverged_name not in parent_manifest["children"]:
            return diverged_name
            break


@attr.s
class SyncConcurrencyError(Exception):
    access = attr.ib()
    manifest = attr.ib()


class FSSyncMixin(FSBase):

    async def sync(self, path: str):
        # TODO: handle arbitrary path sync
        assert path == "/"
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        await self._sync(access, manifest, recursive=True)

    async def _sync(self, access, manifest, recursive=False):
        if is_folder_manifest(manifest):
            return await self._sync_folder(access, manifest, recursive=recursive)
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
                to_sync = {k: v for k, v in manifest["children"].items() if k in recursive}
            else:
                to_sync = manifest["children"]

            # Synchronize the children.
            for child_name, child_access in to_sync.items():
                async with self._rename_entry_on_concurrency_error(access):
                    try:
                        child_manifest = self._local_tree.retrieve_entry_by_access(child_access)
                    except KeyError:
                        # Entry not present in local, nothing to sync then
                        continue

                    if is_placeholder_access(child_access):
                        await self._sync_placeholder(
                            access, child_access, child_manifest, recursive=True
                        )
                    else:
                        await self._sync(child_access, child_manifest, recursive=True)

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
                assert not is_placeholder_access(access)
                target_remote_manifest = await self._manifests_manager.fetch_from_backend(
                    access["id"], access["rts"], access["key"]
                )
            if (
                not target_remote_manifest
                or target_remote_manifest["version"] == base_manifest["base_version"]
            ):
                return access
            final_access = access
        else:
            final_access, target_remote_manifest = await self._sync_folder_actual_sync(
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

        return final_access

    async def _sync_folder_actual_sync(self, access, manifest):
        # The trick here is to retreive the current version of the manifest
        # and remove it placeholders (those are the children created since
        # the start of our sync)
        to_sync_manifest = convert_to_remote_manifest(manifest)
        to_sync_manifest["children"] = {
            k: v for k, v in manifest["children"].items() if not is_placeholder_access(v)
        }
        # Upload the file manifest as new vlob version
        while True:
            try:
                if not access:
                    await self._manifests_manager.sync_user_manifest_with_backend(to_sync_manifest)
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
                print(bad("CONCURRENCY ERROR ! %s" % access))
                if not access:
                    base = await self._manifests_manager.fetch_user_manifest_from_backend(
                        version=to_sync_manifest["version"] - 1
                    )
                    target = await self._manifests_manager.fetch_user_manifest_from_backend()

                else:
                    # Placeholder don't have remote version, so no concurrency is possible
                    assert not is_placeholder_access(access)

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
                to_sync_manifest, sync_needed = merge_remote_folder_manifests(
                    base, to_sync_manifest, target
                )
                if not sync_needed:
                    print(info("SYNC NOT NEEDED %s %s" % (access, to_sync_manifest)))
                    # It maybe possible the changes that cause the concurrency
                    # error were the same than the one we wanted to make in the
                    # first place (e.g. when removing the same file)
                    break
                to_sync_manifest["version"] = target["version"] + 1

        return access, to_sync_manifest

    async def _sync_placeholder(self, parent_access, access, manifest, recursive):
        assert is_placeholder_access(access)
        new_access = await self._sync(access, manifest, recursive=recursive)

        parent_manifest = self._local_tree.retrieve_entry_by_access(parent_access)
        assert is_folder_manifest(parent_manifest)
        try:
            name = next(
                n for n, a in parent_manifest["children"].items() if a["id"] == access["id"]
            )
        except StopIteration:
            # Entry is no longer present in the parent manifest (must have
            # been removed in the meantime), just forget about it
            return
        parent_manifest["children"][name] = new_access

        self._local_tree.update_entry(parent_access, parent_manifest)
        self._local_tree.resolve_placeholder_access(access, new_access)

    @asynccontextmanager
    async def _rename_entry_on_concurrency_error(self, parent_access):
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
            new_name = find_conflicting_name_for_child_entry(parent_manifest, original_name)
            parent_manifest["children"][new_name] = new_access
            self._local_tree.update_entry(parent_access, parent_manifest)

            # Note here we no longer risk to lose our conflicting data.

            # It's possible the conflicting data have been updated since we
            # tried synchronizing them. To avoid another concurrency error,
            # we simply move those changes to our new entry.
            # TODO: improve the way we access local tree
            current_manifest = self._local_tree.retrieve_entry_by_access(exc.access)
            final_manifest = fast_forward_file_manifest(exc.manifest, current_manifest)
            self._local_tree.move_entry_modifications(exc.access, new_access)
            self._local_tree.overwrite_entry(new_access, final_manifest)

    async def _sync_file(self, access, manifest):
        if not manifest["need_sync"]:
            # Placeholder means we need synchro !
            assert not is_placeholder_access(access)

            # This file hasn't been modified locally,
            # just download last version from the backend if any.
            target_remote_manifest = await self._manifests_manager.fetch_from_backend(
                access["id"], access["rts"], access["key"]
            )

            current_manifest = self._local_tree.retrieve_entry_by_access(access)
            if target_remote_manifest["version"] != current_manifest["base_version"]:
                if current_manifest["need_sync"]:
                    raise SyncConcurrencyError(access, convert_to_remote_manifest(current_manifest))
            final_access = access

        else:
            # The file has been locally modified, time to sync it with the backend !
            final_access, target_remote_manifest = await self._sync_file_actual_sync(
                access, manifest
            )
            current_manifest = self._local_tree.retrieve_entry_by_access(access)

        # Finally fast forward the current version of the manifest which may
        # have been modified in the meantime
        final_manifest = fast_forward_file_manifest(target_remote_manifest, current_manifest)
        print(info("SYNC FILE DONE %s %s" % (final_access, final_manifest)))
        self._local_tree.overwrite_entry(access, final_manifest)

        return final_access

    async def _sync_file_actual_sync(self, access, manifest):
        fd = self._opened_files.open_file(access, manifest)
        marker = fd.create_marker()

        to_sync_manifest = convert_to_remote_manifest(manifest)
        blocks = []
        ucs = fd.get_sync_map()
        for cs in ucs.spaces:
            if not cs.need_sync:
                blocks += [bs.data for bs in cs.buffers]
                continue
            # Create a new block from existing data
            data = await self._build_data_from_contiguous_space(cs)
            block_access = new_block_access()
            block_access["id"] = await self._blocks_manager.sync_new_block_with_backend(
                block_access["key"], data
            )
            blocks.append(block_access)
        to_sync_manifest["block"] = blocks
        to_sync_manifest["size"] = fd.size

        try:
            if is_placeholder_access(access):
                print(info("placeholder sync %s %s" % (access, to_sync_manifest)))
                id, rts, wts = await self._manifests_manager.sync_new_entry_with_backend(
                    access["key"], to_sync_manifest
                )
                final_access = {
                    "key": access["key"],
                    "id": id,
                    "rts": rts,
                    "wts": wts,
                    "type": "vlob",
                }
            else:
                print(info("SYNC FILE %s %s" % (access, to_sync_manifest)))
                await self._manifests_manager.sync_with_backend(
                    access["id"], access["wts"], access["key"], to_sync_manifest
                )
                final_access = access

        except BackendConcurrencyError as exc:
            assert not is_placeholder_access(access)
            print(bad("CONCURRENCY ERROR ! %s" % access))
            raise SyncConcurrencyError(access, to_sync_manifest) from exc

        fd.drop_until_marker(marker)

        return final_access, to_sync_manifest
