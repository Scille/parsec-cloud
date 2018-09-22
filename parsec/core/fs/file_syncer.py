import pendulum
from itertools import count

from parsec.core.fs.buffer_ordering import merge_buffers_with_limits_and_alignment
from parsec.core.fs.local_folder_fs import mark_manifest_modified
from parsec.core.fs.local_file_fs import DirtyBlockBuffer, BlockBuffer, NullFillerBuffer
from parsec.core.fs.sync_base import SyncConcurrencyError
from parsec.core.fs.utils import (
    is_file_manifest,
    new_access,
    new_block_access,
    is_placeholder_manifest,
    local_to_remote_manifest,
    remote_to_local_manifest,
)


def fast_forward_file(local_base, local_current, remote_target):
    assert local_base["base_version"] < remote_target["version"]
    assert local_base["base_version"] <= local_current["base_version"]
    assert local_current["base_version"] < remote_target["version"]

    processed_dirty_blocks_ids = [k["id"] for k in local_base["dirty_blocks"]]
    merged = {
        **local_current,
        "blocks": remote_target["blocks"],
        "dirty_blocks": [
            k for k in local_base["dirty_blocks"] if k["id"] not in processed_dirty_blocks_ids
        ],
        "base_version": remote_target["version"],
        "is_placeholder": False,
    }
    merged["need_sync"] = bool(merged["dirty_blocks"] or merged["size"] != remote_target["size"])
    return merged


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


def get_sync_map(manifest, block_size):
    dirty_blocks = [
        DirtyBlockBuffer(x["offset"], x["offset"] + x["size"], x) for x in manifest["dirty_blocks"]
    ]
    blocks = [BlockBuffer(x["offset"], x["offset"] + x["size"], x) for x in manifest["blocks"]]

    return merge_buffers_with_limits_and_alignment(
        blocks + dirty_blocks, 0, manifest["size"], block_size
    )


class FileSyncerMixin:
    async def _build_data_from_contiguous_space(self, cs):
        data = bytearray(cs.size)
        for bs in cs.buffers:
            if isinstance(bs.buffer, BlockBuffer):
                buff = await self._backend_block_get(bs.buffer.access)
            elif isinstance(bs.buffer, DirtyBlockBuffer):
                buff = self.local_file_fs.get_block(bs.buffer.access)
            else:
                assert isinstance(bs.buffer, NullFillerBuffer)
                buff = bs.buffer.data
            assert buff
            data[bs.start - cs.start : bs.end - cs.start] = buff[
                bs.buffer_slice_start : bs.buffer_slice_end
            ]
        return data

    def _sync_file_look_resolve_concurrency(
        self, path, access, diverged_manifest, target_remote_manifest
    ):
        parent_path, entry_name = path.rsplit("/", 1)
        parent_path = parent_path or "/"
        parent_access, parent_manifest = self.local_folder_fs.get_entry(parent_path)
        moved_name = find_conflicting_name_for_child_entry(parent_manifest, entry_name)
        moved_access = new_access()
        parent_manifest["children"][moved_name] = moved_access
        mark_manifest_modified(parent_manifest)

        diverged_manifest["base_version"] = 0
        diverged_manifest["created"] = pendulum.now()
        diverged_manifest["need_sync"] = True
        diverged_manifest["is_placeholder"] = True

        self.local_folder_fs.set_manifest(moved_access, diverged_manifest)
        self.local_folder_fs.set_manifest(parent_access, parent_manifest)
        target_manifest = remote_to_local_manifest(target_remote_manifest)
        self.local_folder_fs.set_manifest(access, target_manifest)

        self.signal_ns.signal("fs.entry.moved").send(
            None, original_id=access["id"], moved_id=moved_access["id"]
        )

    async def _sync_file_look_for_remote_changes(self, path, access, manifest):
        # Placeholder means we need synchro !
        assert not is_placeholder_manifest(manifest)

        # This folder hasn't been modified locally, just download
        # last version from the backend if any.
        target_remote_manifest = await self._backend_vlob_read(access)

        current_manifest = self.local_folder_fs.get_manifest(access)
        if target_remote_manifest["version"] == current_manifest["base_version"]:
            return False

        # Remote version has changed...
        if current_manifest["need_sync"]:
            # ...and modifications occured on our back, now we have a concurrency error !
            self._sync_file_look_resolve_concurrency(
                path, access, current_manifest, target_remote_manifest
            )
        else:
            target_local_manifest = remote_to_local_manifest(target_remote_manifest)
            # Otherwise just fast-forward the local data
            self.local_folder_fs.set_manifest(access, target_local_manifest)
        return True

    async def _sync_file_actual_sync(self, path, access, manifest, notify_beacons):
        assert is_file_manifest(manifest)

        to_sync_manifest = local_to_remote_manifest(manifest)
        to_sync_manifest["version"] += 1

        # Compute the file's blocks and upload the new ones
        blocks = []
        sync_map = get_sync_map(manifest, self.block_size)

        # Upload the new blocks
        for cs in sync_map.spaces:
            data = await self._build_data_from_contiguous_space(cs)
            if not data:
                # Already existing blocks taken verbatim
                blocks += [bs.buffer.data for bs in cs.buffers]
            else:
                # Create a new block from existing data
                block_access = new_block_access(data, cs.start)
                await self._backend_block_post(block_access, data)
                blocks.append(block_access)

        to_sync_manifest["blocks"] = blocks
        to_sync_manifest["size"] = sync_map.size  # TODO: useful ?

        # Upload the file manifest as new vlob version
        try:
            if is_placeholder_manifest(manifest):
                await self._backend_vlob_create(access, to_sync_manifest, notify_beacons)
            else:
                await self._backend_vlob_update(access, to_sync_manifest, notify_beacons)

        except SyncConcurrencyError:
            # Placeholder don't have remote version, so no concurrency is possible
            assert not is_placeholder_manifest(manifest)
            target_remote_manifest = await self._backend_vlob_read(access)

            current_manifest = self.local_folder_fs.get_manifest(access)
            # Do a fast-forward to avoid losing block we have uploaded
            diverged_manifest = fast_forward_file(manifest, current_manifest, to_sync_manifest)

            self._sync_file_look_resolve_concurrency(
                path, access, diverged_manifest, target_remote_manifest
            )

            # # TODO
            # target_local_manifest = remote_to_local_manifest(target_remote_manifest)
            # self._sync_file_look_resolve_concurrency(
            #     path, access, current_manifest, target_local_manifest
            # )
            # await self._backend_vlob_create(access, to_sync_manifest, notify_beacons)

        else:
            # Finally merge with the current version of the manifest which may have
            # been modified in the meantime
            current_manifest = self.local_folder_fs.get_manifest(access)
            assert is_file_manifest(current_manifest)
            final_manifest = fast_forward_file(manifest, current_manifest, to_sync_manifest)
            self.local_folder_fs.set_manifest(access, final_manifest)

    async def _sync_file_nolock(self, path, access, manifest, notify_beacons):
        """
        Raises:
            FileSyncConcurrencyError
            BackendNotAvailable
        """
        assert is_file_manifest(manifest)

        # Now we can synchronize the folder if needed
        if not manifest["need_sync"]:
            need_signal = await self._sync_file_look_for_remote_changes(path, access, manifest)
        else:
            await self._sync_file_actual_sync(path, access, manifest, notify_beacons)
            need_signal = True
        if need_signal:
            self.signal_ns.signal("fs.entry.synced").send(None, path=path, id=access["id"])
