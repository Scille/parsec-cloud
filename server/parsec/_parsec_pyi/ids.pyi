# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

class OrganizationID:
    def __init__(self, data: str) -> None: ...
    def __lt__(self, other: OrganizationID) -> bool: ...
    def __gt__(self, other: OrganizationID) -> bool: ...
    def __le__(self, other: OrganizationID) -> bool: ...
    def __ge__(self, other: OrganizationID) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...

class UserID:
    def __init__(self, data: str) -> None: ...
    def __lt__(self, other: UserID) -> bool: ...
    def __gt__(self, other: UserID) -> bool: ...
    def __le__(self, other: UserID) -> bool: ...
    def __ge__(self, other: UserID) -> bool: ...
    def __hash__(self) -> int: ...
    def to_device_id(self, device_name: DeviceName) -> DeviceID: ...
    @property
    def str(self) -> str: ...

class DeviceName:
    def __init__(self, data: str) -> None: ...
    def __lt__(self, other: DeviceName) -> bool: ...
    def __gt__(self, other: DeviceName) -> bool: ...
    def __le__(self, other: DeviceName) -> bool: ...
    def __ge__(self, other: DeviceName) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...
    @classmethod
    def new(cls) -> DeviceName: ...

class DeviceID:
    def __init__(self, data: str) -> None: ...
    def __lt__(self, other: DeviceID | None) -> bool: ...
    def __gt__(self, other: DeviceID | None) -> bool: ...
    def __le__(self, other: DeviceID | None) -> bool: ...
    def __ge__(self, other: DeviceID | None) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def device_name(self) -> DeviceName: ...
    @classmethod
    def new(cls) -> DeviceID: ...

class DeviceLabel:
    def __init__(self, data: str) -> None: ...
    @staticmethod
    def new_redacted(device_name: DeviceName) -> DeviceLabel: ...
    def __lt__(self, other: DeviceLabel) -> bool: ...
    def __gt__(self, other: DeviceLabel) -> bool: ...
    def __le__(self, other: DeviceLabel) -> bool: ...
    def __ge__(self, other: DeviceLabel) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...

class HumanHandle:
    def __init__(self, email: str, label: str) -> None: ...
    @staticmethod
    def new_redacted(user_id: UserID) -> HumanHandle: ...
    def __lt__(self, other: HumanHandle) -> bool: ...
    def __gt__(self, other: HumanHandle) -> bool: ...
    def __le__(self, other: HumanHandle) -> bool: ...
    def __ge__(self, other: HumanHandle) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def email(self) -> str: ...
    @property
    def label(self) -> str: ...
    @property
    def str(self) -> str: ...

class VlobID:
    def __lt__(self, other: VlobID) -> bool: ...
    def __gt__(self, other: VlobID) -> bool: ...
    def __le__(self, other: VlobID) -> bool: ...
    def __ge__(self, other: VlobID) -> bool: ...
    def __hash__(self) -> int: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> VlobID: ...
    @classmethod
    def from_hex(cls, hex: str) -> VlobID: ...
    @classmethod
    def new(cls) -> VlobID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> int: ...
    @property
    def hyphenated(self) -> str: ...

class BlockID:
    def __lt__(self, other: BlockID) -> bool: ...
    def __gt__(self, other: BlockID) -> bool: ...
    def __le__(self, other: BlockID) -> bool: ...
    def __ge__(self, other: BlockID) -> bool: ...
    def __hash__(self) -> int: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> BlockID: ...
    @classmethod
    def from_hex(cls, hex: str) -> BlockID: ...
    @classmethod
    def new(cls) -> BlockID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> int: ...
    @property
    def hyphenated(self) -> str: ...

class ChunkID:
    def __lt__(self, other: ChunkID | VlobID) -> bool: ...
    def __gt__(self, other: ChunkID | VlobID) -> bool: ...
    def __le__(self, other: ChunkID | VlobID) -> bool: ...
    def __ge__(self, other: ChunkID | VlobID) -> bool: ...
    def __hash__(self) -> int: ...
    @classmethod
    def from_block_id(cls, id: BlockID) -> ChunkID: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> ChunkID: ...
    @classmethod
    def from_hex(cls, hex: str) -> ChunkID: ...
    @classmethod
    def new(cls) -> ChunkID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> int: ...
    @property
    def hyphenated(self) -> str: ...

class SequesterServiceID:
    def __lt__(self, other: SequesterServiceID) -> bool: ...
    def __gt__(self, other: SequesterServiceID) -> bool: ...
    def __le__(self, other: SequesterServiceID) -> bool: ...
    def __ge__(self, other: SequesterServiceID) -> bool: ...
    def __hash__(self) -> int: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> SequesterServiceID: ...
    @classmethod
    def from_hex(cls, hex: str) -> SequesterServiceID: ...
    @classmethod
    def new(cls) -> SequesterServiceID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> int: ...
    @property
    def hyphenated(self) -> str: ...

class EnrollmentID:
    @classmethod
    def from_bytes(cls, bytes: bytes) -> EnrollmentID: ...
    @classmethod
    def from_hex(cls, hex: str) -> EnrollmentID: ...
    @classmethod
    def new(cls) -> EnrollmentID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> int: ...
    @property
    def hyphenated(self) -> str: ...

class InvitationToken:
    def __hash__(self) -> int: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> InvitationToken: ...
    @classmethod
    def from_hex(cls, hex: str) -> InvitationToken: ...
    @classmethod
    def new(cls) -> InvitationToken: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> int: ...
    @property
    def hyphenated(self) -> str: ...
