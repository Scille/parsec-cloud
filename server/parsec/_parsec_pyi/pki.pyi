# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from pathlib import Path

from parsec._parsec_pyi.addrs import ParsecPkiEnrollmentAddr
from parsec._parsec_pyi.crypto import PublicKey, VerifyKey
from parsec._parsec_pyi.enumerate import UserProfile
from parsec._parsec_pyi.ids import DeviceID, DeviceLabel, EnrollmentID, HumanHandle
from parsec._parsec_pyi.time import DateTime

class PkiEnrollmentAnswerPayload:
    def __init__(
        self,
        device_id: DeviceID,
        device_label: DeviceLabel,
        human_handle: HumanHandle,
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ): ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def device_label(self) -> DeviceLabel: ...
    @property
    def human_handle(self) -> HumanHandle: ...
    @property
    def profile(self) -> UserProfile: ...
    @property
    def root_verify_key(self) -> VerifyKey: ...
    @classmethod
    def load(cls, raw: bytes) -> PkiEnrollmentAnswerPayload: ...
    def dump(self) -> bytes: ...

class PkiEnrollmentSubmitPayload:
    def __init__(
        self, verify_key: VerifyKey, public_key: PublicKey, requested_device_label: DeviceLabel
    ) -> None: ...
    @property
    def verify_key(self) -> VerifyKey: ...
    @property
    def public_key(self) -> PublicKey: ...
    @property
    def requested_device_label(self) -> DeviceLabel: ...
    @classmethod
    def load(cls, raw: bytes) -> PkiEnrollmentSubmitPayload: ...
    def dump(self) -> bytes: ...

class X509Certificate:
    def __init__(
        self,
        issuer: dict[str, str],
        subject: dict[str, str],
        der_x509_certificate: bytes,
        certificate_sha256: bytes,
        certificate_id: str | None,
    ) -> None: ...
    def is_available_locally(self) -> bool: ...
    @property
    def subject_common_name(self) -> str | None: ...
    @property
    def subject_email_address(self) -> str | None: ...
    @property
    def issuer_common_name(self) -> str | None: ...
    @property
    def issuer(self) -> dict[str, str]: ...
    @property
    def subject(self) -> dict[str, str]: ...
    @property
    def der_x509_certificate(self) -> bytes: ...
    @property
    def certificate_sha256(self) -> bytes: ...
    @property
    def certificate_id(self) -> str | None: ...

class LocalPendingEnrollment:
    def __init__(
        self,
        x509_certificate: X509Certificate,
        addr: ParsecPkiEnrollmentAddr,
        submitted_on: DateTime,
        enrollment_id: EnrollmentID,
        submit_payload: PkiEnrollmentSubmitPayload,
        encrypted_key: bytes,
        ciphertext: bytes,
    ) -> None: ...
    @classmethod
    def load(cls, raw: bytes) -> LocalPendingEnrollment: ...
    def dump(self) -> bytes: ...
    def save(self, config_dir: Path) -> str: ...
    @classmethod
    def load_from_path(cls, path: Path) -> LocalPendingEnrollment: ...
    @classmethod
    def load_from_enrollment_id(
        cls,
        config_dir: Path,
        enrollment_id: EnrollmentID,
    ) -> LocalPendingEnrollment: ...
    @classmethod
    def remove_from_enrollment_id(
        cls,
        config_dir: Path,
        enrollment_id: EnrollmentID,
    ) -> None: ...
    @classmethod
    def list(cls, config_dir: Path) -> list[LocalPendingEnrollment]: ...
    @property
    def x509_certificate(self) -> X509Certificate: ...
    @property
    def addr(self) -> ParsecPkiEnrollmentAddr: ...
    @property
    def submitted_on(self) -> DateTime: ...
    @property
    def enrollment_id(self) -> EnrollmentID: ...
    @property
    def submit_payload(self) -> PkiEnrollmentSubmitPayload: ...
    @property
    def encrypted_key(self) -> bytes: ...
    @property
    def ciphertext(self) -> bytes: ...
