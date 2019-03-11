# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Tuple, Dict, List, Union

from parsec.types import DeviceID, UserID, FrozenDict
from parsec.serde import UnknownCheckedSchema, OneOfSchema, fields, validate, post_load
from parsec.core.types import remote_manifests
from parsec.core.types.base import EntryName, EntryNameField, serializer_factory
from parsec.core.types.access import (
    BlockAccess,
    ManifestAccess,
    BlockAccessSchema,
    WorkspaceEntry,
    ManifestAccessSchema,
    DirtyBlockAccess,
    DirtyBlockAccessSchema,
    WorkspaceEntrySchema,
)


__all__ = (
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "LocalUserManifest",
    "local_manifest_dumps",
    "local_manifest_loads",
)


# File manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalFileManifest:
    author: DeviceID
    base_version: int = 0
    need_sync: bool = True
    is_placeholder: bool = True
    created: pendulum.Pendulum = None
    updated: pendulum.Pendulum = None
    size: int = 0
    blocks: Tuple[BlockAccess] = attr.ib(converter=tuple, default=())
    dirty_blocks: Tuple[DirtyBlockAccess] = attr.ib(converter=tuple, default=())

    def __attrs_post_init__(self):
        if not self.created:
            object.__setattr__(self, "created", pendulum.now())
        if not self.updated:
            object.__setattr__(self, "updated", self.created)

    def evolve_and_mark_updated(self, **data) -> "LocalFileManifest":
        if "updated" not in data:
            data["updated"] = pendulum.now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalFileManifest":
        return attr.evolve(self, **data)

    def to_remote(self, **data) -> "remote_manifests.FileManifest":
        return remote_manifests.FileManifest(
            author=self.author,
            version=self.base_version,
            created=self.created,
            updated=self.updated,
            size=self.size,
            blocks=self.blocks,
            **data,
        )


class LocalFileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_file_manifest", required=True)
    author = fields.DeviceID(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    is_placeholder = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)
    dirty_blocks = fields.List(fields.Nested(DirtyBlockAccessSchema), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return LocalFileManifest(**data)


local_file_manifest_serializer = serializer_factory(LocalFileManifestSchema)


# Folder manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalFolderManifest:
    author: DeviceID
    base_version: int = 0
    need_sync: bool = True
    is_placeholder: bool = True
    created: pendulum.Pendulum = None
    updated: pendulum.Pendulum = None
    children: Dict[EntryName, ManifestAccess] = attr.ib(converter=FrozenDict, factory=FrozenDict)

    def __attrs_post_init__(self):
        if not self.created:
            object.__setattr__(self, "created", pendulum.now())
        if not self.updated:
            object.__setattr__(self, "updated", self.created)

    def evolve_and_mark_updated(self, **data) -> "LocalFileManifest":
        if "updated" not in data:
            data["updated"] = pendulum.now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalFileManifest":
        return attr.evolve(self, **data)

    def evolve_children_and_mark_updated(self, data) -> "LocalFolderManifest":
        return self.evolve_and_mark_updated(
            children={k: v for k, v in {**self.children, **data}.items() if v is not None}
        )

    def evolve_children(self, data) -> "LocalFolderManifest":
        return self.evolve(
            children={k: v for k, v in {**self.children, **data}.items() if v is not None}
        )

    def to_remote(self, **data) -> "remote_manifests.FolderManifest":
        return remote_manifests.FolderManifest(
            author=self.author,
            version=self.base_version,
            created=self.created,
            updated=self.updated,
            children=self.children,
            **data,
        )


class LocalFolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_folder_manifest", required=True)
    author = fields.DeviceID(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    is_placeholder = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        EntryNameField(validate=validate.Length(min=1, max=256)),
        fields.Nested(ManifestAccessSchema),
        required=True,
    )

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return LocalFolderManifest(**data)


local_folder_manifest_serializer = serializer_factory(LocalFolderManifestSchema)


# Workspace manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalWorkspaceManifest(LocalFolderManifest):
    creator: UserID = None
    participants: Tuple[UserID] = attr.ib(converter=tuple, default=())

    def __attrs_post_init__(self):
        super().__attrs_post_init__()
        if not self.creator:
            object.__setattr__(self, "creator", self.author.user_id)
        if not self.participants:
            object.__setattr__(self, "participants", (self.creator,))

    def to_remote(self, **data) -> "remote_manifests.WorkspaceManifest":
        return remote_manifests.WorkspaceManifest(
            author=self.author,
            version=self.base_version,
            created=self.created,
            updated=self.updated,
            children=self.children,
            creator=self.creator,
            participants=self.participants,
            **data,
        )


class LocalWorkspaceManifestSchema(LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_workspace_manifest", required=True)
    creator = fields.UserID(required=True)
    participants = fields.List(fields.UserID(), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return LocalWorkspaceManifest(**data)


local_workspace_manifest_serializer = serializer_factory(LocalWorkspaceManifestSchema)


# User manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalUserManifest:
    author: DeviceID
    base_version: int = 0
    need_sync: bool = True
    is_placeholder: bool = True
    created: pendulum.Pendulum = None
    updated: pendulum.Pendulum = None
    last_processed_message: int = 0
    workspaces: List[WorkspaceEntry] = attr.ib(converter=tuple, default=())

    def __attrs_post_init__(self):
        if not self.created:
            object.__setattr__(self, "created", pendulum.now())
        if not self.updated:
            object.__setattr__(self, "updated", self.created)

    # TODO: remove me once per-workspace-fs rework has been done
    @property
    def children(self):
        return {w.name: w.access for w in self.workspaces}

    def to_remote(self, **data) -> "remote_manifests.WorkspaceManifest":
        return remote_manifests.UserManifest(
            author=self.author,
            version=self.base_version,
            created=self.created,
            updated=self.updated,
            last_processed_message=self.last_processed_message,
            workspaces=self.workspaces,
            **data,
        )

    def evolve_and_mark_updated(self, **data) -> "LocalUserManifest":
        if "updated" not in data:
            data["updated"] = pendulum.now()
        data.setdefault("need_sync", True)
        return attr.evolve(self, **data)

    def evolve(self, **data) -> "LocalUserManifest":
        return attr.evolve(self, **data)

    def evolve_workspaces_and_mark_updated(self, *data) -> "LocalUserManifest":
        workspaces = {**{w.access.id: w for w in self.workspaces}, **{w.access.id: w for w in data}}
        return self.evolve_and_mark_updated(workspaces=tuple(workspaces.values()))

    def evolve_workspaces(self, *data) -> "LocalUserManifest":
        workspaces = {**{w.access.id: w for w in self.workspaces}, **{w.access.id: w for w in data}}
        return self.evolve(workspaces=tuple(workspaces.values()))


class LocalUserManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_user_manifest", required=True)
    author = fields.DeviceID(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    is_placeholder = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
    workspaces = fields.List(fields.Nested(WorkspaceEntrySchema), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return LocalUserManifest(**data)


local_user_manifest_serializer = serializer_factory(LocalUserManifestSchema)


class TypedLocalManifestSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "local_workspace_manifest": LocalWorkspaceManifestSchema,
        "local_user_manifest": LocalUserManifestSchema,
        "local_folder_manifest": LocalFolderManifestSchema,
        "local_file_manifest": LocalFileManifestSchema,
    }

    def get_obj_type(self, obj):
        if isinstance(obj, LocalWorkspaceManifest):
            return "local_workspace_manifest"
        elif isinstance(obj, LocalUserManifest):
            return "local_user_manifest"
        elif isinstance(obj, LocalFolderManifest):
            return "local_folder_manifest"
        elif isinstance(obj, LocalFileManifest):
            return "local_file_manifest"
        else:
            raise RuntimeError(f"Unknown object {obj}")


local_manifest_serializer = serializer_factory(TypedLocalManifestSchema)


LocalManifest = Union[
    LocalFileManifest, LocalFolderManifest, LocalWorkspaceManifest, LocalUserManifest
]


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
