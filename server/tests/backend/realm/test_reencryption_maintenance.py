# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    BackendEventMessageReceived,
    BackendEventRealmMaintenanceFinished,
    BackendEventRealmMaintenanceStarted,
    DateTime,
    ReencryptionBatchEntry,
    authenticated_cmds,
)
from parsec.api.protocol import (
    ApiV2V3_APIEventMessageReceived,
    ApiV2V3_APIEventRealmMaintenanceFinished,
    ApiV2V3_APIEventRealmMaintenanceStarted,
    ApiV2V3_EventsListenRepNoEvents,
    ApiV2V3_EventsListenRepOk,
    ApiV2V3_Message,
    ApiV2V3_MessageGetRepOk,
    BlockCreateRepInMaintenance,
    BlockID,
    BlockReadRepOk,
    MaintenanceType,
    RealmFinishReencryptionMaintenanceRepBadEncryptionRevision,
    RealmFinishReencryptionMaintenanceRepMaintenanceError,
    RealmFinishReencryptionMaintenanceRepNotAllowed,
    RealmFinishReencryptionMaintenanceRepNotInMaintenance,
    RealmFinishReencryptionMaintenanceRepOk,
    RealmRole,
    RealmStartReencryptionMaintenanceRepBadEncryptionRevision,
    RealmStartReencryptionMaintenanceRepBadTimestamp,
    RealmStartReencryptionMaintenanceRepInMaintenance,
    RealmStartReencryptionMaintenanceRepNotAllowed,
    RealmStartReencryptionMaintenanceRepNotFound,
    RealmStartReencryptionMaintenanceRepOk,
    RealmStartReencryptionMaintenanceRepParticipantMismatch,
    RealmStatusRepOk,
    UserID,
    VlobCreateRepInMaintenance,
    VlobID,
    VlobListVersionsRepOk,
    VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceGetReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceGetReencryptionBatchRepOk,
    VlobMaintenanceSaveReencryptionBatchRepNotAllowed,
    VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceSaveReencryptionBatchRepOk,
    VlobPollChangesRepOk,
    VlobUpdateRepInMaintenance,
)
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.vlob import VlobNotFoundError, VlobVersionError
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
from tests.backend.common import (
    apiv2v3_events_listen_nowait,
    apiv2v3_events_subscribe,
    apiv2v3_message_get,
    apiv2v3_vlob_read,
    block_create,
    block_read,
    realm_finish_reencryption_maintenance,
    realm_start_reencryption_maintenance,
    realm_status,
    vlob_create,
    vlob_list_versions,
    vlob_maintenance_get_reencryption_batch,
    vlob_maintenance_save_reencryption_batch,
    vlob_poll_changes,
    vlob_update,
)
from tests.common import freeze_time, real_clock_timeout


@pytest.mark.trio
async def test_start_bad_encryption_revision(alice_ws, realm, alice):
    rep = await realm_start_reencryption_maintenance(
        alice_ws, realm, 42, DateTime.now(), {alice.user_id: b"wathever"}, check_rep=False
    )
    assert isinstance(rep, RealmStartReencryptionMaintenanceRepBadEncryptionRevision)


@pytest.mark.trio
async def test_start_bad_timestamp(alice_ws, realm, alice):
    with freeze_time() as now:
        rep = await realm_start_reencryption_maintenance(
            alice_ws, realm, 2, DateTime(2000, 1, 1), {alice.user_id: b"wathever"}, check_rep=False
        )
    assert rep == RealmStartReencryptionMaintenanceRepBadTimestamp(
        reason=None,
        backend_timestamp=now,
        ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
        ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
        client_timestamp=DateTime(2000, 1, 1),
    )


@pytest.mark.trio
async def test_start_bad_per_participant_message(
    backend, alice_ws, alice, bob, adam, realm, next_timestamp
):
    # Bob used to be part of the realm
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=RealmRole.READER,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    # Adam is still part of the realm, but is revoked
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=adam.user_id,
            role=RealmRole.READER,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    await backend.user.revoke_user(
        alice.organization_id,
        adam.user_id,
        revoked_user_certificate=b"<dummy>",
        revoked_user_certifier=alice.device_id,
    )

    for msg in [
        {},
        {alice.user_id: b"ok", bob.user_id: b"bad"},
        {alice.user_id: b"ok", UserID("zack"): b"bad"},
        {alice.user_id: b"ok", adam.user_id: b"bad"},
    ]:
        rep = await realm_start_reencryption_maintenance(
            alice_ws, realm, 2, next_timestamp(), msg, check_rep=False
        )
        # The reason is no longer generated
        assert isinstance(rep, RealmStartReencryptionMaintenanceRepParticipantMismatch)

    # Finally make sure the reencryption is possible
    await realm_start_reencryption_maintenance(
        alice_ws, realm, 2, next_timestamp(), {alice.user_id: b"ok"}
    )


@pytest.mark.trio
async def test_start_send_message_to_participants(backend, alice, bob, alice_ws, bob_ws, realm):
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=RealmRole.READER,
            granted_by=alice.device_id,
            granted_on=DateTime.now(),
        ),
    )

    with freeze_time("2000-01-02"):
        await realm_start_reencryption_maintenance(
            alice_ws,
            realm,
            2,
            DateTime.now(),
            {alice.user_id: b"alice msg", bob.user_id: b"bob msg"},
        )

    # Each participant should have received a message
    for user, sock in ((alice, alice_ws), (bob, bob_ws)):
        rep = await apiv2v3_message_get(sock)
        assert rep == ApiV2V3_MessageGetRepOk(
            messages=[
                ApiV2V3_Message(
                    count=1,
                    body=f"{user.user_id.str} msg".encode(),
                    timestamp=DateTime(2000, 1, 2),
                    sender=alice.device_id,
                )
            ],
        )


@pytest.mark.trio
async def test_start_reencryption_update_status(alice_ws, alice, realm):
    with freeze_time("2000-01-02"):
        await realm_start_reencryption_maintenance(
            alice_ws, realm, 2, DateTime.now(), {alice.user_id: b"foo"}
        )
    rep = await realm_status(alice_ws, realm)
    assert rep == RealmStatusRepOk(
        encryption_revision=2,
        in_maintenance=True,
        maintenance_started_by=alice.device_id,
        maintenance_started_on=DateTime(2000, 1, 2),
        maintenance_type=MaintenanceType.REENCRYPTION,
    )


@pytest.mark.trio
async def test_start_already_in_maintenance(alice_ws, realm, alice):
    await realm_start_reencryption_maintenance(
        alice_ws, realm, 2, DateTime.now(), {alice.user_id: b"wathever"}
    )
    # Providing good or bad encryption revision shouldn't change anything
    for encryption_revision in (2, 3):
        rep = await realm_start_reencryption_maintenance(
            alice_ws,
            realm,
            encryption_revision,
            DateTime.now(),
            {alice.user_id: b"wathever"},
            check_rep=False,
        )
        assert isinstance(rep, RealmStartReencryptionMaintenanceRepInMaintenance)


@pytest.mark.trio
async def test_start_check_access_rights(backend, bob_ws, alice, bob, realm, next_timestamp):
    # User not part of the realm
    rep = await realm_start_reencryption_maintenance(
        bob_ws, realm, 2, DateTime.now(), {alice.user_id: b"wathever"}, check_rep=False
    )
    assert isinstance(rep, RealmStartReencryptionMaintenanceRepNotAllowed)

    # User part of the realm with various role
    for not_allowed_role in (RealmRole.READER, RealmRole.CONTRIBUTOR, RealmRole.MANAGER, None):
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=bob.user_id,
                role=not_allowed_role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )

        rep = await realm_start_reencryption_maintenance(
            bob_ws,
            realm,
            2,
            next_timestamp(),
            {alice.user_id: b"foo", bob.user_id: b"bar"},
            check_rep=False,
        )
        assert isinstance(rep, RealmStartReencryptionMaintenanceRepNotAllowed)

    # Finally, just make sure owner can do it
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=RealmRole.OWNER,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )

    rep = await realm_start_reencryption_maintenance(
        bob_ws,
        realm,
        2,
        DateTime.now(),
        {alice.user_id: b"foo", bob.user_id: b"bar"},
        check_rep=False,
    )
    assert isinstance(rep, RealmStartReencryptionMaintenanceRepOk)


@pytest.mark.trio
async def test_start_other_organization(
    backend_asgi_app, ws_from_other_organization_factory, realm, alice
):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await realm_start_reencryption_maintenance(
            sock, realm, 2, DateTime.now(), {alice.user_id: b"foo"}, check_rep=False
        )
    # The reason is no longer generated
    assert isinstance(rep, RealmStartReencryptionMaintenanceRepNotFound)


@pytest.mark.trio
async def test_finish_not_in_maintenance(alice_ws, realm):
    for encryption_revision in (2, 3):
        rep = await realm_finish_reencryption_maintenance(
            alice_ws, realm, encryption_revision, check_rep=False
        )
        # The reason is no longer generated
        assert isinstance(rep, RealmFinishReencryptionMaintenanceRepNotInMaintenance)


@pytest.mark.trio
async def test_finish_while_reencryption_not_done(alice_ws, realm, alice, vlobs):
    await realm_start_reencryption_maintenance(
        alice_ws, realm, 2, DateTime.now(), {alice.user_id: b"wathever"}
    )
    rep = await realm_finish_reencryption_maintenance(alice_ws, realm, 2, check_rep=False)
    # The reason is no longer generated
    assert isinstance(rep, RealmFinishReencryptionMaintenanceRepMaintenanceError)

    # Also try with part of the job done
    rep = await vlob_maintenance_get_reencryption_batch(alice_ws, realm, 2, size=2)
    assert isinstance(rep, VlobMaintenanceGetReencryptionBatchRepOk)
    assert len(rep.batch) == 2

    batch = []
    for entry in rep.batch:
        batch.append(
            ReencryptionBatchEntry(
                entry.vlob_id,
                entry.version,
                f"{entry.vlob_id.hex}::{entry.version} reencrypted".encode(),
            )
        )
    await vlob_maintenance_save_reencryption_batch(alice_ws, realm, 2, batch)

    rep = await realm_finish_reencryption_maintenance(alice_ws, realm, 2, check_rep=False)
    # The reason is no longer generated
    assert isinstance(rep, RealmFinishReencryptionMaintenanceRepMaintenanceError)


@pytest.mark.trio
async def test_reencrypt_and_finish_check_access_rights(
    backend, alice_ws, bob_ws, alice, bob, realm, vlobs, next_timestamp
):
    encryption_revision = 1

    # Changing realm roles is not possible during maintenance,
    # hence those helpers to easily jump in/out of maintenance

    async def _ready_to_finish(bob_in_workspace):
        nonlocal encryption_revision
        encryption_revision += 1
        reencryption_msgs = {alice.user_id: b"foo"}
        if bob_in_workspace:
            reencryption_msgs[bob.user_id] = b"bar"
        await realm_start_reencryption_maintenance(
            alice_ws, realm, encryption_revision, DateTime.now(), reencryption_msgs
        )
        updated_batch = [
            ReencryptionBatchEntry(
                vlob_id=vlob_id,
                version=version,
                blob=f"{vlob_id.hex}::{version}::{encryption_revision}".encode(),
            )
            for vlob_id, version in {(vlobs[0], 1), (vlobs[0], 2), (vlobs[1], 1)}
        ]
        await vlob_maintenance_save_reencryption_batch(
            alice_ws, realm, encryption_revision, updated_batch
        )

    async def _finish():
        await realm_finish_reencryption_maintenance(alice_ws, realm, encryption_revision)

    async def _assert_bob_maintenance_access(allowed):
        rep = await vlob_maintenance_save_reencryption_batch(
            bob_ws, realm, encryption_revision, [], check_rep=False
        )
        if allowed:
            assert isinstance(rep, VlobMaintenanceSaveReencryptionBatchRepOk)
        else:
            assert isinstance(rep, VlobMaintenanceSaveReencryptionBatchRepNotAllowed)

        rep = await realm_finish_reencryption_maintenance(
            bob_ws, realm, encryption_revision, check_rep=False
        )
        if allowed:
            assert isinstance(rep, RealmFinishReencryptionMaintenanceRepOk)
        else:
            assert isinstance(rep, RealmFinishReencryptionMaintenanceRepNotAllowed)

    # User not part of the realm
    await _ready_to_finish(bob_in_workspace=False)
    await _assert_bob_maintenance_access(allowed=False)
    await _finish()

    # User part of the realm with various role
    for not_allowed_role in (RealmRole.READER, RealmRole.CONTRIBUTOR, RealmRole.MANAGER, None):
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=bob.user_id,
                role=not_allowed_role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        await _ready_to_finish(bob_in_workspace=not_allowed_role is not None)
        await _assert_bob_maintenance_access(allowed=False)
        await _finish()

    # Finally, just make sure owner can do it
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=RealmRole.OWNER,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    await _ready_to_finish(bob_in_workspace=True)
    await _assert_bob_maintenance_access(allowed=True)


@pytest.mark.trio
async def test_reencryption_batch_not_during_maintenance(alice_ws, realm):
    rep = await vlob_maintenance_get_reencryption_batch(alice_ws, realm, 1)
    # The reason is no longer generated
    assert isinstance(rep, VlobMaintenanceGetReencryptionBatchRepNotInMaintenance)

    rep = await vlob_maintenance_save_reencryption_batch(alice_ws, realm, 1, [], check_rep=False)
    # The reason is no longer generated
    assert isinstance(rep, VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance)

    rep = await realm_finish_reencryption_maintenance(alice_ws, realm, 1, check_rep=False)
    # The reason is no longer generated
    assert isinstance(rep, RealmFinishReencryptionMaintenanceRepNotInMaintenance)


@pytest.mark.trio
async def test_reencryption_batch_bad_revisison(alice_ws, realm, alice):
    await realm_start_reencryption_maintenance(
        alice_ws, realm, 2, DateTime.now(), {alice.user_id: b"foo"}
    )

    rep = await vlob_maintenance_get_reencryption_batch(alice_ws, realm, 1)
    assert isinstance(rep, VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision)

    rep = await realm_finish_reencryption_maintenance(alice_ws, realm, 1, check_rep=False)
    assert isinstance(rep, RealmFinishReencryptionMaintenanceRepBadEncryptionRevision)


@pytest.mark.trio
async def test_reencryption(alice, alice_ws, realm, vlob_atoms):
    with freeze_time("2000-01-02"):
        await realm_start_reencryption_maintenance(
            alice_ws, realm, 2, DateTime.now(), {alice.user_id: b"foo"}
        )

    # Each participant should have received a message
    rep = await apiv2v3_message_get(alice_ws)
    assert rep == ApiV2V3_MessageGetRepOk(
        messages=[
            ApiV2V3_Message(
                count=1,
                body=b"foo",
                timestamp=DateTime(2000, 1, 2),
                sender=alice.device_id,
            )
        ],
    )

    async def _reencrypt_with_batch_of_2(expected_size, expected_done):
        rep = await vlob_maintenance_get_reencryption_batch(alice_ws, realm, 2, size=2)
        assert isinstance(rep, VlobMaintenanceGetReencryptionBatchRepOk)
        assert len(rep.batch) == expected_size

        batch = []
        for entry in rep.batch:
            batch.append(
                ReencryptionBatchEntry(
                    entry.vlob_id,
                    entry.version,
                    f"{entry.vlob_id.hex}::{entry.version} reencrypted".encode(),
                )
            )
        rep = await vlob_maintenance_save_reencryption_batch(alice_ws, realm, 2, batch)
        assert rep == VlobMaintenanceSaveReencryptionBatchRepOk(total=3, done=expected_done)

    # Should have 2 batch to reencrypt
    await _reencrypt_with_batch_of_2(expected_size=2, expected_done=2)
    await _reencrypt_with_batch_of_2(expected_size=1, expected_done=3)
    await _reencrypt_with_batch_of_2(expected_size=0, expected_done=3)

    # Finish the reencryption
    await realm_finish_reencryption_maintenance(alice_ws, realm, 2)

    # Check the vlob have changed
    for vlob_id, version in vlob_atoms:
        rep = await apiv2v3_vlob_read(alice_ws, vlob_id, version, encryption_revision=2)
        assert rep.blob == f"{vlob_id.hex}::{version} reencrypted".encode()


@pytest.mark.trio
async def test_reencryption_provide_unknown_vlob_atom_and_duplications(
    backend, alice, alice_ws, realm, vlob_atoms
):
    await realm_start_reencryption_maintenance(
        alice_ws, realm, 2, DateTime.now(), {alice.user_id: b"foo"}
    )
    rep = await vlob_maintenance_get_reencryption_batch(alice_ws, realm, 2)
    assert isinstance(rep, VlobMaintenanceGetReencryptionBatchRepOk)
    assert len(rep.batch) == 3

    unknown_vlob_id = VlobID.new()
    duplicated_vlob_id = rep.batch[0].vlob_id
    duplicated_version = rep.batch[0].version
    duplicated_expected_blob = rep.batch[0].blob
    reencrypted_batch = [
        # Reencryption as identity
        *rep.batch,
        # Add an unknown vlob
        ReencryptionBatchEntry(unknown_vlob_id, 1, b"ignored"),
        # Valid vlob ID with invalid version
        ReencryptionBatchEntry(duplicated_vlob_id, 99, b"ignored"),
        # Duplicate a vlob atom, should be ignored given the reencryption has already be done for it
        ReencryptionBatchEntry(duplicated_vlob_id, duplicated_version, b"ignored"),
    ]

    # Another level of duplication !
    for i in range(2):
        rep = await vlob_maintenance_save_reencryption_batch(alice_ws, realm, 2, reencrypted_batch)
        assert rep == VlobMaintenanceSaveReencryptionBatchRepOk(total=3, done=3)

    # Finish the reencryption
    await realm_finish_reencryption_maintenance(alice_ws, realm, 2)

    # Check the vlobs
    with pytest.raises(VlobNotFoundError):
        await backend.vlob.read(
            organization_id=alice.organization_id,
            author=alice.device_id,
            encryption_revision=2,
            vlob_id=unknown_vlob_id,
        )
    with pytest.raises(VlobVersionError):
        await backend.vlob.read(
            organization_id=alice.organization_id,
            author=alice.device_id,
            encryption_revision=2,
            vlob_id=duplicated_vlob_id,
            version=99,
        )
    _, content, _, _, _, _ = await backend.vlob.read(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=2,
        vlob_id=duplicated_vlob_id,
        version=duplicated_version,
    )
    assert content == duplicated_expected_blob


@pytest.mark.trio
async def test_access_during_reencryption(backend, alice_ws, alice, realm_factory, next_timestamp):
    # First initialize a nice realm with block and vlob
    realm_id = await realm_factory(backend, author=alice)
    vlob_id = VlobID.new()
    block_id = BlockID.new()
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm_id,
        encryption_revision=1,
        vlob_id=vlob_id,
        timestamp=next_timestamp(),
        blob=b"v1",
    )
    await backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm_id,
        block_id=block_id,
        created_on=next_timestamp(),
        block=b"<block_data>",
    )

    async def _assert_write_access_disallowed(encryption_revision):
        rep = await vlob_create(
            alice_ws,
            realm_id=realm_id,
            vlob_id=VlobID.new(),
            blob=b"data",
            encryption_revision=encryption_revision,
            check_rep=False,
        )
        assert isinstance(rep, VlobCreateRepInMaintenance)
        rep = await vlob_update(
            alice_ws,
            vlob_id,
            version=2,
            blob=b"data",
            encryption_revision=encryption_revision,
            check_rep=False,
        )
        assert isinstance(rep, VlobUpdateRepInMaintenance)
        rep = await block_create(
            alice_ws, block_id=block_id, realm_id=realm_id, block=b"data", check_rep=False
        )
        assert isinstance(rep, BlockCreateRepInMaintenance)

    async def _assert_read_access_allowed(encryption_revision, expected_blob=b"v1"):
        rep = await apiv2v3_vlob_read(
            alice_ws, vlob_id=vlob_id, version=1, encryption_revision=encryption_revision
        )
        assert isinstance(rep, authenticated_cmds.v3.vlob_read.RepOk)
        assert rep.blob == expected_blob

        rep = await block_read(alice_ws, block_id=block_id)
        assert rep == BlockReadRepOk(b"<block_data>")

        # For good measure, also try those read-only commands even if they
        # are encryption-revision agnostic
        rep = await vlob_list_versions(alice_ws, vlob_id=vlob_id)
        assert isinstance(rep, VlobListVersionsRepOk)
        rep = await vlob_poll_changes(alice_ws, realm_id=realm_id, last_checkpoint=0)
        assert isinstance(rep, VlobPollChangesRepOk)

    async def _assert_read_access_bad_encryption_revision(encryption_revision, expected_status):
        rep = await apiv2v3_vlob_read(
            alice_ws, vlob_id=vlob_id, version=1, encryption_revision=encryption_revision
        )
        assert isinstance(rep, expected_status)

    # Sanity check just to make we can access the data with initial encryption revision
    await _assert_read_access_allowed(1)

    # Now start reencryption
    await backend.realm.start_reencryption_maintenance(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm_id,
        encryption_revision=2,
        per_participant_message={alice.user_id: b"<whatever>"},
        timestamp=DateTime.now(),
    )

    # Only read with old encryption revision is now allowed
    await _assert_read_access_allowed(1)
    await _assert_read_access_bad_encryption_revision(
        2, expected_status=authenticated_cmds.v3.vlob_read.RepInMaintenance
    )
    await _assert_write_access_disallowed(1)
    await _assert_write_access_disallowed(2)

    # Actually reencrypt the vlob data, this shouldn't affect us for the moment
    # given reencryption is not formally finished
    await backend.vlob.maintenance_save_reencryption_batch(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm_id,
        encryption_revision=2,
        batch=[(vlob_id, 1, b"v2")],
    )

    await _assert_read_access_allowed(1)
    await _assert_read_access_bad_encryption_revision(
        2, expected_status=authenticated_cmds.v3.vlob_read.RepInMaintenance
    )
    await _assert_write_access_disallowed(1)
    await _assert_write_access_disallowed(2)

    # Finish the reencryption
    await backend.realm.finish_reencryption_maintenance(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm_id,
        encryption_revision=2,
    )

    # Now only the new encryption revision is allowed
    await _assert_read_access_allowed(2, expected_blob=b"v2")
    await _assert_read_access_bad_encryption_revision(
        1, expected_status=authenticated_cmds.v3.vlob_read.RepBadEncryptionRevision
    )


@pytest.mark.trio
async def test_reencryption_events(backend, alice_ws, alice2_ws, realm, alice, vlobs, vlob_atoms):
    # Start listening events
    await apiv2v3_events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        # Start maintenance and check for events
        await realm_start_reencryption_maintenance(
            alice2_ws, realm, 2, DateTime.now(), {alice.user_id: b"foo"}
        )

        async with real_clock_timeout():
            # No guarantees those events occur before the commands' return
            await spy.wait_multiple(
                [BackendEventRealmMaintenanceStarted, BackendEventMessageReceived]
            )

        rep = await apiv2v3_events_listen_nowait(alice_ws)
        assert rep == ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventRealmMaintenanceStarted(realm, 2))
        rep = await apiv2v3_events_listen_nowait(alice_ws)
        assert rep == ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventMessageReceived(1))

        # Do the reencryption
        rep = await vlob_maintenance_get_reencryption_batch(alice_ws, realm, 2, size=100)
        await vlob_maintenance_save_reencryption_batch(alice_ws, realm, 2, rep.batch)

        # Finish maintenance and check for events
        await realm_finish_reencryption_maintenance(alice2_ws, realm, 2)

        # No guarantees those events occur before the commands' return
        await spy.wait_with_timeout(BackendEventRealmMaintenanceFinished)

        rep = await apiv2v3_events_listen_nowait(alice_ws)
        assert rep == ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventRealmMaintenanceFinished(realm, 2))

    # Sanity check
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == ApiV2V3_EventsListenRepNoEvents()
