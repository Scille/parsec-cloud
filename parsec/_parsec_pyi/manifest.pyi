# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from typing import Any, Tuple, Union

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    EntryID,
    HashDigest,
    SecretKey,
    SigningKey,
    VerifyKey,
)
from parsec.api.protocol import RealmRole
from parsec.types import FrozenDict

AnyRemoteManifest = Union[
    FolderManifest,
    FileManifest,
    WorkspaceManifest,
    UserManifest,
]

class EntryName:
    def __init__(self, name: str) -> None: ...
    def __str__(self) -> str: ...
    def __lt__(self, other: EntryName | None) -> bool: ...
    def __gt__(self, other: EntryName | None) -> bool: ...
    def __le__(self, other: EntryName | None) -> bool: ...
    def __ge__(self, other: EntryName | None) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...

class WorkspaceEntry:
    def __init__(
        self,
        name: EntryName,
        id: EntryID,
        key: SecretKey,
        encryption_revision: int,
        encrypted_on: DateTime,
        role_cached_on: DateTime,
        role: RealmRole | None,
    ) -> None: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def name(self) -> EntryName: ...
    @property
    def key(self) -> SecretKey: ...
    @property
    def encryption_revision(self) -> int: ...
    @property
    def encrypted_on(self) -> DateTime: ...
    @property
    def role_cached_on(self) -> DateTime: ...
    @property
    def role(self) -> RealmRole | None: ...
    @classmethod
    def new(cls, name: EntryName, timestamp: DateTime) -> WorkspaceEntry: ...
    def evolve(self, **kwargs: Any) -> WorkspaceEntry: ...
    def is_revoked(self) -> bool: ...

class BlockAccess:
    def __init__(
        self, id: BlockID, key: SecretKey, offset: int, size: int, digest: HashDigest
    ) -> None: ...
    @property
    def id(self) -> BlockID: ...
    @property
    def key(self) -> SecretKey: ...
    @property
    def offset(self) -> int: ...
    @property
    def size(self) -> int: ...
    @property
    def digest(self) -> HashDigest: ...

class FolderManifest:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: EntryID,
        parent: EntryID,
        version: int,
        created: DateTime,
        updated: DateTime,
        children: FrozenDict[EntryName, EntryID],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def parent(self) -> EntryID: ...
    @property
    def version(self) -> int: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def created(self) -> DateTime: ...
    @property
    def updated(self) -> DateTime: ...
    @property
    def children(self) -> FrozenDict[EntryName, EntryID]: ...
    def evolve(self, **kwargs: Any) -> FolderManifest: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_verify_and_load(
        cls,
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: EntryID | None = None,
        expected_version: int | None = None,
    ) -> FolderManifest: ...

class FileManifest:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: EntryID,
        parent: EntryID,
        version: int,
        created: DateTime,
        updated: DateTime,
        size: int,
        blocksize: int,
        blocks: Tuple[BlockAccess],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def parent(self) -> EntryID: ...
    @property
    def version(self) -> int: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def created(self) -> DateTime: ...
    @property
    def updated(self) -> DateTime: ...
    @property
    def size(self) -> int: ...
    @property
    def blocksize(self) -> int: ...
    @property
    def blocks(self) -> Tuple[BlockAccess]: ...
    def evolve(self, **kwargs: Any) -> FileManifest: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_verify_and_load(
        cls,
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: EntryID | None = None,
        expected_version: int | None = None,
    ) -> FileManifest: ...

class WorkspaceManifest:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: EntryID,
        version: int,
        created: DateTime,
        updated: DateTime,
        children: FrozenDict[EntryName, EntryID],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def version(self) -> int: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def created(self) -> DateTime: ...
    @property
    def updated(self) -> DateTime: ...
    @property
    def children(self) -> FrozenDict[EntryName, EntryID]: ...
    def evolve(self, **kwargs: Any) -> WorkspaceManifest: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_verify_and_load(
        cls,
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: EntryID | None = None,
        expected_version: int | None = None,
    ) -> WorkspaceManifest: ...

class UserManifest:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: EntryID,
        version: int,
        created: DateTime,
        updated: DateTime,
        last_processed_message: int,
        workspaces: Tuple[WorkspaceEntry],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def version(self) -> int: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def created(self) -> DateTime: ...
    @property
    def updated(self) -> DateTime: ...
    @property
    def last_processed_message(self) -> int: ...
    @property
    def workspaces(self) -> Tuple[WorkspaceEntry]: ...
    def evolve(self, **kwargs: Any) -> UserManifest: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    @classmethod
    def decrypt_verify_and_load(
        cls,
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: EntryID | None = None,
        expected_version: int | None = None,
    ) -> UserManifest: ...
    def get_workspace_entry(self, workspace_id: EntryID) -> WorkspaceEntry | None: ...

def manifest_decrypt_and_load(encrypted: bytes, key: SecretKey) -> AnyRemoteManifest: ...
def manifest_decrypt_verify_and_load(
    encrypted: bytes,
    key: SecretKey,
    author_verify_key: VerifyKey,
    expected_author: DeviceID,
    expected_timestamp: DateTime,
    expected_id: EntryID | None = None,
    expected_version: int | None = None,
) -> AnyRemoteManifest: ...
def manifest_verify_and_load(
    signed: bytes,
    author_verify_key: VerifyKey,
    expected_author: DeviceID,
    expected_timestamp: DateTime,
    expected_id: EntryID | None = None,
    expected_version: int | None = None,
) -> AnyRemoteManifest: ...
def manifest_unverified_load(
    data: bytes,
) -> AnyRemoteManifest: ...
