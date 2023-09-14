# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import DateTime, RealmArchivingConfiguration
from parsec.api.data import EntryName
from parsec.core.fs import (
    FSNoAccessError,
    FsPath,
    FSReadOnlyError,
    FSWorkspaceArchivingNotAllowedError,
    FSWorkspaceArchivingPeriodTooShort,
    FSWorkspaceRealmDeleted,
    UserFS,
    WorkspaceFS,
)
from tests.common import create_shared_workspace


@pytest.fixture
@pytest.mark.trio
async def shared_workspaces(alice_user_fs: UserFS, bob_user_fs: UserFS, running_backend):
    wid = await create_shared_workspace(EntryName("w"), alice_user_fs, bob_user_fs)
    alice_workspace = alice_user_fs.get_workspace(wid)
    bob_workspace = bob_user_fs.get_workspace(wid)
    return alice_workspace, bob_workspace


@pytest.fixture
def alice_workspace(shared_workspaces: tuple[WorkspaceFS, WorkspaceFS]) -> WorkspaceFS:
    alice_workspace, _ = shared_workspaces
    return alice_workspace


@pytest.fixture
def bob_workspace(shared_workspaces: tuple[WorkspaceFS, WorkspaceFS]) -> WorkspaceFS:
    _, bob_workspace = shared_workspaces
    return bob_workspace


@pytest.mark.trio
@pytest.mark.parametrize("configuration", ["available", "archived", "deletion_planned", "deleted"])
async def test_configure_archiving(
    alice_workspace: WorkspaceFS,
    bob_workspace: WorkspaceFS,
    alice_user_fs: UserFS,
    bob_user_fs: UserFS,
    configuration: str,
    allow_instant_deletion,
):
    now = DateTime.now()
    if configuration == "available":
        config = RealmArchivingConfiguration.available()
    elif configuration == "archived":
        config = RealmArchivingConfiguration.archived()
    elif configuration == "deletion_planned":
        config = RealmArchivingConfiguration.deletion_planned(now.add(days=31))
    elif configuration == "deleted":
        config = RealmArchivingConfiguration.deletion_planned(now)
    else:
        assert False

    await alice_workspace.configure_archiving(config, now=now)

    # Force an archiving status update
    alice_next_deletion = await alice_user_fs.update_archiving_status()
    bob_next_deletion = await bob_user_fs.update_archiving_status()
    if configuration in ["available", "archived", "deleted"]:
        assert alice_next_deletion is None
        assert bob_next_deletion is None
    elif configuration == "deletion_planned":
        assert alice_next_deletion == config.deletion_date
        assert bob_next_deletion == config.deletion_date
    else:
        assert False

    for workspace in (alice_workspace, bob_workspace):
        path = FsPath("/")
        (
            current_config,
            current_configured_on,
            current_configured_by,
        ) = workspace.get_archiving_configuration()
        assert current_config == config
        assert current_configured_on == now
        assert current_configured_by == alice_workspace.device.device_id

        if configuration == "available":
            assert not workspace.is_archived()
            assert not workspace.is_deletion_planned()
            assert not workspace.is_deleted()
            assert not workspace.is_read_only()
            workspace.transactions.check_read_rights(path)
            workspace.transactions.check_write_rights(path)
        elif configuration == "archived":
            assert workspace.is_archived()
            assert not workspace.is_deletion_planned()
            assert not workspace.is_deleted()
            assert workspace.is_read_only()
            workspace.transactions.check_read_rights(path)
            with pytest.raises(FSReadOnlyError):
                workspace.transactions.check_write_rights(path)
        elif configuration == "deletion_planned":
            assert not workspace.is_archived()
            assert workspace.is_deletion_planned()
            assert not workspace.is_deleted()
            assert workspace.is_read_only()
            workspace.transactions.check_read_rights(path)
            with pytest.raises(FSReadOnlyError):
                workspace.transactions.check_write_rights(path)
        elif configuration == "deleted":
            assert not workspace.is_archived()
            assert workspace.is_deletion_planned()
            assert workspace.is_deleted()
            assert workspace.is_read_only()
            with pytest.raises(FSNoAccessError):
                workspace.transactions.check_read_rights(path)
            with pytest.raises(FSNoAccessError):
                workspace.transactions.check_write_rights(path)
        else:
            assert False

    # Test get_available_workspace_entries
    for user_fs in (alice_user_fs, bob_user_fs):
        available_entries = user_fs.get_available_workspace_entries()
        if configuration == "deleted":
            assert available_entries == []
        elif configuration in ["available", "archived", "deletion_planned"]:
            (entry,) = available_entries
            assert entry.name == EntryName("w")
        else:
            assert False


@pytest.mark.trio
async def test_configure_archiving_not_owner(
    bob_workspace: WorkspaceFS,
):
    now = DateTime.now()
    config = RealmArchivingConfiguration.archived()
    with pytest.raises(FSWorkspaceArchivingNotAllowedError):
        await bob_workspace.configure_archiving(config, now=now)


@pytest.mark.trio
async def test_configure_archiving_already_deleted(
    alice_workspace: WorkspaceFS,
    alice_user_fs: UserFS,
    monkeypatch,
    allow_instant_deletion,
):
    now = DateTime.now()
    config = RealmArchivingConfiguration.deletion_planned(now)
    await alice_workspace.configure_archiving(config, now=now)

    with pytest.raises(FSWorkspaceRealmDeleted):
        await alice_workspace.configure_archiving(RealmArchivingConfiguration.available())

    # Force an archiving status update
    await alice_user_fs.update_archiving_status()

    with pytest.raises(FSWorkspaceRealmDeleted):
        await alice_workspace.configure_archiving(RealmArchivingConfiguration.available())


@pytest.mark.trio
async def test_configure_archiving_period_negative(
    alice_workspace: WorkspaceFS,
):
    now = DateTime.now()
    config = RealmArchivingConfiguration.deletion_planned(now.subtract(seconds=1))
    with pytest.raises(FSWorkspaceArchivingPeriodTooShort, match="negative"):
        await alice_workspace.configure_archiving(config, now=now)


@pytest.mark.trio
async def test_configure_archiving_period_too_short(
    alice_workspace: WorkspaceFS,
):
    now = DateTime.now()
    config = RealmArchivingConfiguration.deletion_planned(now.add(days=1))
    with pytest.raises(FSWorkspaceArchivingPeriodTooShort, match="too short"):
        await alice_workspace.configure_archiving(config, now=now)


@pytest.mark.trio
async def test_configure_archiving_require_greater_timestamp(
    alice_workspace: WorkspaceFS,
    alice_user_fs: UserFS,
    allow_instant_deletion,
):
    now = DateTime.now().subtract(minutes=1)
    config = RealmArchivingConfiguration.deletion_planned(now)
    await alice_workspace.configure_archiving(config, now=now)
    await alice_user_fs.update_archiving_status()
    (
        current_config,
        current_configured_on,
        current_configured_by,
    ) = alice_workspace.get_archiving_configuration()
    updated_now = current_configured_on
    assert current_configured_by == alice_workspace.device.device_id
    assert now < updated_now <= DateTime.now()
    assert current_config == RealmArchivingConfiguration.deletion_planned(updated_now)
