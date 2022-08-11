# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import secrets
import pendulum

from parsec.api.data import DeviceCertificateContent
from parsec.api.protocol import DeviceID, DeviceName, DeviceLabel
from parsec.core.backend_connection import (
    backend_authenticated_cmds_factory,
    BackendConnectionError,
)
from parsec.core.types import LocalDevice
from parsec.crypto import SigningKey, SecretKey
from pendulum import now as pendulum_now


async def _create_new_device_for_self(
    original_device: LocalDevice, new_device_label: DeviceLabel
) -> LocalDevice:
    """
    Raises:
        BackendConnectionError
    """
    new_device = LocalDevice(
        organization_addr=original_device.organization_addr,
        device_id=DeviceID(f"{original_device.user_id}@{DeviceName.new()}"),
        device_label=new_device_label,
        human_handle=original_device.human_handle,
        profile=original_device.profile,
        private_key=original_device.private_key,
        signing_key=SigningKey.generate(),
        user_manifest_id=original_device.user_manifest_id,
        user_manifest_key=original_device.user_manifest_key,
        local_symkey=SecretKey.generate(),
    )
    now = pendulum_now()

    device_certificate = DeviceCertificateContent(
        author=original_device.device_id,
        timestamp=now,
        device_id=new_device.device_id,
        device_label=new_device.device_label,
        verify_key=new_device.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    device_certificate = device_certificate.dump_and_sign(original_device.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(
        original_device.signing_key
    )

    async with backend_authenticated_cmds_factory(
        addr=original_device.organization_addr,
        device_id=original_device.device_id,
        signing_key=original_device.signing_key,
    ) as cmds:
        rep = await cmds.device_create(
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )

    if rep["status"] != "ok":
        raise BackendConnectionError(f"Cannot create recovery device: {rep}")

    return new_device


async def generate_recovery_device(
    original_device: LocalDevice,
) -> LocalDevice:
    """
    Raises:
        BackendConnectionError
    """
    now = pendulum.now()
    # Unique enough label is expected, but unicity is not strongly enforced
    new_device_label = DeviceLabel(
        f"recovery-{now.year}-{now.month}-{now.day}-{secrets.token_hex(2)}"
    )
    return await _create_new_device_for_self(original_device, new_device_label)


async def generate_new_device_from_recovery(
    recovery_device: LocalDevice, new_device_label: DeviceLabel
) -> LocalDevice:
    """
    Raises:
        BackendConnectionError
    """
    return await _create_new_device_for_self(recovery_device, new_device_label)
