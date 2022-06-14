# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Optional
from parsec.api.data.certif import TpekDerVerifyKeyCertificateContent

from parsec.crypto import SigningKey, VerifyKey
from parsec.api.protocol import HumanHandle, DeviceLabel
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.api.data import UserCertificateContent, DeviceCertificateContent, UserProfile
from parsec.core.types import LocalDevice, BackendOrganizationAddr, BackendOrganizationBootstrapAddr
from parsec.core.local_device import generate_new_device
from parsec.core.backend_connection import (
    apiv1_backend_anonymous_cmds_factory,
    organization_bootstrap as cmd_organization_bootstrap,
)
from parsec.core.invite.exceptions import (
    InviteError,
    InviteNotFoundError,
    InviteAlreadyUsedError,
    InvitePeerResetError,
)

from parsec.tpek_crypto import DerPublicKey


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
    addr: BackendOrganizationBootstrapAddr,
    human_handle: Optional[HumanHandle],
    device_label: Optional[DeviceLabel],
    tpek_der_public_key: Optional[DerPublicKey],
) -> LocalDevice:
    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key

    # # TODO: shall it be a certificate from an authority, can it be validated
    # signed_tpek_der_public_key = root_signing_key.sign(tpek_der_public_key)

    organization_addr = BackendOrganizationAddr.build(
        backend_addr=addr.get_backend_addr(),
        organization_id=addr.organization_id,
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

    if tpek_der_public_key:
        tpek_certificate = TpekDerVerifyKeyCertificateContent(
            author=None, verify_key=tpek_der_public_key
        )

        tpek_certificate = tpek_certificate.dump_and_sign(root_verify_key)
    else:
        tpek_certificate = None

    rep = await failsafe_organization_bootstrap(
        addr=addr,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        signed_tpek_cerificate=tpek_certificate,
    )
    _check_rep(rep, step_name="organization bootstrap")

    return device


async def failsafe_organization_bootstrap(
    addr: BackendOrganizationBootstrapAddr,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
    signed_tpek_certificate: Optional[bytes],
) -> dict:
    # Try the new anonymous API
    try:
        rep = await cmd_organization_bootstrap(
            addr=addr,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            signed_tpek_signing_cert=signed_tpek_certificate,
        )
    # If we get a 404 error, maybe the backend is too old to know about the anonymous route (API version < 2.6)
    except BackendNotAvailable as exc:
        inner_exc = exc.args[0]
        if getattr(inner_exc, "status", None) != 404:
            raise
    # If we get an "unknown_command" status, the backend might be too old to know about the "organization_bootstrap" command (API version < 2.7)
    else:
        if rep["status"] != "unknown_command":
            return rep
    # Then we try again with the legacy version
    async with apiv1_backend_anonymous_cmds_factory(addr) as anonymous_cmds:
        if signed_tpek_certificate is not None:
            raise InviteError("Server doesn't support third party encryption key feature")

        return await anonymous_cmds.organization_bootstrap(
            organization_id=addr.organization_id,
            bootstrap_token=addr.token,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
