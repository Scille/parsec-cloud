# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
from typing import Optional, Tuple, TypeVar, Type, Union, Pattern, FrozenSet

from parsec.types import FrozenDict
from parsec.serde import fields, OneOfSchema, validate, post_load
from parsec.api.protocol import RealmRole, BlockID
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
    AnyManifest as RemoteAnyManifest,
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

from parsec._parsec import (
    ChunkID,
    Chunk,
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest,
    LocalWorkspaceManifest,
    DateTime,
)

__all__ = (
    "WorkspaceEntry",  # noqa: Republishing
    "BlockAccess",  # noqa: Republishing
    "BlockID",  # noqa: Republishing
    "WorkspaceRole",
    "Chunk",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalUserManifest",
    "LocalWorkspaceManifest",
    "LocalManifestTypeVar",
)


DEFAULT_BLOCK_SIZE = 512 * 1024  # 512 KB


# Cheap rename
WorkspaceRole = RealmRole


ChunkIDField: Type[fields.Field] = fields.uuid_based_field_factory(ChunkID)


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class _PyChunk(BaseData):
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


class LocalManifestType(Enum):
    LOCAL_FILE_MANIFEST = "local_file_manifest"
    LOCAL_FOLDER_MANIFEST = "local_folder_manifest"
    LOCAL_WORKSPACE_MANIFEST = "local_workspace_manifest"
    LOCAL_USER_MANIFEST = "local_user_manifest"


LocalFileManifestTypeVar = TypeVar("LocalFileManifestTypeVar", bound="LocalFileManifest")
LocalFolderManifestTypeVar = TypeVar("LocalFolderManifestTypeVar", bound="LocalFolderManifest")
LocalWorkspaceManifestTypeVar = TypeVar(
    "LocalWorkspaceManifestTypeVar", bound="LocalWorkspaceManifest"
)
LocalUserManifestTypeVar = TypeVar("LocalUserManifestTypeVar", bound="LocalUserManifest")
LocalManifestTypeVar = Union[
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
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

    @staticmethod
    def from_remote(
        remote: RemoteAnyManifest,
        prevent_sync_pattern: Pattern,
    ) -> LocalManifestTypeVar:
        if isinstance(remote, RemoteFileManifest):
            return LocalFileManifest.from_remote(remote)
        elif isinstance(remote, RemoteFolderManifest):
            return LocalFolderManifest.from_remote(remote, prevent_sync_pattern)
        elif isinstance(remote, RemoteWorkspaceManifest):
            return LocalWorkspaceManifest.from_remote(remote, prevent_sync_pattern)
        elif isinstance(remote, RemoteUserManifest):
            return LocalUserManifest.from_remote(remote)
        raise ValueError("Wrong remote type")

    @staticmethod
    def from_remote_with_local_context(
        remote: RemoteAnyManifest,
        prevent_sync_pattern: Pattern,
        local_manifest: LocalManifestTypeVar,
        timestamp: DateTime,
    ) -> LocalManifestTypeVar:
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


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class _PyLocalFileManifest(BaseLocalManifest):
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


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class _PyLocalFolderManifest(BaseLocalManifest):
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


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class _PyLocalWorkspaceManifest(BaseLocalManifest):
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


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class _PyLocalUserManifest(BaseLocalManifest):
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
