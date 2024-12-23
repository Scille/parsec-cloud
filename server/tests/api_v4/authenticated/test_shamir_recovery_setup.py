# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    InvitationToken,
    RevokedUserCertificate,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    UserID,
    authenticated_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
)


async def test_authenticated_shamir_recovery_setup_ok(
    coolorg: CoolorgRpcClients, backend: Backend, xfail_if_postgresql: None
) -> None:
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    expected_topics.shamir_recovery = {
        coolorg.alice.user_id: dt,
        coolorg.mallory.user_id: dt,
    }

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize(
    "kind",
    (
        "really_isolated",
        "related_by_recipient",
    ),
)
@pytest.mark.usefixtures("ballpark_always_ok")
async def test_authenticated_shamir_recovery_setup_isolated_from_other_users(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, kind: str
) -> None:
    # The chosen timestamp would be invalid for Mallory, but should be fine for Bob
    # since Bob is not among Mallory Shamir recovery's recipients.
    mallory_shamir_topic_timestamp = shamirorg.mallory_shamir_topic_timestamp
    assert mallory_shamir_topic_timestamp > shamirorg.bob_shamir_topic_timestamp

    match kind:
        case "really_isolated":
            recipient = shamirorg.alice
            expected_rep = authenticated_cmds.v4.shamir_recovery_setup.RepOk()
        case "related_by_recipient":
            # Mike is already recipient of Mallory Shamir recovery, hence his
            # topic already contains a brief certificate with the conflicting
            # timestamp !
            recipient = shamirorg.mike
            expected_rep = authenticated_cmds.v4.shamir_recovery_setup.RepRequireGreaterTimestamp(
                strictly_greater_than=shamirorg.mike_shamir_topic_timestamp
            )
        case unknown:
            assert False, unknown

    share = ShamirRecoveryShareCertificate(
        author=shamirorg.bob.device_id,
        user_id=shamirorg.bob.user_id,
        timestamp=mallory_shamir_topic_timestamp,
        recipient=recipient.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=shamirorg.bob.device_id,
        user_id=shamirorg.bob.user_id,
        timestamp=mallory_shamir_topic_timestamp,
        threshold=1,
        per_recipient_shares={recipient.user_id: 2},
    )

    rep = await shamirorg.bob.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(shamirorg.bob.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(shamirorg.bob.signing_key)],
    )
    assert rep == expected_rep


@pytest.mark.parametrize(
    "kind",
    (
        "serialization",
        "signature",
    ),
)
async def test_authenticated_shamir_recovery_setup_invalid_certificate_share_corrupted(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients, kind: str
) -> None:
    dt = DateTime.now()
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    ).dump_and_sign(coolorg.alice.signing_key)
    match kind:
        case "serialization":
            share = b"<dummy>"
        case "signature":
            share = ShamirRecoveryShareCertificate(
                author=coolorg.bob.device_id,
                user_id=coolorg.alice.user_id,
                timestamp=dt,
                recipient=coolorg.alice.user_id,
                ciphered_share=b"abc",
            ).dump_and_sign(coolorg.bob.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief,
        shamir_recovery_share_certificates=[share],
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateShareCorrupted()


@pytest.mark.parametrize(
    "kind",
    (
        "serialization",
        "signature",
    ),
)
async def test_authenticated_shamir_recovery_setup_invalid_certificate_brief_corrupted(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients, kind: str
) -> None:
    dt = DateTime.now()
    match kind:
        case "serialization":
            brief = b"<dummy>"
        case "signature":
            brief = ShamirRecoveryBriefCertificate(
                author=coolorg.bob.device_id,
                user_id=coolorg.alice.user_id,
                timestamp=dt,
                threshold=1,
                per_recipient_shares={coolorg.mallory.user_id: 2},
            ).dump_and_sign(coolorg.bob.signing_key)
        case unknown:
            assert False, unknown

    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.alice.user_id,
        ciphered_share=b"abc",
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief,
        shamir_recovery_share_certificates=[share],
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateBriefCorrupted()


async def test_authenticated_shamir_recovery_setup_invalid_certificate_share_recipient_not_in_brief(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients
) -> None:
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.alice.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert (
        rep
        == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateShareRecipientNotInBrief()
    )


async def test_authenticated_shamir_recovery_setup_invalid_certificate_duplicate_share_for_recipient(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients
) -> None:
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )

    share2 = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.bob.user_id: 1, coolorg.mallory.user_id: 2},
    )

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[
            share.dump_and_sign(coolorg.alice.signing_key),
            share2.dump_and_sign(coolorg.alice.signing_key),
        ],
    )
    assert (
        rep
        == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateDuplicateShareForRecipient()
    )


async def test_authenticated_shamir_recovery_setup_invalid_certificate_author_included_as_recipient(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients
) -> None:
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.alice.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.alice.user_id: 2},
    )

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert (
        rep
        == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateAuthorIncludedAsRecipient()
    )


async def test_authenticated_shamir_recovery_setup_invalid_certificate_missing_share_for_recipient(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients
) -> None:
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2, coolorg.bob.user_id: 1},
    )

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert (
        rep
        == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateMissingShareForRecipient()
    )


async def test_authenticated_shamir_recovery_setup_invalid_certificate_share_inconsistent_timestamp(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients
) -> None:
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=DateTime.now(),
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=share.timestamp.add(microseconds=1),
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert (
        rep
        == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateShareInconsistentTimestamp()
    )


async def test_authenticated_shamir_recovery_setup_invalid_certificate_user_id_must_be_self(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.bob.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.bob.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert (
        rep == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidCertificateUserIdMustBeSelf()
    )

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_setup_recipient_not_found(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    dummy_user_id = UserID.new()

    now = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=shamirorg.mike.device_id,
        user_id=shamirorg.mike.user_id,
        timestamp=now,
        recipient=dummy_user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=shamirorg.mike.device_id,
        user_id=shamirorg.mike.user_id,
        timestamp=now,
        threshold=1,
        per_recipient_shares={dummy_user_id: 2},
    )

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await shamirorg.mike.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(shamirorg.mike.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(shamirorg.mike.signing_key)],
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepRecipientNotFound()

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_setup_revoked_recipient(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    # Revoke Bob...

    t0 = DateTime.now()

    certif = RevokedUserCertificate(
        author=shamirorg.alice.device_id,
        timestamp=t0,
        user_id=shamirorg.bob.user_id,
    )
    outcome = await backend.user.revoke_user(
        now=t0,
        organization_id=shamirorg.organization_id,
        author=shamirorg.alice.device_id,
        author_verify_key=shamirorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif.dump_and_sign(shamirorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)

    # ...then use it as recipient for Mike's new shamir recovery setup

    now = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=shamirorg.mike.device_id,
        user_id=shamirorg.mike.user_id,
        timestamp=now,
        recipient=shamirorg.bob.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=shamirorg.mike.device_id,
        user_id=shamirorg.mike.user_id,
        timestamp=now,
        threshold=1,
        per_recipient_shares={shamirorg.bob.user_id: 2},
    )

    expected_topics = await backend.organization.test_dump_topics(shamirorg.organization_id)

    rep = await shamirorg.mike.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(shamirorg.mike.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(shamirorg.mike.signing_key)],
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepRevokedRecipient(
        last_common_certificate_timestamp=t0
    )

    topics = await backend.organization.test_dump_topics(shamirorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_setup_shamir_recovery_already_exists(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients
) -> None:
    # Setup previous shamir
    dt = DateTime.now()

    share = ShamirRecoveryShareCertificate(
        author=shamirorg.alice.device_id,
        user_id=shamirorg.alice.user_id,
        timestamp=dt,
        recipient=shamirorg.mallory.user_id,
        ciphered_share=b"abc",
    )

    brief = ShamirRecoveryBriefCertificate(
        author=shamirorg.alice.device_id,
        user_id=shamirorg.alice.user_id,
        timestamp=dt,
        threshold=1,
        per_recipient_shares={shamirorg.mallory.user_id: 2},
    )

    # Attempt to overwrite setup

    rep = await shamirorg.alice.shamir_recovery_setup(
        ciphered_data=b"def",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(shamirorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(shamirorg.alice.signing_key)],
    )
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShamirRecoveryAlreadyExists(
        last_shamir_certificate_timestamp=shamirorg.alice_shamir_topic_timestamp
    )


async def test_authenticated_shamir_recovery_setup_timestamp_out_of_ballpark(
    xfail_if_postgresql: None, coolorg: CoolorgRpcClients, timestamp_out_of_ballpark: DateTime
) -> None:
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=timestamp_out_of_ballpark,
        recipient=coolorg.alice.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=timestamp_out_of_ballpark,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    rep = await coolorg.alice.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
    )
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_setup.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize("kind", ("from_recipient", "from_author"))
@pytest.mark.usefixtures("ballpark_always_ok")
async def test_authenticated_shamir_recovery_setup_require_greater_timestamp(
    xfail_if_postgresql: None, shamirorg: ShamirOrgRpcClients, kind: str
) -> None:
    match kind:
        case "from_recipient":
            # Mike has no shamir, but is recipient of Mallory's shamir
            author = shamirorg.mike
            older_timestamp = shamirorg.mallory_brief_certificate.timestamp
        case "from_author":
            # Bob's last shamir interaction was to remove its own shamir
            author = shamirorg.bob
            older_timestamp = shamirorg.bob_remove_certificate.timestamp
        case unknown:
            assert False, unknown

    # ...set the shamir again with a clashing timestamp

    share = ShamirRecoveryShareCertificate(
        author=author.device_id,
        user_id=author.user_id,
        timestamp=older_timestamp,
        recipient=shamirorg.alice.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=author.device_id,
        user_id=author.user_id,
        timestamp=older_timestamp,
        threshold=1,
        per_recipient_shares={shamirorg.alice.user_id: 2},
    )

    rep = await author.shamir_recovery_setup(
        ciphered_data=b"abc",
        reveal_token=InvitationToken.new(),
        shamir_recovery_brief_certificate=brief.dump_and_sign(author.signing_key),
        shamir_recovery_share_certificates=[share.dump_and_sign(author.signing_key)],
    )
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_setup.RepRequireGreaterTimestamp)


async def test_authenticated_shamir_recovery_setup_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        dt = DateTime.now()
        share = ShamirRecoveryShareCertificate(
            author=coolorg.alice.device_id,
            user_id=coolorg.alice.user_id,
            timestamp=dt,
            recipient=coolorg.mallory.user_id,
            ciphered_share=b"abc",
        )
        brief = ShamirRecoveryBriefCertificate(
            author=coolorg.alice.device_id,
            user_id=coolorg.alice.user_id,
            timestamp=dt,
            threshold=1,
            per_recipient_shares={coolorg.mallory.user_id: 2},
        )

        await coolorg.alice.shamir_recovery_setup(
            ciphered_data=b"abc",
            reveal_token=InvitationToken.new(),
            shamir_recovery_brief_certificate=brief.dump_and_sign(coolorg.alice.signing_key),
            shamir_recovery_share_certificates=[share.dump_and_sign(coolorg.alice.signing_key)],
        )

    await authenticated_http_common_errors_tester(do)
