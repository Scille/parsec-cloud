# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import uuid4
from pendulum import Pendulum
from unittest.mock import ANY

from parsec.core.types import (
    WorkspaceEntry,
    WorkspaceRole,
    LocalUserManifest,
    LocalWorkspaceManifest,
)
from parsec.core.fs import FSWorkspaceNotFoundError, FSBackendOfflineError

from tests.common import freeze_time


@pytest.mark.trio
async def test_get_manifest(alice_user_fs):
    um = await alice_user_fs.get_user_manifest()
    assert um.base_version == 1
    assert not um.need_sync
    assert um.workspaces == ()


@pytest.mark.trio
async def test_create_workspace(alice_user_fs, alice):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    um = await alice_user_fs.get_user_manifest()
    expected_um = LocalUserManifest(
        author=alice.device_id,
        base_version=1,
        need_sync=True,
        is_placeholder=False,
        created=Pendulum(2000, 1, 1),
        updated=Pendulum(2000, 1, 2),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name="w1",
                id=wid,
                key=ANY,
                encryption_revision=1,
                encrypted_on=Pendulum(2000, 1, 2),
                role_cached_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    assert um == expected_um

    w_manifest = alice_user_fs.local_storage.get_manifest(wid)
    expected_w_manifest = LocalWorkspaceManifest.make_placeholder(
        author=alice.device_id, parent_id=wid, created=Pendulum(2000, 1, 2)
    )
    assert w_manifest == expected_w_manifest


@pytest.mark.trio
async def test_create_workspace_offline(alice_user_fs, alice, running_backend):
    with running_backend.offline():
        await test_create_workspace(alice_user_fs, alice)


@pytest.mark.trio
async def test_rename_workspace(alice_user_fs, alice):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")

    with freeze_time("2000-01-03"):
        await alice_user_fs.workspace_rename(wid, "w2")

    um = await alice_user_fs.get_user_manifest()
    expected_um = LocalUserManifest(
        author=alice.device_id,
        base_version=1,
        need_sync=True,
        is_placeholder=False,
        created=Pendulum(2000, 1, 1),
        updated=Pendulum(2000, 1, 3),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name="w2",
                id=wid,
                key=ANY,
                encryption_revision=1,
                encrypted_on=Pendulum(2000, 1, 2),
                role_cached_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    assert um == expected_um


@pytest.mark.trio
async def test_rename_workspace_offline(alice_user_fs, alice, running_backend):
    with running_backend.offline():
        await test_rename_workspace(alice_user_fs, alice)


@pytest.mark.trio
async def test_rename_unknown_workspace(alice_user_fs):
    dummy_id = uuid4()
    with pytest.raises(FSWorkspaceNotFoundError):
        await alice_user_fs.workspace_rename(dummy_id, "whatever")


@pytest.mark.trio
async def test_create_workspace_same_name(alice_user_fs):
    with freeze_time("2000-01-02"):
        w1id = await alice_user_fs.workspace_create("w")

    with freeze_time("2000-01-03"):
        w2id = await alice_user_fs.workspace_create("w")

    um = await alice_user_fs.get_user_manifest()
    assert um.updated == Pendulum(2000, 1, 3)
    assert len(um.workspaces) == 2
    assert [(x.id, x.name) for x in um.workspaces] == [(w1id, "w"), (w2id, "w")]


@pytest.mark.trio
async def test_sync_offline(alice_user_fs, alice):
    with freeze_time("2000-01-02"):
        await alice_user_fs.workspace_create("w1")

    with pytest.raises(FSBackendOfflineError):
        await alice_user_fs.sync()


@pytest.mark.trio
async def test_sync(running_backend, alice2_user_fs, alice2):
    with freeze_time("2000-01-02"):
        wid = await alice2_user_fs.workspace_create("w1")

    await alice2_user_fs.sync()

    um = await alice2_user_fs.get_user_manifest()
    expected_um = LocalUserManifest(
        author=alice2.device_id,
        base_version=2,
        need_sync=False,
        is_placeholder=False,
        created=Pendulum(2000, 1, 1),
        updated=Pendulum(2000, 1, 2),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name="w1",
                id=wid,
                key=ANY,
                encryption_revision=1,
                encrypted_on=Pendulum(2000, 1, 2),
                role_cached_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    assert um == expected_um


@pytest.mark.trio
async def test_sync_under_concurrency(
    running_backend, alice_user_fs, alice2_user_fs, alice, alice2
):
    with freeze_time("2000-01-02"):
        waid = await alice_user_fs.workspace_create("wa")

    with freeze_time("2000-01-03"):
        wa2id = await alice2_user_fs.workspace_create("wa2")

    await alice_user_fs.sync()
    await alice2_user_fs.sync()
    # Fetch back alice2's changes
    await alice_user_fs.sync()

    um = await alice_user_fs.get_user_manifest()
    um2 = await alice2_user_fs.get_user_manifest()

    expected_um = LocalUserManifest(
        author=alice.device_id,
        base_version=3,
        need_sync=False,
        is_placeholder=False,
        created=Pendulum(2000, 1, 1),
        updated=Pendulum(2000, 1, 3),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name="wa",
                id=waid,
                key=ANY,
                encryption_revision=1,
                encrypted_on=Pendulum(2000, 1, 2),
                role_cached_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
            WorkspaceEntry(
                name="wa2",
                id=wa2id,
                key=ANY,
                encryption_revision=1,
                encrypted_on=Pendulum(2000, 1, 3),
                role_cached_on=Pendulum(2000, 1, 3),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    assert um == expected_um
    assert um2 == expected_um.evolve(author=alice2.device_id)


@pytest.mark.trio
@pytest.mark.parametrize("with_workspace", (False, True))
async def test_sync_placeholder(
    running_backend,
    backend_data_binder,
    local_device_factory,
    local_storage_factory,
    user_fs_factory,
    with_workspace,
):
    device = local_device_factory()
    local_storage = await local_storage_factory(device, user_manifest_in_v0=True)
    await backend_data_binder.bind_device(device, initial_user_manifest_in_v0=True)

    async with user_fs_factory(device, local_storage=local_storage) as user_fs:
        with freeze_time("2000-01-01"):
            # User manifest should be lazily created on first access
            um = await user_fs.get_user_manifest()

        expected_um = LocalUserManifest(
            author=device.device_id,
            base_version=0,
            need_sync=True,
            is_placeholder=True,
            created=Pendulum(2000, 1, 1),
            updated=Pendulum(2000, 1, 1),
            last_processed_message=0,
            workspaces=(),
        )
        assert um == expected_um

        if with_workspace:
            with freeze_time("2000-01-02"):
                wid = await user_fs.workspace_create("w1")
            um = await user_fs.get_user_manifest()
            expected_um = expected_um.evolve(
                updated=Pendulum(2000, 1, 2),
                workspaces=(
                    WorkspaceEntry(
                        name="w1",
                        id=wid,
                        key=ANY,
                        encryption_revision=1,
                        encrypted_on=Pendulum(2000, 1, 2),
                        role_cached_on=Pendulum(2000, 1, 2),
                        role=WorkspaceRole.OWNER,
                    ),
                ),
            )
            assert um == expected_um

        await user_fs.sync()
        um = await user_fs.get_user_manifest()
        expected_um = expected_um.evolve(base_version=1, need_sync=False, is_placeholder=False)
        assert um == expected_um


@pytest.mark.trio
@pytest.mark.parametrize("dev2_has_changes", (False, True))
async def test_concurrent_sync_placeholder(
    running_backend,
    backend_data_binder,
    local_device_factory,
    local_storage_factory,
    user_fs_factory,
    dev2_has_changes,
):
    device1 = local_device_factory("a@1")
    local_storage1 = await local_storage_factory(device1, user_manifest_in_v0=True)
    await backend_data_binder.bind_device(device1, initial_user_manifest_in_v0=True)

    device2 = local_device_factory("a@2")
    local_storage2 = await local_storage_factory(device2, user_manifest_in_v0=True)
    await backend_data_binder.bind_device(device2, initial_user_manifest_in_v0=True)

    async with user_fs_factory(device1, local_storage=local_storage1) as user_fs1, user_fs_factory(
        device2, local_storage=local_storage2
    ) as user_fs2:
        with freeze_time("2000-01-01"):
            w1id = await user_fs1.workspace_create("w1")
        if dev2_has_changes:
            with freeze_time("2000-01-02"):
                w2id = await user_fs2.workspace_create("w2")

        with freeze_time("2000-01-03"):
            await user_fs1.sync()
        with freeze_time("2000-01-04"):
            await user_fs2.sync()
        if dev2_has_changes:
            with freeze_time("2000-01-03"):
                await user_fs1.sync()

        um1 = await user_fs1.get_user_manifest()
        um2 = await user_fs2.get_user_manifest()
        if dev2_has_changes:
            expected_um = LocalUserManifest(
                author=device1.device_id,
                base_version=2,
                need_sync=False,
                is_placeholder=False,
                created=Pendulum(2000, 1, 1),
                updated=Pendulum(2000, 1, 2),
                last_processed_message=0,
                workspaces=(
                    WorkspaceEntry(
                        name="w1",
                        id=w1id,
                        key=ANY,
                        encryption_revision=1,
                        encrypted_on=Pendulum(2000, 1, 1),
                        role_cached_on=Pendulum(2000, 1, 1),
                        role=WorkspaceRole.OWNER,
                    ),
                    WorkspaceEntry(
                        name="w2",
                        id=w2id,
                        key=ANY,
                        encryption_revision=1,
                        encrypted_on=Pendulum(2000, 1, 2),
                        role_cached_on=Pendulum(2000, 1, 2),
                        role=WorkspaceRole.OWNER,
                    ),
                ),
            )
        else:
            expected_um = LocalUserManifest(
                author=device1.device_id,
                base_version=1,
                need_sync=False,
                is_placeholder=False,
                created=Pendulum(2000, 1, 1),
                updated=Pendulum(2000, 1, 1),
                last_processed_message=0,
                workspaces=(
                    WorkspaceEntry(
                        name="w1",
                        id=w1id,
                        key=ANY,
                        encryption_revision=1,
                        encrypted_on=Pendulum(2000, 1, 1),
                        role_cached_on=Pendulum(2000, 1, 1),
                        role=WorkspaceRole.OWNER,
                    ),
                ),
            )

        assert um1 == expected_um
        assert um2 == expected_um.evolve(author=device2.device_id)


@pytest.mark.trio
async def test_sync_not_needed(running_backend, alice_user_fs, alice2_user_fs, alice, alice2):
    um = await alice_user_fs.get_user_manifest()
    await alice_user_fs.sync()
    um = await alice_user_fs.get_user_manifest()

    assert um == um


@pytest.mark.trio
async def test_sync_remote_changes(running_backend, alice_user_fs, alice2_user_fs, alice, alice2):
    # Alice 2 update the user manifest
    with freeze_time("2000-01-02"):
        wid = await alice2_user_fs.workspace_create("wa")
    await alice2_user_fs.sync()

    # Alice retreive the changes
    um = await alice_user_fs.get_user_manifest()
    await alice_user_fs.sync()

    um = await alice_user_fs.get_user_manifest()
    um2 = await alice2_user_fs.get_user_manifest()

    expected_um = LocalUserManifest(
        author=alice.device_id,
        base_version=2,
        need_sync=False,
        is_placeholder=False,
        created=Pendulum(2000, 1, 1),
        updated=Pendulum(2000, 1, 2),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name="wa",
                id=wid,
                key=ANY,
                encryption_revision=1,
                encrypted_on=Pendulum(2000, 1, 2),
                role_cached_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )

    assert um == expected_um
    assert um2 == expected_um.evolve(author=alice2.device_id)
