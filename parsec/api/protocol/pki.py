# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.api.data.pki import PkiEnrollmentRequest
from parsec.api.data.pki import PkiEnrollmentReply
from parsec.api.protocol.base import BaseRepSchema, BaseReqSchema, CmdSerializer
from parsec.api.protocol.types import HumanHandleField
from parsec.serde import fields


# pki_enrolement_request


class PkiEnrollmentRequestReqSchema(BaseReqSchema):
    request = fields.Nested(PkiEnrollmentRequest.SCHEMA_CLS, required=True)
    certificate_id = fields.Bytes(required=True)
    request_id = fields.UUID(required=True)
    force_flag = fields.Boolean(required=True)


class PkiEnrollmentRequestRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    timestamp = fields.DateTime(required=True)


pki_enrollment_request_serializer = CmdSerializer(
    PkiEnrollmentRequestReqSchema, PkiEnrollmentRequestRepSchema
)

# pki_enrolement_get_requests


class PkiEnrollmentGetRequestsReqSchema(BaseReqSchema):
    pass


class PkiEnrollmentGetRequestsRepSchema(BaseRepSchema):
    requests = fields.List(
        fields.Tuple(
            fields.Bytes(required=True),
            fields.UUID(required=True),
            fields.Nested(PkiEnrollmentRequest.SCHEMA_CLS, required=True),
        ),
        required=True,
    )


pki_enrollment_get_requests_serializer = CmdSerializer(
    PkiEnrollmentGetRequestsReqSchema, PkiEnrollmentGetRequestsRepSchema
)

# pki_enrolement_reply


class PkiEnrollmentReplyReqSchema(BaseReqSchema):
    certificate_id = fields.Bytes(required=True)
    request_id = fields.UUID(required=True)
    reply = fields.Nested(PkiEnrollmentReply.SCHEMA_CLS, required=True, allow_none=True)
    user_id = fields.String(required=True, allow_none=True)  # TODO Move to userid ?
    device_certificate = fields.Bytes(required=True, allow_none=True)
    user_certificate = fields.Bytes(required=True, allow_none=True)
    redacted_user_certificate = fields.Bytes(required=True, allow_none=True)
    redacted_device_certificate = fields.Bytes(required=True, allow_none=True)


class PkiEnrollmentReplyRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    timestamp = fields.DateTime(required=True)


pki_enrollment_reply_serializer = CmdSerializer(
    PkiEnrollmentReplyReqSchema, PkiEnrollmentReplyRepSchema
)


# pki_enrolement_get_reply


class PkiEnrollmentGetReplyReqSchema(BaseReqSchema):
    certificate_id = fields.Bytes(required=True)
    request_id = fields.UUID(required=True)


class PkiEnrollmentGetReplyRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    reply = fields.Nested(PkiEnrollmentReply.SCHEMA_CLS, required=True, allow_none=True)
    timestamp = fields.DateTime(required=True)
    admin_human_handle = HumanHandleField(required=True, allow_none=True)


pki_enrollment_get_reply_serializer = CmdSerializer(
    PkiEnrollmentGetReplyReqSchema, PkiEnrollmentGetReplyRepSchema
)
