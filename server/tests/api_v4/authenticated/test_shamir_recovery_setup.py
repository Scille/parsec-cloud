# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    RevokedUserCertificate,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    ShamirRevealToken,
    authenticated_cmds,
)
from tests.common import Backend, CoolorgRpcClients
from tests.common.client import setup_shamir_for_coolorg


async def test_authenticated_shamir_recovery_setup_ok(
    coolorg: CoolorgRpcClients, with_postgresql: bool
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

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()


async def test_authenticated_shamir_recovery_setup_share_incoherent_timestamp(
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
        timestamp=DateTime.now(),
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShareIncoherentTimestamp()


async def test_authenticated_shamir_recovery_setup_shamir_setup_already_exists(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    # Setup previous shamir
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
    await setup_shamir_for_coolorg(coolorg)

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
    # attempt to overwrite setup
    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"def",
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShamirSetupAlreadyExists()


async def test_authenticated_shamir_recovery_setup_brief_invalid_data(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        ShamirRevealToken.new(),
        b"ijk",
        [b"lmn"],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
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
        ShamirRevealToken.new(),
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
        ShamirRevealToken.new(),
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
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepInvalidRecipient()


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
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepMissingShareForRecipient()


async def test_authenticated_shamir_recovery_setup_require_greater_timestamp(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
    older_timestamp = DateTime.now()
    # initial setup to have a previous shamir
    await setup_shamir_for_coolorg(coolorg)

    share = ShamirRecoveryShareCertificate(
        author=coolorg.bob.device_id,
        user_id=coolorg.bob.user_id,
        timestamp=older_timestamp,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.bob.device_id,
        user_id=coolorg.bob.user_id,
        timestamp=older_timestamp,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.bob.signing_key),
        [share.dump_and_sign(coolorg.bob.signing_key)],
    )
    rep = await coolorg.bob.shamir_recovery_setup(setup)
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_setup.RepRequireGreaterTimestamp)


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
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepShareInvalidData()


async def test_authenticated_shamir_recovery_setup_threshold_greater_than_total_shares(
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
        threshold=3,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepThresholdGreaterThanTotalShares()


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
        ShamirRevealToken.new(),
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
        ShamirRevealToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [share.dump_and_sign(coolorg.alice.signing_key)],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_setup.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark
