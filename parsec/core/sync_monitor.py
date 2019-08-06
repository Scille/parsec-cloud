# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from trio.hazmat import current_clock

from parsec.core.fs import FSBackendOfflineError, FSWorkspaceNotFoundError


MIN_WAIT = 5
MAX_WAIT = 60


def timestamp():
    # Use time from trio clock to easily mock it
    return current_clock().current_time()


async def _refresh_checkpoint(backend_cmds, realm_id, local_storage, r_updated_entries):
    realm_checkpoint = local_storage.get_realm_checkpoint()

    new_checkpoint, changes = await backend_cmds.vlob_poll_changes(realm_id, realm_checkpoint)
    # TODO: handle exceptions
    # BackendCmdsInvalidRequest
    # BackendCmdsInvalidResponse
    # BackendNotAvailable
    # BackendCmdsBadResponse
    await local_storage.update_realm_checkpoint(new_checkpoint, changes.keys())
    need_sync_entries = local_storage.get_need_sync_entries()
    now = timestamp()
    # Use r_updated_entries to return result
    r_updated_entries.update({(realm_id, entry_id): (now, now) for entry_id in need_sync_entries})


async def _get_updated_entries(user_fs):
    updated_entries = {}
    user_manifest = user_fs.get_user_manifest()
    async with trio.open_nursery() as nursery:
        nursery.start_soon(
            _refresh_checkpoint,
            user_fs.backend_cmds,
            user_fs.user_manifest_id,
            user_fs.local_storage,
            updated_entries,
        )
        for entry in user_manifest.workspaces:
            wfs = user_fs.get_workspace(entry.id)
            nursery.start_soon(
                _refresh_checkpoint,
                user_fs.backend_cmds,
                entry.id,
                wfs.local_storage,
                updated_entries,
            )

    return updated_entries


def _sorted_entries(updated_entries):
    result = []
    for (wid, id), (first_updated, last_updated) in updated_entries.items():
        t = min(first_updated + MAX_WAIT, last_updated + MIN_WAIT)
        result.append((t, id, wid))
    return sorted(result)


async def _monitoring_tick(user_fs, updated_entries):
    # Loop over entries to synchronize
    for t, id, wid in _sorted_entries(updated_entries):
        if t > timestamp():
            return

        # Pop from entries before the synchronization
        updated_entries.pop((wid, id), None)

        # Perform the synchronization
        if id == user_fs.user_manifest_id:
            await user_fs.sync()
        elif wid is not None:
            try:
                workspace = user_fs.get_workspace(wid)
            except FSWorkspaceNotFoundError:
                # If the workspace is not present, nothing to do
                # (this can happen when a workspace is just shared with
                # us, hence we receive vlob updated events before having
                # added the workspace entry to our user manifest)
                continue

            # No recursion here: only the manifest that has changed
            # (remotely or locally) should get synchronized
            await workspace.sync_by_id(id, recursive=False)


async def _monitor_sync_online(user_fs, event_bus):
    new_event = trio.Event()

    def _on_entry_updated(event, workspace_id=None, id=None):
        assert id is not None
        try:
            first_updated, _ = updated_entries[workspace_id, id]
            last_updated = timestamp()
        except KeyError:
            first_updated = last_updated = timestamp()
        updated_entries[workspace_id, id] = first_updated, last_updated
        new_event.set()

    def _on_realm_vlobs_updated(sender, realm_id, checkpoint, src_id, src_version):
        updated_entries[realm_id, src_id] = timestamp() - MAX_WAIT, timestamp() - MAX_WAIT
        new_event.set()

    with event_bus.connect_in_context(
        ("fs.entry.updated", _on_entry_updated),
        ("backend.realm.vlobs_updated", _on_realm_vlobs_updated),
    ):

        event_bus.send("sync_monitor.reconnection_sync.started")
        updated_entries = await _get_updated_entries(user_fs)
        event_bus.send("sync_monitor.reconnection_sync.done")

        async with trio.open_nursery() as nursery:
            event_bus.send("sync_monitor.ready")
            while True:

                await new_event.wait()
                new_event.clear()

                await _monitoring_tick(user_fs, updated_entries)
                sorted_entries = _sorted_entries(updated_entries)
                if sorted_entries:
                    t, _, _ = sorted_entries[0]
                    delta = max(0, t - timestamp())

                    async def _wait():
                        await trio.sleep(delta)
                        new_event.set()

                    nursery.start_soon(_wait)


async def monitor_sync(user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    with event_bus.waiter_on("backend.online") as _on_backend_online, event_bus.waiter_on(
        "backend.offline"
    ) as _on_backend_offline:

        task_status.started()

        while True:
            await _on_backend_online.wait()
            _on_backend_online.clear()

            try:
                async with trio.open_nursery() as nursery:

                    async def _reset_on_offline():
                        await _on_backend_offline.wait()
                        _on_backend_offline.clear()
                        nursery.cancel_scope.cancel()

                    nursery.start_soon(_reset_on_offline)
                    await _monitor_sync_online(user_fs, event_bus)

            except FSBackendOfflineError:
                pass

            finally:
                event_bus.send("sync_monitor.disconnected")
