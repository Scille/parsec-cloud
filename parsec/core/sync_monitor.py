# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from trio.hazmat import current_clock
import math
from structlog import get_logger

from parsec.core.types import EntryID, WorkspaceRole
from parsec.core.fs import (
    FSBackendOfflineError,
    FSWorkspaceNotFoundError,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceInMaintenance,
)
from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable


logger = get_logger()

MIN_WAIT = 1
MAX_WAIT = 60
MAINTENANCE_MIN_WAIT = 30
TICK_CRASH_COOLDOWN = 5


async def freeze_sync_monitor_mockpoint():
    """
    Noop function that could be mocked during tests to be able to freeze the
    monitor coroutine running in background
    """
    pass


def timestamp():
    # Use time from trio clock to easily mock it
    return current_clock().current_time()


class LocalChange:
    __slots__ = ("first_changed_on", "last_changed_on", "due_time")

    def __init__(self, now):
        self.first_changed_on = self.last_changed_on = now
        self.due_time = self._compute_due_time()

    def _compute_due_time(self):
        return min(self.last_changed_on + MIN_WAIT, self.first_changed_on + MAX_WAIT)

    def changed(self, changed_on) -> bool:
        self.last_changed_on = changed_on
        self.due_time = self._compute_due_time()
        return self.due_time <= self.last_changed_on


class SyncContext:
    """
    The SyncContext keeps track of local and remote changes and trigger sync
    accordingly.
    There is two way for it to be informed of remote changes:
    - When online it receive `realm.vlobs_updated` updated events
    - Otherwise (typically when the application starts or when back online after
      an disconnection) it uses the realm's checkpoint stored in the persistent
      storage to get the list of changes (entry id + version) it has missed
    """

    def __init__(self, user_fs, id: EntryID, read_only: bool = False):
        self.user_fs = user_fs
        self.id = id
        self.read_only = read_only
        self.local_changes = {}
        self.remote_changes = set()
        self.need_bootstrap = True
        self.bootstrapped = False
        self.due_time = timestamp()

    def _sync(self, entry_id: EntryID):
        raise NotImplementedError

    def _get_backend_cmds(self):
        raise NotImplementedError

    def _get_local_storage(self):
        raise NotImplementedError

    def __repr__(self):
        return f"{type(self).__name__}(id={self.id!r})"

    async def _refresh_checkpoint(self) -> bool:
        # 1) Fetch new checkpoint and changes
        realm_checkpoint = await self._get_local_storage().get_realm_checkpoint()
        try:
            rep = await self._get_backend_cmds().vlob_poll_changes(self.id, realm_checkpoint)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Another backend error
        except BackendConnectionError as exc:
            logger.warning("Unexpected backend response during sync bootstrap", exc_info=exc)
            return False

        if rep["status"] == "not_found":
            # Workspace not yet synchronized with backend
            new_checkpoint = 0
            changes = {}
        if rep["status"] in ("in_maintenance", "not_allowed"):
            return False
        elif rep["status"] != "ok":
            return False
        else:
            new_checkpoint = rep["current_checkpoint"]
            changes = rep["changes"]

        # 2) Store new checkpoint and changes
        await self._get_local_storage().update_realm_checkpoint(new_checkpoint, changes)

        # 3) Compute local and remote changes that need to be synced
        need_sync_local, need_sync_remote = await self._get_local_storage().get_need_sync_entries()
        now = timestamp()
        # Ignore local changes in read only mode
        if not self.read_only:
            self.local_changes = {entry_id: LocalChange(now) for entry_id in need_sync_local}
        self.remote_changes = need_sync_remote

        return True

    def set_local_change(self, entry_id: EntryID) -> bool:
        # Ignore local changes in read only mode
        if self.read_only:
            return

        now = timestamp()
        try:
            new_due_time = self.local_changes[entry_id].changed(now)
        except KeyError:
            local_change = LocalChange(now)
            self.local_changes[entry_id] = local_change
            new_due_time = local_change.due_time

        if new_due_time < self.due_time:
            self.due_time = new_due_time
            return True

        else:
            return False

    def set_remote_change(self, entry_id: EntryID) -> bool:
        self.remote_changes.add(entry_id)
        self.due_time = timestamp()
        return True

    async def tick(self) -> float:
        if not self.bootstrapped:
            if not await self._refresh_checkpoint():
                # Error, no sync possible for the moment
                return math.inf
            self.bootstrapped = True

        now = timestamp()

        if self.due_time > now:
            return self.due_time

        min_due_time = None

        # Remote changes sync have priority over local changes
        if self.remote_changes:
            entry_id = self.remote_changes.pop()
            try:
                await self._sync(entry_id)
            except FSWorkspaceNoReadAccess:
                # We've just lost the read access to the workspace.
                # This likely means a `sharing.updated` event we soon arrive
                # and destroy this sync context.
                # Until then just pretent nothing happened.
                min_due_time = now + MIN_WAIT
                self.remote_changes.add(entry_id)
            except FSWorkspaceNoWriteAccess:
                # We don't have write access and this entry contains local
                # modifications. Hence we can forget about this change given
                # it's `self.local_changes` role to keep track of local changes.
                pass
            except FSWorkspaceInMaintenance:
                # Not the right time for the sync, retry later
                min_due_time = now + MAINTENANCE_MIN_WAIT
                self.remote_changes.add(entry_id)

        elif self.local_changes:
            entry_id = next(
                (
                    entry_id
                    for entry_id, change_info in self.local_changes.items()
                    if change_info.due_time < now
                ),
                None,
            )
            if entry_id:
                del self.local_changes[entry_id]
                try:
                    await self._sync(entry_id)
                except (FSWorkspaceNoReadAccess, FSWorkspaceNoWriteAccess):
                    # We've just lost the write access to the workspace, and
                    # the corresponding `sharing.updated` event hasn't updated
                    # the `read_only` flag yet.
                    # We keep track of the change (given we may be given back
                    # the write access in the future) but pretent it just accured
                    # to avoid a busy sync loop until `read_only` flag is updated.
                    self.local_changes[entry_id] = LocalChange(now)
                except FSWorkspaceInMaintenance:
                    # Not the right time for the sync, retry later
                    min_due_time = now + MAINTENANCE_MIN_WAIT
                    self.local_changes[entry_id] = LocalChange(now)

                # This is where we plug our vacuuming routine
                # as it corresponds to a fresh synchronized state
                if not self.local_changes:
                    await self._get_local_storage().run_vacuum()

        # Re-compute due time
        if self.remote_changes:
            self.due_time = now
        elif self.local_changes:
            # TODO: index changes by due_time to avoid this O(n) operation
            self.due_time = min(change_info.due_time for change_info in self.local_changes.values())
        else:
            self.due_time = math.inf

        if min_due_time:
            self.due_time = max(self.due_time, min_due_time)

        return self.due_time


class WorkspaceSyncContext(SyncContext):
    def __init__(self, user_fs, id: EntryID):
        self.workspace = user_fs.get_workspace(id)
        read_only = self.workspace.get_workspace_entry().role == WorkspaceRole.READER
        super().__init__(user_fs, id, read_only=read_only)

    async def _sync(self, entry_id: EntryID):
        # No recursion here: only the manifest that has changed
        # (remotely or locally) should get synchronized
        await self.workspace.sync_by_id(entry_id, recursive=False)

    def _get_backend_cmds(self):
        return self.workspace.backend_cmds

    def _get_local_storage(self):
        return self.workspace.local_storage


class UserManifestSyncContext(SyncContext):
    async def _sync(self, entry_id: EntryID):
        assert entry_id == self.id
        await self.user_fs.sync()

    def _get_backend_cmds(self):
        return self.user_fs.backend_cmds

    def _get_local_storage(self):
        return self.user_fs.storage


class SyncContextStore:
    """
    Hold the `SyncContext` and lazily create them on need (typically
    when a newly created workspace is modified for the first time)
    """

    def __init__(self, user_fs):
        self.user_fs = user_fs
        self._ctxs = {}

    def iter(self):
        return self._ctxs.copy().values()

    def get(self, entry_id):
        try:
            return self._ctxs[entry_id]
        except KeyError:
            if entry_id == self.user_fs.user_manifest_id:
                ctx = UserManifestSyncContext(self.user_fs, entry_id)
            else:
                try:
                    ctx = WorkspaceSyncContext(self.user_fs, entry_id)
                except FSWorkspaceNotFoundError:
                    # It's possible the workspace is not yet available
                    # (this can happen when a workspace is just shared with
                    # us, hence we receive vlob updated events before having
                    # added the workspace entry to our user manifest)
                    return None
            self._ctxs[entry_id] = ctx
            return ctx

    def discard(self, entry_id):
        self._ctxs.pop(entry_id, None)


async def monitor_sync(user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    ctxs = SyncContextStore(user_fs)
    early_wakeup = trio.Event()

    def _on_entry_updated(event, id, workspace_id=None):
        if workspace_id is None:
            # User manifest
            assert id == user_fs.user_manifest_id
            ctx = ctxs.get(id)
        else:
            ctx = ctxs.get(workspace_id)
        if ctx and ctx.set_local_change(id):
            early_wakeup.set()

    def _on_realm_vlobs_updated(sender, realm_id, checkpoint, src_id, src_version):
        ctx = ctxs.get(realm_id)
        if ctx and ctx.set_remote_change(src_id):
            early_wakeup.set()

    def _on_sharing_updated(sender, new_entry, previous_entry):
        # If role have changed we have to reset the sync context given
        # behavior could have changed a lot (e.g. switching to/from read-only)
        ctxs.discard(new_entry.id)
        if new_entry.role is not None:
            ctx = ctxs.get(new_entry.id)
            if ctx:
                early_wakeup.set()

    async def _ctx_tick(ctx):
        try:
            return await ctx.tick()
        except FSBackendOfflineError:
            raise
        except Exception:
            logger.exception("Sync monitor has crashed", workspace_id=ctx.id)
            # Reset sync context which is now in an undefined state
            ctxs.discard(ctx.id)
            ctx = ctxs.get(ctx.id)
            if ctx:
                # Add small cooldown just to be sure not end up in a crazy busy error loop
                ctx.due_time += TICK_CRASH_COOLDOWN
                return ctx.due_time
            else:
                return math.inf

    with event_bus.connect_in_context(
        ("fs.entry.updated", _on_entry_updated),
        ("backend.realm.vlobs_updated", _on_realm_vlobs_updated),
        ("sharing.updated", _on_sharing_updated),
    ):
        wait_times = []
        wait_times.append(await ctxs.get(user_fs.user_manifest_id).tick())
        user_manifest = user_fs.get_user_manifest()
        for entry in user_manifest.workspaces:
            if entry.role is not None:
                ctx = ctxs.get(entry.id)
                if ctx:
                    wait_times.append(await _ctx_tick(ctx))

        task_status.started()
        while True:
            with trio.move_on_at(min(wait_times)):
                await early_wakeup.wait()
                early_wakeup = trio.Event()
            wait_times.clear()
            await freeze_sync_monitor_mockpoint()
            for ctx in ctxs.iter():
                wait_times.append(await _ctx_tick(ctx))
