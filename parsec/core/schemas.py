from marshmallow import validate

from parsec import schema_fields as fields
from parsec.schema import UnknownCheckedSchema, OneOfSchema


class SharingSchema(UnknownCheckedSchema):
    owner = fields.String(required=True)
    guests = fields.List(fields.String(), required=True)
    notify_sink = fields.String(required=True)


# Synchronized with backend data


class BlockAccessSchema(UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    # TODO: provide digest as hexa string
    digest = fields.String(required=True, validate=validate.Length(min=1, max=64))


class ManifestAccessSchema(UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    rts = fields.String(required=True, validate=validate.Length(min=1, max=32))
    wts = fields.String(required=True, validate=validate.Length(min=1, max=32))


class FileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("file_manifest", required=True)
    author = fields.String(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)
    sharing = fields.Nested(SharingSchema)


class FolderManifestSchema(UnknownCheckedSchema):
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


class UserManifestSchema(FolderManifestSchema):
    type = fields.CheckedConstant("user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
    beacon_id = fields.String(required=True)


# Local data


class DirtyBlockAccessSchema(UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))


class LocalFileManifestSchema(UnknownCheckedSchema):
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


class LocalFolderManifestSchema(UnknownCheckedSchema):
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


class LocalUserManifestSchema(LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))
    beacon_id = fields.String(required=True)


class TypedManifestSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "local_user_manifest": LocalUserManifestSchema,
        "local_folder_manifest": LocalFolderManifestSchema,
        "local_file_manifest": LocalFileManifestSchema,
        "user_manifest": UserManifestSchema,
        "folder_manifest": FolderManifestSchema,
        "file_manifest": FileManifestSchema,
    }

    def get_obj_type(self, obj):
        return obj["type"]


typed_manifest_schema = TypedManifestSchema()


def dumps_manifest(manifest: dict):
    raw, errors = typed_manifest_schema.dumps(manifest)
    assert not errors
    return raw.encode("utf-8")


def loads_manifest(raw: bytes):
    manifest, errors = typed_manifest_schema.loads(raw.decode("utf-8"))
    assert not errors
    return manifest
