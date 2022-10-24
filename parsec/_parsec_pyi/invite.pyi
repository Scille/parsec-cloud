from __future__ import annotations
from typing import List, Optional, Tuple
from uuid import UUID
from parsec._parsec import (
    DeviceID,
    EntryID,
    PrivateKey,
    SecretKey,
    DeviceLabel,
    HumanHandle,
    VerifyKey,
    PublicKey,
)
from parsec.api.protocol.types import UserProfile

class InvitationToken:
    def __init__(self, uuid: UUID) -> None: ...
    def __str__(self) -> str: ...
    def __lt__(self, other: InvitationToken) -> bool: ...
    def __gt__(self, other: InvitationToken) -> bool: ...
    def __le__(self, other: InvitationToken) -> bool: ...
    def __ge__(self, other: InvitationToken) -> bool: ...
    def __hash__(self) -> int: ...
    def uuid(self) -> UUID: ...
    @classmethod
    def new(cls) -> InvitationToken: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> InvitationToken: ...
    @classmethod
    def from_hex(cls, hex: str) -> InvitationToken: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...

class SASCode:
    def __init__(self, code: str) -> None: ...
    def __str__(self) -> str: ...
    def __lt__(self, other: SASCode) -> bool: ...
    def __gt__(self, other: SASCode) -> bool: ...
    def __le__(self, other: SASCode) -> bool: ...
    def __ge__(self, other: SASCode) -> bool: ...
    def __hash__(self) -> int: ...
    @classmethod
    def from_int(cls, num: int) -> SASCode: ...
    @property
    def str(self) -> str: ...

def generate_sas_codes(
    claimer_nonce: bytes, greeter_nonce: bytes, shared_secret_key: SecretKey
) -> Tuple[SASCode, SASCode]: ...
def generate_sas_code_candidates(valid_sas: SASCode, size: int) -> List[SASCode]: ...

class InviteUserData:
    def __init__(
        self,
        requested_device_label: Optional[DeviceLabel],
        requested_human_handle: Optional[HumanHandle],
        public_key: PublicKey,
        verify_key: VerifyKey,
    ) -> None: ...
    @property
    def requested_human_handle(self) -> Optional[HumanHandle]: ...
    @property
    def requested_device_label(self) -> Optional[DeviceLabel]: ...
    @property
    def public_key(self) -> PublicKey: ...
    @property
    def verify_key(self) -> VerifyKey: ...
    @classmethod
    def dump_and_encrypt(cls, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_and_load(cls, encrypted: bytes, key: SecretKey) -> InviteUserData: ...

class InviteUserConfirmation:
    def __init__(
        self,
        device_id: DeviceID,
        device_label: Optional[DeviceLabel],
        human_handle: Optional[HumanHandle],
        profile: UserProfile,
        root_verify_key: VerifyKey,
    ) -> None: ...
    @property
    def human_handle(self) -> Optional[HumanHandle]: ...
    @property
    def device_label(self) -> Optional[DeviceLabel]: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def profile(self) -> UserProfile: ...
    @property
    def root_verify_key(self) -> VerifyKey: ...
    def dump_and_encrypt(self, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_and_load(cls, encrypted: bytes, key: SecretKey) -> InviteUserConfirmation: ...

class InviteDeviceData:
    def __init__(
        self, requested_device_label: Optional[DeviceLabel], verify_key: VerifyKey
    ) -> None: ...
    @property
    def requested_device_label(self) -> Optional[DeviceLabel]: ...
    @property
    def verify_key(self) -> VerifyKey: ...
    def dump_and_encrypt(self, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_and_load(cls, encrypted: bytes, key: SecretKey) -> InviteDeviceData: ...

class InviteDeviceConfirmation:
    def __init__(
        self,
        device_id: DeviceID,
        device_label: Optional[DeviceLabel],
        human_handle: Optional[HumanHandle],
        profile: UserProfile,
        private_key: PrivateKey,
        root_verify_key: VerifyKey,
        user_manifest_id: EntryID,
        user_manifest_key: SecretKey,
    ) -> None: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def device_label(self) -> Optional[DeviceLabel]: ...
    @property
    def human_handle(self) -> Optional[HumanHandle]: ...
    @property
    def profile(self) -> UserProfile: ...
    @property
    def private_key(self) -> PrivateKey: ...
    @property
    def root_verify_key(self) -> VerifyKey: ...
    @property
    def user_manifest_id(self) -> EntryID: ...
    @property
    def user_manifest_key(self) -> SecretKey: ...
    def dump_and_encrypt(self, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_and_load(cls, encrypted: bytes, key: SecretKey) -> InviteDeviceConfirmation: ...
