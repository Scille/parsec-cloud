# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    PKIEnrollmentID,
    PkiSignatureAlgorithm,
    UserProfile,
)
from parsec.components.pki import (
    PkiEnrollmentListBadOutcome,
    PkiEnrollmentListItem,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.pki_trustchain import get_trustchain
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q

_q_list_enrollments = Q("""
SELECT
    enrollment_id,
    submitted_on,
    submitter_x509_cert_sha256_fingerprint,
    submit_payload_signature,
    submit_payload_signature_algorithm,
    submit_payload
FROM pki_enrollment
WHERE
    organization = $organization_internal_id
    AND enrollment_state = 'SUBMITTED'
ORDER BY submitted_on
""")


async def pki_list(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> list[PkiEnrollmentListItem] | PkiEnrollmentListBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as auth_data:
            organization_internal_id = auth_data.organization_internal_id
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return PkiEnrollmentListBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return PkiEnrollmentListBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return PkiEnrollmentListBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return PkiEnrollmentListBadOutcome.AUTHOR_REVOKED

    if auth_data.user_current_profile != UserProfile.ADMIN:
        return PkiEnrollmentListBadOutcome.AUTHOR_NOT_ALLOWED

    rows = await conn.fetch(
        *_q_list_enrollments(
            organization_internal_id=organization_internal_id,
        )
    )

    items = []
    for row in rows:
        match row["enrollment_id"]:
            case str() as raw_enrollment_id:
                enrollment_id = PKIEnrollmentID.from_hex(raw_enrollment_id)
            case _:
                assert False, row

        match row["submitted_on"]:
            case DateTime() as submitted_on:
                pass
            case _:
                assert False, row

        match row["submitter_x509_cert_sha256_fingerprint"]:
            case bytes() as submitter_x509_cert_sha256_fingerprint:
                pass
            case _:
                assert False, row

        match row["submit_payload_signature"]:
            case bytes() as payload_signature:
                pass
            case _:
                assert False, row

        match row["submit_payload_signature_algorithm"]:
            case str() as raw_payload_signature_algorithm:
                payload_signature_algorithm = PkiSignatureAlgorithm.from_str(
                    raw_payload_signature_algorithm
                )
            case _:
                assert False, row

        match row["submit_payload"]:
            case bytes() as payload:
                pass
            case _:
                assert False, row

        leaf, *intermediate = await get_trustchain(conn, submitter_x509_cert_sha256_fingerprint)

        items.append(
            PkiEnrollmentListItem(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                der_x509_certificate=leaf.der_content,
                intermediate_der_x509_certificates=list(map(lambda x: x.der_content, intermediate)),
                payload_signature=payload_signature,
                payload_signature_algorithm=payload_signature_algorithm,
                payload=payload,
            )
        )

    return items
