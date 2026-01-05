# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    OrganizationID,
    PkiSignatureAlgorithm,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentInfo,
    AsyncEnrollmentInfoAccepted,
    AsyncEnrollmentInfoBadOutcome,
    AsyncEnrollmentInfoCancelled,
    AsyncEnrollmentInfoRejected,
    AsyncEnrollmentInfoSubmitted,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_check_organization = Q("""
SELECT
    _id,
    is_expired
FROM organization
WHERE
    organization_id = $organization_id
    AND root_verify_key IS NOT NULL
LIMIT 1
""")


_q_get_enrollment = Q("""
WITH RECURSIVE my_enrollment AS (
    SELECT
        state,
        submitted_on,
        accepted_on,
        accept_payload,
        accept_payload_signature_type,
        accept_payload_signature_pki_signature,
        accept_payload_signature_pki_algorithm,
        submit_payload_signature_pki_author_x509_certificate,
        accept_payload_signature_openbao_signature,
        accept_payload_signature_openbao_author_entity_id,
        rejected_on,
        cancelled_on
    FROM async_enrollment
    WHERE
        organization = $organization_internal_id
        AND enrollment_id = $enrollment_id
    LIMIT 1
),

my_x509_trustchain AS (
    SELECT
        der_content,
        parent
    FROM pki_x509_certificate
    WHERE
        sha256_fingerprint
        = (SELECT my_enrollment.submit_payload_signature_pki_author_x509_certificate FROM my_enrollment)

    UNION ALL

    SELECT
        parent.der_content,
        parent.parent
    FROM pki_x509_certificate AS parent
    INNER JOIN my_x509_trustchain AS child ON parent.sha256_fingerprint = child.parent
)

SELECT
    state,
    submitted_on,
    accepted_on,
    accept_payload,
    accept_payload_signature_type,
    accept_payload_signature_pki_signature,
    accept_payload_signature_pki_algorithm,
    accept_payload_signature_openbao_signature,
    accept_payload_signature_openbao_author_entity_id,
    rejected_on,
    cancelled_on,
    ARRAY(
        SELECT my_x509_trustchain.der_content FROM my_x509_trustchain
    ) AS submit_payload_signature_pki_author_x509_trustchain
FROM my_enrollment
""")


async def async_enrollment_info(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    enrollment_id: AsyncEnrollmentID,
) -> AsyncEnrollmentInfo | AsyncEnrollmentInfoBadOutcome:
    # 1) Check organization exists and is not expired

    org_row = await conn.fetchrow(
        *_q_check_organization(
            organization_id=organization_id.str,
        )
    )
    assert org_row is not None

    match org_row["_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return AsyncEnrollmentInfoBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, org_row

    match org_row["is_expired"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentInfoBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, org_row

    # 2) Get enrollment info

    enrollment_row = await conn.fetchrow(
        *_q_get_enrollment(
            organization_internal_id=organization_internal_id,
            enrollment_id=enrollment_id,
        )
    )

    if enrollment_row is None:
        return AsyncEnrollmentInfoBadOutcome.ENROLLMENT_NOT_FOUND

    match enrollment_row["state"]:
        case "SUBMITTED":
            match enrollment_row["submitted_on"]:
                case DateTime() as submitted_on:
                    pass
                case _:
                    assert False, enrollment_row
            return AsyncEnrollmentInfoSubmitted(submitted_on=submitted_on)

        case "ACCEPTED":
            match enrollment_row["submitted_on"]:
                case DateTime() as submitted_on:
                    pass
                case _:
                    assert False, enrollment_row

            match enrollment_row["accepted_on"]:
                case DateTime() as accepted_on:
                    pass
                case _:
                    assert False, enrollment_row

            match enrollment_row["accept_payload"]:
                case bytes() as accept_payload:
                    pass
                case _:
                    assert False, enrollment_row

            # Reconstruct accept_payload_signature based on type
            match enrollment_row["accept_payload_signature_type"]:
                case "PKI":
                    match enrollment_row["accept_payload_signature_pki_algorithm"]:
                        case str() as algo_str:
                            algorithm = PkiSignatureAlgorithm.from_str(algo_str)
                        case _:
                            assert False, enrollment_row

                    match enrollment_row["accept_payload_signature_pki_signature"]:
                        case bytes() as accept_payload_signature_pki_signature:
                            pass
                        case _:
                            assert False, enrollment_row

                    match enrollment_row["submit_payload_signature_pki_author_x509_trustchain"]:
                        case [author_der_x509_certificate, *intermediate_der_x509_certificates]:
                            pass
                        case _:
                            assert False, enrollment_row

                    accept_payload_signature: AsyncEnrollmentPayloadSignature = (
                        AsyncEnrollmentPayloadSignaturePKI(
                            signature=accept_payload_signature_pki_signature,
                            algorithm=algorithm,
                            author_der_x509_certificate=author_der_x509_certificate,
                            intermediate_der_x509_certificates=intermediate_der_x509_certificates,
                        )
                    )

                case "OPENBAO":
                    match enrollment_row["accept_payload_signature_openbao_signature"]:
                        case str() as accept_payload_signature_openbao_signature:
                            pass
                        case _:
                            assert False, enrollment_row

                    match enrollment_row["accept_payload_signature_openbao_author_entity_id"]:
                        case str() as accept_payload_signature_openbao_author_entity_id:
                            pass
                        case _:
                            assert False, enrollment_row

                    accept_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
                        signature=accept_payload_signature_openbao_signature,
                        author_openbao_entity_id=accept_payload_signature_openbao_author_entity_id,
                    )

                case _:
                    assert False, enrollment_row

            return AsyncEnrollmentInfoAccepted(
                submitted_on=submitted_on,
                accepted_on=accepted_on,
                accept_payload=accept_payload,
                accept_payload_signature=accept_payload_signature,
            )

        case "REJECTED":
            match enrollment_row["submitted_on"]:
                case DateTime() as submitted_on:
                    pass
                case _:
                    assert False, enrollment_row

            match enrollment_row["rejected_on"]:
                case DateTime() as rejected_on:
                    pass
                case _:
                    assert False, enrollment_row

            return AsyncEnrollmentInfoRejected(
                submitted_on=submitted_on,
                rejected_on=rejected_on,
            )

        case "CANCELLED":
            match enrollment_row["submitted_on"]:
                case DateTime() as submitted_on:
                    pass
                case _:
                    assert False, enrollment_row

            match enrollment_row["cancelled_on"]:
                case DateTime() as cancelled_on:
                    pass
                case _:
                    assert False, enrollment_row

            return AsyncEnrollmentInfoCancelled(
                submitted_on=submitted_on,
                cancelled_on=cancelled_on,
            )

        case _:
            assert False, enrollment_row
