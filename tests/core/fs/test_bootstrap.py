# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from pendulum import datetime
from unittest.mock import ANY

from parsec.api.data import UserManifest, WorkspaceEntry
from parsec.core.types import LocalUserManifest, WorkspaceRole
from parsec.core.fs.storage.local_database import LocalDatabase
from parsec.core.fs.storage.manifest_storage import ManifestStorage

from tests.common import freeze_time


@pytest.mark.trio
async def test_user_manifest_access_before_creation(user_fs_factory, alice):
    with freeze_time("2000-01-01"):
        async with user_fs_factory(alice, initialize_in_v0=True) as user_fs:
            with freeze_time("2000-01-02"):
                user_manifest = user_fs.get_user_manifest()

    assert user_manifest.to_stats() == {
        "id": alice.user_manifest_id,
        "base_version": 0,
        "created": datetime(2000, 1, 1),
        "updated": datetime(2000, 1, 1),
        "is_placeholder": True,
        "need_sync": True,
    }


@pytest.mark.trio
async def test_workspace_manifest_access_before_creation(user_fs_factory, alice):
    with freeze_time("2000-01-01"):
        async with user_fs_factory(alice) as user_fs:
            wksp_id = await user_fs.workspace_create("wksp")
            # Remove the workspace manifest from the local storage
            wksp = user_fs.get_workspace(wksp_id)
            wksp_manifest_storage_db_path = wksp.local_storage.manifest_storage.path

    # Manually drop the workspace manifest from database
    async with LocalDatabase.run(path=wksp_manifest_storage_db_path) as localdb:
        async with ManifestStorage.run(
            device=alice, localdb=localdb, realm_id=wksp_id
        ) as manifest_storage:
            await manifest_storage.clear_manifest(wksp_id)

    # Now re-start the userfs (the same local storage will be used)
    with freeze_time("2000-01-02"):
        async with user_fs_factory(alice) as user_fs:
            with freeze_time("2000-01-03"):
                wksp = user_fs.get_workspace(wksp_id)
                root_stat = await wksp.path_info("/")

    assert root_stat == {
        "id": wksp_id,
        "base_version": 0,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 2),
        "is_placeholder": True,
        "need_sync": True,
        "type": "folder",
        "children": [],
        "confinement_point": None,
    }


@pytest.mark.trio
async def test_concurrent_devices_agree_on_user_manifest(
    backend_factory,
    backend_addr,
    backend_data_binder_factory,
    server_factory,
    user_fs_factory,
    coolorg,
    alice,
    alice2,
):
    # I call this "diagonal programming"...

    with freeze_time("2000-01-01"):
        async with user_fs_factory(alice, initialize_in_v0=True) as user_fs1:
            wksp1_id = await user_fs1.workspace_create("wksp1")

            with freeze_time("2000-01-02"):
                async with user_fs_factory(alice2, initialize_in_v0=True) as user_fs2:
                    wksp2_id = await user_fs2.workspace_create("wksp2")

                    with freeze_time("2000-01-03"):
                        # Only now the backend appear online, this is to ensure each
                        # userfs has created a user manifest in isolation
                        async with backend_factory(populated=False) as backend:

                            binder = backend_data_binder_factory(backend)
                            await binder.bind_organization(
                                coolorg, alice, initial_user_manifest_in_v0=True
                            )
                            await binder.bind_device(
                                alice2, certifier=alice, initial_user_manifest_in_v0=True
                            )

                            async with server_factory(backend.handle_client, backend_addr):

                                # Sync user_fs2 first to ensure created_on field is
                                # kept even if further syncs have an earlier value
                                with freeze_time("2000-01-04"):
                                    await user_fs2.sync()
                                with freeze_time("2000-01-05"):
                                    await user_fs1.sync()
                                with freeze_time("2000-01-06"):
                                    await user_fs2.sync()

                                # Now, both user fs should have the same view on data
                                expected_workspaces_entries = (
                                    WorkspaceEntry(
                                        name="wksp1",
                                        id=wksp1_id,
                                        key=ANY,
                                        encryption_revision=1,
                                        encrypted_on=datetime(2000, 1, 1),
                                        role_cached_on=datetime(2000, 1, 1),
                                        role=WorkspaceRole.OWNER,
                                    ),
                                    WorkspaceEntry(
                                        name="wksp2",
                                        id=wksp2_id,
                                        key=ANY,
                                        encryption_revision=1,
                                        encrypted_on=datetime(2000, 1, 2),
                                        role_cached_on=datetime(2000, 1, 2),
                                        role=WorkspaceRole.OWNER,
                                    ),
                                )
                                expected_user_manifest = LocalUserManifest(
                                    base=UserManifest(
                                        id=alice.user_manifest_id,
                                        version=2,
                                        timestamp=datetime(2000, 1, 5),
                                        author=alice.device_id,
                                        created=datetime(2000, 1, 2),
                                        updated=datetime(2000, 1, 2),
                                        last_processed_message=0,
                                        workspaces=expected_workspaces_entries,
                                    ),
                                    need_sync=False,
                                    updated=datetime(2000, 1, 2),
                                    last_processed_message=0,
                                    workspaces=expected_workspaces_entries,
                                )
                                assert user_fs1.get_user_manifest() == expected_user_manifest
                                assert user_fs2.get_user_manifest() == expected_user_manifest


@pytest.mark.trio
async def test_concurrent_devices_agree_on_workspace_manifest(
    running_backend, user_fs_factory, alice, alice2
):
    async with user_fs_factory(alice) as alice_user_fs:
        async with user_fs_factory(alice2) as alice2_user_fs:
            with freeze_time("2000-01-01"):
                wksp_id = await alice_user_fs.workspace_create("wksp")
            # Sync user manifest (containing the workspace entry), but
            # not the corresponding workspace manifest !
            with freeze_time("2000-01-02"):
                await alice_user_fs.sync()

            # Retrieve the user manifest
            with freeze_time("2000-01-03"):
                await alice2_user_fs.sync()

            # Now workspace diverge between devices
            alice_wksp = alice_user_fs.get_workspace(wksp_id)
            alice2_wksp = alice2_user_fs.get_workspace(wksp_id)
            with freeze_time("2000-01-04"):
                await alice_wksp.mkdir("/from_alice")
            with freeze_time("2000-01-05"):
                await alice2_wksp.mkdir("/from_alice2")

            # Sync user_fs2 first to ensure created_on field is
            # kept even if further syncs have an earlier value
            with freeze_time("2000-01-06"):
                await alice2_wksp.sync()
            with freeze_time("2000-01-07"):
                await alice_wksp.sync()
            with freeze_time("2000-01-08"):
                await alice2_wksp.sync()

            # Now, both user fs should have the same view on workspace
            expected_alice_wksp_stat = {
                "id": wksp_id,
                "base_version": 3,
                "created": datetime(2000, 1, 1),
                "updated": datetime(2000, 1, 7),
                "is_placeholder": False,
                "need_sync": False,
                "type": "folder",
                "children": ["from_alice", "from_alice2"],
                "confinement_point": None,
            }
            alice_wksp_stat = await alice_wksp.path_info("/")
            alice2_wksp_stat = await alice2_wksp.path_info("/")
            assert alice_wksp_stat == expected_alice_wksp_stat
            assert alice2_wksp_stat == expected_alice_wksp_stat
