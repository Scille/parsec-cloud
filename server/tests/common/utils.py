# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    DevicePurpose,
    HumanHandle,
    PrivateKeyAlgorithm,
    PublicKey,
    SigningKey,
    SigningKeyAlgorithm,
    UserCertificate,
    UserID,
    UserProfile,
    VerifyKey,
)


@dataclass(frozen=True, slots=True)
class DeviceCertificates:
    certificate: DeviceCertificate
    redacted_certificate: DeviceCertificate
    signed_certificate: bytes
    signed_redacted_certificate: bytes


def generate_new_device_certificates(
    timestamp: DateTime,
    user_id: UserID,
    device_id: DeviceID,
    device_label: DeviceLabel,
    verify_key: VerifyKey,
    author_signing_key: SigningKey,
    author_device_id: DeviceID | None,
    purpose=DevicePurpose.STANDARD,
    algorithm=SigningKeyAlgorithm.ED25519,
) -> DeviceCertificates:
    certificate = DeviceCertificate(
        author=author_device_id,
        timestamp=timestamp,
        purpose=purpose,
        user_id=user_id,
        device_id=device_id,
        device_label=device_label,
        verify_key=verify_key,
        algorithm=algorithm,
    )
    signed_certificate = certificate.dump_and_sign(author_signing_key)

    redacted_certificate = DeviceCertificate(
        author=certificate.author,
        timestamp=certificate.timestamp,
        purpose=certificate.purpose,
        user_id=certificate.user_id,
        device_id=certificate.device_id,
        device_label=None,
        verify_key=certificate.verify_key,
        algorithm=certificate.algorithm,
    )
    signed_redacted_certificate = redacted_certificate.dump_and_sign(author_signing_key)

    return DeviceCertificates(
        certificate, redacted_certificate, signed_certificate, signed_redacted_certificate
    )


@dataclass(frozen=True, slots=True)
class UserCertificates:
    certificate: UserCertificate
    redacted_certificate: UserCertificate
    signed_certificate: bytes
    signed_redacted_certificate: bytes


def generate_new_user_certificates(
    timestamp: DateTime,
    user_id: UserID,
    human_handle: HumanHandle,
    profile: UserProfile,
    public_key: PublicKey,
    author_signing_key: SigningKey,
    author_device_id: DeviceID | None = None,
    algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
) -> UserCertificates:
    certificate = UserCertificate(
        author=author_device_id,
        timestamp=timestamp,
        user_id=user_id,
        human_handle=human_handle,
        profile=profile,
        public_key=public_key,
        algorithm=algorithm,
    )
    signed_certificate = certificate.dump_and_sign(author_signing_key)

    redacted_certificate = UserCertificate(
        author=certificate.author,
        timestamp=certificate.timestamp,
        user_id=certificate.user_id,
        human_handle=None,
        profile=certificate.profile,
        public_key=certificate.public_key,
        algorithm=certificate.algorithm,
    )

    signed_redacted_certificate = redacted_certificate.dump_and_sign(author_signing_key)

    return UserCertificates(
        certificate, redacted_certificate, signed_certificate, signed_redacted_certificate
    )
