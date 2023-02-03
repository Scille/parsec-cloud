# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import time
from collections import deque
from contextlib import contextmanager
from enum import Enum
from typing import Callable, Iterator, Set, Tuple, Union, cast

import structlog
import trio
from attr import define

from parsec._parsec import CoreEvent
from parsec.api.data import BlockAccess
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.storage import BaseWorkspaceStorage
from parsec.core.fs.workspacefs.sync_transactions import ChangesAfterSync, SyncTransactions
from parsec.core.types import (
    BlockID,
    EntryID,
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
)
from parsec.event_bus import EventBus, EventCallback
from parsec.utils import open_service_nursery

logger = structlog.get_logger()


# The purpose of the remanence manager is to keep track of all the referenced blocks
# separating them between "remote only" and "local and remote" blocks.
#
# In order to do that, we need to keep track of everything that can affect this count,
# including:
# - New blocks are referenced:
#    * When a vlob is updated by someone else.
#      We use the DOWNSYNCED event to detect this case and perform a recursive sweep
#      in the remanance manager in order to list all newly referenced blocks as remote only.
#    * When a vlob is updated by our device.
#      We use the SYNCED event to detect this case. We only need to consider the events
#      corresponding to file manifests and list the newly referenced blocks as remote and local.
# - Old blocks are no longer referenced:
#    * When a vlob is updated by someone else.
#      We use the DOWNSYNCED event to detect this case and perform a recursive sweep
#      in the remanance manager in order to remove all the old blocks from both lists.
#    * When a vlob is updated by our device.
#      We use the SYNCED event to detect this case and perform a recursive sweep
#      in the remanance manager in order to remove all the old blocks from both lists.
# - A referenced block is downloaded:
#    * When a read transaction require blocks that are missing.
#      In this case we use a dedicated event.
#    * When block remanence is enabled and the manager downloads the missing blocks.
#      We can simply keep track of the downloaded blocks here
# - Referenced blocks are discarded:
#    * This happens when the local storage for blocks has reached its limit.
#      We use a dedicated event to detect this case and remove those blocks from the remote only list.
#
# Note: we should try to avoid concurrency issue that would render the manager out of
# sync with the actual state of the workspace, although this is not a huge deal either
# as the workspace is going to be fully swept at the next loging.


RemanenceJob = Tuple[CoreEvent, Union[ChangesAfterSync, BlockAccess, Set[BlockID]]]


class RemanenceManagerState(Enum):
    STOPPED = "STOPPED"
    PREPARING = "PREPARING"
    RUNNING = "RUNNING"


@define(frozen=True, slots=True, auto_attribs=True)
class RemanenceManagerInfo:
    is_running: bool
    is_prepared: bool
    is_block_remanent: bool
    total_size: int
    remote_only_size: int
    local_and_remote_size: int


class RemanenceManager:
    def __init__(
        self,
        local_storage: BaseWorkspaceStorage,
        remote_loader: RemoteLoader,
        transactions: SyncTransactions,
        workspace_id: EntryID,
        event_bus: EventBus,
    ):
        self.transactions = transactions
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.workspace_id = workspace_id
        self.event_bus = event_bus

        # State synchronization attributes
        self._events_connected = False
        self._prepared_event = trio.Event()
        self._idle: Callable[[int], None] = lambda _: None
        self._awake: Callable[[int], None] = lambda _: None
        self._main_task_state = RemanenceManagerState.STOPPED
        self._main_task_cancel_scope: trio.CancelScope | None = None
        self._downloader_task_cancel_scope: trio.CancelScope | None = None

        # Data attributes
        self._all_blocks: dict[BlockID, BlockAccess] = {}
        self._total_size: int = 0
        self._remote_only_blocks: set[BlockAccess] = set()
        self._remote_only_size: int = 0
        self._job_queue: deque[RemanenceJob] = deque()

    # Interface

    def is_block_remanent(self) -> bool:
        return self.local_storage.is_block_remanent()

    async def enable_block_remanence(self) -> bool:
        has_changed = await self.local_storage.enable_block_remanence()
        # Force the downsync manager to update
        if has_changed:
            self._wake_up_downloader_task()
        return has_changed

    async def disable_block_remanence(self) -> bool:
        removed_block_ids = await self.local_storage.disable_block_remanence()
        # Register the cleared blocks
        if removed_block_ids is not None:
            self.event_bus.send(
                CoreEvent.FS_BLOCK_REMOVED,
                workspace_id=self.workspace_id,
                block_ids=removed_block_ids,
            )
        # Return whether state has changed or not
        return removed_block_ids is not None

    async def wait_prepared(self, wait_for_connection: bool = False) -> None:
        while True:
            if self._main_task_state == RemanenceManagerState.STOPPED and not wait_for_connection:
                raise RuntimeError("The remanence manager is currently stopped")
            if self._main_task_state == RemanenceManagerState.RUNNING:
                return
            with trio.move_on_after(0.1):
                await self._prepared_event.wait()

    def get_info(self) -> RemanenceManagerInfo:
        return RemanenceManagerInfo(
            is_running=self._main_task_state != RemanenceManagerState.STOPPED,
            is_prepared=self._prepared_event.is_set(),
            is_block_remanent=self.local_storage.is_block_remanent(),
            total_size=self._total_size,
            local_and_remote_size=self._total_size - self._remote_only_size,
            remote_only_size=self._remote_only_size,
        )

    # Main routine

    @contextmanager
    def manage_events(self) -> Iterator[None]:
        try:
            with self.event_bus.connect_in_context(
                (CoreEvent.FS_ENTRY_DOWNSYNCED, cast(EventCallback, self._on_entry_synced)),
                (CoreEvent.FS_ENTRY_SYNCED, cast(EventCallback, self._on_entry_synced)),
                (CoreEvent.FS_BLOCK_DOWNLOADED, cast(EventCallback, self._on_block_downloaded)),
                (CoreEvent.FS_BLOCK_REMOVED, cast(EventCallback, self._on_block_removed)),
            ):
                self._events_connected = True
                yield
        finally:
            self._events_connected = False

    async def run(self, idle: Callable[[int], None], awake: Callable[[int], None]) -> None:
        if not self._events_connected:
            raise RuntimeError("The events are not connected")
        if self._main_task_state != RemanenceManagerState.STOPPED:
            logger.warning("Trying to run an already runnning downsync manager")
            return
        try:
            self._idle = idle
            self._awake = awake
            self._main_task_state = RemanenceManagerState.PREPARING

            # Perform a fullsweep is necessary
            if not self._prepared_event.is_set():
                await self._prepare()
            self._main_task_state = RemanenceManagerState.RUNNING

            # Here we're running two tasks:
            # - processing the jobs
            # - optionally, downloading the missing blocks
            async with open_service_nursery() as nursery:
                nursery.start_soon(self._downloader_task)

                # Loop over jobs
                while True:

                    # It's good practice to tick here to avoid busy loops
                    await trio.sleep(0)

                    # Get the next downsync job and process it
                    if self._job_queue:
                        await self._process_next_job()
                        continue

                    # Pause when nothing else to do
                    with trio.CancelScope() as self._main_task_cancel_scope:
                        self._idle(1)
                        await trio.sleep_forever()

        # Update state
        finally:
            self._main_task_state = RemanenceManagerState.STOPPED

    # Task management

    def _wake_up_main_task(self) -> None:
        # Wake-up the main task
        if (
            self._main_task_cancel_scope is not None
            and not self._main_task_cancel_scope.cancel_called
        ):
            self._main_task_cancel_scope.cancel()
            # Tag the monitor as awaken as soon as this callback runs
            self._awake(1)

    def _wake_up_downloader_task(self) -> None:
        # Wake-up the downloader task
        if (
            self._downloader_task_cancel_scope is not None
            and not self._downloader_task_cancel_scope.cancel_called
        ):
            self._downloader_task_cancel_scope.cancel()
            # Tag the monitor as awaken as soon as this callback runs
            self._awake(2)

    # Event management

    def _ready_for_job(self) -> bool:
        return (
            self._prepared_event.is_set()
            or self._main_task_state == RemanenceManagerState.PREPARING
        )

    def _on_entry_synced(
        self,
        event: CoreEvent,
        id: EntryID,
        changes: ChangesAfterSync | None = None,
        workspace_id: EntryID | None = None,
    ) -> None:
        # Not our workspace
        if workspace_id != self.workspace_id or changes is None:
            return
        # The manager is neither prepared or preparing
        if not self._ready_for_job():
            return
        # Add this event to the queue
        self._job_queue.appendleft((event, changes))
        self._wake_up_main_task()

    def _on_block_downloaded(
        self,
        event: CoreEvent,
        block_access: BlockAccess,
        workspace_id: EntryID,
    ) -> None:
        # Not our workspace
        if workspace_id != self.workspace_id:
            return
        # The manager is neither prepared or preparing
        if not self._ready_for_job():
            return
        # Add this event to the queue
        self._job_queue.appendleft((event, block_access))
        self._wake_up_main_task()

    def _on_block_removed(
        self,
        event: CoreEvent,
        block_ids: set[BlockID],
        workspace_id: EntryID,
    ) -> None:
        # Not our workspace
        if workspace_id != self.workspace_id:
            return
        # The manager is neither prepared or preparing
        if not self._ready_for_job():
            return
        # Add this event to the queue
        self._job_queue.appendleft((event, block_ids))
        self._wake_up_main_task()

    # Internal helpers

    def _register_cleared_blocks(self, block_ids: set[BlockID]) -> None:
        # Convert IDs to accesses
        removed_blocks = {
            self._all_blocks[block_id] for block_id in block_ids if block_id in self._all_blocks
        }
        # Add blocks to remote-only sets
        self._remote_only_size += sum(
            access.size for access in removed_blocks - self._remote_only_blocks
        )
        self._remote_only_blocks |= removed_blocks
        self._wake_up_downloader_task()

    def _register_downloaded_block(self, block_access: BlockAccess) -> None:
        if block_access in self._remote_only_blocks:
            self._remote_only_size -= block_access.size
            self._remote_only_blocks.discard(block_access)

    def _register_added_blocks(
        self, added_blocks: set[BlockAccess], are_present_locally: bool
    ) -> None:
        # Update total
        for access in added_blocks:
            if access.id not in self._all_blocks:
                self._all_blocks[access.id] = access
                self._total_size += access.size
        # Update remote-only if necessary
        if not are_present_locally:
            self._remote_only_size += sum(
                access.size for access in added_blocks - self._remote_only_blocks
            )
            self._remote_only_blocks |= added_blocks
            self._wake_up_downloader_task()

    def _register_removed_blocks(self, removed_blocks: set[BlockAccess]) -> None:
        # Update total
        for access in removed_blocks:
            if self._all_blocks.pop(access.id) is not None:
                self._total_size -= access.size
        # Update remote-only
        self._remote_only_size -= sum(
            access.size for access in removed_blocks & self._remote_only_blocks
        )
        self._remote_only_blocks -= removed_blocks

    # Task methods

    async def _prepare(self, with_cleanup: bool = True) -> None:
        # Sanity check
        assert not self._prepared_event.is_set()
        # Clear job queue, just in case
        self._job_queue.clear()
        # Initialize state
        begining = time.time()
        remote_only_blocks: list[BlockAccess] = []
        local_and_remote_blocks: list[BlockAccess] = []
        entry_ids: list[EntryID] = [self.workspace_id]
        # Loop over entries
        while entry_ids:
            entry_id = entry_ids.pop()
            # Load manifest
            manifest = await self.transactions._load_manifest(entry_id)
            # Browse the directories
            if isinstance(manifest, (LocalWorkspaceManifest, LocalFolderManifest)):
                entry_ids.extend(manifest.children.values())
            # List blocks and missing blocks
            elif isinstance(manifest, LocalFileManifest):
                info = await self.transactions._get_blocks_by_type(manifest, 1000000000)
                remote_only_blocks.extend(info.remote_only_blocks)
                local_and_remote_blocks.extend(info.local_and_remote_blocks)
        # Cleanup unreferenced blocks
        if with_cleanup:
            referenced_block_ids = [access.id for access in local_and_remote_blocks]
            await self.local_storage.clear_unreferenced_blocks(referenced_block_ids, begining)
        # Set internal state
        self._remote_only_blocks = set(remote_only_blocks)
        self._remote_only_size = sum(access.size for access in self._remote_only_blocks)
        self._all_blocks = {
            access.id: access for access in self._remote_only_blocks | set(local_and_remote_blocks)
        }
        self._total_size = sum(access.size for access in self._all_blocks.values())
        self._prepared_event.set()

    async def _list_blocks(self, job: set[EntryID]) -> set[BlockAccess]:
        entry_ids: list[EntryID] = list(job)
        blocks: set[BlockAccess] = set()
        # Download all the corresponding manifests and list the blocks
        while entry_ids:
            entry_id = entry_ids.pop()
            # Download the manifest
            manifest = await self.transactions._load_manifest(entry_id)
            # Browse the directories
            if isinstance(manifest, (LocalWorkspaceManifest, LocalFolderManifest)):
                entry_ids.extend(manifest.children.values())
            # List blocks
            elif isinstance(manifest, LocalFileManifest):
                blocks.update(manifest.base.blocks)
        # Return all the blocks found
        return blocks

    async def _process_next_job(self) -> None:
        # Sanity check
        assert self._job_queue
        event, data = self._job_queue[-1]
        # Block downloaded event
        if event == CoreEvent.FS_BLOCK_DOWNLOADED:
            assert isinstance(data, BlockAccess)
            self._register_downloaded_block(data)
        # Block removed event
        elif event == CoreEvent.FS_BLOCK_REMOVED:
            assert isinstance(data, set)
            self._register_cleared_blocks(data)
        else:
            # Entry sync event
            assert event in (CoreEvent.FS_ENTRY_DOWNSYNCED, CoreEvent.FS_ENTRY_SYNCED)
            assert isinstance(data, ChangesAfterSync)
            changes = data
            # List added blocks
            added_blocks: set[BlockAccess] = set()
            if changes.added_blocks is not None:
                added_blocks |= changes.added_blocks
            if changes.added_entries is not None:
                added_blocks |= await self._list_blocks(changes.added_entries)
            # List removed blocks
            removed_blocks: set[BlockAccess] = set()
            if changes.removed_blocks is not None:
                removed_blocks |= changes.removed_blocks
            if changes.removed_entries is not None:
                removed_blocks |= await self._list_blocks(changes.removed_entries)
            # Process added blocks
            if added_blocks:
                self._register_added_blocks(
                    added_blocks,
                    are_present_locally=event == CoreEvent.FS_ENTRY_SYNCED,
                )
            # Process removed blocks
            if removed_blocks:
                self._register_removed_blocks(removed_blocks)
                removed_block_ids = [access.id for access in removed_blocks]
                await self.local_storage.remove_clean_blocks(removed_block_ids)
        # Pop the job as it's been successfully processed
        self._job_queue.pop()

    async def _downloader_task(self) -> bool:
        while True:

            # Not a block-remanent workspace or no block to download
            if not self.local_storage.is_block_remanent() or not self._remote_only_blocks:
                with trio.CancelScope() as self._downloader_task_cancel_scope:
                    self._idle(2)
                    await trio.sleep_forever()
                continue

            # Open a nursery for downloading in parrallel (4 tasks)
            async with open_service_nursery() as nursery:

                # Open channel and loop over downloaded blocks
                blocks = list(self._remote_only_blocks)
                channel = await self.remote_loader.receive_load_blocks(blocks, nursery)
                async with channel:
                    async for access in channel:

                        # Update the internal sets
                        self._register_downloaded_block(access)

                        # Maybe block remanence has changed in the meantime
                        # No reason to download the other blocks
                        if not self.local_storage.is_block_remanent():
                            nursery.cancel_scope.cancel()
