# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

# import attr
# from parsec.api.data.base import BaseAPIData
# from parsec.serde import fields, post_load
# from parsec.serde.schema import BaseSchema
#
# @attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
# class TpekDerServiceEncryptionKey(BaseAPIData):
#     class SCHEMA_CLS(BaseSchema):
#
#         type = fields.CheckedConstant("tpek_service_der_encryption_key", required=True)
#         verify_key = fields.VerifyKey(required=True)
#         encryption_key = fields.DerPublicKey(required=True)
#         timestamp = fields.
#
#         @post_load
#         def make_obj(self, data: Dict[str, Any]) -> "TpekDerServiceEncryptionKey":
#             data.pop("type")
#             return TpekDerServiceEncryptionKey(**data)
#
#     verify_key: VerifyKey
#     public_key: PublicKey
#     requested_device_label: DeviceLabel
