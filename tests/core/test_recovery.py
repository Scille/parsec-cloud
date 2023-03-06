# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path

import pytest

from parsec._parsec import (
    LocalDevice,
    LocalDeviceCryptoError,
    SecretKey,
    load_recovery_device,
    save_recovery_device,
)
from parsec.api.protocol.types import DeviceLabel
from parsec.core.backend_connection import BackendConnectionRefused, BackendNotAvailable
from parsec.core.local_device import get_recovery_device_file_name
from parsec.core.recovery import generate_new_device_from_recovery, generate_recovery_device
from parsec.crypto import CryptoError


def test_recovery_passphrase():
    passphrase, key = SecretKey.generate_recovery_passphrase()

    key2 = SecretKey.from_recovery_passphrase(passphrase)
    assert key2 == key

    # Add dummy stuff to the passphrase should not cause issues
    altered_passphrase = passphrase.lower().replace("-", "@  白")
    key3 = SecretKey.from_recovery_passphrase(altered_passphrase)
    assert key3 == key


@pytest.mark.parametrize(
    "bad_passphrase",
    [
        # Empty
        "",
        # Only invalid characters (so end up empty)
        "-@//白",
        # Too short
        "D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO",
        # Too long
        "D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO-NU4Q-NU4Q",
    ],
)
def test_invalid_passphrase(bad_passphrase):
    with pytest.raises(CryptoError):
        SecretKey.from_recovery_passphrase(bad_passphrase)


@pytest.mark.trio
async def test_recovery_ok(tmp_path, user_fs_factory, running_backend, bob):
    # 1) Create recovery device and export it as file

    recovery_device = await generate_recovery_device(bob)

    assert recovery_device.organization_addr == bob.organization_addr
    assert recovery_device.device_id != bob.device_id
    assert recovery_device.device_label != bob.device_label
    assert recovery_device.human_handle == bob.human_handle
    assert recovery_device.signing_key != bob.signing_key
    assert recovery_device.private_key == bob.private_key
    assert recovery_device.profile == bob.profile
    assert recovery_device.user_manifest_id == bob.user_manifest_id
    assert recovery_device.user_manifest_key == bob.user_manifest_key
    assert recovery_device.local_symkey != bob.local_symkey

    file_name = get_recovery_device_file_name(recovery_device)

    file_path = tmp_path / file_name
    passphrase = await save_recovery_device(file_path, recovery_device, force=False)

    # 2) Load recovery device file and create a new device

    recovery_device2 = await load_recovery_device(file_path, passphrase)

    new_device = await generate_new_device_from_recovery(
        recovery_device2, DeviceLabel("new_device")
    )

    assert new_device.organization_addr == recovery_device.organization_addr
    assert new_device.device_id != recovery_device.device_id
    assert new_device.device_label != recovery_device.device_label
    assert new_device.human_handle == recovery_device.human_handle
    assert new_device.signing_key != recovery_device.signing_key
    assert new_device.private_key == recovery_device.private_key
    assert new_device.profile == recovery_device.profile
    assert new_device.user_manifest_id == recovery_device.user_manifest_id
    assert new_device.user_manifest_key == recovery_device.user_manifest_key
    assert new_device.local_symkey != recovery_device.local_symkey

    # 3) Make sure the new device can connect to the backend
    async with user_fs_factory(new_device) as new_device_fs:
        # Recovered device should start with a speculative user manifest
        um = new_device_fs.get_user_manifest()
        assert um.is_placeholder
        assert um.speculative

        await new_device_fs.sync()
        um = new_device_fs.get_user_manifest()
        assert not um.is_placeholder
        assert not um.speculative


@pytest.mark.trio
async def test_recovery_with_wrong_passphrase(
    tmp_path: Path, user_fs_factory, running_backend, bob: LocalDevice
):
    # 1) Create recovery device and export it as file

    recovery_device = await generate_recovery_device(bob)

    assert recovery_device.organization_addr == bob.organization_addr
    assert recovery_device.device_id != bob.device_id
    assert recovery_device.device_label != bob.device_label
    assert recovery_device.human_handle == bob.human_handle
    assert recovery_device.signing_key != bob.signing_key
    assert recovery_device.private_key == bob.private_key
    assert recovery_device.profile == bob.profile
    assert recovery_device.user_manifest_id == bob.user_manifest_id
    assert recovery_device.user_manifest_key == bob.user_manifest_key
    assert recovery_device.local_symkey != bob.local_symkey

    file_name = get_recovery_device_file_name(recovery_device)

    file_path = tmp_path / file_name
    passphrase = await save_recovery_device(file_path, recovery_device, force=False)

    # 2) Try to load the recovery device file with the wrong passphrase.
    wrong_passphrase = "-".join([chunk[::-1] for chunk in passphrase.split("-")])
    assert wrong_passphrase != passphrase

    with pytest.raises(LocalDeviceCryptoError):
        _recovery_device2 = await load_recovery_device(file_path, wrong_passphrase)


@pytest.mark.trio
async def test_recovery_while_offline(alice):
    with pytest.raises(BackendNotAvailable):
        await generate_recovery_device(alice)

    with pytest.raises(BackendNotAvailable):
        await generate_new_device_from_recovery(alice, DeviceLabel("new_device"))


@pytest.mark.trio
async def test_recovery_with_revoked_user(running_backend, backend_data_binder, alice, bob):
    await backend_data_binder.bind_revocation(bob.user_id, alice)

    with pytest.raises(BackendConnectionRefused):
        await generate_recovery_device(bob)

    with pytest.raises(BackendConnectionRefused):
        await generate_new_device_from_recovery(bob, DeviceLabel("new_device"))
