# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
import secrets
from pathlib import Path

import string
import base64
import binascii
import pendulum
import trio

from parsec.api.data import DataError, DeviceCertificateContent
from parsec.api.protocol import DeviceID, DeviceName
from parsec.core.backend_connection import (
    backend_authenticated_cmds_factory,
    BackendConnectionError,
    BackendAuthenticatedCmds,
)
from parsec.core.local_device import (
    save_device_with_password_in_config,
    LocalDeviceError,
    save_device_with_password,
)
from parsec.core.types import LocalDevice
from parsec.crypto import SigningKey
from pendulum import now as pendulum_now


class RecoveryError(Exception):
    pass


class RecoveryLocalDeviceError(RecoveryError):
    pass


class RecoveryCreateDeviceError(RecoveryError):
    pass


class RecoveryBackendError(RecoveryError):
    pass


class RecoveryDataError(RecoveryError):
    pass


async def generate_new_device_from_original(
    cmds: BackendAuthenticatedCmds, original_device: LocalDevice, new_device_label: str
) -> LocalDevice:
    """
        RecoveryCreateDeviceError
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
        local_symkey=original_device.local_symkey,
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

    rep = await cmds.device_create(
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )

    if rep["status"] != "ok":
        raise RecoveryCreateDeviceError(f"Cannot create device: {rep}")
    return new_device


async def create_new_device_from_original(
    original_device: LocalDevice,
    device_label: str,
    password: str,
    config_dir: Path,
    key_file: Path = None,
) -> LocalDevice:
    """
        RecoveryLocalDeviceError
        RecoveryDataError
        RecoveryBackendError
        RecoveryCreateDeviceError
    """
    try:
        async with backend_authenticated_cmds_factory(
            addr=original_device.organization_addr,
            device_id=original_device.device_id,
            signing_key=original_device.signing_key,
        ) as cmds:
            new_device = await generate_new_device_from_original(
                cmds, original_device, device_label
            )
            if key_file:
                await trio.to_thread.run_sync(
                    save_device_with_password, key_file, new_device, password, True
                )
            else:
                await trio.to_thread.run_sync(
                    save_device_with_password_in_config, config_dir, new_device, password
                )

            return new_device
    except LocalDeviceError as err:
        raise RecoveryLocalDeviceError from err
    except DataError as err:
        raise RecoveryDataError from err
    except BackendConnectionError as err:
        raise RecoveryBackendError from err
    except RecoveryCreateDeviceError as err:
        raise RecoveryCreateDeviceError from err


def generate_recovery_password() -> str:
    return secrets.token_hex(32)


def generate_recovery_key_name(device: LocalDevice) -> str:
    return f"parsec-recovery-{device.organization_id}-{device.short_user_display}.psrk"


def generate_recovery_device_name() -> str:
    date = pendulum.now()
    return f"recovery-{date.year}-{date.month}-{date.day}"


# We want to provide the user with an easier to type format for the recovery password.
# We use base32 so all characters are in one case, and the alphabet contains less
# colliding letters (ie 0 O) then form 4 letters groups separated by dashes.
# When decoding, we remove any characters not included in the alphabet (spaces, new lines, ...)
# and decode to get our password back.
def generate_passphrase_from_recovery_password(recovery_password: str) -> str:
    b32 = base64.b32encode(bytes.fromhex(recovery_password)).decode().rstrip("=")
    return "-".join(b32[i : i + 4] for i in range(0, len(b32), 4))


def get_recovery_password_from_passphrase(passphrase: str) -> str:
    symbols = set(string.ascii_letters + string.digits)
    filtered = "".join(c for c in passphrase if c in symbols)
    b32 = filtered + "=" * (-len(filtered) % 8)
    return base64.b32decode(b32, casefold=True, map01="I").hex()


def is_valid_passphrase(passphrase: str) -> bool:
    try:
        rec_pwd = get_recovery_password_from_passphrase(passphrase)
    except binascii.Error:
        return False
    return len(rec_pwd) == 64
