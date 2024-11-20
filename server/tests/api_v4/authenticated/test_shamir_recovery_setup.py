# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    InvitationToken,
    RevokedUserCertificate,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    authenticated_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    ShamirOrgRpcClients,
)


async def test_authenticated_shamir_recovery_setup_ok(
    coolorg: CoolorgRpcClients, backend: Backend, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.xfail("TODO: postgre not implemented yet")
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
    expected_topics.shamir_recovery = share.timestamp

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_shamir_recovery_setup_share_inconsistent_timestamp(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.xfail("TODO: postgre not implemented yet")
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShareInconsistentTimestamp()


@pytest.mark.xfail(
    reason="TODO: currently there is a unique shamir topic, we should switch to a per-user shamir topic instead"
)
# Cannot use `with_postgresql` fixture since `shamirorg` init uses fonctions non-implemented in postgresql
@pytest.mark.skipif(
    "bool(config.getoption('--postgresql'))", reason="TODO: postgre not implemented yet"
)
async def test_authenticated_shamir_recovery_setup_shamir_setup_already_exists(
    shamirorg: ShamirOrgRpcClients,
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
    # attempt to overwrite setup
    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"def",
        InvitationToken.new(),
        brief.dump_and_sign(shamirorg.alice.signing_key),
        [share.dump_and_sign(shamirorg.alice.signing_key)],
    )
    rep = await shamirorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShamirSetupAlreadyExists(
        last_shamir_certificate_timestamp=shamirorg.alice_brief_certificate.timestamp
    )


async def test_authenticated_shamir_recovery_setup_brief_invalid_data(
    minimalorg: MinimalorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        b"ijk",
        [b"lmn"],
    )
    rep = await minimalorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepBriefInvalidData()


async def test_authenticated_shamir_recovery_setup_author_included_as_recipient(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepAuthorIncludedAsRecipient()


async def test_authenticated_shamir_recovery_setup_duplicate_share_for_recipient(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
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
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [
            share.dump_and_sign(coolorg.alice.signing_key),
            share2.dump_and_sign(coolorg.alice.signing_key),
        ],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepDuplicateShareForRecipient()


async def test_authenticated_shamir_recovery_setup_invalid_recipient(
    coolorg: CoolorgRpcClients, with_postgresql: bool, backend: Backend
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
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

    # Revoke mallory to make them invalid
    t1 = DateTime.now()
    certif1 = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        user_id=coolorg.mallory.user_id,
    )

    outcome = await backend.user.revoke_user(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif1.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidRecipient(
        coolorg.mallory.user_id
    )


async def test_authenticated_shamir_recovery_setup_missing_share_for_recipient(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepMissingShareForRecipient()


@pytest.mark.parametrize("kind", ("from_recipient", "from_author"))
@pytest.mark.usefixtures("ballpark_always_ok")
# Cannot use `with_postgresql` fixture since `shamirorg` init uses fonctions non-implemented in postgresql
@pytest.mark.skipif(
    "bool(config.getoption('--postgresql'))", reason="TODO: postgre not implemented yet"
)
async def test_authenticated_shamir_recovery_setup_require_greater_timestamp(
    shamirorg: ShamirOrgRpcClients, kind: str
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(author.signing_key),
        [share.dump_and_sign(author.signing_key)],
    )
    rep = await author.shamir_recovery_setup(setup)
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_setup.RepRequireGreaterTimestamp)


@pytest.mark.xfail(
    reason="TODO: currently there is a unique shamir topic, we should switch to a per-user shamir topic instead"
)
@pytest.mark.usefixtures("ballpark_always_ok")
# Cannot use `with_postgresql` fixture since `shamirorg` init uses fonctions non-implemented in postgresql
@pytest.mark.skipif(
    "bool(config.getoption('--postgresql'))", reason="TODO: postgre not implemented yet"
)
async def test_authenticated_shamir_recovery_setup_isolated_from_other_users(
    shamirorg: ShamirOrgRpcClients,
) -> None:
    # The chosen timestamp would be invalid for Bob, but should be fine for Alice
    # since both are isolated from each other.
    bob_shamir_topic_timestamp = shamirorg.bob_shamir_topic_timestamp
    assert bob_shamir_topic_timestamp > shamirorg.alice_shamir_topic_timestamp

    share = ShamirRecoveryShareCertificate(
        author=shamirorg.alice.device_id,
        user_id=shamirorg.alice.user_id,
        timestamp=bob_shamir_topic_timestamp,
        recipient=shamirorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=shamirorg.alice.device_id,
        user_id=shamirorg.alice.user_id,
        timestamp=bob_shamir_topic_timestamp,
        threshold=1,
        per_recipient_shares={shamirorg.mallory.user_id: 2},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(shamirorg.alice.signing_key),
        [share.dump_and_sign(shamirorg.alice.signing_key)],
    )
    rep = await shamirorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()


async def test_authenticated_shamir_recovery_setup_share_invalid_data(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
    dt = DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.mallory.device_id,
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShareInvalidData()


async def test_authenticated_shamir_recovery_setup_share_recipient_not_in_brief(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShareRecipientNotInBrief()


async def test_authenticated_shamir_recovery_setup_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients, with_postgresql: bool, timestamp_out_of_ballpark: DateTime
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_setup.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


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

        setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
            b"abc",
            InvitationToken.new(),
            brief.dump_and_sign(coolorg.alice.signing_key),
            [share.dump_and_sign(coolorg.alice.signing_key)],
        )
        await coolorg.alice.shamir_recovery_setup(setup)

    await authenticated_http_common_errors_tester(do)
