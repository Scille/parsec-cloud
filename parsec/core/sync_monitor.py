# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from trio.hazmat import current_clock
import math

from parsec.core.types import EntryID, WorkspaceRole
from parsec.core.fs import FSBackendOfflineError, FSWorkspaceNotFoundError, FSWorkspaceNoAccess
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendCmdsNotFound,
    BackendCmdsInMaintenance,
    BackendCmdsNotAllowed,
    BackendConnectionError,
)


MIN_WAIT = 5
MAX_WAIT = 60


def timestamp():
    # Use time from trio clock to easily mock it
    return current_clock().current_time()


class LocalChange:
    __slots__ = ("first_changed_on", "last_changed_on", "due_time")

    def __init__(self):
        self.first_changed_on = self.last_changed_on = timestamp()
        self.due_time = self._compute_due_time()

    def _compute_due_time(self):
        return min(self.last_changed_on + MIN_WAIT, self.first_changed_on + MAX_WAIT)

    def changed(self) -> bool:
        self.last_changed_on = timestamp()
        self.due_time = self._compute_due_time()
        return self.due_time <= self.last_changed_on


class SyncContext:
    def __init__(self, user_fs, id: EntryID):
        self.user_fs = user_fs
        self.id = id
        # Always start with read_only=False, if needed we will switch this
        # flag during our first sync raising an access error
        self.read_only = False
        self.local_changes = {}
        self.remote_changes = set()
        self.in_maintenance = False
        self.need_bootstrap = True
        self.due_time = timestamp()

    def _sync(self, entry_id: EntryID):
        raise NotImplementedError

    def _get_backend_cmds(self):
        raise NotImplementedError

    def _get_local_storage(self):
        raise NotImplementedError

    def __repr__(self):
        return f"{type(self).__name__}(id={self.id!r})"

    async def _bootstrap(self) -> bool:
        realm_checkpoint = self._get_local_storage().get_realm_checkpoint()
        try:
            new_checkpoint, changes = await self._get_backend_cmds().vlob_poll_changes(
                self.id, realm_checkpoint
            )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotFound:
            # Workspace not yet synchronized with backend
            new_checkpoint = 0
            changes = {}

        except BackendCmdsInMaintenance:
            self.in_maintenance = True
            return False

        except BackendCmdsNotAllowed:
            return False

        # Another backend error
        except BackendConnectionError:
            # TODO: logger.warning("Unexpected backend response during sync bootstrap", exc_info=exc)
            return False

        self._get_local_storage().update_realm_checkpoint(new_checkpoint, changes)
        need_sync_local, need_sync_remote = self._get_local_storage().get_need_sync_entries()

        for entry_id in need_sync_local:
            self.local_change(entry_id)

        for entry_id in need_sync_remote:
            self.remote_change(entry_id)

        return True

    def local_change(self, entry_id: EntryID) -> bool:
        try:
            new_due_time = self.local_changes[entry_id].changed()
        except KeyError:
            local_change = LocalChange()
            self.local_changes[entry_id] = local_change
            new_due_time = local_change.due_time

        if new_due_time < self.due_time:
            self.due_time = new_due_time
            return True

        else:
            return False

    def remote_change(self, entry_id: EntryID) -> bool:
        self.remote_changes.add(entry_id)
        self.due_time = timestamp()
        return True

    def maintenance_started(self):
        self.in_maintenance = True
        return False

    def maintenance_finished(self):
        self.in_maintenance = False
        self.need_bootstrap = True
        return True

    def role_updated(self, new_role):
        if new_role is None:
            return False

        elif new_role == WorkspaceRole.READER and not self.read_only:
            # Switch to read only, we will ignore local changes from now on
            self.read_only = True
            return False

        elif new_role != WorkspaceRole.READER and self.read_only:
            # Switch to read&write, previously ignored local changes should
            # be checked asap !
            self.read_only = False
            return True

        else:
            # Last possibility: switch between two read&write roles
            return False

    async def tick(self) -> float:
        if self.need_bootstrap:
            if await self._bootstrap():
                self.need_bootstrap = False
            else:
                # Bootstrap error, no sync possible for the moment
                return math.inf

        now = timestamp()
        if self.due_time > now:
            return self.due_time

        # Remote changes sync have priority over local changes
        if self.remote_changes:
            entry_id = self.remote_changes.pop()
            try:
                await self._sync(entry_id)
            except FSWorkspaceNoAccess:
                self.read_only = True
            # TODO: push back entry_id into remote_changes in case of exception ?
            # TODO: handle read only with entry_id modified by both local and remote

        elif self.local_changes and not self.read_only:
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
                except FSWorkspaceNoAccess:
                    # No allowed anymore to do sync
                    self.read_only = True
                # TODO: push back entry_id into remote_changes in case of exception ?

        # Re-compute due time
        if self.remote_changes:
            self.due_time = now
        elif self.local_changes and not self.read_only:
            # TODO: index changes by due_time to avoid this O(n) operation
            self.due_time = min(change_info.due_time for change_info in self.local_changes.values())
        else:
            self.due_time = math.inf

        return self.due_time


class WorkspaceSyncContext(SyncContext):
    def __init__(self, user_fs, id: EntryID):
        super().__init__(user_fs, id)
        self.workspace = self.user_fs.get_workspace(id)

    async def _sync(self, entry_id: EntryID):
        # TODO: handle exceptions
        await self.workspace.sync_by_id(entry_id, recursive=False)

    def _get_backend_cmds(self):
        return self.workspace.backend_cmds

    def _get_local_storage(self):
        return self.workspace.local_storage


class UserManifestSyncContext(SyncContext):
    async def _sync(self, entry_id: EntryID):
        assert entry_id == self.id
        # No recursion here: only the manifest that has changed
        # (remotely or locally) should get synchronized
        # TODO: handle exceptions
        await self.user_fs.sync()

    def _get_backend_cmds(self):
        return self.user_fs.backend_cmds

    def _get_local_storage(self):
        return self.user_fs.local_storage


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
                ctx = WorkspaceSyncContext(self.user_fs, entry_id)
            self._ctxs[entry_id] = ctx
            return ctx

    def discard(self, entry_id):
        self._ctxs.pop(entry_id, None)


async def _monitor_sync_online(user_fs, event_bus):
    ctxs = SyncContextStore(user_fs)
    early_wakeup = trio.Event()

    def _on_entry_updated(event, id, workspace_id=None):
        if workspace_id is None:
            # User manifest
            assert id == user_fs.user_manifest_id
            ctx = ctxs.get(id)
        else:
            ctx = ctxs.get(workspace_id)
        if ctx.local_change(id):
            early_wakeup.set()

    def _on_realm_vlobs_updated(sender, realm_id, checkpoint, src_id, src_version):
        try:
            ctx = ctxs.get(realm_id)
        except FSWorkspaceNotFoundError:
            # If the workspace is not present, nothing to do
            # (this can happen when a workspace is just shared with
            # us, hence we receive vlob updated events before having
            # added the workspace entry to our user manifest)
            return
        # TODO: checkpoint should be saved somewhere to keep track of the changes
        # in case of restart
        if ctx.remote_change(src_id):
            early_wakeup.set()

    def _on_maintenance_started(sender, realm_id, encryption_revision):
        try:
            ctx = ctxs.get(realm_id)
        except FSWorkspaceNotFoundError:
            return
        if ctx.maintenance_started():
            early_wakeup.set()

    def _on_maintenance_finished(sender, realm_id, encryption_revision):
        try:
            ctx = ctxs.get(realm_id)
        except FSWorkspaceNotFoundError:
            return
        if ctx.maintenance_finished():
            early_wakeup.set()

    def _on_sharing_updated(sender, new_entry, previous_entry):
        if new_entry.role is None:
            # No longer access to the workspace
            ctxs.discard(new_entry.id)
        else:
            try:
                ctx = ctxs.get(new_entry.id)
            except FSWorkspaceNotFoundError:
                return
            if ctx.role_updated(new_entry.role):
                early_wakeup.set()

    with event_bus.connect_in_context(
        ("fs.entry.updated", _on_entry_updated),
        ("backend.realm.vlobs_updated", _on_realm_vlobs_updated),
        ("backend.realm.maintenance_started", _on_maintenance_started),
        ("backend.realm.maintenance_finished", _on_maintenance_finished),
        ("sharing.updated", _on_sharing_updated),
    ):

        wait_times = []
        event_bus.send("sync_monitor.reconnection_sync.started")
        wait_times.append(await ctxs.get(user_fs.user_manifest_id).tick())
        user_manifest = user_fs.get_user_manifest()
        for entry in user_manifest.workspaces:
            if entry.role is not None:
                wait_times.append(await ctxs.get(entry.id).tick())
        event_bus.send("sync_monitor.reconnection_sync.done")

        event_bus.send("sync_monitor.ready")
        while True:
            with trio.move_on_after(min(wait_times)):
                await early_wakeup.wait()
                early_wakeup.clear()
            wait_times.clear()
            for ctx in ctxs.iter():
                wait_times.append(await ctx.tick())


async def monitor_sync(user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    cancel_scope = None

    def _on_backend_offline(event):
        if cancel_scope:
            cancel_scope.cancel()

    with event_bus.waiter_on(
        "backend.online"
    ) as backend_online_event, event_bus.connect_in_context(
        ("backend.offline", _on_backend_offline)
    ):

        task_status.started()

        while True:
            await backend_online_event.wait()
            backend_online_event.clear()

            try:
                with trio.CancelScope() as cancel_scope:
                    await _monitor_sync_online(user_fs, event_bus)

            except FSBackendOfflineError:
                pass

            finally:
                event_bus.send("sync_monitor.disconnected")
