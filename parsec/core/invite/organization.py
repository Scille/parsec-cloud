# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import SequesterVerifyKeyDer, SigningKey, VerifyKey
from parsec.api.data import DeviceCertificate, SequesterAuthorityCertificate, UserCertificate
from parsec.api.protocol import DeviceLabel, HumanHandle, UserProfile
from parsec.core.backend_connection import apiv1_backend_anonymous_cmds_factory
from parsec.core.backend_connection import organization_bootstrap as cmd_organization_bootstrap
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.core.invite.exceptions import (
    InviteAlreadyUsedError,
    InviteError,
    InviteNotFoundError,
    InvitePeerResetError,
)
from parsec.core.local_device import generate_new_device
from parsec.core.types import BackendOrganizationAddr, BackendOrganizationBootstrapAddr, LocalDevice


def _check_rep(rep: dict[str, object], step_name: str) -> None:
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
    human_handle: HumanHandle | None,
    device_label: DeviceLabel | None,
    sequester_authority_verify_key: SequesterVerifyKeyDer | None = None,
) -> LocalDevice:
    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key

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
    user_certificate = UserCertificate(
        author=None,
        timestamp=timestamp,
        user_id=device.user_id,
        human_handle=device.human_handle,
        public_key=device.public_key,
        profile=device.profile,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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

    if sequester_authority_verify_key:
        sequester_authority_certificate = SequesterAuthorityCertificate(
            timestamp=timestamp,
            verify_key_der=sequester_authority_verify_key,
        )
        sequester_authority_certificate_signed: bytes | None = (
            sequester_authority_certificate.dump_and_sign(root_signing_key)
        )
    else:
        sequester_authority_certificate_signed = None

    rep = await failsafe_organization_bootstrap(
        addr=addr,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate_signed,
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
    sequester_authority_certificate: bytes | None = None,
) -> dict[str, object]:
    # Try the new anonymous API
    try:
        rep = await cmd_organization_bootstrap(
            addr=addr,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            sequester_authority_certificate=sequester_authority_certificate,
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
    if sequester_authority_certificate:
        raise InviteError("Backend doesn't support sequestered organization")
    async with apiv1_backend_anonymous_cmds_factory(addr) as anonymous_cmds:
        assert addr.token is not None, "Token is required to bootstrap an organization"

        return await anonymous_cmds.organization_bootstrap(
            organization_id=addr.organization_id,
            bootstrap_token=addr.token,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
