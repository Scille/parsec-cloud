# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Any, Union

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    HashDigest,
    SecretKey,
    SigningKey,
    VerifyKey,
    VlobID,
)

ChildManifest = Union[
    FolderManifest,
    FileManifest,
]

class EntryName:
    def __init__(self, name: str) -> None:
        """Raise `ValueError` if `name` is invalid"""
        ...
    def __str__(self) -> str: ...
    def __lt__(self, other: EntryName | None) -> bool: ...
    def __gt__(self, other: EntryName | None) -> bool: ...
    def __le__(self, other: EntryName | None) -> bool: ...
    def __ge__(self, other: EntryName | None) -> bool: ...
    def __hash__(self) -> int: ...
    @property
    def str(self) -> str: ...

class BlockAccess:
    def __init__(
        self, id: BlockID, key_index: int, offset: int, size: int, digest: HashDigest
    ) -> None: ...
    @property
    def id(self) -> BlockID: ...
    @property
    def key_index(self) -> int: ...
    @property
    def key(self) -> SecretKey | None: ...
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
        id: VlobID,
        parent: VlobID,
        version: int,
        created: DateTime,
        updated: DateTime,
        children: dict[EntryName, VlobID],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> VlobID: ...
    @property
    def parent(self) -> VlobID: ...
    @property
    def version(self) -> int: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def created(self) -> DateTime: ...
    @property
    def updated(self) -> DateTime: ...
    @property
    def children(self) -> dict[EntryName, VlobID]: ...
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
        expected_id: VlobID | None = None,
        expected_version: int | None = None,
    ) -> FolderManifest:
        """Raise `ValueError` if invalid"""
        ...

class FileManifest:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        parent: VlobID,
        version: int,
        created: DateTime,
        updated: DateTime,
        size: int,
        blocksize: int,
        blocks: tuple[BlockAccess, ...],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> VlobID: ...
    @property
    def parent(self) -> VlobID: ...
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
    def blocks(self) -> tuple[BlockAccess, ...]: ...
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
        expected_id: VlobID | None = None,
        expected_version: int | None = None,
    ) -> FileManifest:
        """Raise `ValueError` if invalid"""
        ...

class UserManifest:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        version: int,
        created: DateTime,
        updated: DateTime,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> VlobID: ...
    @property
    def version(self) -> int: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def created(self) -> DateTime: ...
    @property
    def updated(self) -> DateTime: ...
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
        expected_id: VlobID | None = None,
        expected_version: int | None = None,
    ) -> UserManifest:
        """Raise `ValueError` if invalid"""
        ...

def child_manifest_decrypt_verify_and_load(
    encrypted: bytes,
    key: SecretKey,
    author_verify_key: VerifyKey,
    expected_author: DeviceID,
    expected_timestamp: DateTime,
    expected_id: VlobID | None = None,
    expected_version: int | None = None,
) -> ChildManifest:
    """Raise `ValueError` if invalid"""
    ...

def child_manifest_verify_and_load(
    signed: bytes,
    author_verify_key: VerifyKey,
    expected_author: DeviceID,
    expected_timestamp: DateTime,
    expected_id: VlobID | None = None,
    expected_version: int | None = None,
) -> ChildManifest:
    """Raise `ValueError` if invalid"""
    ...
