# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Tuple

import attr

from parsec._parsec import DateTime
from parsec.api.data import DataError, DeviceCertificate, UserCertificate
from parsec.api.protocol import DeviceID, DeviceLabel, DeviceName, HumanHandle, UserID, UserProfile
from parsec.crypto import PublicKey, VerifyKey
from parsec.utils import timestamps_in_the_ballpark


class CertificateValidationError(Exception):
    def __init__(self, status: str, reason: str) -> None:
        self.status = status
        self.reason = reason
        super().__init__((status, reason))


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class Device:
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.device_id.str})"

    def evolve(self, **kwargs: Any) -> Device:
        return attr.evolve(self, **kwargs)

    @property
    def device_name(self) -> DeviceName:
        return self.device_id.device_name

    @property
    def user_id(self) -> UserID:
        return self.device_id.user_id

    @property
    def verify_key(self) -> VerifyKey:
        return DeviceCertificate.unsecure_load(self.device_certificate).verify_key

    device_id: DeviceID
    device_label: DeviceLabel | None
    device_certificate: bytes
    redacted_device_certificate: bytes
    device_certifier: DeviceID | None
    created_on: DateTime = attr.ib(factory=DateTime.now)


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class User:
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.user_id.str})"

    def evolve(self, **kwargs: Any) -> User:
        return attr.evolve(self, **kwargs)

    def is_revoked(self) -> bool:
        return self.revoked_on is not None

    @property
    def public_key(self) -> PublicKey:
        return UserCertificate.unsecure_load(self.user_certificate).public_key

    user_id: UserID
    human_handle: HumanHandle | None
    user_certificate: bytes
    redacted_user_certificate: bytes
    user_certifier: DeviceID | None
    profile: UserProfile = UserProfile.STANDARD
    created_on: DateTime = attr.ib(factory=DateTime.now)
    revoked_on: DateTime | None = None
    revoked_user_certificate: bytes | None = None
    revoked_user_certifier: DeviceID | None = None


def validate_new_user_certificates(
    expected_author: DeviceID,
    author_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> Tuple[User, Device]:
    """
    Raises:
        CertificateValidationError
        UserInvalidCertificationError
    """
    try:
        d_data = DeviceCertificate.verify_and_load(
            device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )
        u_data = UserCertificate.verify_and_load(
            user_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )
        ru_data = UserCertificate.verify_and_load(
            redacted_user_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )
        rd_data = DeviceCertificate.verify_and_load(
            redacted_device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )

    except DataError as exc:
        raise CertificateValidationError(
            "invalid_certification", f"Invalid certification data ({exc})."
        )

    if u_data.timestamp != d_data.timestamp:
        raise CertificateValidationError(
            "invalid_data", "Device and User certificates must have the same timestamp."
        )

    now = DateTime.now()
    if not timestamps_in_the_ballpark(u_data.timestamp, now):
        raise CertificateValidationError(
            "invalid_certification", "Invalid timestamp in certificate."
        )

    if u_data.user_id != d_data.device_id.user_id:
        raise CertificateValidationError(
            "invalid_data", "Device and User must have the same user ID."
        )

    if ru_data.evolve(human_handle=u_data.human_handle) != u_data:
        raise CertificateValidationError(
            "invalid_data", "Redacted User certificate differs from User certificate."
        )

    if ru_data.human_handle:
        raise CertificateValidationError(
            "invalid_data",
            "Redacted User certificate must not contain a human_handle field.",
        )

    if rd_data.evolve(device_label=d_data.device_label) != d_data:
        raise CertificateValidationError(
            "invalid_data",
            "Redacted Device certificate differs from Device certificate.",
        )

    if rd_data.device_label:
        raise CertificateValidationError(
            "invalid_data",
            "Redacted Device certificate must not contain a device_label field.",
        )

    user = User(
        user_id=u_data.user_id,
        human_handle=u_data.human_handle,
        profile=u_data.profile,
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        user_certifier=u_data.author,
        created_on=u_data.timestamp,
    )
    first_device = Device(
        device_id=d_data.device_id,
        device_label=d_data.device_label,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
        device_certifier=d_data.author,
        created_on=d_data.timestamp,
    )

    return user, first_device


def validate_new_device_certificate(
    expected_author: DeviceID,
    author_verify_key: VerifyKey,
    device_certificate: bytes,
    redacted_device_certificate: bytes,
) -> Device:
    try:
        data = DeviceCertificate.verify_and_load(
            device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )

        redacted_data = DeviceCertificate.verify_and_load(
            redacted_device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )

    except DataError as exc:
        raise CertificateValidationError(
            "invalid_certification", f"Invalid certification data ({exc})."
        )

    if not timestamps_in_the_ballpark(data.timestamp, DateTime.now()):
        raise CertificateValidationError(
            "invalid_certification", f"Invalid timestamp in certification."
        )

    if data.device_id.user_id != expected_author.user_id:
        raise CertificateValidationError("bad_user_id", "Device must be handled by it own user.")

    if redacted_data.evolve(device_label=data.device_label) != data:
        raise CertificateValidationError(
            "invalid_data",
            "Redacted Device certificate differs from Device certificate.",
        )
    if redacted_data.device_label:
        raise CertificateValidationError(
            "invalid_data",
            "Redacted Device certificate must not contain a device_label field.",
        )

    return Device(
        device_id=data.device_id,
        device_label=data.device_label,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
        device_certifier=data.author,
        created_on=data.timestamp,
    )
