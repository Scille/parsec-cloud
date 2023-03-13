# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Callable

import pytest
import trio

from parsec._parsec import CoreEvent, DateTime, SecretKey
from parsec.api.data import EntryName, PingMessageContent
from parsec.api.protocol import UserID
from parsec.core.backend_connection import BackendConnStatus
from parsec.core.logged_core import LoggedCore, UserFS
from parsec.core.types import WorkspaceEntry, WorkspaceRole
from tests.common import create_shared_workspace, freeze_time


@pytest.mark.trio
async def test_monitors_idle(running_backend, alice_core):
    assert alice_core.are_monitors_idle()

    # Force wakeup of the message monitor
    alice_core.event_bus.send(CoreEvent.BACKEND_MESSAGE_RECEIVED, index=42)
    assert not alice_core.are_monitors_idle()
    await alice_core.wait_idle_monitors()
    assert alice_core.are_monitors_idle()


async def _send_msg(backend, author, recipient, ping="ping"):
    now = DateTime.now()
    message = PingMessageContent(author=author.device_id, timestamp=now, ping=ping)
    ciphered_message = message.dump_sign_and_encrypt_for(
        author_signkey=author.signing_key, recipient_pubkey=recipient.public_key
    )
    await backend.message.send(
        organization_id=recipient.organization_id,
        sender=author.device_id,
        recipient=recipient.user_id,
        timestamp=now,
        body=ciphered_message,
    )


@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
async def test_process_while_offline(
    running_backend, alice_core, bob_user_fs, alice, bob, monkeypatch
):
    # TODO: use global time provider instead of disabling ballpark checks
    monkeypatch.setattr("parsec.utils.BALLPARK_ALWAYS_OK", True)

    assert alice_core.backend_status == BackendConnStatus.READY

    with running_backend.offline():
        with alice_core.event_bus.listen() as spy:
            # Force wakeup of the message monitor
            alice_core.event_bus.send(CoreEvent.BACKEND_MESSAGE_RECEIVED, index=42)

            assert not alice_core.are_monitors_idle()

            with trio.fail_after(1.0):
                await spy.wait(
                    CoreEvent.BACKEND_CONNECTION_CHANGED,
                    {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
                )
                await alice_core.wait_idle_monitors()
            assert alice_core.backend_status == BackendConnStatus.LOST

        # Send message while alice is offline
        await _send_msg(
            backend=running_backend.backend, author=bob, recipient=alice, ping="hello from Bob !"
        )

    with alice_core.event_bus.listen() as spy:
        # Alice is back online, should retrieve Bob's message fine
        alice.time_provider.mock_time(speed=100.0)
        with trio.fail_after(1.0):
            await spy.wait(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.READY, "status_exc": None},
            )
            await alice_core.wait_idle_monitors()
        alice.time_provider.mock_time(speed=1.0)
        assert alice_core.backend_status == BackendConnStatus.READY
        spy.assert_event_occurred(CoreEvent.MESSAGE_PINGED, {"ping": "hello from Bob !"})


@pytest.mark.trio
async def test_new_sharing_trigger_event(
    alice_user_fs: UserFS,
    bob_core: LoggedCore,
    running_backend,
):
    KEY = SecretKey.generate()
    # First, create a folder and sync it on backend
    with freeze_time("2000-01-01", devices=[alice_user_fs.device], freeze_datetime=True):
        wid = await alice_user_fs.workspace_create(EntryName("foo"))
    workspace = alice_user_fs.get_workspace(wid)
    with freeze_time("2000-01-02", devices=[alice_user_fs.device], freeze_datetime=True):
        await workspace.sync()

    # Now we can share this workspace with Bob
    with bob_core.event_bus.listen() as spy:
        with freeze_time("2000-01-03", devices=[alice_user_fs.device], freeze_datetime=True):
            await alice_user_fs.workspace_share(
                wid, recipient=UserID("bob"), role=WorkspaceRole.MANAGER
            )

        def _update_event(event):
            if event.event == CoreEvent.SHARING_UPDATED:
                event.kwargs["new_entry"] = event.kwargs["new_entry"].evolve(
                    key=KEY, role_cached_on=DateTime(2000, 1, 1)
                )
            return event

        # Bob should get a notification
        await spy.wait_with_timeout(
            CoreEvent.SHARING_UPDATED,
            {
                "new_entry": WorkspaceEntry(
                    name=EntryName("foo"),
                    id=wid,
                    key=KEY,
                    encryption_revision=1,
                    encrypted_on=DateTime(2000, 1, 1),
                    role_cached_on=DateTime(2000, 1, 1),
                    role=WorkspaceRole.MANAGER,
                ),
                "previous_entry": None,
            },
            update_event_func=_update_event,
        )


@pytest.mark.trio
async def test_revoke_sharing_trigger_event(
    alice_user_fs: UserFS, bob_core: LoggedCore, running_backend
):
    KEY = SecretKey.generate()

    def _update_event(event):
        if event.event == CoreEvent.SHARING_UPDATED:
            event.kwargs["new_entry"] = event.kwargs["new_entry"].evolve(
                key=KEY, role_cached_on=DateTime(2000, 1, 2)
            )
            event.kwargs["previous_entry"] = event.kwargs["previous_entry"].evolve(
                key=KEY, role_cached_on=DateTime(2000, 1, 2)
            )
        return event

    with freeze_time("2000-01-02"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, bob_core)

    with bob_core.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await alice_user_fs.workspace_share(wid, recipient=UserID("bob"), role=None)

        # Each workspace participant should get the message
        await spy.wait_with_timeout(
            CoreEvent.SHARING_UPDATED,
            {
                "new_entry": WorkspaceEntry(
                    name=EntryName("w"),
                    id=wid,
                    key=KEY,
                    encryption_revision=1,
                    encrypted_on=DateTime(2000, 1, 2),
                    role_cached_on=DateTime(2000, 1, 2),
                    role=None,
                ),
                "previous_entry": WorkspaceEntry(
                    name=EntryName("w"),
                    id=wid,
                    key=KEY,
                    encryption_revision=1,
                    encrypted_on=DateTime(2000, 1, 2),
                    role_cached_on=DateTime(2000, 1, 2),
                    role=WorkspaceRole.MANAGER,
                ),
            },
            update_event_func=_update_event,
        )


@pytest.mark.trio
async def test_new_reencryption_trigger_event(
    alice_core: LoggedCore,
    bob_core: LoggedCore,
    running_backend,
    global_core_monitors_freeze: Callable[[bool], None],
):
    KEY = SecretKey.generate()

    def _update_event(event):
        if event.event == CoreEvent.SHARING_UPDATED:
            event.kwargs["new_entry"] = event.kwargs["new_entry"].evolve(
                key=KEY, role_cached_on=DateTime(2000, 1, 3)
            )
            event.kwargs["previous_entry"] = event.kwargs["previous_entry"].evolve(
                key=KEY, role_cached_on=DateTime(2000, 1, 2)
            )
        return event

    global_core_monitors_freeze(False)
    with freeze_time("2000-01-02", devices=[alice_core.device], freeze_datetime=True):
        wid = await create_shared_workspace(EntryName("w"), alice_core, bob_core)

    global_core_monitors_freeze(True)
    with alice_core.event_bus.listen() as a_spy, bob_core.event_bus.listen() as b_spy:
        with freeze_time("2000-01-03", devices=[alice_core.device], freeze_datetime=True):
            await alice_core.user_fs.workspace_start_reencryption(wid)

        global_core_monitors_freeze(False)
        # Each workspace participant should get the message
        await a_spy.wait_with_timeout(
            CoreEvent.SHARING_UPDATED,
            {
                "new_entry": WorkspaceEntry(
                    name=EntryName("w"),
                    id=wid,
                    key=KEY,
                    encryption_revision=2,
                    encrypted_on=DateTime(2000, 1, 3),
                    role_cached_on=DateTime(2000, 1, 3),
                    role=WorkspaceRole.OWNER,
                ),
                "previous_entry": WorkspaceEntry(
                    name=EntryName("w"),
                    id=wid,
                    key=KEY,
                    encryption_revision=1,
                    encrypted_on=DateTime(2000, 1, 2),
                    role_cached_on=DateTime(2000, 1, 2),
                    role=WorkspaceRole.OWNER,
                ),
            },
            update_event_func=_update_event,
        )
        await b_spy.wait_with_timeout(
            CoreEvent.SHARING_UPDATED,
            {
                "new_entry": WorkspaceEntry(
                    name=EntryName("w"),
                    id=wid,
                    key=KEY,
                    encryption_revision=2,
                    encrypted_on=DateTime(2000, 1, 3),
                    role_cached_on=DateTime(2000, 1, 3),
                    role=WorkspaceRole.MANAGER,
                ),
                "previous_entry": WorkspaceEntry(
                    name=EntryName("w"),
                    id=wid,
                    key=KEY,
                    encryption_revision=1,
                    encrypted_on=DateTime(2000, 1, 2),
                    role_cached_on=DateTime(2000, 1, 2),
                    role=WorkspaceRole.MANAGER,
                ),
            },
            update_event_func=_update_event,
        )
