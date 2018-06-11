from huepy import good, bad, run, info, que

from parsec.core.fs.base import FSBase
from parsec.core.fs.utils import (
    is_placeholder_access,
    is_file_manifest,
    new_block_access,
    convert_to_remote_manifest,
    convert_to_local_manifest,
)
from parsec.core.backend_storage import BackendConcurrencyError


class FileSyncConcurrencyError(Exception):
    def __init__(self, access, target_remote_manifest=None):
        self.access = access
        self.target_remote_manifest = target_remote_manifest


class FSSyncFileMixin(FSBase):
    async def _sync_file(self, access, manifest):
        """
        Raises:
            FileSyncConcurrencyError
            BackendNotAvailable
        """

        # Acquire sync lock
        while True:
            if not self._sync_locks.is_locked(access["id"]):
                break
            print(bad("retry sync manifest %s" % access["id"]))
            # This file is in the middle of a sync, wait for it to
            # end and reload the manifest to avoid concurrency issues
            await self._sync_locks.wait_not_locked(access["id"])
            try:
                manifest = self._local_tree.retrieve_entry_by_access(access)
                assert is_file_manifest(manifest)
            except KeyError:
                # Entry has been removed in the meantime, nothing to do then
                return

        with self._sync_locks.lock(access["id"]):
            # Note from here on, not concurrent sync or flush are allowed,
            # so manifest is guaranteed not to change in our back
            fd = self._opened_files.open_file(access, manifest)
            if not fd.need_sync():
                await self._sync_file_look_for_remote_changes(fd)
                return access
            else:
                return await self._sync_file_actual_sync(fd, manifest)

    async def _sync_file_look_for_remote_changes(self, fd):
        access = fd.access

        # Placeholder means we need synchro !
        assert not is_placeholder_access(access)

        # This file hasn't been modified locally,
        # just download last version from the backend if any.
        target_remote_manifest = await self._manifests_manager.fetch_from_backend(
            access["id"], access["rts"], access["key"]
        )

        # No need to reload manifest to avoid concurrency issue given
        # our caller held the sync lock on this file

        if target_remote_manifest["version"] != fd.base_version:
            # Remote version has changed...
            if fd.need_sync():
                # ...and modifications occured in our back, now we have a concurrency error !
                raise FileSyncConcurrencyError(
                    access, target_remote_manifest=target_remote_manifest
                )
            else:
                target_local_manifest = convert_to_local_manifest(target_remote_manifest)
                fd.fast_forward(new_manifest=target_local_manifest)
                self._local_tree.overwrite_entry(access, target_local_manifest)
        else:
            print(good("file sync not needed (and no remote changes) %s" % access["id"]))

    async def _sync_file_actual_sync(self, fd, manifest):
        fd_snapshot_state = fd.create_snapshot_state()
        to_sync_manifest = convert_to_remote_manifest(manifest)

        # Compute the file's blocks and upload the new ones
        blocks = []
        ucs = fd.get_sync_map()

        # Upload the new blocks
        for cs in ucs.spaces:
            if not cs.need_sync():
                blocks += [bs.buffer.data for bs in cs.buffers]
                continue
            # Create a new block from existing data
            data = await self._build_data_from_contiguous_space(cs)
            block_access = new_block_access(cs.start, data)
            block_access["id"] = await self._blocks_manager.sync_new_block_with_backend(
                block_access["key"], data
            )
            blocks.append(block_access)

        to_sync_manifest["blocks"] = blocks
        to_sync_manifest["size"] = ucs.size
        assert to_sync_manifest["size"] == sum(b["size"] for b in to_sync_manifest["blocks"])

        access = fd.access
        try:
            if is_placeholder_access(access):
                print(run("send file placeholder sync %s %s" % (access["id"], to_sync_manifest)))
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
                print(run("send file sync %s %s" % (access["id"], to_sync_manifest)))
                await self._manifests_manager.sync_with_backend(
                    access["id"], access["wts"], access["key"], to_sync_manifest
                )
                final_access = access

        except BackendConcurrencyError as exc:
            assert not is_placeholder_access(access)
            print(bad("file sync concurrency error %s" % access))

            # In order not to lose the blocks we uploaded, we update the
            # local manifest before raising the concurrency error.
            # Note there is no risk to overwrite a manifest version because
            # we hold the lock preventing concurrent syncs and flushes.
            new_manifest = convert_to_local_manifest(to_sync_manifest, as_need_sync=True)
            assert new_manifest["base_version"] == manifest["base_version"]
            self._local_tree.update_entry(access, new_manifest)
            fd.fast_forward(snapshot_state=fd_snapshot_state, new_manifest=new_manifest)

            raise FileSyncConcurrencyError(access) from exc

        final_manifest = convert_to_local_manifest(to_sync_manifest)
        fd.fast_forward(snapshot_state=fd_snapshot_state, new_manifest=final_manifest)
        self._local_tree.overwrite_entry(access, final_manifest)

        return final_access
