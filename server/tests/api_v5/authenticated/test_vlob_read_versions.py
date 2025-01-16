# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmRole,
    SecretKeyAlgorithm,
    VlobID,
    authenticated_cmds,
)
from parsec.components.vlob import VLOB_READ_REQUEST_ITEMS_LIMIT
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "as_contributor",
        "as_manager",
        "as_owner",
    ),
)
async def test_authenticated_vlob_read_versions_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "as_reader":
            author = coolorg.bob

        case "as_contributor":
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.CONTRIBUTOR,
                now=DateTime(2019, 1, 1),
            )
            author = coolorg.bob

        case "as_manager":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER, now=DateTime(2019, 1, 1)
            )
            author = coolorg.bob

        case "as_owner":
            author = coolorg.alice

        case unknown:
            assert False, unknown

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
            encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
            key_index=2,
            realm_id=coolorg.wksp1_id,
            key_canary=b"",
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    outcome = await backend.vlob.update(
        now=dt3,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        vlob_id=vlob1_id,
        key_index=2,
        version=2,
        blob="<block 1 content v2>".encode(),
        timestamp=dt3,
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
    )
    assert outcome is None

    # Actual test

    rep = await author.vlob_read_versions(
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


async def test_authenticated_vlob_read_versions_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bad_realm_id = VlobID.new()
    rep = await coolorg.alice.vlob_read_versions(realm_id=bad_realm_id, items=[(VlobID.new(), 1)])
    assert rep == authenticated_cmds.v4.vlob_read_versions.RepRealmNotFound()


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_vlob_read_versions_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "never_allowed":
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=None
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.vlob_read_versions(realm_id=coolorg.wksp1_id, items=[(VlobID.new(), 1)])
    assert rep == authenticated_cmds.v4.vlob_read_versions.RepAuthorNotAllowed()


async def test_authenticated_vlob_read_versions_too_many_elements(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    too_many_items = [(VlobID.new(), int(1))] * (VLOB_READ_REQUEST_ITEMS_LIMIT + 1)
    rep = await coolorg.alice.vlob_read_versions(
        realm_id=coolorg.wksp1_id,
        items=too_many_items,
    )
    assert rep == authenticated_cmds.v4.vlob_read_versions.RepTooManyElements()


async def test_authenticated_vlob_read_versions_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    vlob_id = VlobID.new()
    v1_blob = b"<block content 1>"
    v1_timestamp = DateTime.now()
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=v1_blob,
        timestamp=v1_timestamp,
    )
    assert outcome is None, outcome

    async def do():
        await coolorg.alice.vlob_read_versions(
            realm_id=coolorg.wksp1_id,
            items=[(vlob_id, 1)],
        )

    await authenticated_http_common_errors_tester(do)
