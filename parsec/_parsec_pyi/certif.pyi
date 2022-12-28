# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from typing import Any

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
    RealmID,
    SequesterServiceID,
    UserID,
)
from parsec._parsec_pyi.time import DateTime

class UserCertificate:
    def __init__(
        self,
        author: DeviceID | None,
        timestamp: DateTime,
        user_id: UserID,
        human_handle: HumanHandle | None,
        public_key: PublicKey,
        profile: UserProfile,
    ) -> None: ...
    @property
    def is_admin(self) -> bool: ...
    @property
    def author(self) -> DeviceID | None: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def human_handle(self) -> HumanHandle | None: ...
    @property
    def public_key(self) -> PublicKey: ...
    @property
    def profile(self) -> UserProfile: ...
    def evolve(self, **kwargs: Any) -> UserCertificate: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_user: UserID | None = None,
        expected_human_handle: HumanHandle | None = None,
    ) -> UserCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> UserCertificate: ...

class DeviceCertificate:
    def __init__(
        self,
        author: DeviceID | None,
        timestamp: DateTime,
        device_id: DeviceID,
        device_label: DeviceLabel | None,
        verify_key: VerifyKey,
    ) -> None: ...
    @property
    def author(self) -> DeviceID | None: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def device_label(self) -> DeviceLabel | None: ...
    @property
    def verify_key(self) -> VerifyKey: ...
    def evolve(self, **kwargs: Any) -> DeviceCertificate: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_device: DeviceID | None = None,
    ) -> DeviceCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> DeviceCertificate: ...

class RevokedUserCertificate:
    def __init__(self, author: DeviceID, timestamp: DateTime, user_id: UserID) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    def evolve(self, **kwargs: Any) -> RevokedUserCertificate: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_device: DeviceID | None = None,
    ) -> RevokedUserCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RevokedUserCertificate: ...

class RealmRoleCertificate:
    def __init__(
        self,
        author: DeviceID | None,
        timestamp: DateTime,
        realm_id: RealmID,
        user_id: UserID,
        role: RealmRole | None,
    ) -> None: ...
    @property
    def author(self) -> DeviceID | None: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def realm_id(self) -> RealmID: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def role(self) -> RealmRole | None: ...
    def evolve(self, **kwargs: Any) -> RealmRoleCertificate: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None = None,
        expected_realm: RealmID | None = None,
        expected_user: UserID | None = None,
    ) -> RealmRoleCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RealmRoleCertificate: ...
    @classmethod
    def build_realm_root_certif(
        cls, author: DeviceID, timestamp: DateTime, realm_id: RealmID
    ) -> RealmRoleCertificate: ...

class SequesterAuthorityCertificate:
    def __init__(self, timestamp: DateTime, verify_key_der: SequesterVerifyKeyDer) -> None: ...
    @classmethod
    def verify_and_load(
        cls, signed: bytes, author_verify_key: VerifyKey
    ) -> SequesterAuthorityCertificate: ...
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
    def load(cls, data: bytes) -> SequesterServiceCertificate: ...
    def dump(self) -> bytes: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def service_id(self) -> SequesterServiceID: ...
    @property
    def service_label(self) -> str: ...
    @property
    def encryption_key_der(self) -> SequesterPublicKeyDer: ...
