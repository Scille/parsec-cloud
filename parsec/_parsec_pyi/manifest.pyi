from typing import Optional, Tuple
from parsec._parsec import (
    EntryID,
    SecretKey,
    RealmRole,
    BlockID,
    HashDigest,
    DeviceID,
    SigningKey,
    VerifyKey,
)

from parsec.types import FrozenDict

from pendulum import DateTime

class EntryName:
    def __init__(self, name: str) -> None: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: EntryName | None) -> bool: ...
    def __ne__(self, other: EntryName | None) -> bool: ...
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
    def __repr__(self) -> str: ...
    def __eq__(self, other: WorkspaceEntry | None) -> bool: ...
    def __ne__(self, other: WorkspaceEntry | None) -> bool: ...
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
    def new(name: EntryName, timestamp: DateTime) -> WorkspaceEntry: ...
    def evolve(self, **kwargs): ...
    def is_revoked(self) -> bool: ...

class BlockAccess:
    def __init__(
        self, id: BlockID, key: SecretKey, offset: int, size: int, digest: HashDigest
    ) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: BlockAccess) -> bool: ...
    def __ne__(self, other: BlockAccess) -> bool: ...
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
    def __repr__(self) -> str: ...
    def __eq__(self, other: FolderManifest) -> bool: ...
    def __ne__(self, other: FolderManifest) -> bool: ...
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
    def evolve(self, **kwargs): ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    def decrypt_verify_and_load(
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
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
    def __repr__(self) -> str: ...
    def __eq__(self, other: FileManifest) -> bool: ...
    def __ne__(self, other: FileManifest) -> bool: ...
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
    def evolve(self, **kwargs): ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    def decrypt_verify_and_load(
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
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
    def __repr__(self) -> str: ...
    def __eq__(self, other: WorkspaceManifest) -> bool: ...
    def __ne__(self, other: WorkspaceManifest) -> bool: ...
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
    def evolve(self, **kwargs): ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    def decrypt_verify_and_load(
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
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
    def __repr__(self) -> str: ...
    def __eq__(self, other: UserManifest) -> bool: ...
    def __ne__(self, other: UserManifest) -> bool: ...
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
    def evolve(self, **kwargs): ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes: ...
    def decrypt_verify_and_load(
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_id: Optional[EntryID],
        expected_version: Optional[int],
        expected_author: DeviceID,
        expected_timestamp: DateTime,
    ) -> UserManifest: ...
    def get_workspace_entry(self, workspace_id: EntryID) -> Optional[WorkspaceEntry]: ...
