# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Dict, cast
from enum import Enum

from parsec.api.protocol.base import BaseRepSchema, BaseReqSchema, CmdSerializer
from parsec.serde import BaseSchema, OneOfSchema, fields


class PkiEnrollmentStatus(Enum):
    SUBMITTED = "SUBMITTED"
    ACCEPTED = "ACCEPTED"
    REJECTED = "REJECTED"
    CANCELLED = "CANCELLED"


# pki_enrollment_submit


class PkiEnrollmentSubmitReqSchema(BaseReqSchema):
    enrollment_id = fields.UUID(required=True)
    # Existing enrollment with SUMBITTED status prevent submitting new
    # enrollment with similir x509 certificate unless force flag is set.
    force = fields.Boolean(required=True)

    submitter_der_x509_certificate = fields.Bytes(required=True)
    # Duplicated certificate email field. (The backend need to check if the email is used without loading the certificate)
    submitter_der_x509_certificate_email = fields.String(
        required=False, missing=None, allow_none=True
    )
    # Signature should be checked before loading
    submit_payload_signature = fields.Bytes(required=True)
    submit_payload = fields.Bytes(required=True)


class PkiEnrollmentSubmitRepSchema(BaseRepSchema):
    submitted_on = fields.DateTime(required=True)


class PkiEnrollmentErrorSubmitRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    submitted_on = fields.DateTime(required=True)


pki_enrollment_submit_serializer = CmdSerializer(
    PkiEnrollmentSubmitReqSchema,
    PkiEnrollmentSubmitRepSchema,
    extra_error_schemas={"already_submitted": PkiEnrollmentErrorSubmitRepSchema},
)


# pki_enrollment_info


class PkiEnrollmentInfoReqSchema(BaseReqSchema):
    enrollment_id = fields.UUID(required=True)


class PkiEnrollmentInfoRepSubmittedSchema(BaseRepSchema):
    enrollment_status = fields.EnumCheckedConstant(PkiEnrollmentStatus.SUBMITTED, required=True)
    submitted_on = fields.DateTime(required=True)


class PkiEnrollmentInfoRepAcceptedSchema(BaseRepSchema):
    enrollment_status = fields.EnumCheckedConstant(PkiEnrollmentStatus.ACCEPTED, required=True)
    submitted_on = fields.DateTime(required=True)
    accepted_on = fields.DateTime(required=True)

    accepter_der_x509_certificate = fields.Bytes(required=True)
    accept_payload_signature = fields.Bytes(required=True)
    accept_payload = fields.Bytes(required=True)  # Signature should be checked before loading


class PkiEnrollmentInfoRepRejectedSchema(BaseRepSchema):
    enrollment_status = fields.EnumCheckedConstant(PkiEnrollmentStatus.REJECTED, required=True)
    submitted_on = fields.DateTime(required=True)
    rejected_on = fields.DateTime(required=True)


class PkiEnrollmentInfoRepCancelledSchema(BaseRepSchema):
    enrollment_status = fields.EnumCheckedConstant(PkiEnrollmentStatus.CANCELLED, required=True)
    submitted_on = fields.DateTime(required=True)
    cancelled_on = fields.DateTime(required=True)


class PkiEnrollmentInfoRepSchema(OneOfSchema):
    type_field = "enrollment_status"
    type_schemas = {
        PkiEnrollmentStatus.SUBMITTED: PkiEnrollmentInfoRepSubmittedSchema(),
        PkiEnrollmentStatus.ACCEPTED: PkiEnrollmentInfoRepAcceptedSchema(),
        PkiEnrollmentStatus.REJECTED: PkiEnrollmentInfoRepRejectedSchema(),
        PkiEnrollmentStatus.CANCELLED: PkiEnrollmentInfoRepCancelledSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> PkiEnrollmentStatus:
        return cast(PkiEnrollmentStatus, obj["enrollment_status"])


pki_enrollment_info_serializer = CmdSerializer(
    PkiEnrollmentInfoReqSchema, PkiEnrollmentInfoRepSchema
)


# TODO: rename to pki_enrollment_list_submitted ?
# pki_enrollment_list


class PkiEnrollmentListItemSchema(BaseSchema):
    enrollment_id = fields.UUID(required=True)
    submitted_on = fields.DateTime(required=True)
    submitter_der_x509_certificate = fields.Bytes(required=True)
    submit_payload_signature = fields.Bytes(required=True)
    submit_payload = fields.Bytes(required=True)  # Signature should be checked before loading


class PkiEnrollmentListReqSchema(BaseReqSchema):
    pass


class PkiEnrollmentListRepSchema(BaseRepSchema):
    enrollments = fields.List(fields.Nested(PkiEnrollmentListItemSchema), required=True)


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
    accept_payload_signature = fields.Bytes(required=True)
    accept_payload = fields.Bytes(required=True)  # Signature should be checked before loading

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
