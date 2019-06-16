# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Tuple, Dict, Union

from parsec.types import DeviceID, FrozenDict
from parsec.serde import UnknownCheckedSchema, OneOfSchema, fields, validate, post_load
from parsec.core.types import local_manifests
from parsec.core.types.base import (
    EntryID,
    EntryIDField,
    EntryName,
    EntryNameField,
    serializer_factory,
)
from parsec.core.types.access import (
    BlockAccess,
    BlockAccessSchema,
    WorkspaceEntry,
    WorkspaceEntrySchema,
)


__all__ = (
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
    "RemoteManifest",
    "remote_manifest_dumps",
    "remote_manifest_loads",
)


# File manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class FileManifest:
    author: DeviceID
    version: int
    created: pendulum.Pendulum
    updated: pendulum.Pendulum
    size: int
    blocks: Tuple[BlockAccess] = attr.ib(converter=tuple)

    def evolve(self, **data) -> "FileManifest":
        return attr.evolve(self, **data)

    def to_local(self, author: DeviceID) -> "local_manifests.LocalFileManifest":
        return local_manifests.LocalFileManifest(
            author=author,
            base_version=self.version,
            created=self.created,
            updated=self.updated,
            size=self.size,
            blocks=self.blocks,
            is_placeholder=False,
            need_sync=False,
        )


class FileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("file_manifest", required=True)
    author = fields.DeviceID(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return FileManifest(**data)


file_manifest_serializer = serializer_factory(FileManifestSchema)


# Folder manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class FolderManifest:
    author: DeviceID
    version: int
    created: pendulum.Pendulum
    updated: pendulum.Pendulum
    children: Dict[EntryName, EntryID] = attr.ib(converter=FrozenDict)

    def evolve(self, **data) -> "FolderManifest":
        return attr.evolve(self, **data)

    def to_local(self, author: DeviceID) -> "local_manifests.LocalFolderManifest":
        return local_manifests.LocalFolderManifest(
            author=author,
            base_version=self.version,
            created=self.created,
            updated=self.updated,
            children=self.children,
            is_placeholder=False,
            need_sync=False,
        )


class FolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("folder_manifest", required=True)
    author = fields.DeviceID(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        EntryNameField(validate=validate.Length(min=1, max=256)),
        EntryIDField(required=True),
        required=True,
    )

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return FolderManifest(**data)


folder_manifest_serializer = serializer_factory(FolderManifestSchema)


# Workspace manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class WorkspaceManifest(FolderManifest):
    def to_local(self, author: DeviceID) -> "local_manifests.LocalWorkspaceManifest":
        return local_manifests.LocalWorkspaceManifest(
            author=author,
            base_version=self.version,
            created=self.created,
            updated=self.updated,
            children=self.children,
            is_placeholder=False,
            need_sync=False,
        )


class WorkspaceManifestSchema(FolderManifestSchema):
    type = fields.CheckedConstant("workspace_manifest", required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return WorkspaceManifest(**data)


workspace_manifest_serializer = serializer_factory(WorkspaceManifestSchema)


# User manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserManifest:
    author: DeviceID
    version: int
    created: pendulum.Pendulum
    updated: pendulum.Pendulum
    last_processed_message: int
    workspaces: Tuple[WorkspaceEntry] = attr.ib(converter=tuple)

    def get_workspace_entry(self, workspace_id: EntryID) -> WorkspaceEntry:
        return next((w for w in self.workspaces if w.id == workspace_id), None)

    def evolve(self, **data) -> "UserManifest":
        return attr.evolve(self, **data)

    def to_local(self, author: DeviceID) -> "local_manifests.LocalUserManifest":
        return local_manifests.LocalUserManifest(
            author=author,
            base_version=self.version,
            created=self.created,
            updated=self.updated,
            workspaces=self.workspaces,
            last_processed_message=self.last_processed_message,
            is_placeholder=False,
            need_sync=False,
        )


class UserManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("user_manifest", required=True)
    author = fields.DeviceID(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
    workspaces = fields.List(fields.Nested(WorkspaceEntrySchema), required=True)

    @post_load
    def make_obj(self, data):
        data.pop("type")
        data.pop("format")
        return UserManifest(**data)


user_manifest_serializer = serializer_factory(UserManifestSchema)


class TypedRemoteManifestSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "workspace_manifest": WorkspaceManifestSchema,
        "user_manifest": UserManifestSchema,
        "folder_manifest": FolderManifestSchema,
        "file_manifest": FileManifestSchema,
    }

    def get_obj_type(self, obj):
        if isinstance(obj, WorkspaceManifest):
            return "workspace_manifest"
        elif isinstance(obj, UserManifest):
            return "user_manifest"
        elif isinstance(obj, FolderManifest):
            return "folder_manifest"
        elif isinstance(obj, FileManifest):
            return "file_manifest"
        else:
            raise RuntimeError(f"Unknown object {obj}")


remote_manifest_serializer = serializer_factory(TypedRemoteManifestSchema)


RemoteManifest = Union[FileManifest, FolderManifest, WorkspaceManifest, UserManifest]


def remote_manifest_dumps(manifest: RemoteManifest) -> bytes:
    """
    Raises:
        SerdeError
    """
    return remote_manifest_serializer.dumps(manifest)


def remote_manifest_loads(raw: bytes) -> RemoteManifest:
    """
    Raises:
        SerdeError
    """
    return remote_manifest_serializer.loads(raw)
