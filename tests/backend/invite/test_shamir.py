# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

import pytest

from parsec._parsec import (
    BackendInvitationAddr,
    DateTime,
    HashDigest,
    InvitationDeletedReason,
    InvitationType,
    Invite3aClaimerSignifyTrustRepOk,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3bClaimerWaitPeerTrustRepOk,
    Invite3bGreeterSignifyTrustRepOk,
    InviteDeleteRepOk,
    InviteInfoRepOk,
    InviteListRepOk,
    InviteNewRepNotAvailable,
    InviteNewRepOk,
    InviteShamirRecoveryRevealRepOk,
    LocalDevice,
    PrivateKey,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryOthersListRepNotAllowed,
    ShamirRecoveryOthersListRepOk,
    ShamirRecoveryRecipient,
    ShamirRecoverySelfInfoRepOk,
    ShamirRecoverySetup,
    ShamirRecoverySetupRepOk,
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareData,
    Share,
    Sharks,
    UserProfile,
    VerifyKey,
)
from tests.backend.common import (
    invite_delete,
    invite_info,
    invite_list,
    invite_new,
    invite_shamir_recovery_reveal,
    shamir_recovery_others_list,
    shamir_recovery_self_info,
    shamir_recovery_setup,
)
from tests.backend.invite.test_exchange import PeerController, exchange_testbed_context
from tests.common.fixtures_customisation import customize_fixtures


@pytest.mark.trio
async def test_shamir_recovery(alice: LocalDevice, bob: LocalDevice, alice_ws, bob_ws, invited_ws):
    alice_brief = ShamirRecoveryBriefCertificate(
        alice.device_id,
        DateTime.now(),
        threshold=1,
        per_recipient_shares={alice.device_id.user_id: 1, bob.device_id.user_id: 1},
    ).dump()
    setup = ShamirRecoverySetup(
        b"alice_ciphered_data", b"alice_reveal_token", alice_brief, [b"share0", b"share1"]
    )
    rep = await shamir_recovery_setup(alice_ws, setup)
    assert isinstance(rep, ShamirRecoverySetupRepOk)

    bob_brief = ShamirRecoveryBriefCertificate(
        bob.device_id,
        DateTime.now(),
        threshold=1,
        per_recipient_shares={alice.device_id.user_id: 1, bob.device_id.user_id: 1},
    ).dump()
    setup = ShamirRecoverySetup(
        b"bob_ciphered_data", b"bob_reveal_token", bob_brief, [b"share0", b"share1"]
    )
    rep = await shamir_recovery_setup(bob_ws, setup)
    assert isinstance(rep, ShamirRecoverySetupRepOk)

    rep = await shamir_recovery_self_info(alice_ws)
    assert isinstance(rep, ShamirRecoverySelfInfoRepOk)
    assert rep.self_info == alice_brief

    rep = await shamir_recovery_self_info(bob_ws)
    assert isinstance(rep, ShamirRecoverySelfInfoRepOk)
    assert rep.self_info == bob_brief

    rep = await shamir_recovery_others_list(alice_ws)
    assert isinstance(rep, ShamirRecoveryOthersListRepOk)
    assert set(rep.others) == {alice_brief, bob_brief}

    rep = await invite_shamir_recovery_reveal(invited_ws, b"alice_reveal_token")
    assert isinstance(rep, InviteShamirRecoveryRevealRepOk)
    assert rep.ciphered_data == b"alice_ciphered_data"


@pytest.mark.trio
async def test_shamir_recovery_others_list_not_allowed(bob_ws):
    rep = await shamir_recovery_others_list(bob_ws)
    assert isinstance(rep, ShamirRecoveryOthersListRepNotAllowed)


@pytest.mark.trio
async def test_invite_shamir_not_available(bob: LocalDevice, alice_ws):
    rep = await invite_new(
        alice_ws, type=InvitationType.SHAMIR_RECOVERY, claimer_user_id=bob.user_id
    )
    assert isinstance(rep, InviteNewRepNotAvailable)


async def _shamir_exchange(
    claimer_ctlr: PeerController,
    greeter_ctlr: PeerController,
    claimer_verify_key: VerifyKey,
    greeter_private_key: PrivateKey,
    raw_certificate: bytes,
) -> None:
    # Step 1
    await claimer_ctlr.send_order("1_wait_peer")
    await greeter_ctlr.send_order("1_wait_peer")

    greeter_rep = await greeter_ctlr.get_result()
    claimer_rep = await claimer_ctlr.get_result()

    # Step 2
    await claimer_ctlr.send_order("2a_send_hashed_nonce")
    await greeter_ctlr.send_order("2a_get_hashed_nonce")

    greeter_rep = await greeter_ctlr.get_result()
    assert greeter_rep.claimer_hashed_nonce == HashDigest.from_data(b"<claimer_nonce>")
    await greeter_ctlr.send_order("2b_send_nonce")

    claimer_rep = await claimer_ctlr.get_result()
    assert claimer_rep.greeter_nonce == b"<greeter_nonce>"
    await claimer_ctlr.send_order("2b_send_nonce")

    greeter_rep = await greeter_ctlr.get_result()
    assert greeter_rep.claimer_nonce == b"<claimer_nonce>"
    claimer_rep = await claimer_ctlr.get_result()

    # Step 3a
    await claimer_ctlr.send_order("3a_signify_trust")
    await greeter_ctlr.send_order("3a_wait_peer_trust")
    greeter_rep = await greeter_ctlr.get_result()
    assert isinstance(greeter_rep, Invite3aGreeterWaitPeerTrustRepOk)
    claimer_rep = await claimer_ctlr.get_result()
    assert isinstance(claimer_rep, Invite3aClaimerSignifyTrustRepOk)

    # Step 3b
    await claimer_ctlr.send_order("3b_wait_peer_trust")
    await greeter_ctlr.send_order("3b_signify_trust")
    greeter_rep = await greeter_ctlr.get_result()
    assert isinstance(greeter_rep, Invite3bGreeterSignifyTrustRepOk)
    claimer_rep = await claimer_ctlr.get_result()
    assert isinstance(claimer_rep, Invite3bClaimerWaitPeerTrustRepOk)

    # Step 4
    certificate = ShamirRecoveryShareCertificate.load(raw_certificate)
    share = ShamirRecoveryShareData.decrypt_verify_and_load_for(
        certificate.ciphered_share,
        greeter_private_key,
        claimer_verify_key,
    )
    await claimer_ctlr.send_order("4_communicate", b"")
    await greeter_ctlr.send_order("4_communicate", share.dump())
    greeter_rep = await greeter_ctlr.get_result()
    assert greeter_rep.payload == b""
    claimer_rep = await claimer_ctlr.get_result()
    return ShamirRecoveryShareData.load(claimer_rep.payload)


@pytest.mark.trio
@customize_fixtures(bob_profile=UserProfile.ADMIN)
async def test_full_shamir(
    alice: LocalDevice,
    bob: LocalDevice,
    adam: LocalDevice,
    alice_ws,
    bob_ws,
    adam_ws,
    backend_asgi_app,
    backend_invited_ws_factory,
    backend_addr,
):
    reveal_token = b"token"
    secret_key = b"secret_key"
    ciphered_data = b"ciphered_data"

    # 2 out of 10 shares are required to recover the data
    sharks = Sharks(2)
    tokens = sharks.dealer(reveal_token, 10)
    keys = sharks.dealer(secret_key, 10)

    now = DateTime.now()

    # Alice creates ShamirRecoveryShareCertificate for Bob & Adam
    srsd_bob = ShamirRecoveryShareData(tokens[0].dump(), keys[0].dump())
    srsd_bob_ciphered = srsd_bob.dump_sign_and_encrypt_for(alice.signing_key, bob.public_key)
    raw_srsc_bob = ShamirRecoveryShareCertificate(
        alice.device_id, now, bob.user_id, srsd_bob_ciphered
    ).dump()

    srsd_adam = ShamirRecoveryShareData(tokens[1].dump(), keys[1].dump())
    srsd_adam_ciphered = srsd_adam.dump_sign_and_encrypt_for(alice.signing_key, adam.public_key)
    raw_srsc_adam = ShamirRecoveryShareCertificate(
        alice.device_id,
        now,
        bob.user_id,
        srsd_adam_ciphered,
    ).dump()

    # Alice creates ShamirRecoveryBriefCertificate
    raw_srbc = ShamirRecoveryBriefCertificate(
        alice.device_id, now, 2, {bob.user_id: 1, adam.user_id: 1}
    ).dump()

    # Alice setup shamir recovery
    setup = ShamirRecoverySetup(
        ciphered_data, reveal_token, raw_srbc, [raw_srsc_bob, raw_srsc_adam]
    )
    rep = await shamir_recovery_setup(alice_ws, setup)
    assert isinstance(rep, ShamirRecoverySetupRepOk)

    # Alice lost her password, then Adam as ADMIN invites Alice
    rep = await invite_new(
        adam_ws, type=InvitationType.SHAMIR_RECOVERY, claimer_user_id=alice.user_id
    )
    assert isinstance(rep, InviteNewRepOk)

    invitation_url = BackendInvitationAddr.build(
        backend_addr, adam.organization_id, InvitationType.SHAMIR_RECOVERY, rep.token
    ).to_url()
    assert "action=claim_shamir_recovery" in invitation_url

    invitation_address = BackendInvitationAddr.from_url(invitation_url)

    # Alice comes back with adam invitation
    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=invitation_address.organization_id,
        invitation_type=invitation_address.invitation_type,
        token=invitation_address.token,
    ) as invited_ws:
        # Alice gets information
        rep = await invite_info(invited_ws)
        assert isinstance(rep, InviteInfoRepOk)

        assert rep.threshold == 2
        assert len(rep.recipients) == 2
        assert sorted(rep.recipients, key=lambda x: x.human_handle or "") == [
            ShamirRecoveryRecipient(adam.user_id, adam.human_handle, 1),
            ShamirRecoveryRecipient(bob.user_id, bob.human_handle, 1),
        ]

        async with exchange_testbed_context(
            adam,
            adam_ws,
            invitation_address,
            invited_ws,
        ) as (_, _, adam_ctlr, alice_ctlr):
            adam_share = await _shamir_exchange(
                alice_ctlr,
                adam_ctlr,
                alice.verify_key,
                adam.private_key,
                raw_srsc_adam,
            )

    # Alice reiterates with Bob
    rep = await invite_new(
        bob_ws, type=InvitationType.SHAMIR_RECOVERY, claimer_user_id=alice.user_id
    )
    assert isinstance(rep, InviteNewRepOk)

    invitation_url = BackendInvitationAddr.build(
        backend_addr, bob.organization_id, InvitationType.SHAMIR_RECOVERY, rep.token
    ).to_url()
    assert "action=claim_shamir_recovery" in invitation_url

    invitation_address = BackendInvitationAddr.from_url(invitation_url)

    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=invitation_address.organization_id,
        invitation_type=invitation_address.invitation_type,
        token=invitation_address.token,
    ) as invited_ws:
        async with exchange_testbed_context(
            bob,
            bob_ws,
            invitation_address,
            invited_ws,
        ) as (_, _, bob_ctlr, alice_ctlr):
            bob_share = await _shamir_exchange(
                alice_ctlr,
                bob_ctlr,
                alice.verify_key,
                bob.private_key,
                raw_srsc_bob,
            )

    # Now Alice can recover her device
    recovered_token = sharks.recover(
        (
            Share.load(adam_share.reveal_token_share),
            Share.load(bob_share.reveal_token_share),
        )
    )
    recovered_key = sharks.recover(
        (
            Share.load(adam_share.data_key_share),
            Share.load(bob_share.data_key_share),
        )
    )

    assert recovered_token == reveal_token
    assert recovered_key == secret_key

    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=invitation_address.organization_id,
        invitation_type=invitation_address.invitation_type,
        token=invitation_address.token,
    ) as invited_ws:
        rep = await invite_shamir_recovery_reveal(invited_ws, recovered_token)
        assert isinstance(rep, InviteShamirRecoveryRevealRepOk)

    assert rep.ciphered_data == ciphered_data

    # TODO: Decrypt ciphered data with recovered_key


@pytest.mark.trio
async def test_shamir_list(
    alice: LocalDevice,
    adam: LocalDevice,
    alice_ws,
    adam_ws,
):
    alice_brief = ShamirRecoveryBriefCertificate(
        alice.device_id,
        DateTime.now(),
        threshold=1,
        per_recipient_shares={adam.device_id.user_id: 1},
    ).dump()
    setup = ShamirRecoverySetup(
        b"alice_ciphered_data", b"alice_reveal_token", alice_brief, [b"share0", b"share1"]
    )
    rep = await shamir_recovery_setup(alice_ws, setup)
    assert isinstance(rep, ShamirRecoverySetupRepOk)

    rep = await invite_new(
        adam_ws, type=InvitationType.SHAMIR_RECOVERY, claimer_user_id=alice.user_id
    )
    assert isinstance(rep, InviteNewRepOk)

    invitation_token = rep.token

    rep = await invite_list(adam_ws)
    assert isinstance(rep, InviteListRepOk)

    assert len(rep.invitations) == 1
    assert rep.invitations[0].token == invitation_token
    assert alice.human_handle is not None
    assert rep.invitations[0].claimer_email == alice.human_handle.email

    rep = await invite_delete(adam_ws, invitation_token, InvitationDeletedReason.CANCELLED)
    assert isinstance(rep, InviteDeleteRepOk)

    rep = await invite_list(adam_ws)
    assert isinstance(rep, InviteListRepOk)

    assert len(rep.invitations) == 0
