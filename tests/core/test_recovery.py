# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from pathlib import Path

import pytest

from parsec.backend.user import Device
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.local_device import get_key_file, load_device_with_password
from parsec.core.recovery import (
    generate_new_device_from_original,
    create_new_device_from_original,
    generate_recovery_password,
    generate_passphrase_from_recovery_password,
    get_recovery_password_from_passphrase,
    is_valid_passphrase,
)


@pytest.mark.trio
async def test_generate_new_device_from_original(backend, running_backend, bob):
    new_device_label = "NEWDEVICE"
    async with backend_authenticated_cmds_factory(
        addr=bob.organization_addr, device_id=bob.device_id, signing_key=bob.signing_key
    ) as cmds:
        new_device = await generate_new_device_from_original(cmds, bob, new_device_label)

    assert bob.organization_addr == new_device.organization_addr
    assert bob.human_handle == new_device.human_handle
    assert bob.profile == new_device.profile
    assert bob.private_key == new_device.private_key
    assert bob.user_manifest_id == new_device.user_manifest_id
    assert bob.user_manifest_key == new_device.user_manifest_key
    assert bob.local_symkey == new_device.local_symkey

    assert bob.device_id != new_device.device_id
    assert (
        bob.device_label != new_device.device_label and new_device_label == new_device.device_label
    )
    assert bob.signing_key != new_device.signing_key

    _, backend_new_device = await backend.user.get_user_with_device(
        new_device.organization_id, new_device.device_id
    )
    _, backend_bob_device = await backend.user.get_user_with_device(
        bob.organization_id, bob.device_id
    )
    backend_new_device: Device
    backend_bob_device: Device

    assert backend_new_device != backend_bob_device

    assert backend_new_device.user_id == backend_bob_device.user_id

    assert backend_new_device.device_id != backend_bob_device.device_id
    assert (
        backend_new_device.device_label != backend_bob_device.device_label
        and backend_new_device.device_label == new_device_label
    )
    assert backend_new_device.device_certificate != backend_bob_device.device_certificate
    assert backend_new_device.device_certifier != backend_bob_device.device_certifier
    assert backend_new_device.created_on != backend_bob_device.created_on
    assert backend_new_device.device_name != backend_bob_device.device_name


@pytest.mark.trio
async def test_create_new_device_from_original(tmpdir, running_backend, core_config, alice):
    new_device_label = "NEWDEVICE"
    password = "NEWPASSWORD"
    config_dir = Path(tmpdir)
    new_alice = await create_new_device_from_original(alice, new_device_label, password, config_dir)

    key_file = get_key_file(config_dir, new_alice)
    alice_reloaded = load_device_with_password(key_file, password)
    assert new_alice == alice_reloaded
    assert new_alice != alice


def test_create_recovery_password():
    recovery_password_orig = generate_recovery_password()

    passphrase = generate_passphrase_from_recovery_password(recovery_password_orig)
    assert is_valid_passphrase(passphrase) is True
    assert is_valid_passphrase("ABDEEFKFWDWDJWDWNDJWDJ") is False
    assert is_valid_passphrase("\n" + passphrase + "  ") is True
    assert is_valid_passphrase(passphrase.replace("-", "")) is True
    recovery_password = get_recovery_password_from_passphrase(passphrase)
    assert recovery_password_orig == recovery_password
