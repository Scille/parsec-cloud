from marshmallow import validate

from parsec import schema_fields as fields
from parsec.schema import _UnknownCheckedSchema, OneOfSchema


# Synchronized with backend data


class _BlockAccessSchema(_UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    # TODO: provide digest as hexa string
    digest = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))


class _SyncedAccessSchema(_UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    rts = fields.String(required=True, validate=validate.Length(min=1, max=32))
    wts = fields.String(required=True, validate=validate.Length(min=1, max=32))


class _FileManifestSchema(_UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("file_manifest", required=True)
    user_id = fields.String(required=True)
    device_name = fields.String(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(_BlockAccessSchema), required=True)


class _FolderManifestSchema(_UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("folder_manifest", required=True)
    user_id = fields.String(required=True)
    device_name = fields.String(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.String(validate=validate.Length(min=1, max=256)),
        fields.Nested(_SyncedAccessSchema),
        required=True,
    )


class _UserManifestSchema(_FolderManifestSchema):
    type = fields.CheckedConstant("user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))


# Local data


class _DirtyBlockAccessSchema(_UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))


class _LocalVlobAccessSchema(_SyncedAccessSchema):
    type = fields.CheckedConstant("vlob")


class _LocalPlaceHolderAccessSchema(_UnknownCheckedSchema):
    type = fields.CheckedConstant("placeholder")
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))


LocalVlobAccessSchema = _LocalVlobAccessSchema()
LocalPlaceHolderAccessSchema = _LocalPlaceHolderAccessSchema()


class _LocalAccessSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {"placeholder": LocalPlaceHolderAccessSchema, "vlob": LocalVlobAccessSchema}

    def get_obj_type(self, obj):
        return obj["type"]


class _LocalFileManifestSchema(_UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_file_manifest", required=True)
    user_id = fields.String(required=True)
    device_name = fields.String(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(_BlockAccessSchema), required=True)
    dirty_blocks = fields.List(fields.Nested(_DirtyBlockAccessSchema), required=True)


class _LocalFolderManifestSchema(_UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_folder_manifest", required=True)
    user_id = fields.String(required=True)
    device_name = fields.String(required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.String(validate=validate.Length(min=1, max=256)),
        fields.Nested(_LocalAccessSchema),
        required=True,
    )


class _LocalUserManifestSchema(_LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_user_manifest", required=True)
    last_processed_message = fields.Integer(required=True, validate=validate.Range(min=0))


FileManifestSchema = _FileManifestSchema()
FolderManifestSchema = _FolderManifestSchema()
UserManifestSchema = _UserManifestSchema()
LocalFileManifestSchema = _LocalFileManifestSchema()
LocalFolderManifestSchema = _LocalFolderManifestSchema()
LocalUserManifestSchema = _LocalUserManifestSchema()


class _TypedManifestSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "file_manifest": FileManifestSchema,
        "folder_manifest": FolderManifestSchema,
        "user_manifest": UserManifestSchema,
        "local_file_manifest": LocalFileManifestSchema,
        "local_folder_manifest": LocalFolderManifestSchema,
        "local_user_manifest": LocalUserManifestSchema,
    }

    def get_obj_type(self, obj):
        return obj["type"]


SyncedAccessSchema = _SyncedAccessSchema()
TypedManifestSchema = _TypedManifestSchema(strict=True)
BlockAccessSchema = _BlockAccessSchema()
DirtyBlockAccessSchema = _DirtyBlockAccessSchema()
LocalAccessSchema = _LocalAccessSchema()
