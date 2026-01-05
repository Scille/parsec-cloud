# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    DeviceID,
    OrganizationID,
    PkiSignatureAlgorithm,
    UserProfile,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentListBadOutcome,
    AsyncEnrollmentListItem,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import AuthNoLockBadOutcome, AuthNoLockData, auth_no_lock
from parsec.components.postgresql.utils import Q

_q_list_submitted_enrollments = Q("""
SELECT
    enrollment_id,
    submitted_on,
    submit_payload,
    submit_payload_signature_type,
    submit_payload_signature_pki_signature,
    submit_payload_signature_pki_algorithm,
    -- TODO: SQL-Fluff fail to parse this part even if it's a valid PostgreSQL query :/
    -- noqa: disable=all
    ARRAY(
        WITH RECURSIVE my_x509_trustchain AS (
            SELECT
                der_content,
                parent
            FROM pki_x509_certificate
            WHERE sha256_fingerprint = async_enrollment.submit_payload_signature_pki_author_x509_certificate

            UNION ALL

            SELECT
                parent.der_content,
                parent.parent
            FROM pki_x509_certificate AS parent
            INNER JOIN my_x509_trustchain AS child ON parent.sha256_fingerprint = child.parent
        )
        SELECT der_content from my_x509_trustchain
    ) as submit_payload_signature_pki_author_x509_trustchain,
    submit_payload_signature_openbao_signature,
    submit_payload_signature_openbao_author_entity_id
FROM async_enrollment
WHERE
    organization = $organization_internal_id
    AND state = 'SUBMITTED'
ORDER BY submitted_on ASC
""")


async def async_enrollment_list(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> list[AsyncEnrollmentListItem] | AsyncEnrollmentListBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as auth_data:
            organization_internal_id = auth_data.organization_internal_id
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return AsyncEnrollmentListBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return AsyncEnrollmentListBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return AsyncEnrollmentListBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return AsyncEnrollmentListBadOutcome.AUTHOR_REVOKED

    if auth_data.user_current_profile != UserProfile.ADMIN:
        return AsyncEnrollmentListBadOutcome.AUTHOR_NOT_ALLOWED

    # Get list of submitted enrollments

    enrollment_rows = await conn.fetch(
        *_q_list_submitted_enrollments(
            organization_internal_id=organization_internal_id,
        )
    )

    result: list[AsyncEnrollmentListItem] = []
    for row in enrollment_rows:
        match row["enrollment_id"]:
            case str() as raw_enrollment_id:
                enrollment_id = AsyncEnrollmentID.from_hex(raw_enrollment_id)
            case _:
                assert False, row

        match row["submitted_on"]:
            case DateTime() as submitted_on:
                pass
            case _:
                assert False, row

        match row["submit_payload"]:
            case bytes() as submit_payload:
                pass
            case _:
                assert False, row

        # Reconstruct submit_payload_signature based on type
        match row["submit_payload_signature_type"]:
            case "PKI":
                match row["submit_payload_signature_pki_signature"]:
                    case bytes() as pki_signature:
                        pass
                    case _:
                        assert False, row

                match row["submit_payload_signature_pki_algorithm"]:
                    case str() as algorithm_raw:
                        algorithm = PkiSignatureAlgorithm.from_str(algorithm_raw)
                    case _:
                        assert False, row

                match row["submit_payload_signature_pki_author_x509_trustchain"]:
                    case [author_der_x509_certificate, *intermediate_der_x509_certificates]:
                        pass
                    case _:
                        assert False, row

                submit_payload_signature: AsyncEnrollmentPayloadSignature = (
                    AsyncEnrollmentPayloadSignaturePKI(
                        signature=pki_signature,
                        algorithm=algorithm,
                        author_der_x509_certificate=author_der_x509_certificate,
                        intermediate_der_x509_certificates=intermediate_der_x509_certificates,
                    )
                )

            case "OPENBAO":
                match row["submit_payload_signature_openbao_signature"]:
                    case str() as openbao_signature:
                        pass
                    case _:
                        assert False, row

                match row["submit_payload_signature_openbao_author_entity_id"]:
                    case str() as openbao_author_entity_id:
                        pass
                    case _:
                        assert False, row

                submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
                    signature=openbao_signature,
                    author_openbao_entity_id=openbao_author_entity_id,
                )

            case _:
                assert False, row

        result.append(
            AsyncEnrollmentListItem(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submit_payload=submit_payload,
                submit_payload_signature=submit_payload_signature,
            )
        )

    return result
