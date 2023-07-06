# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.logged_core import LoggedCore
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
