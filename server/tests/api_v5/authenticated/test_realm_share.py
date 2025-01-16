# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Awaitable, Callable

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKey,
    SecretKeyAlgorithm,
    UserID,
    UserProfile,
    VlobID,
    authenticated_cmds,
)
from parsec.components.realm import KeysBundle
from parsec.events import EventRealmCertificate
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    alice_gives_profile,
    generate_realm_role_certificate,
    get_last_realm_certificate_timestamp,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


@pytest.mark.parametrize(
    "kind",
    (
        "as_manager_giving_reader_access",
        "as_manager_giving_contributor_access",
        "as_manager_switching_from_contributor_to_reader",
        "as_owner_giving_reader_access",
        "as_owner_giving_contributor_access",
        "as_owner_giving_manager_access",
        "as_owner_giving_owner_access",
        "as_owner_switching_from_owner_to_reader",
        "as_owner_switching_from_manager_to_reader",
    ),
)
async def test_authenticated_realm_share_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "as_manager_giving_reader_access":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            role = RealmRole.READER
            author = coolorg.bob

        case "as_manager_giving_contributor_access":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            role = RealmRole.CONTRIBUTOR
            author = coolorg.bob

        case "as_manager_switching_from_contributor_to_reader":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            role = RealmRole.READER
            author = coolorg.bob

        case "as_owner_giving_reader_access":
            role = RealmRole.READER
            author = coolorg.alice

        case "as_owner_giving_contributor_access":
            role = RealmRole.CONTRIBUTOR
            author = coolorg.alice

        case "as_owner_giving_manager_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `MANAGER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            role = RealmRole.MANAGER
            author = coolorg.alice

        case "as_owner_giving_owner_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `OWNER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            role = RealmRole.MANAGER
            author = coolorg.alice

        case "as_owner_switching_from_owner_to_reader":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `OWNER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.mallory.user_id, RealmRole.OWNER)
            role = RealmRole.READER
            author = coolorg.alice

        case "as_owner_switching_from_manager_to_reader":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `MANAGER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.MANAGER
            )
            role = RealmRole.READER
            author = coolorg.alice

        case unknown:
            assert False, unknown

    certif = RealmRoleCertificate(
        author=author.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.mallory.user_id,
        role=role,
    )
    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    expected_topics.realms[coolorg.wksp1_id] = certif.timestamp

    with backend.event_bus.spy() as spy:
        rep = await author.realm_share(
            key_index=1,
            realm_role_certificate=certif.dump_and_sign(author.signing_key),
            # Keys bundle access is not readable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<mallory keys bundle access>",
        )
        assert rep == authenticated_cmds.latest.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=False,
            )
        )

    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert mallory_realms[coolorg.wksp1_id] == role

    keys_bundle_info = await backend.realm.get_keys_bundle(
        organization_id=coolorg.organization_id,
        author=coolorg.mallory.device_id,
        realm_id=coolorg.wksp1_id,
        key_index=1,
    )
    assert isinstance(keys_bundle_info, KeysBundle)
    assert keys_bundle_info.keys_bundle_access == b"<mallory keys bundle access>"

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_realm_share_role_already_granted(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.READER

    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
        user_id=coolorg.bob.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepRoleAlreadyGranted(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            coolorg.testbed_template, coolorg.wksp1_id
        )
    )


@pytest.mark.parametrize(
    "kind",
    (
        "dummy_certif",
        "role_none",
        "self_share_and_only_owner",
        "self_share_with_other_owners",
    ),
)
async def test_authenticated_realm_share_invalid_certificate(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "dummy_certif":
            certif = b"<dummy>"

        case "role_none":
            certif = generate_realm_role_certificate(
                coolorg, user_id=coolorg.bob.user_id, role=None
            ).dump_and_sign(coolorg.alice.signing_key)

        case "self_share_and_only_owner":
            certif = generate_realm_role_certificate(
                coolorg, user_id=coolorg.alice.user_id, role=RealmRole.CONTRIBUTOR
            ).dump_and_sign(coolorg.alice.signing_key)

        case "self_share_with_other_owners":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.OWNER)
            certif = generate_realm_role_certificate(
                coolorg, user_id=coolorg.alice.user_id, role=RealmRole.CONTRIBUTOR
            ).dump_and_sign(coolorg.alice.signing_key)

        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepInvalidCertificate()


@pytest.mark.parametrize(
    "kind",
    (
        "key_index_too_old",
        "key_index_too_far_forward",
        "key_index_is_zero",
        "realm_had_no_key_rotation_yet",
    ),
)
async def test_authenticated_realm_share_key_bad_key_index(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "key_index_too_old":
            # Do another key rotation so that current key index is 2
            t0 = DateTime.now()
            key = SecretKey.generate()
            key_canary = key.encrypt(b"")
            certif = RealmKeyRotationCertificate(
                author=coolorg.alice.device_id,
                timestamp=t0,
                realm_id=coolorg.wksp1_id,
                key_index=2,
                encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
                hash_algorithm=HashAlgorithm.SHA256,
                key_canary=key_canary,
            )
            realm_key_rotation_certificate = certif.dump_and_sign(coolorg.alice.signing_key)
            outcome = await backend.realm.rotate_key(
                now=t0,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                keys_bundle=b"<keys bundle>",
                per_participant_keys_bundle_access={
                    coolorg.alice.user_id: b"<alice keys bundle access>",
                    coolorg.bob.user_id: b"<bob keys bundle access>",
                },
                realm_key_rotation_certificate=realm_key_rotation_certificate,
            )
            assert isinstance(outcome, RealmKeyRotationCertificate)
            bad_key_index = 1
            wksp_id = coolorg.wksp1_id
            wksp_last_certificate_timestamp = t0
        case "key_index_too_far_forward":
            bad_key_index = 2
            wksp_id = coolorg.wksp1_id
            wksp_last_certificate_timestamp = get_last_realm_certificate_timestamp(
                coolorg.testbed_template, wksp_id
            )
        case "key_index_is_zero":
            bad_key_index = 0
            wksp_id = coolorg.wksp1_id
            wksp_last_certificate_timestamp = get_last_realm_certificate_timestamp(
                coolorg.testbed_template, wksp_id
            )
        case "realm_had_no_key_rotation_yet":
            t0 = DateTime.now()
            wksp_id = VlobID.new()
            certif = RealmRoleCertificate(
                author=coolorg.alice.device_id,
                timestamp=t0,
                realm_id=wksp_id,
                role=RealmRole.OWNER,
                user_id=coolorg.alice.user_id,
            )
            outcome = await backend.realm.create(
                now=t0,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            )
            assert isinstance(outcome, RealmRoleCertificate)
            bad_key_index = 1
            wksp_last_certificate_timestamp = t0
        case _:
            assert False

    certif = generate_realm_role_certificate(
        coolorg,
        user_id=coolorg.bob.user_id,
        role=RealmRole.CONTRIBUTOR,
        timestamp=DateTime.now(),
        realm_id=wksp_id,
    )
    rep = await coolorg.alice.realm_share(
        key_index=bad_key_index,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


async def test_authenticated_realm_share_recipient_revoked(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    # 1) Revoke user Mallory

    revoked_timestamp = DateTime.now()
    outcome = await backend.user.revoke_user(
        now=revoked_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=RevokedUserCertificate(
            author=coolorg.alice.device_id,
            timestamp=revoked_timestamp,
            user_id=coolorg.mallory.user_id,
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)

    # 2) Try to share with mallory (which is now revoked)

    certif = generate_realm_role_certificate(
        coolorg, user_id=coolorg.mallory.user_id, role=RealmRole.READER, timestamp=DateTime.now()
    )
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepRecipientRevoked()


async def test_authenticated_realm_share_bad_key_index(
    coolorg: CoolorgRpcClients,
) -> None:
    certif = generate_realm_role_certificate(
        coolorg,
        user_id=coolorg.mallory.user_id,
        role=RealmRole.READER,
    )
    rep = await coolorg.alice.realm_share(
        key_index=2,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepBadKeyIndex(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            coolorg.testbed_template, coolorg.wksp1_id
        )
    )


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "as_contributor",
        "as_manager_giving_owner_access",
        "as_manager_giving_manager_access",
        "never_allowed",
        "no_longer_allowed",
        "bad_key_index_and_not_allowed",
        "already_shared_and_not_allowed",
        "role_incompatible_with_outsider_and_not_allowed",
        "require_greater_timestamp_and_not_allowed",
    ),
)
async def test_authenticated_realm_share_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    key_index = 1
    match kind:
        case "as_reader":
            # Nothing to do as Bob is reader in wksp1
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.alice.user_id,
                role=RealmRole.READER,
            )
            author = coolorg.bob

        case "as_contributor":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR
            )
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.user_id,
                role=RealmRole.READER,
            )
            author = coolorg.bob

        case "as_manager_giving_owner_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `OWNER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.user_id,
                role=RealmRole.OWNER,
            )
            author = coolorg.bob

        case "as_manager_giving_manager_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `MANAGER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.user_id,
                role=RealmRole.MANAGER,
            )
            author = coolorg.bob

        case "never_allowed":
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.CONTRIBUTOR,
            )
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=RealmRole.CONTRIBUTOR
            )
            certif = RealmRoleCertificate(
                author=coolorg.alice.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.user_id,
                role=RealmRole.READER,
            )
            author = coolorg.alice

        case "bad_key_index_and_not_allowed":
            key_index = 42
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.CONTRIBUTOR,
            )
            author = coolorg.mallory

        case "already_shared_and_not_allowed":
            # Nothing to do as Bob is reader in wksp1
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.READER,
            )
            author = coolorg.mallory

        case "role_incompatible_with_outsider_and_not_allowed":
            await alice_gives_profile(coolorg, backend, coolorg.bob.user_id, UserProfile.OUTSIDER)
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.MANAGER,
            )
            author = coolorg.mallory

        case "require_greater_timestamp_and_not_allowed":
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.CONTRIBUTOR,
            )
            # Just create any certificate in the realm
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, None)
            author = coolorg.mallory

        case unknown:
            assert False, unknown

    rep = await author.realm_share(
        key_index=key_index,
        realm_role_certificate=certif.dump_and_sign(author.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<recipient keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepAuthorNotAllowed()


async def test_authenticated_realm_share_realm_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    bad_realm_id = VlobID.new()
    certif = generate_realm_role_certificate(
        coolorg,
        user_id=coolorg.mallory.user_id,
        realm_id=bad_realm_id,
        role=RealmRole.READER,
    )
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepRealmNotFound()


async def test_authenticated_realm_share_recipient_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    bad_user_id = UserID.new()
    certif = generate_realm_role_certificate(
        coolorg,
        user_id=bad_user_id,
        role=RealmRole.READER,
    )
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepRecipientNotFound()


@pytest.mark.parametrize("role", (RealmRole.MANAGER, RealmRole.OWNER))
async def test_authenticated_realm_share_role_incompatible_with_outsider(
    coolorg: CoolorgRpcClients,
    role: RealmRole,
) -> None:
    certif = generate_realm_role_certificate(coolorg, user_id=coolorg.mallory.user_id, role=role)
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepRoleIncompatibleWithOutsider()


async def test_authenticated_realm_share_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = generate_realm_role_certificate(
        coolorg,
        user_id=coolorg.bob.user_id,
        role=RealmRole.CONTRIBUTOR,
        timestamp=timestamp_out_of_ballpark,
    )
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert isinstance(rep, authenticated_cmds.latest.realm_share.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize("timestamp_kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_realm_share_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_kind: str,
    alice_generated_realm_wksp1_data: Callable[[DateTime], Awaitable[None]],
) -> None:
    # 0) Bob must become OWNER to be able to share with Alice

    t0 = DateTime.now().subtract(seconds=100)
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.bob.user_id,
        timestamp=t0,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.OWNER,
    )
    await backend.realm.share(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )

    # 1) Create some data (e.g. certificate, vlob) with a given timestamp

    now = DateTime.now()
    match timestamp_kind:
        case "same_timestamp":
            realm_unshare_timestamp = now
        case "previous_timestamp":
            realm_unshare_timestamp = now.subtract(seconds=1)
        case unknown:
            assert False, unknown

    await alice_generated_realm_wksp1_data(now)

    # 2) Create realm share certificate where timestamp is clashing with the previous data

    certif = RealmRoleCertificate(
        author=coolorg.bob.device_id,
        timestamp=realm_unshare_timestamp,
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.alice.user_id,
        role=RealmRole.READER,
    )
    rep = await coolorg.bob.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<alice keys bundle access>",
    )
    assert rep == authenticated_cmds.latest.realm_share.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )


async def test_authenticated_realm_share_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        alice_unshare_bob_certificate = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=DateTime.now(),
            realm_id=coolorg.wksp1_id,
            user_id=coolorg.bob.user_id,
            role=RealmRole.CONTRIBUTOR,
        )

        await coolorg.alice.realm_share(
            realm_role_certificate=alice_unshare_bob_certificate.dump_and_sign(
                coolorg.alice.signing_key
            ),
            key_index=1,
            recipient_keys_bundle_access=b"<bob keys bundle access>",
        )

    await authenticated_http_common_errors_tester(do)
