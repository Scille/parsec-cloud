from typing import List

from parsec.types import DeviceID, UserID
from parsec.schema import UnknownCheckedSchema, fields, validate, post_load
from parsec.core.types.base import SchemaSerializationError, EntryName, EntryNameField
from parsec.core.types.access import (
    BlockAccess,
    ManifestAccess,
    BlockAccessSchema,
    ManifestAccessSchema,
    DirtyBlockAccess,
    DirtyBlockAccessSchema,
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
    base_version: int
    need_sync: bool
    is_placeholder: bool
    created: pendulum.Pendulum
    updated: pendulum.Pendulum
    size: int
    blocks: List[BlockAccess]
    dirty_blocks: List[DirtyBlockAccess]


class LocalFileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_file_manifest", required=True)
    author = fields.DeviceID(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=1))
    need_sync = fields.Boolean(required=True)
    is_placeholder = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)
    dirty_blocks = fields.List(fields.Nested(DirtyBlockAccessSchema), required=True)

    @post_load
    def make_obj(self, data):
        return FileManifest(**data)


local_file_manifest_schema = LocalFileManifestSchema(strict=True)


# Folder manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalFolderManifest:
    author: DeviceID
    base_version: int
    need_sync: bool
    is_placeholder: bool
    created: pendulum.Pendulum
    updated: pendulum.Pendulum
    children: Dict[EntryName, ManifestAccess]


class LocalFolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_folder_manifest", required=True)
    author = fields.DeviceID(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=1))
    base_version = fields.Integer(required=True, validate=validate.Range(min=1))
    need_sync = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.EntryNameField(validate=validate.Length(min=1, max=256)),
        fields.Nested(ManifestAccessSchema),
        required=True,
    )

    @post_load
    def make_obj(self, data):
        return FolderManifest(**data)


local_folder_manifest_schema = LocalFolderManifestSchema(strict=True)


# Workspace manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalWorkspaceManifest(LocalFolderManifest):
    creator: UserID
    participants: List[UserID]


class LocalWorkspaceManifestSchema(LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_workspace_manifest", required=True)
    creator = fields.UserID(required=True)
    participants = fields.List(fields.UserID(), required=True)

    @post_load
    def make_obj(self, data):
        return WorkspaceManifest(**data)


local_workspace_manifest_schema = LocalWorkspaceManifestSchema(strict=True)


# User manifest


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalUserManifest(LocalFolderManifest):
    last_processed_message: int


class LocalUserManifestSchema(LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))

    @post_load
    def make_obj(self, data):
        return UserManifest(**data)


local_user_manifest_schema = LocalUserManifestSchema(strict=True)


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
        return obj["type"]


typed_local_manifest_schema = TypedLocalManifestSchema(strict=True)


LocalManifest = Union[
    LocalUserManifest, LocalFolderManifest, LocalWorkspaceManifest, LocalUserManifest
]


def local_manifest_dumps(manifest: LocalManifest) -> bytes:
    """
    Raises:
        SchemaSerializationError
    """
    try:
        data = typed_local_manifest_schema.dumps(manifest).data

    except ValidationError as exc:
        raise SchemaSerializationError(exc.messages) from exc

    return data.encode("utf8")


def local_manifest_loads(raw: bytes) -> LocalManifest:
    """
    Raises:
        SchemaSerializationError
    """
    try:
        return typed_local_manifest_schema.loads(raw.decode("utf8")).data

    except ValidationError as exc:
        raise SchemaSerializationError(exc.messages) from exc

    except ValueError as exc:
        raise SchemaSerializationError(str(exc))
