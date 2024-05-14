# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi.crypto import (
    PublicKey,
    SequesterPublicKeyDer,
    SequesterVerifyKeyDer,
    SigningKey,
    VerifyKey,
)
from parsec._parsec_pyi.enumerate import RealmRole, UserProfile
from parsec._parsec_pyi.ids import (
    DeviceID,
    DeviceLabel,
    HumanHandle,
    SequesterServiceID,
    UserID,
    VlobID,
)
from parsec._parsec_pyi.time import DateTime

class PrivateKeyAlgorithm:
    X25519_XSALSA20_POLY1305: PrivateKeyAlgorithm
    VALUES: tuple[SecretKeyAlgorithm, ...]

    @classmethod
    def from_str(cls, value: str) -> PrivateKeyAlgorithm: ...
    @property
    def str(self) -> str: ...

class UserCertificate:
    def __init__(
        self,
        author: DeviceID | None,
        timestamp: DateTime,
        user_id: UserID,
        human_handle: HumanHandle | None,
        public_key: PublicKey,
        algorithm: PrivateKeyAlgorithm,
        profile: UserProfile,
    ) -> None: ...
    @property
    def author(self) -> DeviceID | None: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def human_handle(self) -> HumanHandle: ...
    @property
    def is_redacted(self) -> bool: ...
    @property
    def public_key(self) -> PublicKey: ...
    @property
    def algorithm(self) -> PrivateKeyAlgorithm: ...
    @property
    def profile(self) -> UserProfile: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_user: UserID | None = None,
        expected_human_handle: HumanHandle | None = None,
    ) -> UserCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> UserCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def redacted_compare(self, redacted: "UserCertificate") -> bool: ...

class SigningKeyAlgorithm:
    ED25519: SigningKeyAlgorithm
    VALUES: tuple[SecretKeyAlgorithm, ...]

    @classmethod
    def from_str(cls, value: str) -> SigningKeyAlgorithm: ...
    @property
    def str(self) -> str: ...

class DeviceCertificate:
    def __init__(
        self,
        author: DeviceID | None,
        timestamp: DateTime,
        user_id: UserID,
        device_id: DeviceID,
        device_label: DeviceLabel | None,
        verify_key: VerifyKey,
        algorithm: SigningKeyAlgorithm,
    ) -> None: ...
    @property
    def author(self) -> DeviceID | None: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def device_label(self) -> DeviceLabel: ...
    @property
    def is_redacted(self) -> bool: ...
    @property
    def verify_key(self) -> VerifyKey: ...
    @property
    def algorithm(self) -> SigningKeyAlgorithm: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_device: DeviceID | None = None,
    ) -> DeviceCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> DeviceCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def redacted_compare(self, redacted: "DeviceCertificate") -> bool: ...

class RevokedUserCertificate:
    def __init__(self, author: DeviceID, timestamp: DateTime, user_id: UserID) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_device: DeviceID | None = None,
    ) -> RevokedUserCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RevokedUserCertificate:
        """Raise `ValueError` if invalid"""
        ...

class UserUpdateCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        user_id: UserID,
        new_profile: UserProfile,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def new_profile(self) -> UserProfile: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_user: UserID | None = None,
    ) -> UserUpdateCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> UserUpdateCertificate:
        """Raise `ValueError` if invalid"""
        ...

class RealmRoleCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        user_id: UserID,
        role: RealmRole | None,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def realm_id(self) -> VlobID: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def role(self) -> RealmRole | None: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_realm: VlobID | None = None,
        expected_user: UserID | None = None,
    ) -> RealmRoleCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RealmRoleCertificate:
        """Raise `ValueError` if invalid"""
        ...

class RealmNameCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        key_index: int,
        encrypted_name: bytes,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def realm_id(self) -> VlobID: ...
    @property
    def key_index(self) -> int: ...
    @property
    def encrypted_name(self) -> bytes: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_realm: VlobID | None = None,
    ) -> RealmNameCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RealmNameCertificate:
        """Raise `ValueError` if invalid"""
        ...

class SecretKeyAlgorithm:
    BLAKE2B_XSALSA20_POLY1305: SecretKeyAlgorithm
    VALUES: tuple[SecretKeyAlgorithm, ...]

    @classmethod
    def from_str(cls, value: str) -> SecretKeyAlgorithm: ...
    @property
    def str(self) -> str: ...

class HashAlgorithm:
    SHA256: HashAlgorithm
    VALUES: tuple[HashAlgorithm, ...]

    @classmethod
    def from_str(cls, value: str) -> HashAlgorithm: ...
    @property
    def str(self) -> str: ...

class RealmKeyRotationCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        key_index: int,
        encryption_algorithm: SecretKeyAlgorithm,
        hash_algorithm: HashAlgorithm,
        key_canary: bytes,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def realm_id(self) -> VlobID: ...
    @property
    def key_index(self) -> int: ...
    @property
    def encryption_algorithm(self) -> SecretKeyAlgorithm: ...
    @property
    def hash_algorithm(self) -> HashAlgorithm: ...
    @property
    def key_canary(self) -> bytes: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_realm: VlobID | None = None,
    ) -> RealmKeyRotationCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RealmKeyRotationCertificate:
        """Raise `ValueError` if invalid"""
        ...

class RealmArchivingConfiguration:
    AVAILABLE: RealmArchivingConfiguration
    ARCHIVED: RealmArchivingConfiguration

    @classmethod
    def deletion_planned(cls, deletion_date: DateTime) -> RealmArchivingConfiguration: ...
    @property
    def deletion_date(self) -> DateTime: ...

class RealmArchivingCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        configuration: RealmArchivingConfiguration,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def realm_id(self) -> VlobID: ...
    @property
    def configuration(self) -> RealmArchivingConfiguration: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_realm: VlobID | None = None,
    ) -> RealmArchivingCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RealmArchivingCertificate:
        """Raise `ValueError` if invalid"""
        ...

class ShamirRecoveryBriefCertificate:
    def __init__(
        self,
        author: DeviceID,
        user_id: UserID,
        timestamp: DateTime,
        threshold: int,
        per_recipient_shares: dict[UserID, int],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def threshold(self) -> int: ...
    @property
    def per_recipient_shares(self) -> dict[UserID, int]: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
    ) -> ShamirRecoveryBriefCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> ShamirRecoveryBriefCertificate:
        """Raise `ValueError` if invalid"""
        ...

class ShamirRecoveryShareCertificate:
    def __init__(
        self,
        author: DeviceID,
        user_id: UserID,
        timestamp: DateTime,
        recipient: UserID,
        ciphered_share: bytes,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def recipient(self) -> UserID: ...
    @property
    def ciphered_share(self) -> bytes: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_recipient: UserID | None,
    ) -> ShamirRecoveryShareCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> ShamirRecoveryShareCertificate:
        """Raise `ValueError` if invalid"""
        ...

class SequesterAuthorityCertificate:
    def __init__(self, timestamp: DateTime, verify_key_der: SequesterVerifyKeyDer) -> None: ...
    @classmethod
    def verify_and_load(
        cls, signed: bytes, author_verify_key: VerifyKey
    ) -> SequesterAuthorityCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def verify_key_der(self) -> SequesterVerifyKeyDer: ...

class SequesterServiceCertificate:
    def __init__(
        self,
        timestamp: DateTime,
        service_id: SequesterServiceID,
        service_label: str,
        encryption_key_der: SequesterPublicKeyDer,
    ) -> None: ...
    @classmethod
    def load(cls, data: bytes) -> SequesterServiceCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump(self) -> bytes: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def service_id(self) -> SequesterServiceID: ...
    @property
    def service_label(self) -> str: ...
    @property
    def encryption_key_der(self) -> SequesterPublicKeyDer: ...

class SequesterRevokedServiceCertificate:
    def __init__(
        self,
        timestamp: DateTime,
        service_id: SequesterServiceID,
    ) -> None: ...
    @classmethod
    def load(cls, data: bytes) -> SequesterRevokedServiceCertificate:
        """Raise `ValueError` if invalid"""
        ...
    def dump(self) -> bytes: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def service_id(self) -> SequesterServiceID: ...
