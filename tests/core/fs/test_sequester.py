# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.api.data import EntryName
from parsec.api.protocol import RealmID, VlobID
from parsec.sequester_crypto import sequester_service_decrypt

from tests.common import customize_fixtures, sequester_service_factory


@pytest.mark.trio
@customize_fixtures(
    coolorg_is_sequestered_organization=True,
    alice_initial_remote_user_manifest="not_synced",
    alice_initial_local_user_manifest="speculative_v0",
    bob_initial_remote_user_manifest="not_synced",
    bob_initial_local_user_manifest="non_speculative_v0",
)
async def test_userfs_sequester_sync(
    running_backend, backend, alice_user_fs, bob_user_fs, coolorg, alice, bob
):
    async def _new_sequester_service(label: str):
        service = sequester_service_factory(authority=coolorg.sequester_authority, label=label)
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )
        return service

    async def _assert_sequester_dump(service, local_device, expected_versions):
        realm_dump = await backend.sequester.dump_realm(
            organization_id=coolorg.organization_id,
            service_id=service.service_id,
            realm_id=RealmID(local_device.user_manifest_id.uuid),
        )
        user_manifest_vlob_id = VlobID(local_device.user_manifest_id.uuid)

        # Simple check: make sure we retreive the expected items
        items = {(vlob_id, version) for (vlob_id, version, _) in realm_dump}
        expected_items = {(user_manifest_vlob_id, v) for v in expected_versions}
        assert items == expected_items

        # Advanced check: make sure each item contain the valid data
        for _, version, sequestered_blob in realm_dump:
            clear_blob_from_sequester = sequester_service_decrypt(
                service.decryption_key, sequestered_blob
            )
            _, blob, _, _, _ = await backend.vlob.read(
                organization_id=coolorg.organization_id,
                author=local_device.device_id,
                encryption_revision=1,
                vlob_id=user_manifest_vlob_id,
                version=version,
            )
            clear_blob = local_device.user_manifest_key.decrypt(blob)

            assert clear_blob == clear_blob_from_sequester

    # 1) Alice sync v1 without sequester services
    await alice_user_fs.sync()

    # Create a sequester service
    s1 = await _new_sequester_service("Sequester service 1")

    # 2) Bob sync v1 with a sequester service
    await bob_user_fs.sync()

    # 3) Bob sync v2
    await bob_user_fs.workspace_create(EntryName("bob_w1"))
    await bob_user_fs.sync()

    # New sequester service, next syncs should take it into account
    s2 = await _new_sequester_service("Sequester service 2")

    # 4) Alice sync v2 with a new sequester service
    await alice_user_fs.workspace_create(EntryName("alice_w2"))
    await alice_user_fs.sync()

    # 5) Finally check the sequester services contains valid data

    # S1 is expected to contain Alice v2 and Bob v1&v2
    await _assert_sequester_dump(service=s1, local_device=alice, expected_versions={2})
    await _assert_sequester_dump(service=s1, local_device=bob, expected_versions={1, 2})

    # S2 is expected to contain only Bob v2
    await _assert_sequester_dump(service=s2, local_device=alice, expected_versions={2})
    await _assert_sequester_dump(service=s2, local_device=bob, expected_versions={})


@pytest.mark.trio
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_workspacefs_sequester_sync(running_backend, backend, alice_user_fs, coolorg, alice):
    async def _new_sequester_service(label: str):
        service = sequester_service_factory(authority=coolorg.sequester_authority, label=label)
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )
        return service

    async def _assert_sequester_dump(service, workspace, expected_items):
        realm_id = RealmID(workspace.workspace_id.uuid)
        realm_dump = await backend.sequester.dump_realm(
            organization_id=coolorg.organization_id,
            service_id=service.service_id,
            realm_id=realm_id,
        )

        # Simple check: make sure we retreive the expected items
        items = {(vlob_id, version) for (vlob_id, version, _) in realm_dump}
        expected_items = {
            (VlobID(entry_id.uuid), version) for (entry_id, version) in expected_items
        }
        assert items == expected_items

        # Advanced check: make sure each item contain the valid data
        for vlob_id, version, sequestered_blob in realm_dump:
            clear_blob_from_sequester = sequester_service_decrypt(
                service.decryption_key, sequestered_blob
            )
            _, blob, _, _, _ = await backend.vlob.read(
                organization_id=coolorg.organization_id,
                author=alice.device_id,
                encryption_revision=1,
                vlob_id=vlob_id,
                version=version,
            )
            clear_blob = workspace.get_workspace_entry().key.decrypt(blob)

            assert clear_blob == clear_blob_from_sequester

    # 1) No sequester services, sync w1@v1 and w1f1@v1
    w1_id = await alice_user_fs.workspace_create(EntryName("w1"))
    w1 = alice_user_fs.get_workspace(w1_id)
    await w1.touch("/w1f1")
    await w1.sync()

    # Create a sequester service
    s1 = await _new_sequester_service("Sequester service 1")

    # 2) Sync w2@v1 with a sequester service
    w2_id = await alice_user_fs.workspace_create(EntryName("w2"))
    w2 = alice_user_fs.get_workspace(w2_id)
    await w2.sync()

    # 3) Sync w2@v2, w2f1@v1
    await w2.touch("/w2f1")
    w2f1_id = await w2.path_id("/w2f1")
    await w2.sync()

    # New sequester service, next syncs should take it into account
    s2 = await _new_sequester_service("Sequester service 2")

    # 4) Sync w1@v2 with a new sequester service
    await w1.unlink("/w1f1")
    await w1.sync()

    # 5) Finally check the sequester services contains valid data

    # S1 is expected to contain w2@v1, w2@v2, w2f1@v1, w1@v2
    await _assert_sequester_dump(service=s1, workspace=w1, expected_items={(w1_id, 2)})
    await _assert_sequester_dump(
        service=s1, workspace=w2, expected_items={(w2_id, 1), (w2_id, 2), (w2f1_id, 1)}
    )

    # S1 is expected to contain only w1@v2
    await _assert_sequester_dump(service=s2, workspace=w1, expected_items={(w1_id, 2)})
    await _assert_sequester_dump(service=s2, workspace=w2, expected_items={})
