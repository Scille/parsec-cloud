# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Dict, cast
from enum import Enum

from parsec.api.protocol.base import BaseRepSchema, BaseReqSchema, CmdSerializer
from parsec.serde import BaseSchema, OneOfSchema, fields


class PkiEnrollmentStatus(Enum):
    SUBMITTED = "SUBMITTED"
    ACCEPTED = "ACCEPTED"
    REJECTED = "REJECTED"


PkiEnrollmentStatusField = fields.enum_field_factory(PkiEnrollmentStatus)


# pki_enrollment_submit


class PkiEnrollmentSubmitReqSchema(BaseReqSchema):
    enrollment_id = fields.UUID(required=True)
    # Existing enrollment with SUMBITTED status prevent submitting new
    # enrollement with similir x509 certificate unless force flag is set.
    force = fields.Boolean(required=True)

    submitter_der_x509_certificate = fields.Bytes(require=True)
    submitted_signature = fields.Bytes(required=True)
    submitted_payload = fields.Bytes(required=True)  # Signature should be checked before loading


class PkiEnrollmentSubmitRepSchema(BaseRepSchema):
    pass


pki_enrollment_submit_serializer = CmdSerializer(
    PkiEnrollmentSubmitReqSchema, PkiEnrollmentSubmitRepSchema
)


# pki_enrollment_info


class PkiEnrollmentInfoReqSchema(BaseReqSchema):
    enrollment_id = fields.UUID(required=True)


class PkiEnrollmentInfoRepSubmittedSchema(BaseRepSchema):
    type = fields.EnumCheckedConstant(PkiEnrollmentStatus.SUBMITTED, required=True)
    submitted_on = fields.DateTime(required=True)


class PkiEnrollmentInfoRepAcceptedSchema(BaseRepSchema):
    type = fields.EnumCheckedConstant(PkiEnrollmentStatus.ACCEPTED, required=True)
    submitted_on = fields.DateTime(required=True)
    accepted_on = fields.DateTime(required=True)

    accepter_der_x509_certificate = fields.Bytes(required=True)
    accepted_signature = fields.Bytes(required=True)
    accepted_payload = fields.Bytes(required=True)  # Signature should be checked before loading


class PkiEnrollmentInfoRepRejectedSchema(BaseRepSchema):
    type = fields.EnumCheckedConstant(PkiEnrollmentStatus.REJECTED, required=True)
    submitted_on = fields.DateTime(required=True)
    rejected_on = fields.DateTime(required=True)


class PkiEnrollmentInfoRepSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {
        PkiEnrollmentStatus.SUBMITTED: PkiEnrollmentInfoRepSubmittedSchema(),
        PkiEnrollmentStatus.ACCEPTED: PkiEnrollmentInfoRepAcceptedSchema(),
        PkiEnrollmentStatus.REJECTED: PkiEnrollmentInfoRepRejectedSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> PkiEnrollmentStatus:
        return cast(PkiEnrollmentStatus, obj["type"])


pki_enrollment_info_serializer = CmdSerializer(
    PkiEnrollmentInfoReqSchema, PkiEnrollmentInfoRepSchema
)


# pki_enrollment_list


class PkiEnrollmentListItemSchema(BaseSchema):
    enrollment_id = fields.UUID(required=True)
    submitted_on = fields.DateTime(required=True)
    submitter_der_x509_certificate = fields.Bytes(require=True)
    submitted_signature = fields.Bytes(required=True)
    submitted_payload = fields.Bytes(required=True)  # Signature should be checked before loading


class PkiEnrollmentListReqSchema(BaseReqSchema):
    pass


class PkiEnrollmentListRepSchema(BaseRepSchema):
    requests = fields.List(fields.Nested(PkiEnrollmentListItemSchema), required=True)


pki_enrollment_list_serializer = CmdSerializer(
    PkiEnrollmentListReqSchema, PkiEnrollmentListRepSchema
)


# pki_enrollment_reject


class PkiEnrollmentRejectReqSchema(BaseReqSchema):
    enrollment_id = fields.UUID(required=True)


class PkiEnrollmentRejectRepSchema(BaseRepSchema):
    pass


pki_enrollment_reject_serializer = CmdSerializer(
    PkiEnrollmentRejectReqSchema, PkiEnrollmentRejectRepSchema
)


# pki_enrollment_accept


class PkiEnrollmentAcceptReqSchema(BaseReqSchema):
    enrollment_id = fields.UUID(required=True)

    accepter_der_x509_certificate = fields.Bytes(required=True)
    accepted_signature = fields.Bytes(required=True)
    accepted_payload = fields.Bytes(required=True)  # Signature should be checked before loading

    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    # Same certificates than above, but expurged of human_handle/device_label
    redacted_user_certificate = fields.Bytes(required=True)
    redacted_device_certificate = fields.Bytes(required=True)


class PkiEnrollmentAcceptRepSchema(BaseRepSchema):
    pass


pki_enrollment_accept_serializer = CmdSerializer(
    PkiEnrollmentAcceptReqSchema, PkiEnrollmentAcceptRepSchema
)
