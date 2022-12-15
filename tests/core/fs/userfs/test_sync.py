# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
    EntryID,
    EntryName,
    LocalDevice,
    LocalUserManifest,
    LocalWorkspaceManifest,
    SecretKey,
    UserManifest,
    WorkspaceEntry,
)
from parsec.core.fs import FSBackendOfflineError, FSWorkspaceNotFoundError, UserFS
from parsec.core.fs.remote_loader import MANIFEST_STAMP_AHEAD_US
from parsec.core.types import WorkspaceRole
from tests.common import freeze_time
from tests.common.backend import RunningBackend
from tests.common.binder import InitialUserManifestState

KEY = SecretKey.generate()


def _update_user_manifest_key(um):
    return um.evolve(
        base=um.base.evolve(workspaces=tuple(w.evolve(key=KEY) for w in um.base.workspaces)),
        workspaces=tuple(w.evolve(key=KEY) for w in um.workspaces),
    )


@pytest.mark.trio
async def test_get_manifest(alice_user_fs):
    um = alice_user_fs.get_user_manifest()
    assert um.base_version == 1
    assert not um.need_sync
    assert um.workspaces == ()


@pytest.mark.trio
async def test_create_workspace(
    initial_user_manifest_state: InitialUserManifestState, alice_user_fs: UserFS, alice: LocalDevice
):
    with freeze_time("2000-01-02", devices=[alice]):
        wid = await alice_user_fs.workspace_create(EntryName("w1"))
    um = alice_user_fs.get_user_manifest()
    expected_base_um = initial_user_manifest_state.get_user_manifest_v1_for_backend(alice)
    expected_um = LocalUserManifest(
        base=expected_base_um,
        need_sync=True,
        updated=DateTime(2000, 1, 2),
        last_processed_message=expected_base_um.last_processed_message,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("w1"),
                id=wid,
                key=KEY,
                encryption_revision=1,
                encrypted_on=DateTime(2000, 1, 2),
                role_cached_on=DateTime(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
        speculative=False,
    )
    um = _update_user_manifest_key(um)
    assert um == expected_um

    w_manifest = await alice_user_fs.get_workspace(wid).local_storage.get_manifest(wid)
    expected_w_manifest = LocalWorkspaceManifest.new_placeholder(
        alice.device_id, id=w_manifest.id, timestamp=DateTime(2000, 1, 2), speculative=False
    )
    assert w_manifest == expected_w_manifest


@pytest.mark.trio
async def test_create_workspace_offline(
    initial_user_manifest_state: InitialUserManifestState,
    alice_user_fs: UserFS,
    alice: LocalDevice,
    running_backend: RunningBackend,
):
    with running_backend.offline():
        await test_create_workspace(initial_user_manifest_state, alice_user_fs, alice)


@pytest.mark.trio
async def test_rename_workspace(initial_user_manifest_state, alice_user_fs, alice):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create(EntryName("w1"))

    with freeze_time("2000-01-03"):
        await alice_user_fs.workspace_rename(wid, EntryName("w2"))

    um = alice_user_fs.get_user_manifest()
    expected_base_um = initial_user_manifest_state.get_user_manifest_v1_for_backend(alice)
    expected_um = LocalUserManifest(
        base=expected_base_um,
        need_sync=True,
        updated=DateTime(2000, 1, 3),
        last_processed_message=expected_base_um.last_processed_message,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("w2"),
                id=wid,
                key=KEY,
                encryption_revision=1,
                encrypted_on=DateTime(2000, 1, 2),
                role_cached_on=DateTime(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
        speculative=False,
    )
    um = _update_user_manifest_key(um)
    assert um == expected_um


@pytest.mark.trio
async def test_rename_workspace_offline(
    initial_user_manifest_state, alice_user_fs, alice, running_backend
):
    with running_backend.offline():
        await test_rename_workspace(initial_user_manifest_state, alice_user_fs, alice)


@pytest.mark.trio
async def test_rename_unknown_workspace(alice_user_fs):
    dummy_id = EntryID.new()
    with pytest.raises(FSWorkspaceNotFoundError):
        await alice_user_fs.workspace_rename(dummy_id, EntryName("whatever"))


@pytest.mark.trio
async def test_create_workspace_same_name(alice_user_fs):
    with freeze_time("2000-01-02"):
        w1id = await alice_user_fs.workspace_create(EntryName("w"))

    with freeze_time("2000-01-03"):
        w2id = await alice_user_fs.workspace_create(EntryName("w"))

    um = alice_user_fs.get_user_manifest()
    assert um.updated == DateTime(2000, 1, 3)
    assert len(um.workspaces) == 2
    assert sorted((x.id, x.name) for x in um.workspaces) == sorted(
        [(w1id, EntryName("w")), (w2id, EntryName("w"))]
    )


@pytest.mark.trio
async def test_sync_offline(running_backend, alice_user_fs):
    with freeze_time("2000-01-02"):
        await alice_user_fs.workspace_create(EntryName("w1"))

    with pytest.raises(FSBackendOfflineError):
        with running_backend.offline():
            await alice_user_fs.sync()


@pytest.mark.trio
async def test_sync(running_backend, alice2_user_fs, alice2):
    with freeze_time("2000-01-02"):
        wid = await alice2_user_fs.workspace_create(EntryName("w1"))

    with freeze_time("2000-01-03"):
        await alice2_user_fs.sync()

    um = alice2_user_fs.get_user_manifest()
    expected_base_um = UserManifest(
        author=alice2.device_id,
        timestamp=DateTime(2000, 1, 3),
        id=alice2.user_manifest_id,
        version=2,
        created=DateTime(2000, 1, 1),
        updated=DateTime(2000, 1, 2),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("w1"),
                id=wid,
                key=KEY,
                encryption_revision=1,
                encrypted_on=DateTime(2000, 1, 2),
                role_cached_on=DateTime(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    expected_um = LocalUserManifest.from_remote(expected_base_um)
    um = _update_user_manifest_key(um)
    assert um == expected_um


@pytest.mark.trio
async def test_sync_under_concurrency(
    running_backend, alice_user_fs, alice2_user_fs, alice, alice2
):
    with freeze_time("2000-01-02"):
        wksp_alice_id = await alice_user_fs.workspace_create(EntryName("wa"))

    with freeze_time("2000-01-03"):
        wksp_alice2_id = await alice2_user_fs.workspace_create(EntryName("wa2"))

    with freeze_time("2000-01-04"):
        await alice_user_fs.sync()
    with freeze_time("2000-01-05"):
        await alice2_user_fs.sync()
    # Fetch back alice2's changes
    with freeze_time("2000-01-06"):
        await alice_user_fs.sync()

    um = alice_user_fs.get_user_manifest()
    um2 = alice2_user_fs.get_user_manifest()

    expected_base_um = UserManifest(
        author=alice2.device_id,
        timestamp=DateTime(2000, 1, 5),
        id=alice2.user_manifest_id,
        version=3,
        created=DateTime(2000, 1, 1),
        updated=DateTime(2000, 1, 3),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("wa"),
                id=wksp_alice_id,
                key=KEY,
                encryption_revision=1,
                encrypted_on=DateTime(2000, 1, 2),
                role_cached_on=DateTime(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
            WorkspaceEntry(
                name=EntryName("wa2"),
                id=wksp_alice2_id,
                key=KEY,
                encryption_revision=1,
                encrypted_on=DateTime(2000, 1, 3),
                role_cached_on=DateTime(2000, 1, 3),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    expected_um = LocalUserManifest.from_remote(expected_base_um)

    um = _update_user_manifest_key(um)
    um2 = _update_user_manifest_key(um2)

    assert um == expected_um
    assert um2 == expected_um


@pytest.mark.trio
async def test_modify_user_manifest_placeholder(
    running_backend,
    backend_data_binder,
    local_device_factory,
    user_fs_factory,
    data_base_dir,
    initialize_local_user_manifest,
):
    device = local_device_factory()
    await backend_data_binder.bind_device(device, initial_user_manifest="not_synced")
    await initialize_local_user_manifest(
        data_base_dir, device, initial_user_manifest="non_speculative_v0"
    )

    async with user_fs_factory(device) as user_fs:
        um_v0 = user_fs.get_user_manifest()
        with freeze_time("2000-01-02"):
            wid = await user_fs.workspace_create(EntryName("w1"))
        um = user_fs.get_user_manifest()

        um = _update_user_manifest_key(um)

        expected_um = um_v0.evolve(
            updated=DateTime(2000, 1, 2),
            workspaces=(
                WorkspaceEntry(
                    name=EntryName("w1"),
                    id=wid,
                    key=KEY,
                    encryption_revision=1,
                    encrypted_on=DateTime(2000, 1, 2),
                    role_cached_on=DateTime(2000, 1, 2),
                    role=WorkspaceRole.OWNER,
                ),
            ),
        )
        assert um == expected_um

    # Make sure we can fetch back data from the database on user_fs restart
    async with user_fs_factory(device) as user_fs2:
        um2 = user_fs2.get_user_manifest()
        um2 = _update_user_manifest_key(um2)
        assert um2 == expected_um


@pytest.mark.trio
@pytest.mark.parametrize("with_workspace", (False, True))
@pytest.mark.parametrize("initial_user_manifest", ("non_speculative_v0", "speculative_v0"))
async def test_sync_placeholder(
    running_backend,
    backend_data_binder,
    local_device_factory,
    user_fs_factory,
    data_base_dir,
    initialize_local_user_manifest,
    with_workspace,
    initial_user_manifest,
):
    device = local_device_factory()
    await backend_data_binder.bind_device(device, initial_user_manifest="not_synced")
    await initialize_local_user_manifest(
        data_base_dir, device, initial_user_manifest=initial_user_manifest
    )

    async with user_fs_factory(device) as user_fs:
        um_v0 = user_fs.get_user_manifest()

        expected_um = LocalUserManifest.new_placeholder(
            device.device_id,
            id=device.user_manifest_id,
            timestamp=um_v0.created,
            speculative=(initial_user_manifest == "speculative_v0"),
        )
        assert um_v0 == expected_um

        if with_workspace:
            with freeze_time("2000-01-02"):
                wid = await user_fs.workspace_create(EntryName("w1"))
            um = user_fs.get_user_manifest()
            expected_um = um_v0.evolve(
                updated=DateTime(2000, 1, 2),
                workspaces=(
                    WorkspaceEntry(
                        name=EntryName("w1"),
                        id=wid,
                        key=KEY,
                        encryption_revision=1,
                        encrypted_on=DateTime(2000, 1, 2),
                        role_cached_on=DateTime(2000, 1, 2),
                        role=WorkspaceRole.OWNER,
                    ),
                ),
            )
            um = _update_user_manifest_key(um)
            assert um == expected_um

        with freeze_time("2000-01-02"):
            await user_fs.sync()
        um = user_fs.get_user_manifest()
        expected_base_um = UserManifest(
            author=device.device_id,
            # Add extra time due to the user realm being already created at 2000-01-02
            timestamp=DateTime(2000, 1, 2).add(microseconds=MANIFEST_STAMP_AHEAD_US),
            id=device.user_manifest_id,
            version=1,
            created=expected_um.created,
            updated=expected_um.updated,
            last_processed_message=0,
            workspaces=expected_um.workspaces,
        )
        expected_um = LocalUserManifest(
            base=expected_base_um,
            need_sync=False,
            updated=expected_um.updated,
            last_processed_message=0,
            workspaces=expected_base_um.workspaces,
            speculative=False,
        )

        um = _update_user_manifest_key(um)

        assert um.base == expected_base_um
        assert um == expected_um


@pytest.mark.trio
@pytest.mark.parametrize("dev2_has_changes", (False, True))
async def test_concurrent_sync_placeholder(
    running_backend,
    backend_data_binder,
    local_device_factory,
    user_fs_factory,
    data_base_dir,
    initialize_local_user_manifest,
    dev2_has_changes,
):
    device1 = local_device_factory("a@1")
    await backend_data_binder.bind_device(device1, initial_user_manifest="not_synced")
    await initialize_local_user_manifest(
        data_base_dir, device1, initial_user_manifest="non_speculative_v0"
    )

    device2 = local_device_factory("a@2")
    await backend_data_binder.bind_device(device2)
    await initialize_local_user_manifest(
        data_base_dir, device2, initial_user_manifest="speculative_v0"
    )

    async with user_fs_factory(device1) as user_fs1, user_fs_factory(device2) as user_fs2:
        # fs2's created value is different and will be overwritten when
        # merging synced manifest from fs1
        um_created_v0_fs1 = user_fs1.get_user_manifest().created

        with freeze_time("2000-01-01"):
            # Sync user manifests now to avoid extra milliseconds from restamping
            await user_fs1.sync()
            await user_fs2.sync()
            w1id = await user_fs1.workspace_create(EntryName("w1"))
        if dev2_has_changes:
            with freeze_time("2000-01-02"):
                w2id = await user_fs2.workspace_create(EntryName("w2"))

        with freeze_time("2000-01-03"):
            await user_fs1.sync()
        with freeze_time("2000-01-04"):
            await user_fs2.sync()
        if dev2_has_changes:
            with freeze_time("2000-01-05"):
                await user_fs1.sync()

        um1 = user_fs1.get_user_manifest()
        um2 = user_fs2.get_user_manifest()
        if dev2_has_changes:
            expected_base_um = UserManifest(
                author=device2.device_id,
                id=device2.user_manifest_id,
                timestamp=DateTime(2000, 1, 4),
                version=3,
                created=um_created_v0_fs1,
                updated=DateTime(2000, 1, 2),
                last_processed_message=0,
                workspaces=(
                    WorkspaceEntry(
                        name=EntryName("w1"),
                        id=w1id,
                        key=KEY,
                        encryption_revision=1,
                        encrypted_on=DateTime(2000, 1, 1),
                        role_cached_on=DateTime(2000, 1, 1),
                        role=WorkspaceRole.OWNER,
                    ),
                    WorkspaceEntry(
                        name=EntryName("w2"),
                        id=w2id,
                        key=KEY,
                        encryption_revision=1,
                        encrypted_on=DateTime(2000, 1, 2),
                        role_cached_on=DateTime(2000, 1, 2),
                        role=WorkspaceRole.OWNER,
                    ),
                ),
            )
            expected_um = LocalUserManifest(
                base=expected_base_um,
                need_sync=False,
                updated=DateTime(2000, 1, 2),
                last_processed_message=0,
                workspaces=expected_base_um.workspaces,
                speculative=False,
            )

        else:
            expected_base_um = UserManifest(
                author=device1.device_id,
                timestamp=DateTime(2000, 1, 3),
                id=device1.user_manifest_id,
                version=2,
                created=um_created_v0_fs1,
                updated=DateTime(2000, 1, 1),
                last_processed_message=0,
                workspaces=(
                    WorkspaceEntry(
                        name=EntryName("w1"),
                        id=w1id,
                        key=KEY,
                        encryption_revision=1,
                        encrypted_on=DateTime(2000, 1, 1),
                        role_cached_on=DateTime(2000, 1, 1),
                        role=WorkspaceRole.OWNER,
                    ),
                ),
            )
            expected_um = LocalUserManifest(
                base=expected_base_um,
                need_sync=False,
                updated=DateTime(2000, 1, 1),
                last_processed_message=0,
                workspaces=expected_base_um.workspaces,
                speculative=False,
            )
        um1 = _update_user_manifest_key(um1)
        um2 = _update_user_manifest_key(um2)

        assert um1 == expected_um
        assert um2 == expected_um


@pytest.mark.trio
async def test_sync_not_needed(running_backend, alice_user_fs, alice2_user_fs, alice, alice2):
    um1 = alice_user_fs.get_user_manifest()
    await alice_user_fs.sync()
    um2 = alice_user_fs.get_user_manifest()

    assert um1 == um2


@pytest.mark.trio
async def test_sync_remote_changes(running_backend, alice_user_fs, alice2_user_fs, alice, alice2):
    # Alice 2 update the user manifest
    with freeze_time("2000-01-02"):
        wid = await alice2_user_fs.workspace_create(EntryName("wa"))
    with freeze_time("2000-01-03"):
        await alice2_user_fs.sync()

    # Alice retrieve the changes
    um = alice_user_fs.get_user_manifest()
    await alice_user_fs.sync()

    um = alice_user_fs.get_user_manifest()
    um2 = alice2_user_fs.get_user_manifest()

    expected_base_um = UserManifest(
        author=alice2.device_id,
        timestamp=DateTime(2000, 1, 3),
        id=alice2.user_manifest_id,
        version=2,
        created=DateTime(2000, 1, 1),
        updated=DateTime(2000, 1, 2),
        last_processed_message=0,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("wa"),
                id=wid,
                key=KEY,
                encryption_revision=1,
                encrypted_on=DateTime(2000, 1, 2),
                role_cached_on=DateTime(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        ),
    )
    expected_um = LocalUserManifest(
        base=expected_base_um,
        need_sync=False,
        updated=DateTime(2000, 1, 2),
        last_processed_message=0,
        workspaces=expected_base_um.workspaces,
        speculative=False,
    )
    um = _update_user_manifest_key(um)
    um2 = _update_user_manifest_key(um2)

    assert um == expected_um
    assert um2 == expected_um
