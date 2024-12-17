# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    RevokedUserCertificate,
    ShamirRecoveryDeletionCertificate,
    authenticated_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
)


@pytest.mark.parametrize(
    "kind",
    (
        "single_recipient",
        "multiple_recipients",
    ),
)
async def test_authenticated_shamir_recovery_delete_ok(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend, kind: str
) -> None:
    dt = DateTime.now()

    match kind:
        case "single_recipient":
            author = shamirorg.mallory
            brief = shamirorg.mallory_brief_certificate
        case "multiple_recipients":
            author = shamirorg.alice
            brief = shamirorg.alice_brief_certificate
        case unknown:
            assert False, unknown

    deletion = ShamirRecoveryDeletionCertificate(
        author=author.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=brief.timestamp,
        setup_to_delete_user_id=brief.user_id,
        share_recipients=set(brief.per_recipient_shares.keys()),
    )
    raw_deletion = deletion.dump_and_sign(author.signing_key)

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    expected_topics.shamir_recovery = dt

    rep = await author.shamir_recovery_delete(shamir_recovery_deletion_certificate=raw_deletion)
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepOk()

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize(
    "kind",
    (
        "serialization",
        "signature",
    ),
)
async def test_authenticated_shamir_recovery_delete_invalid_certificate_corrupted(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend, kind: str
) -> None:
    dt = DateTime.now()

    match kind:
        case "serialization":
            certif = b"<dummy>"
        case "signature":
            certif = ShamirRecoveryDeletionCertificate(
                author=shamirorg.alice.device_id,
                timestamp=dt,
                setup_to_delete_timestamp=shamirorg.alice_brief_certificate.timestamp,
                setup_to_delete_user_id=shamirorg.alice.user_id,
                share_recipients=set(shamirorg.alice_brief_certificate.per_recipient_shares.keys()),
            ).dump_and_sign(shamirorg.bob.signing_key)
        case unknown:
            assert False, unknown

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await shamirorg.alice.shamir_recovery_delete(shamir_recovery_deletion_certificate=certif)
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepInvalidCertificateCorrupted()

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_delete_invalid_certificate_user_id_must_be_self(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    dt = DateTime.now()

    certif = ShamirRecoveryDeletionCertificate(
        author=shamirorg.alice.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=shamirorg.alice_brief_certificate.timestamp,
        setup_to_delete_user_id=shamirorg.bob.user_id,
        share_recipients=set(shamirorg.alice_brief_certificate.per_recipient_shares.keys()),
    ).dump_and_sign(shamirorg.alice.signing_key)

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await shamirorg.alice.shamir_recovery_delete(shamir_recovery_deletion_certificate=certif)
    assert (
        rep == authenticated_cmds.v4.shamir_recovery_delete.RepInvalidCertificateUserIdMustBeSelf()
    )

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_delete_shamir_recovery_not_found(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    dt = DateTime.now()

    deletion = ShamirRecoveryDeletionCertificate(
        author=shamirorg.alice.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=DateTime(2000, 1, 1),  # Dummy timestamp
        setup_to_delete_user_id=shamirorg.alice_brief_certificate.user_id,
        share_recipients=set(shamirorg.alice_brief_certificate.per_recipient_shares.keys()),
    )
    raw_deletion = deletion.dump_and_sign(shamirorg.alice.signing_key)

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await shamirorg.alice.shamir_recovery_delete(
        shamir_recovery_deletion_certificate=raw_deletion
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepShamirRecoveryNotFound()

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize(
    "kind",
    (
        "missing_good_recipient",
        "added_bad_recipient",
    ),
)
async def test_authenticated_shamir_recovery_delete_recipients_mismatch(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend, kind: str
) -> None:
    dt = DateTime.now()

    match kind:
        case "missing_good_recipient":
            author = shamirorg.alice
            brief = shamirorg.alice_brief_certificate
            all_recipients = iter(shamirorg.alice_brief_certificate.per_recipient_shares.keys())
            next(all_recipients)
            bad_share_recipients = set(all_recipients)
        case "added_bad_recipient":
            author = shamirorg.mallory
            brief = shamirorg.mallory_brief_certificate
            all_recipients = shamirorg.mallory_brief_certificate.per_recipient_shares.keys()
            assert all_recipients == {shamirorg.mike.user_id}  # Sanity check
            bad_share_recipients = {shamirorg.bob.user_id}  # Replace Mike by Bob in recipients
        case unknown:
            assert False, unknown

    deletion = ShamirRecoveryDeletionCertificate(
        author=author.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=brief.timestamp,
        setup_to_delete_user_id=brief.user_id,
        share_recipients=bad_share_recipients,
    )
    raw_deletion = deletion.dump_and_sign(author.signing_key)

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await author.shamir_recovery_delete(shamir_recovery_deletion_certificate=raw_deletion)
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepRecipientsMismatch()

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_delete_shamir_recovery_already_deleted(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    dt = DateTime.now()

    deletion = ShamirRecoveryDeletionCertificate(
        author=shamirorg.bob.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=shamirorg.bob_brief_certificate.timestamp,
        setup_to_delete_user_id=shamirorg.bob_brief_certificate.user_id,
        share_recipients=set(shamirorg.bob_brief_certificate.per_recipient_shares.keys()),
    )
    raw_deletion = deletion.dump_and_sign(shamirorg.bob.signing_key)

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await shamirorg.bob.shamir_recovery_delete(
        shamir_recovery_deletion_certificate=raw_deletion
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepShamirRecoveryAlreadyDeleted(
        last_shamir_certificate_timestamp=shamirorg.shamir_topic_timestamp
    )

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_delete_timestamp_out_of_ballpark(
    xfail_if_postgresql: None,
    shamirorg: ShamirOrgRpcClients,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)

    deletion = ShamirRecoveryDeletionCertificate(
        author=shamirorg.alice.device_id,
        timestamp=t0,
        setup_to_delete_timestamp=shamirorg.alice_brief_certificate.timestamp,
        setup_to_delete_user_id=shamirorg.alice_brief_certificate.user_id,
        share_recipients=set(shamirorg.alice_brief_certificate.per_recipient_shares.keys()),
    )
    raw_deletion = deletion.dump_and_sign(shamirorg.alice.signing_key)

    rep = await shamirorg.alice.shamir_recovery_delete(
        shamir_recovery_deletion_certificate=raw_deletion
    )
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_delete.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize("kind", ("from_recipient", "from_author", "newer_common_certificate"))
@pytest.mark.usefixtures("ballpark_always_ok")
async def test_authenticated_shamir_recovery_delete_require_greater_timestamp(
    xfail_if_postgresql: None, backend: Backend, shamirorg: ShamirOrgRpcClients, kind: str
) -> None:
    match kind:
        case "from_author":
            # Mallory's shamir topic timestamp correspond to her own shamir
            # recovery setup since she sets it up last.
            author = shamirorg.mallory
            brief = shamirorg.mallory_brief_certificate
            older_timestamp = shamirorg.mallory_brief_certificate.timestamp
            expected_strictly_greater_than = shamirorg.shamir_topic_timestamp
        case "from_recipient":
            # Alice setups her shamir recovery first, so her shamir topic
            # timestamp correspond to Bob's deleted shamir recovery where
            # she is recipient.
            author = shamirorg.alice
            brief = shamirorg.alice_brief_certificate
            older_timestamp = shamirorg.bob_remove_certificate.timestamp
            # The deletion of Alice shamir recovery impacts all recipients, hence the
            # deletion must respect each recipient's shamir recovery topic causality.
            # In practice this means here that Mallory's shamir recovery setup
            # constitutes the lower bound since this setup involves Mallory & Mike
            # that are also recipient of Alice's recovery shamir setup that we
            # want to delete here.
            expected_strictly_greater_than = shamirorg.shamir_topic_timestamp
        case "newer_common_certificate":
            dt = DateTime.now()
            certif = RevokedUserCertificate(
                author=shamirorg.alice.device_id,
                timestamp=dt,
                user_id=shamirorg.bob.user_id,
            )
            outcome = await backend.user.revoke_user(
                now=dt,
                organization_id=shamirorg.organization_id,
                author=shamirorg.alice.device_id,
                author_verify_key=shamirorg.alice.signing_key.verify_key,
                revoked_user_certificate=certif.dump_and_sign(shamirorg.alice.signing_key),
            )
            assert isinstance(outcome, RevokedUserCertificate)

            author = shamirorg.mallory
            brief = shamirorg.mallory_brief_certificate
            expected_strictly_greater_than = older_timestamp = dt
        case unknown:
            assert False, unknown

    deletion = ShamirRecoveryDeletionCertificate(
        author=author.device_id,
        timestamp=older_timestamp,
        setup_to_delete_timestamp=brief.timestamp,
        setup_to_delete_user_id=brief.user_id,
        share_recipients=set(brief.per_recipient_shares.keys()),
    )
    raw_deletion = deletion.dump_and_sign(author.signing_key)

    rep = await author.shamir_recovery_delete(shamir_recovery_deletion_certificate=raw_deletion)
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepRequireGreaterTimestamp(
        strictly_greater_than=expected_strictly_greater_than
    )


async def test_authenticated_shamir_recovery_delete_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        # CoolOrg has no shamir recovery set (and it is the only org the tester supports),
        # so the deletion we do here cannot succeed !
        # However this is okay since the error we want to check should occur before.
        now = DateTime.now()
        deletion = ShamirRecoveryDeletionCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            setup_to_delete_timestamp=DateTime(2000, 1, 1),
            setup_to_delete_user_id=coolorg.alice.user_id,
            share_recipients={coolorg.bob.user_id},
        )
        raw_deletion = deletion.dump_and_sign(coolorg.alice.signing_key)

        await coolorg.alice.shamir_recovery_delete(
            shamir_recovery_deletion_certificate=raw_deletion
        )

    await authenticated_http_common_errors_tester(do)
