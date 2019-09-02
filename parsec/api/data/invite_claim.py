# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.crypto import VerifyKey, PublicKey, PrivateKey, SecretKey
from parsec.serde import fields, post_load
from parsec.api.protocol import DeviceID, DeviceIDField
from parsec.api.data.base import BaseAPIData, BaseSchema
from parsec.api.data.entry import EntryID, EntryIDField


class UserClaimContent(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("user_claim", required=True)
        token = fields.String(required=True)
        # Note claiming user also imply creating a first device
        device_id = DeviceIDField(required=True)
        public_key = fields.PublicKey(required=True)
        verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return UserClaimContent(**data)

    token: str
    device_id: DeviceID
    public_key: PublicKey
    verify_key: VerifyKey


class DeviceClaimContent(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("device_claim", required=True)
        token = fields.String(required=True)
        device_id = DeviceIDField(required=True)
        verify_key = fields.VerifyKey(required=True)
        answer_public_key = fields.PublicKey(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return DeviceClaimContent(**data)

    token: str
    device_id: DeviceID
    verify_key: VerifyKey
    answer_public_key: PublicKey


class DeviceClaimAnswerContent(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("device_claim_answer", required=True)
        private_key = fields.PrivateKey(required=True)
        user_manifest_id = EntryIDField(required=True)
        user_manifest_key = fields.SecretKey(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return DeviceClaimAnswerContent(**data)

    private_key: PrivateKey
    user_manifest_id: EntryID
    user_manifest_key: SecretKey
