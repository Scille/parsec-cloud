import pendulum
from async_generator import asynccontextmanager
from itertools import count
from huepy import good, bad, run, info, que

from parsec.core.fs.base import FSBase
from parsec.core.fs.utils import (
    normalize_path,
    is_placeholder_access,
    is_folder_manifest,
    is_file_manifest,
    new_placeholder_access,
    new_block_access,
    convert_to_remote_manifest,
    convert_to_local_manifest,
)
from parsec.core.fs.merge_folders import merge_remote_folder_manifests, merge_local_folder_manifests
from parsec.core.fs.sync_file import FileSyncConcurrencyError
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


class RetrySync(Exception):
    pass


class FSSyncFolderMixin(FSBase):
    async def _sync_folder(self, access, manifest, recursive=False):
        """
        Args:
            recursive: whether the folder's children must be synced before itself.
                Can be a boolean, list or dict to describe complex sync nesting
                useful when syncing an entry with placeholders as parent.
        """
        access_id = access["id"] if access else "<root>"

        # Acquire sync lock
        while True:
            if not self._sync_locks.is_locked(access_id):
                break
            print(bad("retry sync manifest %s" % access_id))
            # This folder is in the middle of a sync, wait for it to
            # end and reload the manifest to avoid concurrency issues
            await self._sync_locks.wait_not_locked(access_id)
            try:
                manifest = self._local_tree.retrieve_entry_by_access(access)
                assert is_folder_manifest(manifest)
            except KeyError:
                # Entry has been removed in the meantime, nothing to do then
                return

        with self._sync_locks.lock(access_id):
            # Synchronizing a folder is divided into three steps:
            # - first synchronizing it children
            # - then sychronize itself
            # - finally merge the synchronized version with the current one (that
            #   may have been updated in the meantime)
            # The tricky part is given this function is asynchronous, a concurrent
            # update can occurs at anytime.

            # Synchronizing children
            await self._sync_folder_sync_children(access, manifest, recursive)

            try:
                # The trick here is to retreive the current version of the manifest
                # and remove it placeholders (those are the children created since
                # the start of our sync)
                base_manifest = self._local_tree.retrieve_entry_by_access(access)
                base_manifest["children"] = {
                    k: v
                    for k, v in base_manifest["children"].items()
                    if not is_placeholder_access(v)
                }
            except KeyError:
                # Either the entry is no longer available locally or it has
                # been removed, either way there is nothing to sync anymore
                print(info("sync cancelled by concurrent modification %s" % access["id"]))
                return
            assert is_folder_manifest(base_manifest)

            # Now we can synchronize the folder if needed
            if not base_manifest["need_sync"]:
                target_remote_manifest = await self._sync_folder_look_for_remote_changes(
                    access, base_manifest
                )
                # Quick exit if nothing's new
                if not target_remote_manifest:
                    print(info("nothing new in remote %s" % access_id))
                    return access
                final_access = access
            else:
                final_access, target_remote_manifest = await self._sync_folder_actual_sync(
                    access, base_manifest
                )

            # Finally merge with the current version of the manifest which may have
            # been modified in the meantime
            try:
                current_manifest = self._local_tree.retrieve_entry_by_access(access)
            except KeyError:
                # Either the entry is no longer available locally or it has
                # been removed, either way there is nothing to sync anymore
                print(info("sync cancelled by concurrent modification %s" % access["id"]))
                return
            assert is_folder_manifest(current_manifest)
            assert is_folder_manifest(target_remote_manifest)

            target_manifest = convert_to_local_manifest(target_remote_manifest)
            final_manifest = merge_local_folder_manifests(
                base_manifest, current_manifest, target_manifest
            )
            self._local_tree.overwrite_entry(access, final_manifest)

        print(good("sync done %s %s" % (access_id, final_manifest)))

        return final_access

    async def _sync_folder_sync_children(self, access, manifest, recursive):
        if not recursive:
            return

        # Build a list of the children to synchronize. This children created
        # during the synchronization are ignored.
        # Recursive
        if isinstance(recursive, (set, tuple, list)):
            to_sync = {k: v for k, v in manifest["children"].items() if k in recursive}
            determine_child_recursiveness = lambda x: True
        elif isinstance(recursive, dict):
            # Such overkill recursive system is needed when asking to
            determine_child_recursiveness = lambda x: recursive[x]
            to_sync = {k: v for k, v in manifest["children"].items() if k in recursive.keys()}
        else:
            to_sync = manifest["children"]
            determine_child_recursiveness = lambda x: True

        # Synchronize the children.
        for child_name, child_access in to_sync.items():
            async with self._rename_entry_on_concurrency_error(access):
                try:
                    child_manifest = self._local_tree.retrieve_entry_by_access(child_access)
                except KeyError:
                    # Entry not present in local, nothing to sync then
                    continue
                child_recursive = determine_child_recursiveness(child_name)
                if is_placeholder_access(child_access):
                    print(
                        info("asked to sync placeholder %s %s" % (child_name, child_access["id"]))
                    )
                    await self._sync_placeholder(
                        access, child_access, child_manifest, recursive=child_recursive
                    )
                else:
                    print(info("asked to sync %s %s" % (child_name, child_access["id"])))
                    await self._sync(child_access, child_manifest, recursive=child_recursive)

    async def _sync_folder_look_for_remote_changes(self, access, base_manifest):
        # This folder hasn't been modified locally, just download
        # last version from the backend if any.
        if not access:
            print(info("root sync not needed, check remote"))
            target_remote_manifest = (
                await self._manifests_manager.fetch_user_manifest_from_backend()
            )
        else:
            print(info("sync not needed, check remote %s" % access["id"]))
            # Placeholder means we need synchro !
            assert not is_placeholder_access(access)
            target_remote_manifest = await self._manifests_manager.fetch_from_backend(
                access["id"], access["rts"], access["key"]
            )
        if (
            not target_remote_manifest
            or target_remote_manifest["version"] == base_manifest["base_version"]
        ):
            return None
        return target_remote_manifest

    async def _sync_folder_actual_sync(self, access, manifest):
        access_id = access["id"] if access else "<root>"
        to_sync_manifest = convert_to_remote_manifest(manifest)

        # Upload the file manifest as new vlob version
        while True:
            try:
                if not access:
                    print(run("send folder sync %s %s" % (access_id, to_sync_manifest)))
                    await self._manifests_manager.sync_user_manifest_with_backend(to_sync_manifest)
                elif is_placeholder_access(access):
                    print(run("send folder placeholder sync %s %s" % (access_id, to_sync_manifest)))
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
                    print(run("send folder sync %s %s" % (access_id, to_sync_manifest)))
                    await self._manifests_manager.sync_with_backend(
                        access["id"], access["wts"], access["key"], to_sync_manifest
                    )
                break

            except BackendConcurrencyError:
                # Do a 3-ways merge to fix the concurrency error, first we must
                # fetch the base version and the new one present in the backend
                # TODO: base should be available locally
                print(bad("folder sync concurrency error %s" % access_id))
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
                    print(info("folder sync concurrency merged, sync not needed %s" % access_id))
                    # It maybe possible the changes that cause the concurrency
                    # error were the same than the one we wanted to make in the
                    # first place (e.g. when removing the same file)
                    break
                to_sync_manifest["version"] = target["version"] + 1

        return access, to_sync_manifest

    async def _sync_placeholder(self, parent_access, access, manifest, recursive=False):
        assert is_placeholder_access(access)
        new_access = await self._sync(access, manifest, recursive=recursive)
        if not new_access:
            # Synchronization has failed due to a concurrent remove of the placeholder
            return

        try:
            parent_manifest = self._local_tree.retrieve_entry_by_access(parent_access)
        except KeyError:
            # Parent no longer present (must have been removed during
            # the sync), nothing to do
            return
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

        print(good("placeholder resolved %s => %s" % (access["id"], new_access)))

    @asynccontextmanager
    async def _rename_entry_on_concurrency_error(self, parent_access):
        try:
            yield

        except FileSyncConcurrencyError as exc:
            # Concurrency error occured, to solve this we have to:
            # - Move our local diverged version to a new file
            # - Revert local modification of the file to have it consistent
            #   with the backend

            # This is basically a hack: the function which has raised
            # FileSyncConcurrencyError was holding the sync lock on this file.
            # Given we are the first catching it (or so we hope...), the lock
            # should be free and we don't fear modifying access or manifest.
            # Furthermore, given we do everything synchronously we can go
            # "hold me beer" style and don't take the lock.
            assert not self._sync_locks.is_locked(exc.access["id"])

            try:
                parent_manifest = self._local_tree.retrieve_entry_by_access(parent_access)
            except KeyError:
                # Two possibilities:
                # - Parent path has been removed, hence there is nothing to
                #   do here anymore
                # - Parent is no longer loaded in local tree. This is not
                #   supposed to happened with a folder so we should be safe...
                return

            # Retreive original name of the conflicting child and determine
            # a valid name for our duplicated file
            for candidate_name, candidate_access in parent_manifest["children"].items():
                if exc.access["id"] == candidate_access["id"]:
                    original_name = candidate_name
                    dup_name = find_conflicting_name_for_child_entry(parent_manifest, original_name)
                    break
            else:
                # The original entry has been remove in the meantime, there
                # is no point in trying to keep the conflicting data anymore !
                return

            # Now we can create the duplicated file as a placeholder and
            # insert it into the parent folder
            dup_access = new_placeholder_access()
            parent_manifest["children"][dup_name] = dup_access
            self._local_tree.update_entry(parent_access, parent_manifest)

            # Move changes into the duplicated file and update the file entry
            # with up to date data if available
            self._local_tree.move_modifications(exc.access, dup_access)
            self._opened_files.move_modifications(exc.access, dup_access)
            if exc.target_remote_manifest:
                target_local_manifest = convert_to_local_manifest(exc.target_remote_manifest)
                self._local_tree.overwrite_entry(exc.access, target_local_manifest)

            # Finally synchronize the placeholder
            try:
                dup_manifest = self._local_tree.retrieve_entry_by_access(dup_access)
            except KeyError:
                # Nothing to sync...
                return
            await self._sync_placeholder(parent_access, dup_access, dup_manifest)
