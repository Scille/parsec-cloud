# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from asyncpg import Record

from parsec._parsec import (
    DateTime,
    OrganizationID,
    PKIEnrollmentID,
    PkiSignatureAlgorithm,
)
from parsec.components.pki import (
    PkiEnrollmentInfo,
    PkiEnrollmentInfoAccepted,
    PkiEnrollmentInfoBadOutcome,
    PkiEnrollmentInfoCancelled,
    PkiEnrollmentInfoRejected,
    PkiEnrollmentInfoSubmitted,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.pki_trustchain import get_trustchain
from parsec.components.postgresql.utils import Q

_q_get_enrollment_info = Q("""
WITH my_org AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_enrollment AS (
    SELECT
        enrollment_state,
        submitted_on,
        info_accepted,
        info_rejected,
        info_cancelled
    FROM pki_enrollment
    WHERE
        organization = (SELECT my_org._id FROM my_org)
        AND enrollment_id = $enrollment_id
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_org) AS organization_internal_id,
    (SELECT is_expired FROM my_org) AS organization_is_expired,
    (SELECT enrollment_state FROM my_enrollment) AS enrollment_state,
    (SELECT submitted_on FROM my_enrollment) AS enrollment_submitted_on,
    (SELECT info_accepted FROM my_enrollment) AS enrollment_info_accepted,
    (SELECT info_rejected FROM my_enrollment) AS enrollment_info_rejected,
    (SELECT info_cancelled FROM my_enrollment) AS enrollment_info_cancelled
""")


async def pki_info(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    enrollment_id: PKIEnrollmentID,
) -> PkiEnrollmentInfo | PkiEnrollmentInfoBadOutcome:
    row = await conn.fetchrow(
        *_q_get_enrollment_info(
            organization_id=organization_id.str,
            enrollment_id=enrollment_id,
        )
    )
    assert row is not None

    match row["organization_internal_id"]:
        case None:
            return PkiEnrollmentInfoBadOutcome.ORGANIZATION_NOT_FOUND
        case int():
            pass
        case _:
            assert False, row

    match row["organization_is_expired"]:
        case True:
            return PkiEnrollmentInfoBadOutcome.ORGANIZATION_EXPIRED
        case False:
            pass
        case _:
            assert False, row

    match row["enrollment_submitted_on"]:
        case None:
            return PkiEnrollmentInfoBadOutcome.ENROLLMENT_NOT_FOUND
        case DateTime() as submitted_on:
            pass
        case _:
            assert False, row

    match row["enrollment_state"]:
        case "SUBMITTED":
            return PkiEnrollmentInfoSubmitted(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
            )

        case "ACCEPTED":
            info_accepted = row["enrollment_info_accepted"]
            assert isinstance(info_accepted, Record), row
            match dict(info_accepted.items()):
                case {
                    "accepted_on": DateTime() as accepted_on,
                    "accepter_x509_cert_sha256_fingerprint": bytes() as accepter_x509_cert_sha256_fingerprint,
                    "accept_payload_signature": bytes() as accept_payload_signature,
                    "accept_payload": bytes() as accept_payload,
                    "accept_payload_signature_algorithm": str() as raw_accept_payload_signature_algorithm,
                    **rest,
                } if not rest:
                    accept_payload_signature_algorithm = PkiSignatureAlgorithm.from_str(
                        raw_accept_payload_signature_algorithm
                    )
                case _:
                    assert False, row

            cert, *intermediate = map(
                lambda x: x.der_content,
                await get_trustchain(conn, accepter_x509_cert_sha256_fingerprint),
            )

            return PkiEnrollmentInfoAccepted(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                accepted_on=accepted_on,
                accepter_der_x509_certificate=cert,
                accepter_intermediate_der_x509_certificates=intermediate,
                accept_payload_signature=accept_payload_signature,
                accept_payload_signature_algorithm=accept_payload_signature_algorithm,
                accept_payload=accept_payload,
            )

        case "REJECTED":
            info_rejected = row["enrollment_info_rejected"]
            assert isinstance(info_rejected, Record), row
            match dict(info_rejected.items()):
                case {"rejected_on": DateTime() as rejected_on, **rest} if not rest:
                    pass
                case _:
                    assert False, row

            return PkiEnrollmentInfoRejected(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                rejected_on=rejected_on,
            )

        case "CANCELLED":
            info_cancelled = row["enrollment_info_cancelled"]
            assert isinstance(info_cancelled, Record), row
            match dict(info_cancelled.items()):
                case {"cancelled_on": DateTime() as cancelled_on, **rest} if not rest:
                    pass
                case _:
                    assert False, row

            return PkiEnrollmentInfoCancelled(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                cancelled_on=cancelled_on,
            )

        case _:
            assert False, row
