# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.api.data.pki import PkiEnrollmentRequest
from parsec.api.data.pki import PkiEnrollmentReply
from parsec.api.protocol.base import BaseRepSchema, BaseReqSchema, CmdSerializer
from parsec.serde import fields
from parsec.serde.schema import BaseSchema
from parsec.serde.serializer import MsgpackSerializer


# pki_enrolement_request


class PkiEnrollmentRequestReqSchema(BaseSchema):
    request = fields.Nested(PkiEnrollmentRequest.SCHEMA_CLS, required=True)
    certificate_id = fields.Bytes(required=True)
    request_id = fields.UUID(required=True)
    force_flag = fields.Boolean(required=True)


class PkiEnrollmentRequestRepSchema(BaseSchema):
    status = fields.String(Required=True)
    timestamp = fields.DateTime(required=True)


pki_enrollment_request_req_serializer = MsgpackSerializer(PkiEnrollmentRequestReqSchema)
pki_enrollment_request_rep_serializer = MsgpackSerializer(PkiEnrollmentRequestRepSchema)

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
    reply = fields.Nested(PkiEnrollmentReply.SCHEMA_CLS, Required=True)
    user_id = fields.String(required=False)  # TODO Move to userid ?


class PkiEnrollmentReplyRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    timestamp = fields.DateTime(required=True)


pki_enrollment_reply_serializer = CmdSerializer(
    PkiEnrollmentReplyReqSchema, PkiEnrollmentReplyRepSchema
)
# pki_enrolement_get_reply


class PkiEnrollmentGetReplyReqSchema(BaseSchema):
    certificate_id = fields.Bytes(required=True)
    request_id = fields.UUID(required=True)


class PkiEnrollmentGetReplyRepSchema(BaseSchema):
    status = fields.String(Required=True)
    reply = fields.Nested(PkiEnrollmentReply.SCHEMA_CLS, Required=True)
    timestamp = fields.DateTime(Required=False)


pki_enrollment_get_reply_req_serializer = MsgpackSerializer(PkiEnrollmentGetReplyReqSchema)
pki_enrollment_get_reply_rep_serializer = MsgpackSerializer(PkiEnrollmentGetReplyRepSchema)
