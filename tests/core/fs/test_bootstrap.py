# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sqlite3
from pathlib import Path
from typing import Literal

import pytest
import trio

from parsec._parsec import (
    DateTime,
    EntryName,
    LocalDevice,
    LocalUserManifest,
    SecretKey,
    UserManifest,
    WorkspaceEntry,
    user_storage_non_speculative_init,
)
from parsec.core.fs import UserFS
from parsec.core.types import WorkspaceRole
from tests.common import freeze_time
from tests.common.fixtures_customisation import customize_fixtures
from tests.core.conftest import UserFsFactory


@pytest.mark.trio
async def test_user_manifest_access_while_speculative(
    user_fs_factory: UserFsFactory, alice: LocalDevice
):
    with freeze_time("2000-01-01", devices=[alice]):
        async with user_fs_factory(alice) as user_fs:
            with freeze_time("2000-01-02", devices=[alice]):
                user_manifest = user_fs.get_user_manifest()

    assert user_manifest.to_stats() == {
        "id": alice.user_manifest_id,
        "base_version": 0,
        "created": DateTime(2000, 1, 1),
        "updated": DateTime(2000, 1, 1),
        "is_placeholder": True,
        "need_sync": True,
    }


@pytest.mark.trio
@customize_fixtures(real_data_storage=True)
async def test_workspace_manifest_access_while_speculative(
    user_fs_factory: UserFsFactory, alice: LocalDevice, tmp_path: Path
):
    # Speculative workspace occurs when workspace is shared to a new user, or
    # when a device gets it local data removed. We use the latter here (even if
    # it is the less likely of the two) given it is simpler to do in the test.

    with freeze_time("2000-01-01", devices=[alice]):
        async with user_fs_factory(alice) as user_fs:
            user_fs: UserFS = user_fs
            wksp_id = await user_fs.workspace_create(EntryName("wksp"))
            # Retrieve where the database is stored
            wksp = user_fs.get_workspace(wksp_id)

    manifest_sqlite_db = tmp_path / "data" / alice.slug / wksp_id.hex / "workspace_data-v1.sqlite"

    # Manually drop the workspace manifest from database
    with sqlite3.connect(manifest_sqlite_db) as conn:
        before_delete = next(
            conn.execute("SELECT vlob_id FROM vlobs WHERE vlob_id = :id", {"id": wksp_id.bytes})
        )
        # Sanity check: workspace manifest should be present
        assert before_delete is not None

        conn.execute("DELETE FROM vlobs WHERE vlob_id = :id;", {"id": wksp_id.bytes})

        after_delete = next(
            conn.execute("SELECT vlob_id FROM vlobs WHERE vlob_id = :id", {"id": wksp_id.bytes}),
            None,
        )
        # Sanity check: workspace manifest should be deleted
        assert after_delete is None

    # Now re-start the userfs (the same local storage will be used)
    with freeze_time("2000-01-02", devices=[alice]):
        async with user_fs_factory(alice) as user_fs:
            with freeze_time("2000-01-03"):
                wksp = user_fs.get_workspace(wksp_id)
                root_stat = await wksp.path_info("/")

    assert root_stat == {
        "id": wksp_id,
        "base_version": 0,
        "created": DateTime(2000, 1, 2),
        "updated": DateTime(2000, 1, 2),
        "is_placeholder": True,
        "need_sync": True,
        "type": "folder",
        "children": [],
        "confinement_point": None,
    }


@pytest.mark.trio
@pytest.mark.parametrize("with_speculative", ("none", "alice2", "both"))
@customize_fixtures(backend_not_populated=True)
async def test_concurrent_devices_agree_on_user_manifest(
    running_backend,
    backend_data_binder,
    data_base_dir,
    user_fs_factory,
    coolorg,
    alice: LocalDevice,
    alice2: LocalDevice,
    with_speculative: Literal["none", "both", "alice2"],
):
    KEY = SecretKey.generate()

    async def _switch_running_backend_offline(task_status):
        should_switch_online = trio.Event()
        backend_online = trio.Event()

        async def _switch_backend_online():
            should_switch_online.set()
            await backend_online.wait()

        with running_backend.offline():
            task_status.started(_switch_backend_online)
            await should_switch_online.wait()
        backend_online.set()

    # I call this "diagonal programming"...
    async with trio.open_nursery() as nursery:
        switch_back_online = await nursery.start(_switch_running_backend_offline)

        with freeze_time("2000-01-01", devices=[alice]):
            if with_speculative != "both":
                await user_storage_non_speculative_init(data_base_dir=data_base_dir, device=alice)
            async with user_fs_factory(alice, data_base_dir=data_base_dir) as user_fs1:
                wksp1_id = await user_fs1.workspace_create(EntryName("wksp1"))

                with freeze_time("2000-01-02", devices=[alice, alice2]):
                    if with_speculative not in ("both", "alice2"):
                        await user_storage_non_speculative_init(
                            data_base_dir=data_base_dir, device=alice2
                        )
                    async with user_fs_factory(alice2, data_base_dir=data_base_dir) as user_fs2:
                        wksp2_id = await user_fs2.workspace_create(EntryName("wksp2"))

                        with freeze_time("2000-01-03", devices=[alice, alice2]):
                            # Only now the backend appear offline, this is to ensure each
                            # UserFS has created a user manifest in isolation
                            await backend_data_binder.bind_organization(
                                coolorg, alice, initial_user_manifest="not_synced"
                            )
                            await backend_data_binder.bind_device(alice2, certifier=alice)

                            await switch_back_online()

                            # Sync user_fs2 first to ensure created_on field is
                            # kept even if further syncs have an earlier value
                            with freeze_time("2000-01-04", devices=[alice2], freeze_datetime=True):
                                await user_fs2.sync()
                            with freeze_time("2000-01-05", devices=[alice], freeze_datetime=True):
                                await user_fs1.sync()
                            with freeze_time("2000-01-06", devices=[alice2], freeze_datetime=True):
                                await user_fs2.sync()

                            # Now, both user fs should have the same view on data
                            expected_workspaces_entries = (
                                WorkspaceEntry(
                                    name=EntryName("wksp1"),
                                    id=wksp1_id,
                                    key=KEY,
                                    encryption_revision=1,
                                    encrypted_on=DateTime(2000, 1, 1),
                                    role_cached_on=DateTime(2000, 1, 1),
                                    role=WorkspaceRole.OWNER,
                                ),
                                WorkspaceEntry(
                                    name=EntryName("wksp2"),
                                    id=wksp2_id,
                                    key=KEY,
                                    encryption_revision=1,
                                    encrypted_on=DateTime(2000, 1, 2),
                                    role_cached_on=DateTime(2000, 1, 2),
                                    role=WorkspaceRole.OWNER,
                                ),
                            )
                            expected_user_manifest = LocalUserManifest(
                                base=UserManifest(
                                    id=alice.user_manifest_id,
                                    version=2,
                                    timestamp=DateTime(2000, 1, 5),
                                    author=alice.device_id,
                                    created=DateTime(2000, 1, 2),
                                    updated=DateTime(2000, 1, 2),
                                    last_processed_message=0,
                                    workspaces=expected_workspaces_entries,
                                ),
                                need_sync=False,
                                updated=DateTime(2000, 1, 2),
                                last_processed_message=0,
                                workspaces=expected_workspaces_entries,
                                speculative=False,
                            )

                            user_fs1_manifest = user_fs1.get_user_manifest()
                            user_fs2_manifest = user_fs2.get_user_manifest()

                            # We use to use ANY for the "key" argument in expected_user_manifest,
                            # so that we could compare the two instances safely. Sadly, ANY doesn't
                            # play nicely with the Rust bindings, so we instead update the instances
                            # to change the key.
                            user_fs1_manifest = user_fs1_manifest.evolve(
                                workspaces=tuple(
                                    w.evolve(key=KEY) for w in user_fs1_manifest.workspaces
                                ),
                                base=user_fs1_manifest.base.evolve(
                                    workspaces=tuple(
                                        w.evolve(key=KEY) for w in user_fs1_manifest.base.workspaces
                                    )
                                ),
                            )
                            user_fs2_manifest = user_fs2_manifest.evolve(
                                workspaces=tuple(
                                    w.evolve(key=KEY) for w in user_fs2_manifest.workspaces
                                ),
                                base=user_fs2_manifest.base.evolve(
                                    workspaces=tuple(
                                        w.evolve(key=KEY) for w in user_fs2_manifest.base.workspaces
                                    )
                                ),
                            )

                            assert user_fs1_manifest == expected_user_manifest
                            assert user_fs2_manifest == expected_user_manifest


@pytest.mark.trio
async def test_concurrent_devices_agree_on_workspace_manifest(
    running_backend, user_fs_factory, data_base_dir, initialize_local_user_manifest, alice, alice2
):
    await initialize_local_user_manifest(data_base_dir, alice, initial_user_manifest="v1")
    await initialize_local_user_manifest(data_base_dir, alice2, initial_user_manifest="v1")

    async with user_fs_factory(alice) as alice_user_fs:
        alice_user_fs: UserFS = alice_user_fs
        async with user_fs_factory(alice2) as alice2_user_fs:
            alice2_user_fs: UserFS = alice2_user_fs
            with freeze_time("2000-01-01", devices=[alice]):
                wksp_id = await alice_user_fs.workspace_create(EntryName("wksp"))
            # Sync user manifest (containing the workspace entry), but
            # not the corresponding workspace manifest !
            with freeze_time("2000-01-02", devices=[alice], freeze_datetime=True):
                await alice_user_fs.sync()

            # Retrieve the user manifest but not the workspace manifest, Alice2 hence has a speculative workspace manifest
            with freeze_time("2000-01-03", devices=[alice2], freeze_datetime=True):
                await alice2_user_fs.sync()

            # Now workspace diverge between devices
            alice_wksp = alice_user_fs.get_workspace(wksp_id)
            alice2_wksp = alice2_user_fs.get_workspace(wksp_id)
            with freeze_time("2000-01-04", devices=[alice]):
                await alice_wksp.mkdir("/from_alice")
            with freeze_time("2000-01-05", devices=[alice2]):
                await alice2_wksp.mkdir("/from_alice2")

            # Sync user_fs2 first to ensure created_on field is
            # kept even if further syncs have an earlier value
            with freeze_time("2000-01-06", devices=[alice2], freeze_datetime=True):
                await alice2_wksp.sync()
            with freeze_time("2000-01-07", devices=[alice], freeze_datetime=True):
                await alice_wksp.sync()
            with freeze_time("2000-01-08", devices=[alice2], freeze_datetime=True):
                await alice2_wksp.sync()

            # Now, both user fs should have the same view on workspace
            expected_alice_wksp_stat = {
                "id": wksp_id,
                "base_version": 3,
                "created": DateTime(2000, 1, 1),
                "updated": DateTime(2000, 1, 7),
                "is_placeholder": False,
                "need_sync": False,
                "type": "folder",
                "children": [EntryName("from_alice"), EntryName("from_alice2")],
                "confinement_point": None,
            }
            alice_wksp_stat = await alice_wksp.path_info("/")
            alice2_wksp_stat = await alice2_wksp.path_info("/")
            assert alice_wksp_stat == expected_alice_wksp_stat
            assert alice2_wksp_stat == expected_alice_wksp_stat


@pytest.mark.trio
async def test_empty_user_manifest_placeholder_noop_on_resolve_sync(
    running_backend, user_fs_factory, alice: LocalDevice, alice2: LocalDevice
):
    # Alice creates a workspace and sync it
    async with user_fs_factory(alice) as alice_user_fs:
        with freeze_time("2000-01-02", devices=[alice]):
            await alice_user_fs.workspace_create(EntryName("wksp1"))
        with freeze_time("2000-01-03", devices=[alice], freeze_datetime=True):
            await alice_user_fs.sync()
        alice_user_manifest_v1 = alice_user_fs.get_user_manifest()
        assert alice_user_manifest_v1.to_stats() == {
            "id": alice.user_manifest_id,
            # Fixtures populate backend with an empty v1 user manifest created at 2000-01-01
            "base_version": 2,
            "created": DateTime(2000, 1, 1),
            "updated": DateTime(2000, 1, 2),
            "is_placeholder": False,
            "need_sync": False,
        }

        with freeze_time("2000-01-04", devices=[alice2]):
            # Now Alice2 comes into play with it speculative user manifest
            async with user_fs_factory(alice2) as alice2_user_fs:
                with freeze_time("2000-01-05", devices=[alice2]):
                    # Access the user manifest to ensure it is created, but do not modify it !
                    alice2_user_manifest_v0 = alice2_user_fs.get_user_manifest()
                    assert alice2_user_manifest_v0.to_stats() == {
                        "id": alice.user_manifest_id,
                        "base_version": 0,
                        "created": DateTime(2000, 1, 4),
                        "updated": DateTime(2000, 1, 4),
                        "is_placeholder": True,
                        "need_sync": True,
                    }

                    # Finally Alice2 sync, this should not create any remote change
                    with freeze_time("2000-01-06", devices=[alice2], freeze_datetime=True):
                        await alice2_user_fs.sync()

                    alice2_user_manifest_v1 = alice2_user_fs.get_user_manifest()
                    assert alice2_user_manifest_v1 == alice_user_manifest_v1


@pytest.mark.trio
async def test_empty_workspace_manifest_placeholder_noop_on_resolve_sync(
    running_backend, user_fs_factory, data_base_dir, initialize_local_user_manifest, alice, alice2
):
    await initialize_local_user_manifest(data_base_dir, alice, initial_user_manifest="v1")
    await initialize_local_user_manifest(data_base_dir, alice2, initial_user_manifest="v1")

    async with user_fs_factory(alice) as alice_user_fs:
        alice_user_fs: UserFS = alice_user_fs
        async with user_fs_factory(alice2) as alice2_user_fs:
            alice2_user_fs: UserFS = alice2_user_fs
            # First Alice creates a workspace, then populates and syncs it
            with freeze_time("2000-01-01", devices=[alice]):
                wksp_id = await alice_user_fs.workspace_create(EntryName("wksp"))
            alice_wksp = alice_user_fs.get_workspace(wksp_id)
            with freeze_time("2000-01-02", devices=[alice]):
                await alice_wksp.mkdir("/from_alice")
            with freeze_time("2000-01-03", devices=[alice], freeze_datetime=True):
                await alice_wksp.sync()
                await alice_user_fs.sync()

            # Alice2 retrieves the user manifest but NOT the workspace manifest
            # hence Alice2 end up with a speculative workspace manifest
            with freeze_time("2000-01-04", devices=[alice2], freeze_datetime=True):
                await alice2_user_fs.sync()
            alice2_wksp = alice2_user_fs.get_workspace(wksp_id)

            # Access the workspace manifest to ensure it is created, but do not modify it !
            with freeze_time("2000-01-05", devices=[alice2]):
                alice2_wksp_stat_v0 = await alice2_wksp.path_info("/")
            assert alice2_wksp_stat_v0 == {
                "id": wksp_id,
                "base_version": 0,
                "created": DateTime(2000, 1, 4),
                "updated": DateTime(2000, 1, 4),
                "is_placeholder": True,
                "need_sync": True,
                "type": "folder",
                "children": [],
                "confinement_point": None,
            }

            # Now proceed to sync, this should end up with no remote changes
            with freeze_time("2000-01-06", devices=[alice2], freeze_datetime=True):
                await alice2_wksp.sync()
            alice_wksp_stat_v1 = await alice_wksp.path_info("/")
            alice2_wksp_stat_v1 = await alice2_wksp.path_info("/")
            assert alice2_wksp_stat_v1 == alice_wksp_stat_v1
            assert alice2_wksp_stat_v1 == {
                "id": wksp_id,
                "base_version": 1,
                "created": DateTime(2000, 1, 1),
                "updated": DateTime(2000, 1, 2),
                "is_placeholder": False,
                "need_sync": False,
                "type": "folder",
                "children": [EntryName("from_alice")],
                "confinement_point": None,
            }
