# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pendulum
from typing import List
from structlog import get_logger

from parsec.core.types import (
    FsPath,
    LocalFileManifest,
    FileManifest,
    Access,
    ManifestAccess,
    BlockAccess,
)
from parsec.core.fs.merge_folders import find_conflicting_name_for_child_entry
from parsec.core.fs.buffer_ordering import merge_buffers_with_limits_and_alignment
from parsec.core.fs.local_file_fs import Buffer, DirtyBlockBuffer, BlockBuffer, NullFillerBuffer
from parsec.core.fs.sync_base import SyncConcurrencyError, BaseSyncer
from parsec.core.fs.utils import is_file_manifest, is_placeholder_manifest


logger = get_logger()


def fast_forward_file(
    local_base: LocalFileManifest, local_current: LocalFileManifest, remote_target: FileManifest
) -> LocalFileManifest:
    assert local_base.base_version < remote_target.version
    assert local_base.base_version <= local_current.base_version
    assert local_current.base_version < remote_target.version

    processed_dirty_blocks_ids = [k.id for k in local_base.dirty_blocks]
    merged_dirty_blocks = [
        k for k in local_current.dirty_blocks if k.id not in processed_dirty_blocks_ids
    ]
    merged_need_sync = bool(merged_dirty_blocks or local_current.size != remote_target.size)
    return local_current.evolve(
        blocks=remote_target.blocks,
        dirty_blocks=merged_dirty_blocks,
        base_version=remote_target.version,
        is_placeholder=False,
        need_sync=merged_need_sync,
    )


def get_sync_map(manifest, block_size: int) -> List[Buffer]:
    dirty_blocks: List[Buffer] = [
        DirtyBlockBuffer(x.offset, x.offset + x.size, x) for x in manifest.dirty_blocks
    ]
    blocks: List[Buffer] = [BlockBuffer(x.offset, x.offset + x.size, x) for x in manifest.blocks]

    return merge_buffers_with_limits_and_alignment(
        blocks + dirty_blocks, 0, manifest.size, block_size
    )


class FileSyncerMixin(BaseSyncer):
    async def _build_data_from_contiguous_space(self, cs):
        data = bytearray(cs.size)
        buffers = cs.buffers.copy()

        async def _process_buffer():
            while buffers:
                bs = buffers.pop()
                if isinstance(bs.buffer, BlockBuffer):
                    buff = await self._backend_block_read(bs.buffer.access)
                elif isinstance(bs.buffer, DirtyBlockBuffer):
                    buff = self.local_file_fs.get_block(bs.buffer.access)
                else:
                    assert isinstance(bs.buffer, NullFillerBuffer)
                    buff = bs.buffer.data
                assert buff
                data[bs.start - cs.start : bs.end - cs.start] = buff[
                    bs.buffer_slice_start : bs.buffer_slice_end
                ]

        if len(buffers) < 2:
            await _process_buffer()

        else:
            async with trio.open_nursery() as nursery:
                nursery.start_soon(_process_buffer)
                nursery.start_soon(_process_buffer)
                nursery.start_soon(_process_buffer)
                nursery.start_soon(_process_buffer)

        return data

    def _sync_file_look_resolve_concurrency(
        self,
        path: FsPath,
        access: Access,
        diverged_manifest: LocalFileManifest,
        target_remote_manifest: FileManifest,
    ) -> None:
        parent_access, parent_manifest = self.local_folder_fs.get_entry(path.parent)
        moved_name = find_conflicting_name_for_child_entry(
            path.name, lambda name: name not in parent_manifest.children
        )
        moved_access = ManifestAccess()
        parent_manifest = parent_manifest.evolve_children_and_mark_updated(
            {moved_name: moved_access}
        )

        diverged_manifest = diverged_manifest.evolve(
            base_version=0, created=pendulum.now(), need_sync=True, is_placeholder=True
        )

        self.local_folder_fs.set_dirty_manifest(moved_access, diverged_manifest)
        self.local_folder_fs.set_dirty_manifest(parent_access, parent_manifest)
        target_manifest = target_remote_manifest.to_local()
        self.local_folder_fs.set_dirty_manifest(access, target_manifest)

        self.event_bus.send(
            "fs.entry.file_update_conflicted",
            path=str(path),
            diverged_path=str(path.parent / moved_name),
            original_id=access.id,
            diverged_id=moved_access.id,
        )
        self.event_bus.send("fs.entry.updated", id=moved_access.id)

    async def _sync_file_look_for_remote_changes(
        self, path: FsPath, access: Access, manifest: LocalFileManifest
    ) -> bool:
        # Placeholder means we need synchro !
        assert not is_placeholder_manifest(manifest)

        # This folder hasn't been modified locally, just download
        # last version from the backend if any.
        target_remote_manifest = await self._backend_vlob_read(access)

        current_manifest = self.local_folder_fs.get_manifest(access)
        if target_remote_manifest.version == current_manifest.base_version:
            return False

        # Remote version has changed...
        if current_manifest.need_sync:
            # ...and modifications occured on our back, now we have a concurrency error !
            self._sync_file_look_resolve_concurrency(
                path, access, current_manifest, target_remote_manifest
            )
        else:
            target_local_manifest = target_remote_manifest.to_local()
            # Otherwise just fast-forward the local data
            self.local_folder_fs.set_clean_manifest(access, target_local_manifest)
        return True

    async def _sync_file_actual_sync(
        self, path: FsPath, access: Access, manifest: LocalFileManifest
    ) -> None:
        assert is_file_manifest(manifest)

        # to_sync_manifest = manifest.to_remote(version=manifest.base_version + 1)
        to_sync_manifest = manifest.to_remote()
        to_sync_manifest = to_sync_manifest.evolve(version=manifest.base_version + 1)

        # Compute the file's blocks and upload the new ones
        blocks = []
        sync_map = get_sync_map(manifest, self.block_size)

        # Upload the new blocks
        spaces = sync_map.spaces
        blocks = []

        vlob_group = self.local_folder_fs.get_vlob_group(path)

        async def _process_spaces():
            nonlocal blocks
            while spaces:
                cs = spaces.pop()
                data = await self._build_data_from_contiguous_space(cs)
                if not data:
                    # Already existing blocks taken verbatim
                    blocks += [bs.buffer.data for bs in cs.buffers]
                else:
                    # Create a new block from existing data
                    block_access = BlockAccess.from_block(data, cs.start)
                    await self._backend_block_create(vlob_group, block_access, data)
                    blocks.append(block_access)

                    # The block has been successfully uploaded
                    # Keep it in the remote storage
                    self.local_file_fs.set_clean_block(block_access, data)

        if len(spaces) < 2:
            await _process_spaces()

        else:
            async with trio.open_nursery() as nursery:
                nursery.start_soon(_process_spaces)
                nursery.start_soon(_process_spaces)
                nursery.start_soon(_process_spaces)
                nursery.start_soon(_process_spaces)

        to_sync_manifest = to_sync_manifest.evolve(
            blocks=blocks, size=sync_map.size  # TODO: useful ?
        )

        # Upload the file manifest as new vlob version
        try:
            if is_placeholder_manifest(manifest):
                await self._backend_vlob_create(vlob_group, access, to_sync_manifest)
            else:
                await self._backend_vlob_update(access, to_sync_manifest)

        except SyncConcurrencyError:
            # Placeholder don't have remote version, so concurrency shouldn't
            # be possible. However it's possible a previous attempt of
            # uploading this manifest succeeded but we didn't receive the
            # backend's answer, hence wrongly believing this is still a
            # placeholder.
            if is_placeholder_manifest(manifest):
                logger.warning("Concurrency error while creating vlob", access_id=access.id)

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

            # The vlob has been successfully uploaded - clean up the dirty blocks
            for dirty_block in manifest.dirty_blocks:
                self.local_file_fs.clear_dirty_block(dirty_block)

            # Also clean up the outdated blocks from the remote cache
            outdated_blocks = set(manifest.blocks) - set(to_sync_manifest.blocks)
            for remote_block in outdated_blocks:
                self.local_file_fs.clear_clean_block(remote_block)

            self._sync_file_merge_back(access, manifest, to_sync_manifest)

        return to_sync_manifest

    async def _sync_file(self, path: FsPath, access: Access, manifest: LocalFileManifest) -> None:
        """
        Raises:
            FileSyncConcurrencyError
            BackendNotAvailable
        """
        assert not is_placeholder_manifest(manifest)
        assert is_file_manifest(manifest)

        # Now we can synchronize the folder if needed
        if not manifest.need_sync:
            changed = await self._sync_file_look_for_remote_changes(path, access, manifest)
        else:
            await self._sync_file_actual_sync(path, access, manifest)
            changed = True
        if changed:
            self.event_bus.send("fs.entry.synced", path=str(path), id=access.id)

    async def _minimal_sync_file(
        self, path: FsPath, access: Access, manifest: LocalFileManifest
    ) -> bool:
        """
        Returns: If additional sync are needed
        Raises:
            FileSyncConcurrencyError
            BackendNotAvailable
        """
        if not is_placeholder_manifest(manifest):
            return manifest.need_sync

        need_more_sync = bool(manifest.dirty_blocks)
        # Don't sync the dirty blocks for fast synchronization
        try:
            last_block = manifest.blocks[-1]
            size = last_block.offset + last_block.size
        except IndexError:
            size = 0
        minimal_manifest = manifest.evolve(
            updated=manifest.created if need_more_sync else manifest.updated,
            size=size,
            blocks=manifest.blocks,
            dirty_blocks=(),
        )

        await self._sync_file_actual_sync(path, access, minimal_manifest)

        self.event_bus.send("fs.entry.minimal_synced", path=str(path), id=access.id)
        return need_more_sync

    def _sync_file_merge_back(
        self, access: Access, base_manifest: LocalFileManifest, target_remote_manifest: FileManifest
    ) -> None:
        # Merge with the current version of the manifest which may have
        # been modified in the meantime
        assert is_file_manifest(target_remote_manifest)
        current_manifest = self.local_folder_fs.get_manifest(access)
        assert is_file_manifest(current_manifest)

        final_manifest = fast_forward_file(base_manifest, current_manifest, target_remote_manifest)
        if final_manifest.need_sync:
            # New manifest is not up to date with the remote
            self.local_folder_fs.set_dirty_manifest(access, final_manifest)
        else:
            # New manifest is up to date with the remote: safely clean up the local manifest
            self.local_folder_fs.set_clean_manifest(access, final_manifest, force=True)
