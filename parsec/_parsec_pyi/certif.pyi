from typing import Any, Optional, List

from parsec._parsec_pyi.crypto import PublicKey, SigningKey, VerifyKey
from parsec._parsec_pyi.ids import DeviceID, DeviceLabel, HumanHandle, RealmID, UserID
from parsec._parsec_pyi.time import DateTime

from parsec.api.protocol import UserProfile

class RealmRole:
    values: List[RealmRole]
    OWNER: RealmRole
    MANAGER: RealmRole
    CONTRIBUTOR: RealmRole
    READER: RealmRole
    @property
    def value(self) -> str: ...

class UserCertificate:
    def __init__(
        self,
        author: Optional[DeviceID],
        timestamp: DateTime,
        user_id: UserID,
        human_handle: Optional[HumanHandle],
        public_key: PublicKey,
        profile: UserProfile,
    ) -> None: ...
    @property
    def is_admin(self) -> bool: ...
    @property
    def author(self) -> Optional[DeviceID]: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def human_handle(self) -> Optional[HumanHandle]: ...
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
        expected_author: Optional[DeviceID] = None,
        expected_user: Optional[UserID] = None,
        expected_human_handle: Optional[HumanHandle] = None,
    ) -> UserCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> UserCertificate: ...

class DeviceCertificate:
    def __init__(
        self,
        author: Optional[DeviceID],
        timestamp: DateTime,
        device_id: DeviceID,
        device_label: Optional[DeviceLabel],
        verify_key: VerifyKey,
    ) -> None: ...
    @property
    def author(self) -> Optional[DeviceID]: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def device_label(self) -> Optional[DeviceLabel]: ...
    @property
    def verify_key(self) -> VerifyKey: ...
    def evolve(self, **kwargs: Any) -> DeviceCertificate: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: Optional[DeviceID] = None,
        expected_device: Optional[DeviceID] = None,
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
        expected_author: Optional[DeviceID] = None,
        expected_device: Optional[DeviceID] = None,
    ) -> RevokedUserCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RevokedUserCertificate: ...

class RealmRoleCertificate:
    def __init__(
        self,
        author: Optional[DeviceID],
        timestamp: DateTime,
        realm_id: RealmID,
        user_id: UserID,
        role: Optional[RealmRole],
    ) -> None: ...
    @property
    def author(self) -> Optional[DeviceID]: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def realm_id(self) -> RealmID: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def role(self) -> Optional[RealmRole]: ...
    def evolve(self, **kwargs: Any) -> RealmRoleCertificate: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: Optional[DeviceID] = None,
        expected_realm: Optional[RealmID] = None,
        expected_user: Optional[UserID] = None,
    ) -> RealmRoleCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, signed: bytes) -> RealmRoleCertificate: ...
    @classmethod
    def build_realm_root_certif(
        cls, author: DeviceID, timestamp: DateTime, realm_id: RealmID
    ) -> RealmRoleCertificate: ...
