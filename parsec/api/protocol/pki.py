# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    PkiEnrollmentAcceptRep,
    PkiEnrollmentAcceptReq,
    PkiEnrollmentInfoRep,
    PkiEnrollmentInfoReq,
    PkiEnrollmentListRep,
    PkiEnrollmentListReq,
    PkiEnrollmentRejectRep,
    PkiEnrollmentRejectReq,
    PkiEnrollmentSubmitRep,
    PkiEnrollmentSubmitReq,
)
from parsec.api.protocol.base import ApiCommandSerializer

# pki_enrollment_submit
pki_enrollment_submit_serializer = ApiCommandSerializer(
    PkiEnrollmentSubmitReq, PkiEnrollmentSubmitRep
)

pki_enrollment_info_serializer = ApiCommandSerializer(PkiEnrollmentInfoReq, PkiEnrollmentInfoRep)


# pki_enrollment_list
pki_enrollment_list_serializer = ApiCommandSerializer(PkiEnrollmentListReq, PkiEnrollmentListRep)


# pki_enrollment_reject
pki_enrollment_reject_serializer = ApiCommandSerializer(
    PkiEnrollmentRejectReq, PkiEnrollmentRejectRep
)


# pki_enrollment_accept
pki_enrollment_accept_serializer = ApiCommandSerializer(
    PkiEnrollmentAcceptReq, PkiEnrollmentAcceptRep
)
