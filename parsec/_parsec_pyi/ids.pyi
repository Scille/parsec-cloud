# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

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
    def __lt__(self, other: DeviceLabel) -> bool: ...
    def __gt__(self, other: DeviceLabel) -> bool: ...
    def __le__(self, other: DeviceLabel) -> bool: ...
    def __ge__(self, other: DeviceLabel) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...

class HumanHandle:
    def __init__(self, email: str, label: str) -> None: ...
    def __lt__(self, other: HumanHandle | None) -> bool: ...
    def __gt__(self, other: HumanHandle | None) -> bool: ...
    def __le__(self, other: HumanHandle | None) -> bool: ...
    def __ge__(self, other: HumanHandle | None) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def email(self) -> str: ...
    @property
    def label(self) -> str: ...
    @property
    def str(self) -> str: ...

class RealmID:
    def __lt__(self, other: RealmID) -> bool: ...
    def __gt__(self, other: RealmID) -> bool: ...
    def __le__(self, other: RealmID) -> bool: ...
    def __ge__(self, other: RealmID) -> bool: ...
    def __hash__(self) -> int: ...
    def to_entry_id(self) -> EntryID: ...
    @classmethod
    def from_entry_id(cls, id: EntryID) -> RealmID: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> RealmID: ...
    @classmethod
    def from_hex(cls, hex: str) -> RealmID: ...
    @classmethod
    def new(cls) -> RealmID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
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

class VlobID:
    def __lt__(self, other: VlobID) -> bool: ...
    def __gt__(self, other: VlobID) -> bool: ...
    def __le__(self, other: VlobID) -> bool: ...
    def __ge__(self, other: VlobID) -> bool: ...
    def __hash__(self) -> int: ...
    def to_entry_id(self) -> EntryID: ...
    @classmethod
    def from_entry_id(cls, id: EntryID) -> VlobID: ...
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
    def hyphenated(self) -> str: ...

class EntryID:
    def __lt__(self, other: EntryID) -> bool: ...
    def __gt__(self, other: EntryID) -> bool: ...
    def __le__(self, other: EntryID) -> bool: ...
    def __ge__(self, other: EntryID) -> bool: ...
    def __hash__(self) -> int: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> EntryID: ...
    @classmethod
    def from_hex(cls, hex: str) -> EntryID: ...
    @classmethod
    def new(cls) -> EntryID: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def hyphenated(self) -> str: ...

class ChunkID:
    def __lt__(self, other: ChunkID | EntryID) -> bool: ...
    def __gt__(self, other: ChunkID | EntryID) -> bool: ...
    def __le__(self, other: ChunkID | EntryID) -> bool: ...
    def __ge__(self, other: ChunkID | EntryID) -> bool: ...
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
    def hyphenated(self) -> str: ...

class ShamirRevealToken:
    def __hash__(self) -> int: ...
    @classmethod
    def from_bytes(cls, bytes: bytes) -> ShamirRevealToken: ...
    @classmethod
    def from_hex(cls, hex: str) -> ShamirRevealToken: ...
    @classmethod
    def new(cls) -> ShamirRevealToken: ...
    @property
    def bytes(self) -> bytes: ...
    @property
    def hex(self) -> str: ...
    @property
    def hyphenated(self) -> str: ...
