# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    SecretKeyAlgorithm,
    VlobID,
    authenticated_cmds,
)
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_vlob_read_versions_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    dt1 = DateTime(2020, 1, 1)
    dt2 = DateTime(2020, 1, 2)
    dt3 = DateTime(2020, 1, 3)
    vlob1_id = VlobID.new()
    vlob2_id = VlobID.new()

    # Populate the realm

    outcome = await backend.vlob.create(
        now=dt1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob1_id,
        key_index=1,
        blob="<block 1 content v1>".encode(),
        timestamp=dt1,
        sequester_blob=None,
    )
    assert outcome is None

    outcome = await backend.vlob.create(
        now=dt1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob2_id,
        key_index=1,
        blob="<block 2 content v1>".encode(),
        timestamp=dt1,
        sequester_blob=None,
    )
    assert outcome is None

    outcome = await backend.realm.rotate_key(
        now=dt2,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"",
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"",
            coolorg.bob.user_id: b"",
        },
        realm_key_rotation_certificate=RealmKeyRotationCertificate(
            author=coolorg.alice.device_id,
            timestamp=dt2,
            hash_algorithm=HashAlgorithm.SHA256,
            encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
            key_index=2,
            realm_id=coolorg.wksp1_id,
            key_canary=b"",
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    outcome = await backend.vlob.update(
        now=dt2,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        vlob_id=vlob1_id,
        key_index=2,
        version=2,
        blob="<block 1 content v2>".encode(),
        timestamp=dt2,
        sequester_blob=None,
    )
    assert outcome is None

    outcome = await backend.vlob.update(
        now=dt3,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        vlob_id=vlob1_id,
        key_index=2,
        version=3,
        blob="<block 1 content v3>".encode(),
        timestamp=dt3,
        sequester_blob=None,
    )
    assert outcome is None

    # Actual test

    rep = await coolorg.alice.vlob_read_versions(
        realm_id=coolorg.wksp1_id,
        # Omit vlob 1 v2, ask for vlob 2 v2 which doesn't exist and a dummy vlob ID
        items=[(vlob1_id, 1), (vlob2_id, 1), (vlob2_id, 2), (vlob1_id, 3), (VlobID.new(), 1)],
    )
    assert rep == authenticated_cmds.v4.vlob_read_versions.RepOk(
        items=[
            (
                vlob1_id,
                1,
                coolorg.alice.device_id,
                1,
                DateTime(2020, 1, 1),
                b"<block 1 content v1>",
            ),
            (
                vlob2_id,
                1,
                coolorg.alice.device_id,
                1,
                DateTime(2020, 1, 1),
                b"<block 2 content v1>",
            ),
            (
                vlob1_id,
                2,
                coolorg.alice.device_id,
                3,
                DateTime(2020, 1, 3),
                b"<block 1 content v3>",
            ),
        ],
        needed_common_certificate_timestamp=DateTime(2000, 1, 6),
        needed_realm_certificate_timestamp=DateTime(2020, 1, 2),
    )
