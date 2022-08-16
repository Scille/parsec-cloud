# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
import functools
from typing import Optional, Tuple, TypeVar, Type, Union, FrozenSet, Pattern, Dict, TYPE_CHECKING
from parsec._parsec import DateTime

from parsec.types import FrozenDict
from parsec.crypto import SecretKey, HashDigest
from parsec.serde import fields, OneOfSchema, validate, post_load
from parsec.api.protocol import DeviceID, RealmRole, BlockID
from parsec.api.data import (
    BaseSchema,
    BaseData,
    WorkspaceEntry,
    BlockAccess,
    UserManifest as RemoteUserManifest,
    BaseManifest as BaseRemoteManifest,
    WorkspaceManifest as RemoteWorkspaceManifest,
    FolderManifest as RemoteFolderManifest,
    FileManifest as RemoteFileManifest,
    EntryID,
    EntryName,
    EntryNameField,
    EntryIDField,
)
from parsec.api.data.manifest import (
    _PyBlockAccess,
    _PyFileManifest,
    _PyFolderManifest,
    _PyWorkspaceManifest,
    _PyUserManifest,
    _PyWorkspaceEntry,
)
from parsec.core.types.base import BaseLocalData
from enum import Enum
from parsec._parsec import ChunkID

__all__ = (
    "WorkspaceEntry",  # noqa: Republishing
    "BlockAccess",  # noqa: Republishing
    "BlockID",  # noqa: Republishing
    "WorkspaceRole",
)


DEFAULT_BLOCK_SIZE = 512 * 1024  # 512 KB


# Cheap rename
WorkspaceRole = RealmRole


ChunkIDField = fields.uuid_based_field_factory(ChunkID)


@functools.total_ordering
@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class Chunk(BaseData):
    """Represents a chunk of a data in file manifest.

    The raw data is identified by its `id` attribute and is aligned using the
    `raw_offset` attribute with respect to the file addressing. The raw data
    size is stored as `raw_size`.

    The `start` and `stop` attributes then describes the span of the actual data
    still with respect to the file addressing.

    This means the following rule applies:
        raw_offset <= start < stop <= raw_offset + raw_size

    Access is an optional block access that can be used to produce a remote manifest
    when the chunk corresponds to an actual block within the context of this manifest.
    """

    class SCHEMA_CLS(BaseSchema):
        id = ChunkIDField(required=True)
        start = fields.Integer(required=True, validate=validate.Range(min=0))
        stop = fields.Integer(required=True, validate=validate.Range(min=1))
        raw_offset = fields.Integer(required=True, validate=validate.Range(min=0))
        raw_size = fields.Integer(required=True, validate=validate.Range(min=1))
        access = fields.Nested(_PyBlockAccess.SCHEMA_CLS, required=True, allow_none=True)

        @post_load
        def make_obj(self, data):
            return Chunk(**data)

    id: ChunkID
    start: int
    stop: int
    raw_offset: int
    raw_size: int
    access: Optional[BlockAccess]

    # Integrity

    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        assert self.raw_offset <= self.start < self.stop <= self.raw_offset + self.raw_size

    # Ordering

    def __lt__(self, other: object) -> bool:
        if isinstance(other, int):
            return self.start.__lt__(other)
        raise TypeError

    def __eq__(self, other: object) -> bool:
        if isinstance(other, int):
            return self.start.__eq__(other)
        if isinstance(other, Chunk):
            return attr.astuple(self).__eq__(attr.astuple(other))
        raise TypeError

    def is_block(self):
        # Requires an access
        if self.access is None:
            return False
        # Pseudo block
        if not self.is_pseudo_block():
            return False
        # Offset inconsistent
        if self.raw_offset != self.access.offset:
            return False
        # Size inconsistent
        if self.raw_size != self.access.size:
            return False
        return True

    def is_pseudo_block(self):
        # Not left aligned
        if self.start != self.raw_offset:
            return False
        # Not right aligned
        if self.stop != self.raw_offset + self.raw_size:
            return False
        return True

    # Create

    @classmethod
    def new(cls, start: int, stop: int) -> "Chunk":
        assert start < stop
        return cls(
            id=ChunkID.new(),
            start=start,
            stop=stop,
            raw_offset=start,
            raw_size=stop - start,
            access=None,
        )

    @classmethod
    def from_block_access(cls, block_access: BlockAccess):
        return cls(
            id=ChunkID(block_access.id.uuid),
            raw_offset=block_access.offset,
            raw_size=block_access.size,
            start=block_access.offset,
            stop=block_access.offset + block_access.size,
            access=block_access,
        )

    # Evolve

    def evolve_as_block(self, data: bytes) -> "Chunk":
        # No-op
        if self.is_block():
            return self

        # Check alignement
        if self.raw_offset != self.start:
            raise TypeError("This chunk is not aligned")

        # Craft access
        access = BlockAccess(
            id=BlockID(self.id.uuid),
            key=SecretKey.generate(),
            offset=self.start,
            size=self.stop - self.start,
            digest=HashDigest.from_data(data),
        )

        # Evolve
        return self.evolve(access=access)

    # Export

    def get_block_access(self) -> BlockAccess:
        if not self.is_block():
            raise TypeError("This chunk does not correspond to a block")
        assert self.access is not None
        return self.access


_PyChunk = Chunk
if not TYPE_CHECKING:
    try:
        from libparsec.types import Chunk as _RsChunk
    except:
        pass
    else:
        Chunk = _RsChunk


# Manifests data classes


class LocalManifestType(Enum):
    LOCAL_FILE_MANIFEST = "local_file_manifest"
    LOCAL_FOLDER_MANIFEST = "local_folder_manifest"
    LOCAL_WORKSPACE_MANIFEST = "local_workspace_manifest"
    LOCAL_USER_MANIFEST = "local_user_manifest"


LocalManifestTypeVar = TypeVar("LocalManifestTypeVar", bound="BaseLocalManifest")
LocalFileManifestTypeVar = TypeVar("LocalFileManifestTypeVar", bound="LocalFileManifest")
LocalFolderManifestTypeVar = TypeVar("LocalFolderManifestTypeVar", bound="LocalFolderManifest")
LocalWorkspaceManifestTypeVar = TypeVar(
    "LocalWorkspaceManifestTypeVar", bound="LocalWorkspaceManifest"
)
LocalUserManifestTypeVar = TypeVar("LocalUserManifestTypeVar", bound="LocalUserManifest")
LocalManifestsTypeVar = Union[
    LocalManifestTypeVar,
    LocalFileManifestTypeVar,
    LocalFolderManifestTypeVar,
    LocalWorkspaceManifestTypeVar,
    LocalUserManifestTypeVar,
]


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BaseLocalManifest(BaseLocalData):
    class SCHEMA_CLS(OneOfSchema, BaseSchema):
        type_field = "type"
        base = fields.Nested(BaseRemoteManifest.SCHEMA_CLS, required=True)
        need_sync = fields.Boolean(required=True)
        updated = fields.DateTime(required=True)

        @property
        def type_schemas(self):
            return {
                LocalManifestType.LOCAL_FILE_MANIFEST: _PyLocalFileManifest.SCHEMA_CLS,
                LocalManifestType.LOCAL_FOLDER_MANIFEST: _PyLocalFolderManifest.SCHEMA_CLS,
                LocalManifestType.LOCAL_WORKSPACE_MANIFEST: _PyLocalWorkspaceManifest.SCHEMA_CLS,
                LocalManifestType.LOCAL_USER_MANIFEST: _PyLocalUserManifest.SCHEMA_CLS,
            }

        def get_obj_type(self, obj):
            return obj["type"]

    need_sync: bool
    updated: DateTime
    base: BaseRemoteManifest  # base must be overwritten in subclass

    # Properties

    @property
    def id(self):
        return self.base.id

    @property
    def created(self):
        return self.base.created

    @property
    def base_version(self):
        return self.base.version

    @property
    def is_placeholder(self):
        return self.base.version == 0

    # Evolve methods

    def evolve_and_mark_updated(
        self: LocalManifestTypeVar, timestamp: DateTime, **data
    ) -> LocalManifestTypeVar:
        if "need_sync" in data:
            raise TypeError("Unexpected keyword argument `need_sync`")
        data["need_sync"] = True
        return self.evolve(updated=timestamp, **data)

    # Remote methods

    @classmethod
    def from_remote(
        cls: Type[LocalManifestTypeVar], remote: BaseRemoteManifest, prevent_sync_pattern: Pattern
    ) -> LocalManifestsTypeVar:
        if isinstance(remote, RemoteFileManifest):
            return LocalFileManifest.from_remote(remote)
        elif isinstance(remote, RemoteFolderManifest):
            return LocalFolderManifest.from_remote(remote, prevent_sync_pattern)
        elif isinstance(remote, RemoteWorkspaceManifest):
            return LocalWorkspaceManifest.from_remote(remote, prevent_sync_pattern)
        elif isinstance(remote, RemoteUserManifest):
            return LocalUserManifest.from_remote(remote)
        raise ValueError("Wrong remote type")

    @classmethod
    def from_remote_with_local_context(
        cls: Type[LocalManifestTypeVar],
        remote: BaseRemoteManifest,
        prevent_sync_pattern: Pattern,
        local_manifest: LocalManifestTypeVar,
        timestamp: DateTime,
    ) -> LocalManifestsTypeVar:
        if isinstance(remote, RemoteFileManifest):
            return LocalFileManifest.from_remote(remote)
        elif isinstance(remote, RemoteFolderManifest):
            return LocalFolderManifest.from_remote_with_local_context(
                remote, prevent_sync_pattern, local_manifest, timestamp=timestamp
            )
        elif isinstance(remote, RemoteWorkspaceManifest):
            return LocalWorkspaceManifest.from_remote_with_local_context(
                remote, prevent_sync_pattern, local_manifest, timestamp=timestamp
            )
        elif isinstance(remote, RemoteUserManifest):
            return LocalUserManifest.from_remote(remote)
        raise ValueError("Wrong remote type")

    def to_remote(
        self, author: Optional[DeviceID], timestamp: DateTime = None
    ) -> BaseRemoteManifest:
        raise NotImplementedError

    def match_remote(self, remote_manifest: BaseRemoteManifest) -> bool:
        reference = self.to_remote(
            author=remote_manifest.author, timestamp=remote_manifest.timestamp
        )
        return reference.evolve(version=remote_manifest.version) == remote_manifest

    # Stat method

    def to_stats(self):
        # General stats
        stats = {
            "id": self.id,
            "created": self.created,
            "updated": self.updated,
            "base_version": self.base_version,
            "is_placeholder": self.is_placeholder,
            "need_sync": self.need_sync,
        }
        return stats

    # Debugging

    def asdict(self):
        dct = attr.asdict(self)
        dct.pop("base")
        for name in "base_version", "is_placeholder", "created":
            dct[name] = getattr(self, name)
        if hasattr(self.base, "parent"):
            dct["parent"] = self.parent
        return dct


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class LocalFileManifest(BaseLocalManifest):
    class SCHEMA_CLS(BaseSchema):
        type = fields.EnumCheckedConstant(LocalManifestType.LOCAL_FILE_MANIFEST, required=True)
        base = fields.Nested(_PyFileManifest.SCHEMA_CLS, required=True)
        need_sync = fields.Boolean(required=True)
        updated = fields.DateTime(required=True)
        size = fields.Integer(required=True, validate=validate.Range(min=0))
        blocksize = fields.Integer(required=True, validate=validate.Range(min=8))
        blocks = fields.FrozenList(
            fields.FrozenList(fields.Nested(_PyChunk.SCHEMA_CLS)), required=True
        )

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return LocalFileManifest(**data)

    base: RemoteFileManifest
    size: int
    blocksize: int
    blocks: Tuple[Tuple[Chunk, ...], ...]

    @classmethod
    def new_placeholder(
        cls,
        author: DeviceID,
        parent: EntryID,
        timestamp: DateTime,
        blocksize: int = DEFAULT_BLOCK_SIZE,
    ) -> "LocalFileManifest":
        id = EntryID.new()
        blocks = ()
        return cls(
            base=RemoteFileManifest(
                author=author,
                timestamp=timestamp,
                id=id,
                parent=parent,
                version=0,
                created=timestamp,
                updated=timestamp,
                blocksize=blocksize,
                size=0,
                blocks=blocks,
            ),
            need_sync=True,
            updated=timestamp,
            blocksize=blocksize,
            size=0,
            blocks=blocks,
        )

    def to_stats(self):
        stats = super().to_stats()
        stats["type"] = "file"
        stats["size"] = self.size
        return stats

    # Properties

    @property
    def parent(self):
        return self.base.parent

    # Helper methods

    def get_chunks(self, block: int) -> Tuple[Chunk]:
        try:
            return self.blocks[block]
        except IndexError:
            return ()

    def is_reshaped(self) -> bool:
        for chunks in self.blocks:
            if len(chunks) != 1:
                return False
            if not chunks[0].is_block():
                return False
        return True

    def assert_integrity(self) -> None:
        current = 0
        assert isinstance(self.blocks, tuple)
        for i, chunks in enumerate(self.blocks):
            assert i * self.blocksize == current
            assert isinstance(chunks, tuple)
            assert len(chunks) > 0
            for chunk in chunks:
                assert chunk.start == current
                assert chunk.start < chunk.stop
                assert chunk.raw_offset <= chunk.start
                assert chunk.stop <= chunk.raw_offset + chunk.raw_size
                current = chunk.stop
        assert current == self.size

    def evolve_single_block(self, block: int, new_chunk: Chunk) -> "LocalFileManifest":
        blocks = list(self.blocks)
        blocks[block] = (new_chunk,)
        return self.evolve(blocks=tuple(blocks))

    # Remote methods

    @classmethod
    def from_remote(
        cls: Type[LocalFileManifestTypeVar], remote: RemoteFileManifest
    ) -> LocalFileManifestTypeVar:
        return cls(
            base=remote,
            need_sync=False,
            updated=remote.updated,
            size=remote.size,
            blocksize=remote.blocksize,
            blocks=tuple(
                (Chunk.from_block_access(block_access),) for block_access in remote.blocks
            ),
        )

    def to_remote(self, author: DeviceID, timestamp: DateTime) -> RemoteFileManifest:
        # Checks
        self.assert_integrity()
        assert self.is_reshaped()

        # Blocks
        blocks = tuple(chunks[0].get_block_access() for chunks in self.blocks)

        return RemoteFileManifest(
            author=author,
            timestamp=timestamp,
            id=self.id,
            parent=self.parent,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            size=self.size,
            blocksize=self.blocksize,
            blocks=blocks,
        )

    def match_remote(self, remote_manifest: RemoteFileManifest) -> bool:
        if not self.is_reshaped():
            return False
        return super().match_remote(remote_manifest)


_PyLocalFileManifest = LocalFileManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import LocalFileManifest as _RsLocalFileManifest
    except:
        pass
    else:
        LocalFileManifest = _RsLocalFileManifest

LocalFolderishManifestTypeVar = TypeVar(
    "LocalFolderishManifestTypeVar", bound="LocalFolderishManifestMixin"
)


class LocalFolderishManifestMixin:
    """Common methods for LocalFolderManifest and LocalWorkspaceManifest"""

    # Evolve methods

    def evolve_children_and_mark_updated(
        self: LocalFolderishManifestTypeVar,
        data: Dict[EntryName, Optional[EntryID]],
        prevent_sync_pattern: Pattern,
        timestamp: DateTime,
    ) -> LocalFolderishManifestTypeVar:
        actually_updated = False
        new_children = dict(self.children)
        new_local_confinement_points = set(self.local_confinement_points)

        # Deal with removal first
        for name, entry_id in data.items():
            # Here `entry_id` can be either:
            # - a new entry id that might overwrite the previous one with the same name if it exists
            # - `None` which means the entry for the corresponding name should be removed
            if name not in new_children:
                # Make sure we don't remove a name that does not exist
                assert entry_id is not None
                continue
            # Remove old entry
            old_entry_id = new_children.pop(name)
            if old_entry_id in new_local_confinement_points:
                new_local_confinement_points.discard(old_entry_id)
            else:
                actually_updated = True

        # Make sure no entry_id is duplicated
        assert not set(data.values()).intersection(new_children.values())

        # Deal with additions second
        for name, entry_id in data.items():
            if entry_id is None:
                continue
            # Add new entry
            new_children[name] = entry_id
            if prevent_sync_pattern.match(name.str):
                new_local_confinement_points.add(entry_id)
            else:
                actually_updated = True

        new_local_confinement_points = frozenset(new_local_confinement_points)
        if not actually_updated:
            return self.evolve(
                children=new_children, local_confinement_points=new_local_confinement_points
            )
        return self.evolve_and_mark_updated(
            children=new_children,
            local_confinement_points=new_local_confinement_points,
            timestamp=timestamp,
        )

    # Filtering and confinement helpers

    def _filter_local_confinement_points(
        self: LocalFolderishManifestTypeVar,
    ) -> LocalFolderishManifestTypeVar:
        if not self.local_confinement_points:
            return self
        children = {
            name: entry_id
            for name, entry_id in self.children.items()
            if entry_id not in self.local_confinement_points
        }
        return self.evolve(local_confinement_points=frozenset(), children=children)

    def _restore_local_confinement_points(
        self: LocalFolderishManifestTypeVar,
        other: LocalFolderishManifestTypeVar,
        prevent_sync_pattern: Pattern,
        timestamp: DateTime,
    ) -> LocalFolderishManifestTypeVar:
        # Using self.remote_confinement_points is useful to restore entries that were present locally
        # before applying a new filter that filtered those entries from the remote manifest
        if not other.local_confinement_points and not self.remote_confinement_points:
            return self
        # Create a set for fast lookup in order to make sure no entry gets duplicated.
        # This might happen when a synchronized entry is renamed to a confined name locally.
        self_entry_ids = set(self.children.values())
        previously_local_confinement_points = {
            name: entry_id
            for name, entry_id in other.children.items()
            if entry_id not in self_entry_ids
            and (
                entry_id in other.local_confinement_points
                or entry_id in self.remote_confinement_points
            )
        }
        return self.evolve_children_and_mark_updated(
            previously_local_confinement_points, prevent_sync_pattern, timestamp
        )

    def _filter_remote_entries(
        self: LocalFolderishManifestTypeVar, prevent_sync_pattern: Pattern
    ) -> LocalFolderishManifestTypeVar:
        remote_confinement_points = frozenset(
            {
                entry_id
                for name, entry_id in self.children.items()
                if prevent_sync_pattern.match(name.str)
            }
        )
        if not remote_confinement_points:
            return self
        children = {
            name: entry_id
            for name, entry_id in self.children.items()
            if entry_id not in remote_confinement_points
        }
        return self.evolve(children=children, remote_confinement_points=remote_confinement_points)

    def _restore_remote_confinement_points(
        self: LocalFolderishManifestTypeVar,
    ) -> LocalFolderishManifestTypeVar:
        if not self.remote_confinement_points:
            return self
        children = dict(self.children)
        for name, entry_id in self.base.children.items():
            if entry_id in self.remote_confinement_points:
                children[name] = entry_id
        return self.evolve(remote_confinement_points=frozenset(), children=children)

    # Apply "prevent sync" pattern

    def apply_prevent_sync_pattern(
        self: LocalFolderishManifestTypeVar, prevent_sync_pattern: Pattern, timestamp: DateTime
    ) -> LocalFolderishManifestTypeVar:
        # Filter local confinement points
        result = self._filter_local_confinement_points()
        # Restore remote confinement points
        result = result._restore_remote_confinement_points()
        # Filter remote confinement_points
        result = result._filter_remote_entries(prevent_sync_pattern)
        # Restore local confinement points
        return result._restore_local_confinement_points(
            self, prevent_sync_pattern, timestamp=timestamp
        )


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class LocalFolderManifest(BaseLocalManifest, LocalFolderishManifestMixin):
    class SCHEMA_CLS(BaseSchema):
        type = fields.EnumCheckedConstant(LocalManifestType.LOCAL_FOLDER_MANIFEST, required=True)
        base = fields.Nested(_PyFolderManifest.SCHEMA_CLS, required=True)
        need_sync = fields.Boolean(required=True)
        updated = fields.DateTime(required=True)
        children = fields.FrozenMap(EntryNameField(), EntryIDField(), required=True)
        # Added in Parsec v1.15
        # Confined entries are entries that are meant to stay locally and not be added
        # to the uploaded remote manifest when synchronizing. The criteria for being
        # confined is to have a filename that matched the "prevent sync" pattern at the time of
        # the last change (or when a new filter was successfully applied)
        local_confinement_points = fields.FrozenSet(
            EntryIDField(), allow_none=False, required=False, missing=frozenset()
        )
        # Added in Parsec v1.15
        # Filtered entries are entries present in the base manifest that are not exposed
        # locally. We keep track of them to remember that those entries have not been
        # deleted locally and hence should be restored when crafting the remote manifest
        # to upload.
        remote_confinement_points = fields.FrozenSet(
            EntryIDField(), allow_none=False, required=False, missing=frozenset()
        )

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return LocalFolderManifest(**data)

    base: RemoteFolderManifest
    children: FrozenDict[EntryName, EntryID]
    local_confinement_points: FrozenSet[EntryID]
    remote_confinement_points: FrozenSet[EntryID]

    @classmethod
    def new_placeholder(
        cls, author: DeviceID, parent: EntryID, timestamp: DateTime
    ) -> "LocalFolderManifest":
        id = EntryID.new()
        children = FrozenDict()
        return cls(
            base=RemoteFolderManifest(
                author=author,
                timestamp=timestamp,
                id=id,
                parent=parent,
                version=0,
                created=timestamp,
                updated=timestamp,
                children=children,
            ),
            need_sync=True,
            updated=timestamp,
            children=children,
            local_confinement_points=frozenset(),
            remote_confinement_points=frozenset(),
        )

    # Properties

    def to_stats(self):
        stats = super().to_stats()
        stats["type"] = "folder"
        stats["children"] = sorted(self.children.keys())
        return stats

    @property
    def parent(self):
        return self.base.parent

    # Remote methods

    @classmethod
    def from_remote(
        cls, remote: RemoteFolderManifest, prevent_sync_pattern: Pattern
    ) -> "LocalFolderManifest":
        # Create local manifest
        result = cls(
            base=remote,
            need_sync=False,
            updated=remote.updated,
            children=remote.children,
            local_confinement_points=frozenset(),
            remote_confinement_points=frozenset(),
        )
        # Filter remote entries
        return result._filter_remote_entries(prevent_sync_pattern)

    @classmethod
    def from_remote_with_local_context(
        cls,
        remote: RemoteFolderManifest,
        prevent_sync_pattern: Pattern,
        local_manifest: "LocalFolderManifest",
        timestamp: DateTime,
    ) -> "LocalFolderManifest":
        result = cls.from_remote(remote, prevent_sync_pattern)
        return result._restore_local_confinement_points(
            local_manifest, prevent_sync_pattern, timestamp=timestamp
        )

    def to_remote(self, author: DeviceID, timestamp: DateTime) -> RemoteFolderManifest:
        # Filter confined entries
        processed_manifest = self._filter_local_confinement_points()
        # Restore filtered entries
        processed_manifest = processed_manifest._restore_remote_confinement_points()
        # Create remote manifest
        return RemoteFolderManifest(
            author=author,
            timestamp=timestamp,
            id=self.id,
            parent=self.parent,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            children=processed_manifest.children,
        )


_PyLocalFolderManifest = LocalFolderManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import LocalFolderManifest as _RsLocalFolderManifest
    except:
        pass
    else:
        LocalFolderManifest = _RsLocalFolderManifest


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class LocalWorkspaceManifest(BaseLocalManifest, LocalFolderishManifestMixin):
    class SCHEMA_CLS(BaseSchema):
        type = fields.EnumCheckedConstant(LocalManifestType.LOCAL_WORKSPACE_MANIFEST, required=True)
        base = fields.Nested(_PyWorkspaceManifest.SCHEMA_CLS, required=True)
        need_sync = fields.Boolean(required=True)
        updated = fields.DateTime(required=True)
        children = fields.FrozenMap(EntryNameField(), EntryIDField(), required=True)
        # Added in Parsec v1.15
        # Confined entries are entries that are meant to stay locally and not be added
        # to the uploaded remote manifest when synchronizing. The criteria for being
        # confined is to have a filename that matched the "prevent sync" pattern at the time of
        # the last change (or when a new filter was successfully applied)
        local_confinement_points = fields.FrozenSet(
            EntryIDField(), allow_none=False, required=False, missing=frozenset()
        )
        # Added in Parsec v1.15
        # Filtered entries are entries present in the base manifest that are not exposed
        # locally. We keep track of them to remember that those entries have not been
        # deleted locally and hence should be restored when crafting the remote manifest
        # to upload.
        remote_confinement_points = fields.FrozenSet(
            EntryIDField(), allow_none=False, required=False, missing=frozenset()
        )
        # Added in Parsec v1.15
        # Speculative placeholders are created when we want to access a workspace
        # but didn't retrieve manifest data from backend yet. This implies:
        # - non-placeholders cannot be speculative
        # - the only non-speculative placeholder is the placeholder initialized
        #   during the initial workspace creation
        # This speculative information is useful during merge to understand if
        # a data is not present in the placeholder compared with a remote because:
        # a) the data is not locally known (speculative is True)
        # b) the data is known, but has been locally removed (speculative is False)
        # Prevented to be `required=True` by backward compatibility
        speculative = fields.Boolean(allow_none=False, required=False, missing=False)

        @post_load
        def make_obj(self, data):
            # TODO: Ensure non-placeholder cannot be marked speculative
            assert data["speculative"] is False or data["base"].version == 0
            # TODO: Should this assert be in remote workspace manifest definition instead ?
            # TODO: but in theory remote workspace manifest should assert version > 0 !
            assert data["base"].version != 0 or not data["base"].children
            data.pop("type")
            return LocalWorkspaceManifest(**data)

    base: RemoteWorkspaceManifest
    children: FrozenDict[EntryName, EntryID]
    local_confinement_points: FrozenSet[EntryID]
    remote_confinement_points: FrozenSet[EntryID]

    speculative: bool

    @classmethod
    def new_placeholder(
        cls, author: DeviceID, timestamp: DateTime, id: EntryID = None, speculative: bool = False
    ) -> "LocalWorkspaceManifest":
        children = FrozenDict()
        return cls(
            base=RemoteWorkspaceManifest(
                author=author,
                timestamp=timestamp,
                id=id or EntryID.new(),
                version=0,
                created=timestamp,
                updated=timestamp,
                children=children,
            ),
            need_sync=True,
            updated=timestamp,
            children=children,
            local_confinement_points=frozenset(),
            remote_confinement_points=frozenset(),
            speculative=speculative,
        )

    # Evolve methods

    def to_stats(self):
        stats = super().to_stats()
        stats["type"] = "folder"
        stats["children"] = sorted(self.children.keys())
        return stats

    # Remote methods

    @classmethod
    def from_remote(
        cls, remote: RemoteFolderManifest, prevent_sync_pattern: Pattern
    ) -> "LocalWorkspaceManifest":
        # Create local manifest
        result = cls(
            base=remote,
            need_sync=False,
            updated=remote.updated,
            children=remote.children,
            local_confinement_points=frozenset(),
            remote_confinement_points=frozenset(),
            speculative=False,
        )
        # Filter remote entries
        return result._filter_remote_entries(prevent_sync_pattern)

    @classmethod
    def from_remote_with_local_context(
        cls,
        remote: RemoteFolderManifest,
        prevent_sync_pattern: Pattern,
        local_manifest: "LocalWorkspaceManifest",
        timestamp: DateTime,
    ) -> "LocalWorkspaceManifest":
        result = cls.from_remote(remote, prevent_sync_pattern)
        return result._restore_local_confinement_points(
            local_manifest, prevent_sync_pattern, timestamp=timestamp
        )

    def to_remote(self, author: DeviceID, timestamp: DateTime) -> RemoteWorkspaceManifest:
        # Filter confined entries
        processed_manifest = self._filter_local_confinement_points()
        # Restore filtered entries
        processed_manifest = processed_manifest._restore_remote_confinement_points()
        # Create remote manifest
        return RemoteWorkspaceManifest(
            author=author,
            timestamp=timestamp,
            id=self.id,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            children=processed_manifest.children,
        )


_PyLocalWorkspaceManifest = LocalWorkspaceManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import LocalWorkspaceManifest as _RsLocalWorkspaceManifest
    except:
        pass
    else:
        LocalWorkspaceManifest = _RsLocalWorkspaceManifest


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class LocalUserManifest(BaseLocalManifest):
    class SCHEMA_CLS(BaseSchema):
        type = fields.EnumCheckedConstant(LocalManifestType.LOCAL_USER_MANIFEST, required=True)
        base = fields.Nested(_PyUserManifest.SCHEMA_CLS, required=True)
        need_sync = fields.Boolean(required=True)
        updated = fields.DateTime(required=True)
        last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
        workspaces = fields.FrozenList(fields.Nested(_PyWorkspaceEntry.SCHEMA_CLS), required=True)
        # Added in Parsec v1.15
        # Speculative placeholders are created when we want to access the
        # user manifest but didn't retrieve it from backend yet. This implies:
        # - non-placeholders cannot be speculative
        # - the only non-speculative placeholder is the placeholder initialized
        #   during the initial user claim (by opposition of subsequent device
        #   claims on the same user)
        # This speculative information is useful during merge to understand if
        # a data is not present in the placeholder compared with a remote because:
        # a) the data is not locally known (speculative is True)
        # b) the data is known, but has been locally removed (speculative is False)
        # Prevented to be `required=True` by backward compatibility
        speculative = fields.Boolean(allow_none=False, required=False, missing=False)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            # TODO: Ensure non-placeholder cannot be marked speculative
            assert data["speculative"] is False or data["base"].version == 0
            # TODO: Should this assert be in remote workspace manifest definition instead ?
            # TODO: but in theory remote workspace manifest should assert version > 0 !
            assert data["base"].version != 0 or not data["base"].workspaces
            return LocalUserManifest(**data)

    base: RemoteUserManifest

    last_processed_message: int
    workspaces: Tuple[WorkspaceEntry, ...]

    speculative: bool

    @classmethod
    def new_placeholder(
        cls, author: DeviceID, timestamp: DateTime, id: EntryID = None, speculative: bool = False
    ) -> "LocalUserManifest":
        return cls(
            base=RemoteUserManifest(
                author=author,
                timestamp=timestamp,
                id=id or EntryID.new(),
                version=0,
                created=timestamp,
                updated=timestamp,
                last_processed_message=0,
                workspaces=(),
            ),
            need_sync=True,
            updated=timestamp,
            last_processed_message=0,
            workspaces=(),
            speculative=speculative,
        )

    # Helper

    def get_workspace_entry(self, workspace_id: EntryID) -> WorkspaceEntry:
        return next((w for w in self.workspaces if w.id == workspace_id), None)

    # Evolve methods

    def evolve_workspaces_and_mark_updated(self, timestamp: DateTime, *data) -> "LocalUserManifest":
        workspaces = {**{w.id: w for w in self.workspaces}, **{w.id: w for w in data}}
        return self.evolve_and_mark_updated(
            workspaces=tuple(workspaces.values()), timestamp=timestamp
        )

    def evolve_workspaces(self, *data) -> "LocalUserManifest":
        workspaces = {**{w.id: w for w in self.workspaces}, **{w.id: w for w in data}}
        return self.evolve(workspaces=tuple(workspaces.values()))

    # Remote methods

    @classmethod
    def from_remote(cls, remote: RemoteUserManifest) -> "LocalUserManifest":
        return cls(
            base=remote,
            need_sync=False,
            updated=remote.updated,
            last_processed_message=remote.last_processed_message,
            workspaces=remote.workspaces,
            speculative=False,
        )

    def to_remote(self, author: DeviceID, timestamp: DateTime) -> RemoteUserManifest:
        return RemoteUserManifest(
            author=author,
            timestamp=timestamp,
            id=self.id,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            last_processed_message=self.last_processed_message,
            workspaces=self.workspaces,
        )


_PyLocalUserManifest = LocalUserManifest
if not TYPE_CHECKING:
    try:
        from libparsec.types import LocalUserManifest as _RsLocalUserManifest
    except:
        pass
    else:
        LocalUserManifest = _RsLocalUserManifest
