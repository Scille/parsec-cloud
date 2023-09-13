# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import CoreEvent, EntryName, LocalDevice, RealmArchivingConfiguration
from parsec.core.logged_core import LoggedCore
from tests.common import RunningBackend


@pytest.mark.trio
async def test_archiving_monitor(
    running_backend: RunningBackend,
    alice: LocalDevice,
    alice2: LocalDevice,
    alice_core: LoggedCore,
    alice2_core_factory,
    monkeypatch,
    allow_instant_deletion,
):
    # Go fast
    alice.time_provider.mock_time(speed=1000.0)
    alice2.time_provider.mock_time(speed=1000.0)
    monkeypatch.setattr("parsec.utils.BALLPARK_ALWAYS_OK", True)

    # Create a workspace
    await alice_core.wait_idle_monitors()
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    workspace_alice = alice_core.user_fs.get_workspace(wid)
    await alice_core.user_fs.sync()
    await alice_core.wait_idle_monitors()

    # Check workspace state
    assert not workspace_alice.is_archived()
    assert not workspace_alice.is_deleted()
    assert not workspace_alice.is_deletion_planned()
    assert not workspace_alice.is_read_only()
    assert workspace_alice.get_archiving_configuration() == (
        RealmArchivingConfiguration.available(),
        None,
    )

    # Alice
    archived = RealmArchivingConfiguration.archived()
    configured_on = alice_core.device.time_provider.now()
    with alice_core.event_bus.waiter_on(CoreEvent.ARCHIVING_UPDATED) as waiter:
        await workspace_alice.configure_archiving(archived, now=configured_on)
        await waiter.wait()

    assert workspace_alice.is_archived()
    assert not workspace_alice.is_deleted()
    assert not workspace_alice.is_deletion_planned()
    assert workspace_alice.is_read_only()
    assert workspace_alice.get_archiving_configuration() == (archived, configured_on)
    # A new core get the right config
    alice_core_2: LoggedCore
    async with alice2_core_factory() as alice_core_2:
        alice2.time_provider.mock_time(speed=1000.0)
        await alice_core_2.wait_idle_monitors()
        workspace_alice2 = alice_core_2.user_fs.get_workspace(wid)
        assert workspace_alice2.is_archived()
        assert not workspace_alice2.is_deleted()
        assert not workspace_alice2.is_deletion_planned()
        assert workspace_alice2.is_read_only()
        assert workspace_alice2.get_archiving_configuration() == (archived, configured_on)

        # Change configuration to deletion planned
        configured_on = alice_core.device.time_provider.now()
        deletion_planned = RealmArchivingConfiguration.deletion_planned(configured_on.add(days=31))
        with alice_core.event_bus.waiter_on(CoreEvent.ARCHIVING_UPDATED) as waiter1:
            with alice_core_2.event_bus.waiter_on(CoreEvent.ARCHIVING_UPDATED) as waiter2:
                await workspace_alice2.configure_archiving(deletion_planned, now=configured_on)
                await waiter1.wait()
                await waiter2.wait()

        # Both workspace see the update
        for workspace in (workspace_alice, workspace_alice2):
            assert not workspace.is_archived()
            assert not workspace.is_deleted()
            assert workspace.is_deletion_planned()
            assert workspace.is_read_only()
            assert workspace.get_archiving_configuration() == (deletion_planned, configured_on)

    # Now delete the workspace in 15 minutes
    configured_on = alice_core.device.time_provider.now()
    deletion_planned = RealmArchivingConfiguration.deletion_planned(configured_on.add(minutes=15))
    with alice_core.event_bus.waiter_on(CoreEvent.ARCHIVING_UPDATED) as waiter:
        await workspace_alice.configure_archiving(deletion_planned, now=configured_on)
        await waiter.wait()

    # Alice 2 logs back
    alice_core_2: LoggedCore
    async with alice2_core_factory() as alice_core_2:
        alice2.time_provider.mock_time(speed=1000.0)
        await alice_core_2.wait_idle_monitors()
        workspace_alice2 = alice_core_2.user_fs.get_workspace(wid)

        # Both workspace see the update
        for workspace in (workspace_alice, workspace_alice2):
            assert not workspace.is_archived()
            assert not workspace.is_deleted()
            assert workspace.is_deletion_planned()
            assert workspace.is_read_only()
            assert workspace.get_archiving_configuration() == (deletion_planned, configured_on)

        # Now both alice wait for the deletion
        with alice_core.event_bus.waiter_on(CoreEvent.ARCHIVING_UPDATED) as waiter1:
            with alice_core_2.event_bus.waiter_on(CoreEvent.ARCHIVING_UPDATED) as waiter2:
                await waiter2.wait()
                await waiter1.wait()

        # Both workspace see the update
        for workspace in (workspace_alice, workspace_alice2):
            assert not workspace.is_archived()
            assert workspace.is_deleted()
            assert workspace.is_deletion_planned()
            assert workspace.is_read_only()
            assert workspace.get_archiving_configuration() == (deletion_planned, configured_on)

    # Alice 2 logs back
    alice_core_2: LoggedCore
    async with alice2_core_factory() as alice_core_2:
        alice2.time_provider.mock_time(speed=1000.0)
        await alice_core_2.wait_idle_monitors()
        workspace_alice2 = alice_core_2.user_fs.get_workspace(wid)

        # Both workspace see the update
        for workspace in (workspace_alice, workspace_alice2):
            assert not workspace.is_archived()
            assert workspace.is_deleted()
            assert workspace.is_deletion_planned()
            assert workspace.is_read_only()
            assert workspace.get_archiving_configuration() == (deletion_planned, configured_on)
