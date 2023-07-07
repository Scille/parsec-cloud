# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.logged_core import BackendConnectionError, LoggedCore
from parsec.core.shamir import (
    ShamirRecoveryNotSetError,
    create_shamir_recovery_device,
    get_shamir_recovery_others_list,
    get_shamir_recovery_self_info,
    remove_shamir_recovery_device,
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
    address_from_bob, email_sent = await bob_core.new_shamir_recovery_invitation(
        alice.user_id, send_email=True
    )
    assert email_sent == email_sent.SUCCESS

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
    address_from_adam, email_sent = await bob_core.new_shamir_recovery_invitation(
        alice.user_id, send_email=True
    )
    old_token = address_from_bob.token
    new_token = address_from_adam.token
    assert email_sent == email_sent.SUCCESS
    assert new_token != old_token

    # The list is updated
    (invite_item_from_bob,) = await bob_core.list_invitations()
    (invite_item_from_adam,) = await adam_core.list_invitations()
    assert invite_item_from_bob == invite_item_from_adam
    assert invite_item_from_bob.claimer_user_id == alice.user_id
    assert invite_item_from_bob.token == new_token
