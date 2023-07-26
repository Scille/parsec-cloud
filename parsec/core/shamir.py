# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from itertools import islice
from typing import Sequence

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    SecretKey,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryOthersListRepOk,
    ShamirRecoveryOthersListRepUnknownStatus,
    ShamirRecoverySecret,
    ShamirRecoverySelfInfoRepOk,
    ShamirRecoverySelfInfoRepUnknownStatus,
    ShamirRecoverySetup,
    ShamirRecoverySetupRepAlreadySet,
    ShamirRecoverySetupRepInvalidCertification,
    ShamirRecoverySetupRepInvalidData,
    ShamirRecoverySetupRepOk,
    ShamirRecoverySetupRepUnknownStatus,
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareData,
    ShamirRevealToken,
    UserCertificate,
    UserID,
    shamir_make_shares,
)
from parsec.core.logged_core import LoggedCore
from parsec.core.recovery import generate_recovery_device


class ShamirRecoveryError(Exception):
    pass


class ShamirRecoveryInvalidCertificationError(ShamirRecoveryError):
    pass


class ShamirRecoveryInvalidDataError(ShamirRecoveryError):
    pass


class ShamirRecoveryAlreadySetError(ShamirRecoveryError):
    pass


class ShamirRecoveryNotSetError(ShamirRecoveryError):
    pass


async def create_shamir_recovery_device(
    core: LoggedCore,
    user_certificates: Sequence[UserCertificate],
    threshold: int,
    weights: Sequence[int] | None = None,
) -> None:
    device = core.device
    cmds = core._backend_conn.cmds
    if weights is None:
        weights = [1 for _ in user_certificates]

    # Encrypt a recovery device
    secret_key = SecretKey.generate()
    reveal_token = ShamirRevealToken.new()
    recovery_device = await generate_recovery_device(device)
    secret = ShamirRecoverySecret(secret_key, reveal_token)
    ciphered_data = secret_key.encrypt(recovery_device.dump())

    # Create the shares
    now = DateTime.now()
    nb_shares = sum(weights)
    shares = shamir_make_shares(threshold, secret.dump(), nb_shares)
    shares_iterator = iter(shares)

    # Create the share certificates
    share_certificates: list[ShamirRecoveryShareCertificate] = []
    for certificate, weight in zip(user_certificates, weights):
        share_data = ShamirRecoveryShareData(list(islice(shares_iterator, weight)))
        share_data_encrypted = share_data.dump_sign_and_encrypt_for(
            device.signing_key, certificate.public_key
        )
        share_certificate = ShamirRecoveryShareCertificate(
            device.device_id, now, certificate.user_id, share_data_encrypted
        )
        share_certificates.append(share_certificate)

    # Create the brief certificate
    per_recipient_shares = {
        certificate.user_id: weight for certificate, weight in zip(user_certificates, weights)
    }
    brief_certificate = ShamirRecoveryBriefCertificate(
        device.device_id,
        now,
        threshold,
        per_recipient_shares,
    )

    # Submit shamir recovery setup
    setup = ShamirRecoverySetup(
        ciphered_data,
        reveal_token,
        brief_certificate.dump_and_sign(device.signing_key),
        [
            share_certificate.dump_and_sign(device.signing_key)
            for share_certificate in share_certificates
        ],
    )

    rep = await cmds.shamir_recovery_setup(setup)
    if isinstance(rep, ShamirRecoverySetupRepInvalidCertification):
        raise ShamirRecoveryInvalidCertificationError()
    if isinstance(rep, ShamirRecoverySetupRepInvalidData):
        raise ShamirRecoveryInvalidDataError()
    if isinstance(rep, ShamirRecoverySetupRepAlreadySet):
        raise ShamirRecoveryAlreadySetError()
    if isinstance(rep, ShamirRecoverySetupRepUnknownStatus):
        raise ShamirRecoveryError(rep.status)
    assert isinstance(rep, ShamirRecoverySetupRepOk)


async def remove_shamir_recovery_device(
    core: LoggedCore,
) -> None:
    cmds = core._backend_conn.cmds

    rep = await cmds.shamir_recovery_setup(None)
    if isinstance(rep, ShamirRecoverySetupRepInvalidCertification):
        raise ShamirRecoveryInvalidCertificationError()
    if isinstance(rep, ShamirRecoverySetupRepInvalidData):
        raise ShamirRecoveryInvalidDataError()
    if isinstance(rep, ShamirRecoverySetupRepAlreadySet):
        raise ShamirRecoveryAlreadySetError()
    if isinstance(rep, ShamirRecoverySetupRepUnknownStatus):
        raise ShamirRecoveryError(rep.status)
    assert isinstance(rep, ShamirRecoverySetupRepOk)


async def get_shamir_recovery_self_info(
    core: LoggedCore,
) -> tuple[DeviceCertificate, ShamirRecoveryBriefCertificate]:
    cmds = core._backend_conn.cmds
    remote_devices_manager = core._remote_devices_manager

    # Ask the server
    rep = await cmds.shamir_recovery_self_info()
    if isinstance(rep, ShamirRecoverySelfInfoRepUnknownStatus):
        raise ShamirRecoveryError(rep.status)
    assert isinstance(rep, ShamirRecoverySelfInfoRepOk)
    if rep.self_info is None:
        raise ShamirRecoveryNotSetError

    # Load brief certificate
    unsecure_certificate = ShamirRecoveryBriefCertificate.unsecure_load(rep.self_info)
    author_certificate = await remote_devices_manager.get_device(unsecure_certificate.author)
    brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
        rep.self_info, author_certificate.verify_key, unsecure_certificate.author
    )
    return author_certificate, brief_certificate


async def get_shamir_recovery_others_list(
    core: LoggedCore,
) -> list[
    tuple[
        DeviceCertificate,
        UserCertificate,
        ShamirRecoveryBriefCertificate,
        ShamirRecoveryShareData | None,
    ]
]:
    device = core.device
    cmds = core._backend_conn.cmds
    remote_devices_manager = core._remote_devices_manager

    rep = await cmds.shamir_recovery_others_list()
    if isinstance(rep, ShamirRecoveryOthersListRepUnknownStatus):
        raise ShamirRecoveryError(rep.status)
    assert isinstance(rep, ShamirRecoveryOthersListRepOk)

    # Load share certificates
    share_certificates: dict[UserID, ShamirRecoveryShareData] = {}
    for raw in rep.share_certificates:
        unsecure_share_certificate = ShamirRecoveryShareCertificate.unsecure_load(raw)
        author_certificate = await remote_devices_manager.get_device(
            unsecure_share_certificate.author
        )
        share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
            raw, author_certificate.verify_key, unsecure_share_certificate.author
        )
        share_data = ShamirRecoveryShareData.decrypt_verify_and_load_for(
            share_certificate.ciphered_share, device.private_key, author_certificate.verify_key
        )
        share_certificates[author_certificate.device_id.user_id] = share_data

    # Load brief certificates
    result = []
    for i, raw in enumerate(rep.brief_certificates):
        unsecure_brief_certificate = ShamirRecoveryBriefCertificate.unsecure_load(raw)
        author_certificate = await remote_devices_manager.get_device(
            unsecure_brief_certificate.author
        )
        brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
            raw, author_certificate.verify_key, unsecure_brief_certificate.author
        )
        user_certificate, _ = await remote_devices_manager.get_user(
            author_certificate.device_id.user_id
        )
        maybe_share_data = share_certificates.get(author_certificate.device_id.user_id)
        item = (author_certificate, user_certificate, brief_certificate, maybe_share_data)
        result.append(item)

    return result
