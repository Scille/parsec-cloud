import attr
import pendulum
from typing import List, Dict, Union

from parsec.types import DeviceID, UserID
from parsec.schema import UnknownCheckedSchema, OneOfSchema, fields, validate, post_load
from parsec.core.types.base import SchemaSerializationError, EntryName, EntryNameField
from parsec.core.types.access import (
    BlockAccess,
    ManifestAccess,
    BlockAccessSchema,
    ManifestAccessSchema,
)


__all__ = (
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
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
    blocks: List[BlockAccess]


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
        return FileManifest(**data)


file_manifest_schema = FileManifestSchema(strict=True)


# Folder manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class FolderManifest:
    author: DeviceID
    version: int
    created: pendulum.Pendulum
    updated: pendulum.Pendulum
    children: Dict[EntryName, ManifestAccess]


class FolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("folder_manifest", required=True)
    author = fields.DeviceID(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        EntryNameField(validate=validate.Length(min=1, max=256)),
        fields.Nested(ManifestAccessSchema),
        required=True,
    )

    @post_load
    def make_obj(self, data):
        return FolderManifest(**data)


folder_manifest_schema = FolderManifestSchema(strict=True)


# Workspace manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class WorkspaceManifest(FolderManifest):
    creator: UserID
    participants: List[UserID]


class WorkspaceManifestSchema(FolderManifestSchema):
    type = fields.CheckedConstant("workspace_manifest", required=True)
    creator = fields.UserID(required=True)
    participants = fields.List(fields.UserID(), required=True)

    @post_load
    def make_obj(self, data):
        return WorkspaceManifest(**data)


workspace_manifest_schema = WorkspaceManifestSchema(strict=True)


# User manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserManifest(FolderManifest):
    last_processed_message: int


class UserManifestSchema(FolderManifestSchema):
    type = fields.CheckedConstant("user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))

    @post_load
    def make_obj(self, data):
        return UserManifest(**data)


user_manifest_schema = UserManifestSchema(strict=True)


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
        return obj["type"]


typed_remote_manifest_schema = TypedRemoteManifestSchema(strict=True)


RemoteManifest = Union[UserManifest, FolderManifest, WorkspaceManifest, UserManifest]


def remote_manifest_dumps(manifest: RemoteManifest) -> bytes:
    """
    Raises:
        SchemaSerializationError
    """
    try:
        data = typed_remote_manifest_schema.dumps(manifest).data

    except ValidationError as exc:
        raise SchemaSerializationError(exc.messages) from exc

    return data.encode("utf8")


def remote_manifest_loads(raw: bytes) -> RemoteManifest:
    """
    Raises:
        SchemaSerializationError
    """
    try:
        return typed_remote_manifest_schema.loads(raw.decode("utf8")).data

    except ValidationError as exc:
        raise SchemaSerializationError(exc.messages) from exc

    except ValueError as exc:
        raise SchemaSerializationError(str(exc))
