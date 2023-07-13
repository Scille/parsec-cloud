# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import contextlib

import pytest
import trio

from parsec import FEATURE_FLAGS
from parsec._parsec import (
    BackendInvitationAddr,
    CoreEvent,
    DeviceLabel,
    InvitationDeletedReason,
    InvitationStatus,
    InvitationToken,
)
from parsec.core.backend_connection import backend_invited_cmds_factory
from parsec.core.invite import ShamirRecoveryClaimPreludeCtx, claimer_retrieve_info
from parsec.core.logged_core import BackendConnectionError, LoggedCore
from parsec.core.recovery import generate_new_device_from_recovery
from parsec.core.shamir import (
    ShamirRecoveryNotSetError,
    create_shamir_recovery_device,
    get_shamir_recovery_others_list,
    get_shamir_recovery_self_info,
    remove_shamir_recovery_device,
)
from tests.common import real_clock_timeout


@contextlib.asynccontextmanager
async def assert_invite_status_changed_event(
    core1: LoggedCore,
    core2: LoggedCore,
    status: InvitationStatus,
    token: InvitationToken | None = None,
):
    def feed_token(fed: InvitationToken) -> None:
        nonlocal token
        assert token is None
        token = fed

    with core1.event_bus.listen() as spy1, core2.event_bus.listen() as spy2:
        yield feed_token
        assert token is not None
        await spy1.wait_with_timeout(
            CoreEvent.INVITE_STATUS_CHANGED,
            {"status": status, "token": token},
        )
        await spy2.wait_with_timeout(
            CoreEvent.INVITE_STATUS_CHANGED,
            {"status": status, "token": token},
        )


@pytest.mark.trio
async def test_shamir_recovery_setup(
    alice_core: LoggedCore,
    bob_core: LoggedCore,
    adam_core: LoggedCore,
    running_backend,
):
    alice = alice_core.device
    bob = bob_core.device
    adam = adam_core.device

    certificates = [
        (await alice_core._remote_devices_manager.get_user(device.user_id))[0]
        for device in (adam, bob)
    ]

    # No shamir recovery set yet
    with pytest.raises(ShamirRecoveryNotSetError):
        await get_shamir_recovery_self_info(alice_core)
    assert await get_shamir_recovery_others_list(bob_core) == []
    assert await get_shamir_recovery_others_list(adam_core) == []

    # Create a first shamir recovery device for alice
    await create_shamir_recovery_device(alice_core, certificates, threshold=2)

    # Alice can see it
    author_certificate, first_brief_certificate = await get_shamir_recovery_self_info(alice_core)
    assert author_certificate.device_id == alice.device_id
    assert first_brief_certificate.threshold == 2
    assert first_brief_certificate.per_recipient_shares == {bob.user_id: 1, adam.user_id: 1}

    # Bob can see it
    (
        (device_certificate, user_certificate, brief_certificate_from_bob, bob_share),
    ) = await get_shamir_recovery_others_list(bob_core)
    assert device_certificate.device_id == alice.device_id
    assert user_certificate.user_id == alice.user_id
    assert brief_certificate_from_bob == first_brief_certificate
    assert len(bob_share.weighted_share) == 1

    # Adam can see it
    (
        (device_certificate, user_certificate, brief_certificate_from_adam, adam_share),
    ) = await get_shamir_recovery_others_list(adam_core)
    assert device_certificate.device_id == alice.device_id
    assert user_certificate.user_id == alice.user_id
    assert brief_certificate_from_adam == first_brief_certificate
    assert len(adam_share.weighted_share) == 1

    # Alice creates a new shamir recovery device
    await create_shamir_recovery_device(alice_core, certificates, threshold=4, weights=[3, 2])

    # Alice can see the new setup
    author_certificate, second_brief_certificate = await get_shamir_recovery_self_info(alice_core)
    assert author_certificate.device_id == alice.device_id
    assert second_brief_certificate.threshold == 4
    assert second_brief_certificate.per_recipient_shares == {bob.user_id: 2, adam.user_id: 3}

    # Bob can see the new setup
    (
        (device_certificate, user_certificate, brief_certificate_from_bob, bob_share),
    ) = await get_shamir_recovery_others_list(bob_core)
    assert device_certificate.device_id == alice.device_id
    assert user_certificate.user_id == alice.user_id
    assert brief_certificate_from_bob == second_brief_certificate
    assert len(bob_share.weighted_share) == 2

    # Adam can see the new setup
    (
        (device_certificate, user_certificate, brief_certificate_from_adam, adam_share),
    ) = await get_shamir_recovery_others_list(adam_core)
    assert device_certificate.device_id == alice.device_id
    assert user_certificate.user_id == alice.user_id
    assert brief_certificate_from_adam == second_brief_certificate
    assert len(adam_share.weighted_share) == 3

    # Alice removes their shamir recovery device
    await remove_shamir_recovery_device(alice_core)

    # It is no longer available for anyone
    with pytest.raises(ShamirRecoveryNotSetError):
        await get_shamir_recovery_self_info(alice_core)
    assert await get_shamir_recovery_others_list(bob_core) == []
    assert await get_shamir_recovery_others_list(adam_core) == []


@pytest.mark.trio
@pytest.mark.parametrize(
    "deletion",
    ["by_alice", "by_bob", "by_adam", "by_new_setup"],
    ids=["deletion_by_alice", "deletion_by_bob", "deletion_by_adam", "deletion_by_new_setup"],
)
async def test_shamir_recovery_invitation(
    alice_core: LoggedCore,
    bob_core: LoggedCore,
    adam_core: LoggedCore,
    deletion: str,
    running_backend,
):
    alice = alice_core.device
    bob = bob_core.device
    adam = adam_core.device

    # Bob tries to create an invitation for Alice
    # but the shamir recovery device hasn't been created yet
    with pytest.raises(BackendConnectionError):
        await bob_core.new_shamir_recovery_invitation(alice.user_id, send_email=True)

    # Create a first shamir recovery device for alice
    certificates = [
        (await alice_core._remote_devices_manager.get_user(device.user_id))[0]
        for device in (adam, bob)
    ]
    await create_shamir_recovery_device(alice_core, certificates, threshold=2)

    # Bob creates an invitation for Alice
    # Bob doesn't have to be an administrator
    async with assert_invite_status_changed_event(
        bob_core, adam_core, InvitationStatus.IDLE
    ) as feed_token:
        address_from_bob, email_sent = await bob_core.new_shamir_recovery_invitation(
            alice.user_id, send_email=True
        )
        assert email_sent == email_sent.SUCCESS
        feed_token(address_from_bob.token)

    # Both Bob and Adam can find the invitation in their list
    (invite_item_from_bob,) = await bob_core.list_invitations()
    (invite_item_from_adam,) = await adam_core.list_invitations()
    assert invite_item_from_bob == invite_item_from_adam
    assert invite_item_from_bob.claimer_user_id == alice.user_id
    assert invite_item_from_bob.token == address_from_bob.token

    # Adam also creates an invitation for Alice
    # They receive the same token as the one from Bob
    address_from_adam, email_sent = await bob_core.new_shamir_recovery_invitation(
        alice.user_id, send_email=True
    )
    assert email_sent == email_sent.SUCCESS
    assert address_from_adam == address_from_bob

    # The list is unaffected
    (invite_item_from_bob,) = await bob_core.list_invitations()
    (invite_item_from_adam,) = await adam_core.list_invitations()
    assert invite_item_from_bob == invite_item_from_adam
    assert invite_item_from_bob.claimer_user_id == alice.user_id
    assert invite_item_from_bob.token == address_from_bob.token

    # Different deletion scenario
    async with assert_invite_status_changed_event(
        bob_core, adam_core, InvitationStatus.DELETED, address_from_bob.token
    ):
        token = invite_item_from_bob.token
        if deletion == "by_alice":
            await alice_core.delete_invitation(token=token)
        elif deletion == "by_bob":
            await bob_core.delete_invitation(token=token)
        elif deletion == "by_adam":
            await adam_core.delete_invitation(token=token)
        elif deletion == "by_new_setup":
            await create_shamir_recovery_device(alice_core, certificates, threshold=2)
        else:
            assert False

    # The list is empty
    assert await bob_core.list_invitations() == []
    assert await adam_core.list_invitations() == []

    # But an invitation with a new token can be recreated
    async with assert_invite_status_changed_event(
        bob_core, adam_core, InvitationStatus.IDLE
    ) as feed_token:
        address_from_adam, email_sent = await bob_core.new_shamir_recovery_invitation(
            alice.user_id, send_email=True
        )
        old_token = address_from_bob.token
        new_token = address_from_adam.token
        assert email_sent == email_sent.SUCCESS
        assert new_token != old_token
        feed_token(new_token)

    # The list is updated
    (invite_item_from_bob,) = await bob_core.list_invitations()
    (invite_item_from_adam,) = await adam_core.list_invitations()
    assert invite_item_from_bob == invite_item_from_adam
    assert invite_item_from_bob.claimer_user_id == alice.user_id
    assert invite_item_from_bob.token == new_token


@pytest.mark.trio
@pytest.mark.skipif(
    FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"],
    reason="claimer_retrieve_info not oxidized for shamir yet",
)
async def test_shamir_recovery_claim(
    alice_core: LoggedCore,
    bob_core: LoggedCore,
    adam_core: LoggedCore,
    core_factory,
    running_backend,
):
    alice = alice_core.device
    bob = bob_core.device
    adam = adam_core.device

    # Create a first shamir recovery device for alice
    certificates = [
        (await alice_core._remote_devices_manager.get_user(device.user_id))[0]
        for device in (adam, bob)
    ]
    await create_shamir_recovery_device(alice_core, certificates, threshold=4, weights=[2, 3])

    # Bob creates an invitation for Alice
    address_from_bob, email_sent = await bob_core.new_shamir_recovery_invitation(
        alice.user_id, send_email=True
    )
    assert email_sent == email_sent.SUCCESS
    invitation_type = address_from_bob.invitation_type
    assert invitation_type == invitation_type.SHAMIR_RECOVERY
    send_this_to_alice = address_from_bob.to_url()

    # Concurrent environement
    async with real_clock_timeout():
        async with trio.open_nursery() as nursery:
            # Adam decides to greet right away for some reason
            adam_done_with_alice = trio.Event()

            async def _run_adam_greeter():
                (invite_item_from_adam,) = await adam_core.list_invitations()
                adam_share_data, adam_ctx = await adam_core.start_greeting_shamir_recovery(
                    invite_item_from_adam.token,
                    invite_item_from_adam.claimer_user_id,
                )
                adam_ctx = await adam_ctx.do_wait_peer_trust()
                adam_ctx.generate_claimer_sas_choices(size=3)
                # Skip SAS code checks
                adam_ctx = await adam_ctx.do_signify_trust()
                await adam_ctx.send_share_data(adam_share_data)
                adam_done_with_alice.set()

            nursery.start_soon(_run_adam_greeter)

            # Alice receives the invitation address
            address = BackendInvitationAddr.from_url(send_this_to_alice)
            async with backend_invited_cmds_factory(addr=address) as cmds:
                prelude_ctx = await claimer_retrieve_info(cmds)
                assert isinstance(prelude_ctx, ShamirRecoveryClaimPreludeCtx)
                recipient1, recipient2 = prelude_ctx.recipients
                if recipient1.user_id == adam.user_id:
                    adam_recipient, bob_recipient = recipient1, recipient2
                else:
                    adam_recipient, bob_recipient = recipient2, recipient1
                assert adam_recipient.user_id == adam.user_id
                assert adam_recipient.shares == 2
                assert bob_recipient.user_id == bob.user_id
                assert bob_recipient.shares == 3

                # Alice decides to start with Bob
                alice_done_with_bob = trio.Event()
                alice_initial_ctx = prelude_ctx.get_initial_ctx(bob_recipient)

                async def _run_first_alice_claimer():
                    async with assert_invite_status_changed_event(
                        bob_core, adam_core, InvitationStatus.READY, address.token
                    ):
                        alice_ctx = await alice_initial_ctx.do_wait_peer()
                    alice_ctx.generate_greeter_sas_choices(size=3)
                    # Skip SAS code checks
                    alice_ctx = await alice_ctx.do_signify_trust()
                    alice_ctx = await alice_ctx.do_wait_peer_trust()
                    new_shares = await alice_ctx.do_recover_share()
                    assert isinstance(prelude_ctx, ShamirRecoveryClaimPreludeCtx)
                    assert not prelude_ctx.add_shares(bob_recipient, new_shares)
                    assert len(prelude_ctx.shares) == 3
                    alice_done_with_bob.set()

                nursery.start_soon(_run_first_alice_claimer)

                # Bob joins in
                (invite_item_from_bob,) = await adam_core.list_invitations()
                bob_share_data, bob_ctx = await bob_core.start_greeting_shamir_recovery(
                    invite_item_from_bob.token,
                    invite_item_from_bob.claimer_user_id,
                )
                bob_ctx = await bob_ctx.do_wait_peer_trust()
                bob_ctx.generate_claimer_sas_choices(size=3)
                # Skip SAS code checks
                bob_ctx = await bob_ctx.do_signify_trust()
                await bob_ctx.send_share_data(bob_share_data)
                await alice_done_with_bob.wait()

                # Alice then continues with Adam
                assert len(prelude_ctx.shares) == 3
                alice_initial_ctx = prelude_ctx.get_initial_ctx(adam_recipient)
                alice_ctx = await alice_initial_ctx.do_wait_peer()
                alice_ctx.generate_greeter_sas_choices(size=3)
                # Skip SAS code checks
                alice_ctx = await alice_ctx.do_signify_trust()
                alice_ctx = await alice_ctx.do_wait_peer_trust()
                new_shares = await alice_ctx.do_recover_share()
                assert isinstance(prelude_ctx, ShamirRecoveryClaimPreludeCtx)
                assert prelude_ctx.add_shares(adam_recipient, new_shares)
                assert len(prelude_ctx.shares) == 5
                await adam_done_with_alice.wait()

                # Alice retrieves her recovery device
                alice_recovery_device = await prelude_ctx.retreive_recovery_device()

    # Alice creates a new device and deletes the invitation
    assert alice_recovery_device.device_id.user_id == alice.user_id
    alice_new_device = await generate_new_device_from_recovery(
        alice_recovery_device, DeviceLabel("new label")
    )
    async with core_factory(alice_new_device) as new_alice_core:
        new_alice_core: LoggedCore
        await new_alice_core.delete_invitation(
            address.token, reason=InvitationDeletedReason.FINISHED
        )

    # The list is updated
    assert await bob_core.list_invitations() == []
    assert await adam_core.list_invitations() == []
