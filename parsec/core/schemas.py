from marshmallow import validate

from parsec import schema_fields as fields
from parsec.schema import UnknownCheckedSchema, OneOfSchema


class _SharingSchema(UnknownCheckedSchema):
    owner = fields.String(required=True)
    guests = fields.List(fields.String(), required=True)
    notify_sink = fields.String(required=True)


SharingSchema = _SharingSchema()


# Synchronized with backend data


class _BlockAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    # TODO: provide digest as hexa string
    digest = fields.String(required=True, validate=validate.Length(min=1, max=64))


BlockAccessSchema = _BlockAccessSchema()


class _ManifestAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    rts = fields.String(required=True, validate=validate.Length(min=1, max=32))
    wts = fields.String(required=True, validate=validate.Length(min=1, max=32))


ManifestAccessSchema = _ManifestAccessSchema()


class _FileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("file_manifest", required=True)
    author = fields.String(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)
    sharing = fields.Nested(SharingSchema)


FileManifestSchema = _FileManifestSchema()


class _FolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("folder_manifest", required=True)
    author = fields.String(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.String(validate=validate.Length(min=1, max=256)),
        fields.Nested(ManifestAccessSchema),
        required=True,
    )
    sharing = fields.Nested(SharingSchema)


FolderManifestSchema = _FolderManifestSchema()


class _WorkspaceManifestSchema(_FolderManifestSchema):
    type = fields.CheckedConstant("workspace_manifest", required=True)
    beacon_id = fields.UUID(required=True)


WorkspaceManifestSchema = _WorkspaceManifestSchema()


class _UserManifestSchema(_WorkspaceManifestSchema):
    type = fields.CheckedConstant("user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))


UserManifestSchema = _UserManifestSchema()


# Local data


class _DirtyBlockAccessSchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))


DirtyBlockAccessSchema = _DirtyBlockAccessSchema()


class _LocalFileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_file_manifest", required=True)
    author = fields.String(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    is_placeholder = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)
    dirty_blocks = fields.List(fields.Nested(DirtyBlockAccessSchema), required=True)
    sharing = fields.Nested(SharingSchema)


LocalFileManifestSchema = _LocalFileManifestSchema()


class _LocalFolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_folder_manifest", required=True)
    author = fields.String(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    is_placeholder = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.String(validate=validate.Length(min=1, max=256)),
        fields.Nested(ManifestAccessSchema),
        required=True,
    )
    sharing = fields.Nested(SharingSchema)


LocalFolderManifestSchema = _LocalFolderManifestSchema()


class _LocalWorkspaceManifest(_LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_workspace_manifest", required=True)
    beacon_id = fields.UUID(required=True)


LocalWorkspaceManifest = _LocalWorkspaceManifest()


class _LocalUserManifestSchema(_LocalWorkspaceManifest):
    type = fields.CheckedConstant("local_user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))


LocalUserManifestSchema = _LocalUserManifestSchema()


class TypedManifestSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "local_user_manifest": LocalUserManifestSchema,
        "local_workspace_manifest": LocalWorkspaceManifest,
        "local_folder_manifest": LocalFolderManifestSchema,
        "local_file_manifest": LocalFileManifestSchema,
        "workspace_manifest": WorkspaceManifestSchema,
        "user_manifest": UserManifestSchema,
        "folder_manifest": FolderManifestSchema,
        "file_manifest": FileManifestSchema,
    }

    def get_obj_type(self, obj):
        return obj["type"]


typed_manifest_schema = TypedManifestSchema()


def dumps_manifest(manifest: dict):
    raw, errors = typed_manifest_schema.dumps(manifest)
    assert not errors, errors
    return raw.encode("utf-8")


def loads_manifest(raw: bytes):
    manifest, errors = typed_manifest_schema.loads(raw.decode("utf-8"))
    assert not errors, errors
    return manifest
