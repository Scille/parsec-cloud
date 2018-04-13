from marshmallow import validate

from parsec import schema_fields as fields
from parsec.schema import UnknownCheckedSchema, OneOfSchema


# Synchronized with backend data


class BlockAccessSchema(UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    offset = fields.Integer(required=True, validate=validate.Range(min=0))
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    digest = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))


class SyncedAccessSchema(UnknownCheckedSchema):
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))
    rts = fields.String(required=True, validate=validate.Length(min=1, max=32))
    wts = fields.String(required=True, validate=validate.Length(min=1, max=32))


class FileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("file_manifest", required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)


class FolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("folder_manifest", required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.String(validate=validate.Length(min=1, max=256)),
        fields.Nested(SyncedAccessSchema),
        required=True,
    )


class UserManifestSchema(FolderManifestSchema):
    type = fields.CheckedConstant("user_manifest", required=True)


# Local data


class TypedVlobAccessSchema(SyncedAccessSchema):
    type = fields.CheckedConstant("vlob")


class TypedPlaceHolderAccessSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("placeholder")
    id = fields.String(required=True, validate=validate.Length(min=1, max=32))
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=1, max=4096))


class TypedAccessSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "placeholder": TypedPlaceHolderAccessSchema, "vlob": TypedVlobAccessSchema
    }

    def get_obj_type(self, obj):
        return obj["type"]


class LocalFileManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_file_manifest", required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0))
    blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)
    dirty_blocks = fields.List(fields.Nested(BlockAccessSchema), required=True)


class LocalFolderManifestSchema(UnknownCheckedSchema):
    format = fields.CheckedConstant(1, required=True)
    type = fields.CheckedConstant("local_folder_manifest", required=True)
    base_version = fields.Integer(required=True, validate=validate.Range(min=0))
    need_sync = fields.Boolean(required=True)
    created = fields.DateTime(required=True)
    updated = fields.DateTime(required=True)
    children = fields.Map(
        fields.String(validate=validate.Length(min=1, max=256)),
        fields.Nested(TypedAccessSchema),
        required=True,
    )


class LocalUserManifestSchema(LocalFolderManifestSchema):
    type = fields.CheckedConstant("local_user_manifest", required=True)


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
