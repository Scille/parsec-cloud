# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Awaitable, Callable

import pytest

from parsec._parsec import (
    DateTime,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserID,
    UserProfile,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventRealmCertificate
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    alice_gives_profile,
    generate_realm_role_certificate,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


@pytest.mark.parametrize(
    "kind",
    (
        "revoked_user",
        "as_manager_removing_reader_access",
        "as_manager_removing_contributor_access",
        "as_owner_removing_reader_access",
        "as_owner_removing_contributor_access",
        "as_owner_removing_manager_access",
        "as_owner_removing_owner_access",
    ),
)
async def test_authenticated_realm_unshare_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "revoked_user":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            await backend.user.revoke_user(
                now=DateTime.now(),
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                revoked_user_certificate=RevokedUserCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=DateTime.now(),
                    user_id=coolorg.mallory.user_id,
                ).dump_and_sign(coolorg.alice.signing_key),
            )
            author = coolorg.alice

        case "as_manager_removing_reader_access":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            author = coolorg.bob

        case "as_manager_removing_contributor_access":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
            )
            author = coolorg.bob

        case "as_owner_removing_reader_access":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            author = coolorg.alice

        case "as_owner_removing_contributor_access":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
            )
            author = coolorg.alice

        case "as_owner_removing_manager_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `MANAGER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.MANAGER
            )
            author = coolorg.alice

        case "as_owner_removing_owner_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `OWNER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.mallory.user_id, RealmRole.OWNER)
            author = coolorg.alice

        case unknown:
            assert False, unknown

    certif = RealmRoleCertificate(
        author=author.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.mallory.user_id,
        role=None,
    )
    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    expected_topics.realms[coolorg.wksp1_id] = certif.timestamp

    with backend.event_bus.spy() as spy:
        rep = await author.realm_unshare(
            realm_role_certificate=certif.dump_and_sign(author.signing_key)
        )
        assert rep == authenticated_cmds.v4.realm_unshare.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=True,
            )
        )

    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert coolorg.wksp1_id not in mallory_realms

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "as_contributor",
        "as_manager_removing_owner_access",
        "as_manager_removing_manager_access",
        "never_allowed",
        "no_longer_allowed",
        "already_unshared_and_not_allowed",
        "require_greater_timestamp_and_not_allowed",
    ),
)
async def test_authenticated_realm_unshare_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "as_reader":
            # Nothing to do as Bob is reader in wksp1
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.alice.user_id,
                role=None,
            )
            author = coolorg.bob

        case "as_contributor":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
            )
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=None,
            )
            author = coolorg.mallory

        case "as_manager_removing_owner_access":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.alice.user_id,
                role=None,
            )
            author = coolorg.bob

        case "as_manager_removing_manager_access":
            # Mallory starts as `OUTSIDER`, which is incompatible with a `MANAGER` role
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.MANAGER
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            certif = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.user_id,
                role=None,
            )
            author = coolorg.bob

        case "never_allowed":
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=None,
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
                user_id=coolorg.bob.user_id,
                role=None,
            )
            author = coolorg.alice

        case "already_unshared_and_not_allowed":
            t0 = DateTime.now()
            certif = RealmRoleCertificate(
                author=coolorg.alice.device_id,
                timestamp=t0,
                realm_id=coolorg.wksp1_id,
                role=None,
                user_id=coolorg.bob.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
            outcome = await backend.realm.unshare(
                now=t0,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                realm_role_certificate=certif,
            )
            assert isinstance(outcome, RealmRoleCertificate)

            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=None,
            )
            author = coolorg.mallory

        case "require_greater_timestamp_and_not_allowed":
            certif = RealmRoleCertificate(
                author=coolorg.mallory.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=None,
            )
            # Just create any certificate in the realm
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, None)
            author = coolorg.mallory

        case unknown:
            assert False, unknown

    rep = await author.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(author.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepAuthorNotAllowed()


async def test_authenticated_realm_unshare_realm_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    bad_realm_id = VlobID.new()
    certif = generate_realm_role_certificate(
        coolorg, user_id=coolorg.bob.user_id, realm_id=bad_realm_id, role=None
    )
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRealmNotFound()


async def test_authenticated_realm_unshare_recipient_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    bad_recipient = UserID.new()
    certif = generate_realm_role_certificate(
        coolorg, user_id=bad_recipient, timestamp=DateTime.now(), role=None
    )
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRecipientNotFound()


async def test_authenticated_realm_unshare_recipient_already_unshared(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    # 1) Use certificate to unshare Bob via the backend

    last_realm_certificate_timestamp = DateTime.now()
    await wksp1_alice_gives_role(
        coolorg,
        backend,
        recipient=coolorg.bob.user_id,
        new_role=None,
        now=last_realm_certificate_timestamp,
    )

    # 2) Try to unshare Bob again, now via the API

    certif = generate_realm_role_certificate(coolorg, user_id=coolorg.bob.user_id, role=None)
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRecipientAlreadyUnshared(
        last_realm_certificate_timestamp=last_realm_certificate_timestamp
    )


@pytest.mark.parametrize(
    "kind",
    (
        "dummy_certif",
        "invalid_role",
        "self_unshare_and_only_owner",
        "self_unshare_with_other_owners",
    ),
)
async def test_authenticated_realm_unshare_invalid_certificate(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "dummy_certif":
            certif = b"<dummy>"

        case "invalid_role":
            certif = generate_realm_role_certificate(
                coolorg, user_id=coolorg.bob.user_id, role=RealmRole.READER
            ).dump_and_sign(coolorg.alice.signing_key)

        case "self_unshare_and_only_owner":
            certif = generate_realm_role_certificate(
                coolorg, user_id=coolorg.alice.user_id, role=None
            ).dump_and_sign(coolorg.alice.signing_key)

        case "self_unshare_with_other_owners":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.OWNER)
            certif = generate_realm_role_certificate(
                coolorg, user_id=coolorg.alice.user_id, role=None
            ).dump_and_sign(coolorg.alice.signing_key)

        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif,
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepInvalidCertificate()


async def test_authenticated_realm_unshare_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = generate_realm_role_certificate(
        coolorg, user_id=coolorg.bob.user_id, role=None, timestamp=timestamp_out_of_ballpark
    )
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(rep, authenticated_cmds.v4.realm_unshare.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize("timestamp_kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_realm_unshare_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_kind: str,
    alice_generated_realm_wksp1_data: Callable[[DateTime], Awaitable[None]],
) -> None:
    # 0) Bob must become OWNER to be able to unshare with Alice

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

    # 2) Create realm unshare certificate where timestamp is clashing with the previous data

    certif = RealmRoleCertificate(
        author=coolorg.bob.device_id,
        timestamp=realm_unshare_timestamp,
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.alice.user_id,
        role=None,
    )
    rep = await coolorg.bob.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )


async def test_authenticated_realm_unshare_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        alice_unshare_bob_certificate = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=DateTime.now(),
            realm_id=coolorg.wksp1_id,
            user_id=coolorg.bob.user_id,
            role=None,
        )

        await coolorg.alice.realm_unshare(
            realm_role_certificate=alice_unshare_bob_certificate.dump_and_sign(
                coolorg.alice.signing_key
            ),
        )

    await authenticated_http_common_errors_tester(do)
