# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationBootstrapRepAlreadyBootstrapped,
    OrganizationBootstrapRepNotFound,
    OrganizationBootstrapRepOk,
    SequesterVerifyKeyDer,
    SigningKey,
)
from parsec.api.data import DeviceCertificate, SequesterAuthorityCertificate, UserCertificate
from parsec.api.protocol import DeviceLabel, HumanHandle, UserProfile
from parsec.core.backend_connection import organization_bootstrap as cmd_organization_bootstrap
from parsec.core.invite.exceptions import InviteAlreadyUsedError, InviteError, InviteNotFoundError
from parsec.core.local_device import generate_new_device
from parsec.core.types import BackendOrganizationAddr, BackendOrganizationBootstrapAddr, LocalDevice


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

    rep = await cmd_organization_bootstrap(
        addr=addr,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate_signed,
    )

    if isinstance(rep, OrganizationBootstrapRepNotFound):
        raise InviteNotFoundError
    elif isinstance(rep, OrganizationBootstrapRepAlreadyBootstrapped):
        raise InviteAlreadyUsedError
    elif not isinstance(rep, OrganizationBootstrapRepOk):
        raise InviteError(f"Backend error during organization bootstrap: {rep}")

    return device
