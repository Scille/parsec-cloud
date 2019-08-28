# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from pendulum import Pendulum, now as pendulum_now
from typing import Tuple

from parsec.types import FrozenDict
from parsec.serde import UnknownCheckedSchema, OneOfSchema, fields, validate, post_load
from parsec.api.protocol import DeviceID
from parsec.api.data import (
    FileManifest as RemoteFileManifest,
    FolderManifest as RemoteFolderManifest,
    WorkspaceManifest as RemoteWorkspaceManifest,
)
from parsec.core.types.manifest import LocalManifest, Chunk
from parsec.core.types.base import (
    EntryID,
    EntryIDField,
    EntryName,
    EntryNameField,
    serializer_factory,
)


__all__ = (
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "local_manifest_dumps",
    "local_manifest_loads",
)


DEFAULT_BLOCK_SIZE = 512 * 1024  # 512 KB


# File manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalFileManifest:
    base: RemoteFileManifest
    need_sync: bool
    updated: Pendulum
    size: int
    blocksize: int
    blocks: Tuple[Tuple[Chunk], ...]

    @classmethod
    def new_placeholder(cls, parent: EntryID, id: EntryID = None, now: Pendulum = None):
        now = now or pendulum_now()
        blocks = ()
        return cls(
            base=RemoteFileManifest(
                author=None,
                timestamp=now,
                id=id or EntryID(),
                parent=parent,
                version=0,
                created=now,
                updated=now,
                blocksize=DEFAULT_BLOCK_SIZE,
                size=0,
                blocks=blocks,
            ),
            need_sync=True,
            updated=now,
            blocksize=DEFAULT_BLOCK_SIZE,
            size=0,
            blocks=blocks,
        )

    # Properties

    @property
    def id(self):
        return self.base.id

    @property
    def parent(self):
        return self.base.parent

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

    def evolve_and_mark_updated(self, **data) -> "LocalFileManifest":
        if "updated" not in data:
            data["updated"] = pendulum_now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalFileManifest":
        return attr.evolve(self, **data)

    # File methods

    def get_chunks(self, block: int) -> Tuple[Chunk]:
        try:
            return self.blocks[block]
        except IndexError:
            return ()

    def is_reshaped(self) -> bool:
        for chunks in self.blocks:
            if len(chunks) != 1:
                return False
            if not chunks[0].is_block:
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

    # Export methods

    def corresponds_to(self, remote_manifest: RemoteFileManifest) -> bool:
        if not self.is_reshaped():
            return False
        return (
            self.to_remote(
                author=remote_manifest.author, timestamp=remote_manifest.timestamp
            ).evolve(version=remote_manifest.version)
            == remote_manifest
        )

    @classmethod
    def from_remote(cls, remote: RemoteFileManifest) -> "LocalFileManifest":
        return cls(
            base=remote,
            need_sync=False,
            updated=remote.updated,
            size=remote.size,
            blocksize=remote.blocksize,
            blocks=tuple((Chunk.from_block_acess(block_access),) for block_access in remote.blocks),
        )

    def to_remote(self, author: DeviceID, timestamp: Pendulum = None) -> RemoteFileManifest:
        # Checks
        self.assert_integrity()
        assert self.is_reshaped()

        # Blocks
        blocks = tuple(chunks[0].get_block_access() for chunks in self.blocks)

        return RemoteFileManifest(
            author=author,
            timestamp=timestamp or pendulum_now(),
            id=self.id,
            parent=self.parent,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            size=self.size,
            blocksize=self.blocksize,
            blocks=blocks,
        )

    def asdict(self):
        dct = attr.asdict(self)
        dct.pop("base")
        props = "base_version", "is_placeholder", "parent", "created"
        for name in props:
            dct[name] = getattr(self, name)
        return dct


class LocalFileManifestSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("local_file_manifest", required=True)
    base = fields.Nested(RemoteFileManifest.SCHEMA_CLS, required=True)
    need_sync = fields.Boolean(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocksize = fields.Integer(required=True, validate=validate.Range(min=8))
    blocks = fields.FrozenList(fields.FrozenList(fields.Nested(Chunk.SCHEMA_CLS)), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        return LocalFileManifest(**data)


local_file_manifest_serializer = serializer_factory(LocalFileManifestSchema)


# Folder manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalFolderManifest:
    base: RemoteFolderManifest
    need_sync: bool
    updated: Pendulum
    children: FrozenDict[EntryName, EntryID]

    @classmethod
    def new_placeholder(cls, parent: EntryID, id: EntryID = None, now: Pendulum = None):
        now = now or pendulum_now()
        children = FrozenDict()
        return cls(
            base=RemoteFolderManifest(
                author=None,
                timestamp=now,
                id=id or EntryID(),
                parent=parent,
                version=0,
                created=now,
                updated=now,
                children=children,
            ),
            need_sync=True,
            updated=now,
            children=children,
        )

    # Properties

    @property
    def id(self):
        return self.base.id

    @property
    def parent(self):
        return self.base.parent

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

    def evolve_and_mark_updated(self, **data) -> "LocalFolderManifest":
        if "updated" not in data:
            data["updated"] = pendulum_now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalFolderManifest":
        return attr.evolve(self, **data)

    def evolve_children_and_mark_updated(self, data) -> "LocalFolderManifest":
        return self.evolve_and_mark_updated(
            children={k: v for k, v in {**self.children, **data}.items() if v is not None}
        )

    def evolve_children(self, data) -> "LocalFolderManifest":
        return self.evolve(
            children={k: v for k, v in {**self.children, **data}.items() if v is not None}
        )

    # Export methods

    def corresponds_to(self, remote_manifest: RemoteFolderManifest) -> bool:
        return (
            self.to_remote(
                author=remote_manifest.author, timestamp=remote_manifest.timestamp
            ).evolve(version=remote_manifest.version)
            == remote_manifest
        )

    @classmethod
    def from_remote(cls, remote: RemoteFolderManifest) -> "LocalFolderManifest":
        return cls(base=remote, need_sync=False, updated=remote.updated, children=remote.children)

    def to_remote(self, author: DeviceID, timestamp: Pendulum = None) -> RemoteFolderManifest:
        return RemoteFolderManifest(
            author=author,
            timestamp=timestamp or pendulum_now(),
            id=self.id,
            parent=self.parent,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            children=self.children,
        )

    def asdict(self):
        dct = attr.asdict(self)
        dct.pop("base")
        props = "base_version", "is_placeholder", "parent", "created"
        for name in props:
            dct[name] = getattr(self, name)
        return dct


class LocalFolderManifestSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("local_folder_manifest", required=True)
    base = fields.Nested(RemoteFolderManifest.SCHEMA_CLS, required=True)
    need_sync = fields.Boolean(required=True)
    updated = fields.DateTime(required=True)
    children = fields.FrozenMap(EntryNameField(), EntryIDField(required=True), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        return LocalFolderManifest(**data)


local_folder_manifest_serializer = serializer_factory(LocalFolderManifestSchema)


# Workspace manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalWorkspaceManifest:
    base: RemoteWorkspaceManifest
    need_sync: bool
    updated: Pendulum
    children: FrozenDict[EntryName, EntryID]

    @classmethod
    def new_placeholder(cls, id: EntryID = None, now: Pendulum = None):
        now = now or pendulum_now()
        children = FrozenDict()
        return cls(
            base=RemoteWorkspaceManifest(
                author=None,
                timestamp=now,
                id=id or EntryID(),
                version=0,
                created=now,
                updated=now,
                children=children,
            ),
            need_sync=True,
            updated=now,
            children=children,
        )

    # Properties

    @property
    def id(self):
        return self.base.id

    @property
    def parent(self):
        return self.base.parent

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

    def evolve_and_mark_updated(self, **data) -> "LocalWorkspaceManifest":
        if "updated" not in data:
            data["updated"] = pendulum_now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalWorkspaceManifest":
        return attr.evolve(self, **data)

    def evolve_children_and_mark_updated(self, data) -> "LocalWorkspaceManifest":
        return self.evolve_and_mark_updated(
            children={k: v for k, v in {**self.children, **data}.items() if v is not None}
        )

    def evolve_children(self, data) -> "LocalWorkspaceManifest":
        return self.evolve(
            children={k: v for k, v in {**self.children, **data}.items() if v is not None}
        )

    # Export methods

    def corresponds_to(self, remote_manifest: RemoteWorkspaceManifest) -> bool:
        return (
            self.to_remote(
                author=remote_manifest.author, timestamp=remote_manifest.timestamp
            ).evolve(version=remote_manifest.version)
            == remote_manifest
        )

    @classmethod
    def from_remote(cls, remote: RemoteWorkspaceManifest) -> "LocalWorkspaceManifest":
        return cls(base=remote, need_sync=False, updated=remote.updated, children=remote.children)

    def to_remote(self, author: DeviceID, timestamp: Pendulum = None) -> RemoteWorkspaceManifest:
        return RemoteWorkspaceManifest(
            author=author,
            timestamp=timestamp or pendulum_now(),
            id=self.id,
            version=self.base_version + 1,
            created=self.created,
            updated=self.updated,
            children=self.children,
        )

    def asdict(self):
        dct = attr.asdict(self)
        dct.pop("base")
        props = "base_version", "is_placeholder", "created"
        for name in props:
            dct[name] = getattr(self, name)
        return dct


class LocalWorkspaceManifestSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("local_workspace_manifest", required=True)
    base = fields.Nested(RemoteWorkspaceManifest.SCHEMA_CLS, required=True)
    need_sync = fields.Boolean(required=True)
    updated = fields.DateTime(required=True)
    children = fields.FrozenMap(EntryNameField(), EntryIDField(required=True), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        return LocalWorkspaceManifest(**data)


local_workspace_manifest_serializer = serializer_factory(LocalWorkspaceManifestSchema)


class TypedLocalManifestSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "local_workspace_manifest": LocalWorkspaceManifestSchema,
        "local_folder_manifest": LocalFolderManifestSchema,
        "local_file_manifest": LocalFileManifestSchema,
    }

    def get_obj_type(self, obj):
        if isinstance(obj, LocalWorkspaceManifest):
            return "local_workspace_manifest"
        elif isinstance(obj, LocalFolderManifest):
            return "local_folder_manifest"
        elif isinstance(obj, LocalFileManifest):
            return "local_file_manifest"
        else:
            raise RuntimeError(f"Unknown object {obj}")


local_manifest_serializer = serializer_factory(TypedLocalManifestSchema)


def local_manifest_dumps(manifest: LocalManifest) -> bytes:
    """
    Raises:
        SerdeError
    """
    return local_manifest_serializer.dumps(manifest)


def local_manifest_loads(raw: bytes) -> LocalManifest:
    """
    Raises:
        SerdeError
    """
    return local_manifest_serializer.loads(raw)
