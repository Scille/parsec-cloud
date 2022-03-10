# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Optional

from parsec.crypto import SigningKey
from parsec.api.data import UserCertificateContent, DeviceCertificateContent, UserProfile
from parsec.api.protocol import HumanHandle, DeviceLabel
from parsec.core.types import LocalDevice, BackendOrganizationAddr
from parsec.core.local_device import generate_new_device
from parsec.core.backend_connection import APIV1_BackendAnonymousCmds
from parsec.core.invite.exceptions import (
    InviteError,
    InviteNotFoundError,
    InviteAlreadyUsedError,
    InvitePeerResetError,
)


def _check_rep(rep, step_name):
    if rep["status"] == "not_found":
        raise InviteNotFoundError
    elif rep["status"] == "already_bootstrapped":
        raise InviteAlreadyUsedError
    elif rep["status"] == "invalid_state":
        raise InvitePeerResetError
    elif rep["status"] != "ok":
        raise InviteError(f"Backend error during {step_name}: {rep}")


async def bootstrap_organization(
    cmds: APIV1_BackendAnonymousCmds,
    human_handle: Optional[HumanHandle],
    device_label: Optional[DeviceLabel],
) -> LocalDevice:
    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key

    organization_addr = BackendOrganizationAddr.build(
        backend_addr=cmds.addr.get_backend_addr(),
        organization_id=cmds.addr.organization_id,
        root_verify_key=root_verify_key,
    )

    device = generate_new_device(
        organization_addr=organization_addr,
        profile=UserProfile.ADMIN,
        human_handle=human_handle,
        device_label=device_label,
    )

    timestamp = device.timestamp()
    user_certificate = UserCertificateContent(
        author=None,
        timestamp=timestamp,
        user_id=device.user_id,
        human_handle=device.human_handle,
        public_key=device.public_key,
        profile=device.profile,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificateContent(
        author=None,
        timestamp=timestamp,
        device_id=device.device_id,
        device_label=device.device_label,
        verify_key=device.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user_certificate = user_certificate.dump_and_sign(root_signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(root_signing_key)
    device_certificate = device_certificate.dump_and_sign(root_signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(root_signing_key)

    rep = await cmds.organization_bootstrap(
        organization_id=cmds.addr.organization_id,
        bootstrap_token=cmds.addr.token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    _check_rep(rep, step_name="organization bootstrap")

    return device
