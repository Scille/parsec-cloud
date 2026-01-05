# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    OrganizationID,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentEmailAlreadySubmitted,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
    AsyncEnrollmentSubmitBadOutcome,
    AsyncEnrollmentSubmitValidateBadOutcome,
    async_enrollment_submit_validate,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.utils import Q
from parsec.events import EventAsyncEnrollment
from parsec.locks import AdvisoryLock

_q_lock_common_read_and_fetch_checks = Q(
    """
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

-- Common topic write lock must occur ASAP
-- This is needed since async enrollment is not allowed for existing users!
my_locked_common_topic AS (
    SELECT last_timestamp
    FROM common_topic
    WHERE organization = (SELECT my_organization._id FROM my_organization)
    LIMIT 1
    FOR SHARE
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_expired,
    -- Ensure my_locked_common_topic is used here (even if last_common_certificate_timestamp is
    -- not needed in the result), otherwise PostgreSQL will not execute it no row will be locked!
    (SELECT last_timestamp FROM my_locked_common_topic) AS last_common_certificate_timestamp
"""
)


_q_async_enrollment_checks = Q("""
SELECT
    COALESCE(
        (
            SELECT TRUE
            FROM async_enrollment
            WHERE
                organization = $organization_internal_id
                AND enrollment_id = $enrollment_id
            LIMIT 1
        ),
        FALSE
    ) AS enrollment_id_already_exists,

    COALESCE(
        (
            SELECT TRUE
            FROM user_
            INNER JOIN human ON user_.human = human._id
            WHERE
                user_.organization = $organization_internal_id
                AND human.email = $email
                AND user_.revoked_on IS NULL
            LIMIT 1
        ),
        FALSE
    ) AS email_already_enrolled,

    (
        SELECT submitted_on
        FROM async_enrollment
        WHERE
            organization = $organization_internal_id
            AND submit_payload_requested_human_handle_email = $email
            AND state = 'SUBMITTED'
        LIMIT 1
    ) AS enrollment_in_submitted_state_with_email
""")


_q_insert_pki_x509_certificate = Q("""
INSERT INTO pki_x509_certificate (
    sha256_fingerprint,
    parent,
    der_content
) VALUES (
    $sha256_fingerprint,
    $parent,
    $der_content
)
ON CONFLICT DO NOTHING
""")


_q_insert_enrollment = Q("""
WITH my_cancelled_enrollments AS (
    UPDATE async_enrollment
    SET
        state = 'CANCELLED',
        cancelled_on = $submitted_on
    WHERE
        organization = $organization_internal_id
        AND submit_payload_requested_human_handle_email = $submit_payload_requested_human_handle_email
        AND state = 'SUBMITTED'
    RETURNING TRUE
),

my_inserted_enrollment AS (
    INSERT INTO async_enrollment (
        organization,
        enrollment_id,
        submitted_on,
        submit_payload,
        submit_payload_requested_human_handle_email,
        submit_payload_signature_type,
        submit_payload_signature_pki_signature,
        submit_payload_signature_pki_algorithm,
        submit_payload_signature_pki_author_x509_certificate,
        submit_payload_signature_openbao_signature,
        submit_payload_signature_openbao_author_entity_id,
        state
    )
    VALUES (
        $organization_internal_id,
        $enrollment_id,
        $submitted_on,
        $submit_payload,
        $submit_payload_requested_human_handle_email,
        $submit_payload_signature_type,
        $submit_payload_signature_pki_signature,
        $submit_payload_signature_pki_algorithm,
        $submit_payload_signature_pki_author_x509_certificate,
        $submit_payload_signature_openbao_signature,
        $submit_payload_signature_openbao_author_entity_id,
        'SUBMITTED'
    )
    RETURNING TRUE
)

SELECT
    COUNT((SELECT * FROM my_cancelled_enrollments)) AS cancelled,
    COALESCE((SELECT * FROM my_inserted_enrollment), FALSE) AS insert_ok
""")


_q_lock = Q(f"""
SELECT PG_ADVISORY_XACT_LOCK({AdvisoryLock.AsyncEnrollmentCreation}, _id)
FROM organization
WHERE organization_id = $organization_id
""")


async def _q_take_async_enrollment_create_write_lock(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> None:
    """
    Only a single active async enrollment for a given email is allowed.

    However we cannot enforce this purely in PostgreSQL (e.g. with a unique index)
    since async enrollments are never removed but only marked cancelled/rejected/accepted.

    So the easy way to solve this is to get rid of the concurrency altogether
    by requesting a per-organization PostgreSQL Advisory Lock to be held before
    the async enrollment creation procedure starts any check.
    """
    await conn.execute(*_q_lock(organization_id=organization_id.str))


async def async_enrollment_submit(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    enrollment_id: AsyncEnrollmentID,
    force: bool,
    submit_payload: bytes,
    submit_payload_signature: AsyncEnrollmentPayloadSignature,
) -> (
    None
    | AsyncEnrollmentSubmitValidateBadOutcome
    | AsyncEnrollmentEmailAlreadySubmitted
    | AsyncEnrollmentSubmitBadOutcome
):
    # 1) Validate payload

    match async_enrollment_submit_validate(
        now=now,
        submit_payload=submit_payload,
        submit_payload_signature=submit_payload_signature,
    ):
        case (s_payload, x509_trustchain):
            pass
        case error:
            return error

    # 2) Check organization and take common topic lock since async enrollment
    # is not allowed for existing users!

    org_row = await conn.fetchrow(
        *_q_lock_common_read_and_fetch_checks(
            organization_id=organization_id.str,
        )
    )
    assert org_row is not None

    match org_row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return AsyncEnrollmentSubmitBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, org_row

    match org_row["organization_expired"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentSubmitBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, org_row

    # 3) Take lock to prevent any concurrent async enrollment creation, and check
    # the async enrollment doesn't already exist

    await _q_take_async_enrollment_create_write_lock(conn, organization_id)

    row = await conn.fetchrow(
        *_q_async_enrollment_checks(
            organization_internal_id=organization_internal_id,
            enrollment_id=enrollment_id,
            email=s_payload.requested_human_handle.email.str,
        )
    )
    assert row is not None

    match row["enrollment_id_already_exists"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentSubmitBadOutcome.ID_ALREADY_USED
        case _:
            assert False, row

    match row["email_already_enrolled"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentSubmitBadOutcome.EMAIL_ALREADY_ENROLLED
        case _:
            assert False, row

    match row["enrollment_in_submitted_state_with_email"]:
        case None:
            pass
        case DateTime() as submitted_on:
            if not force:
                return AsyncEnrollmentEmailAlreadySubmitted(
                    submitted_on=submitted_on,
                )
        case _:
            assert False, row

    # 4) All checks are good, now we do the actual insertion.
    # (Also note this query will also mark as cancelled any already submitted
    # enrollment for this email that is currently is the SUBMITTED state)

    match submit_payload_signature:
        case AsyncEnrollmentPayloadSignaturePKI() as pki_sig:
            assert x509_trustchain is not None

            # Insert all certificates in reverse order (from root to leaf)
            # to ensure foreign key constraints are satisfied
            await conn.executemany(
                _q_insert_pki_x509_certificate.sql,
                (
                    _q_insert_pki_x509_certificate.arg_only(
                        sha256_fingerprint=cert.sha256_fingerprint,
                        parent=next(
                            (
                                c.sha256_fingerprint
                                for c in x509_trustchain
                                if c.subject == cert.issuer and c != cert
                            ),
                            None,
                        ),
                        der_content=cert.der,
                    )
                    for cert in reversed(x509_trustchain)
                ),
            )

            # Now insert the enrollment record
            await conn.execute(
                *_q_insert_enrollment(
                    organization_internal_id=organization_internal_id,
                    enrollment_id=enrollment_id,
                    submitted_on=now,
                    submit_payload=submit_payload,
                    submit_payload_requested_human_handle_email=s_payload.requested_human_handle.email.str,
                    submit_payload_signature_type="PKI",
                    submit_payload_signature_pki_signature=pki_sig.signature,
                    submit_payload_signature_pki_algorithm=pki_sig.algorithm.str,
                    submit_payload_signature_pki_author_x509_certificate=x509_trustchain[
                        0
                    ].sha256_fingerprint,
                    submit_payload_signature_openbao_signature=None,
                    submit_payload_signature_openbao_author_entity_id=None,
                )
            )

        case AsyncEnrollmentPayloadSignatureOpenBao() as openbao_sig:
            assert x509_trustchain is None, x509_trustchain

            await conn.execute(
                *_q_insert_enrollment(
                    organization_internal_id=organization_internal_id,
                    enrollment_id=enrollment_id,
                    submitted_on=now,
                    submit_payload=submit_payload,
                    submit_payload_requested_human_handle_email=s_payload.requested_human_handle.email.str,
                    submit_payload_signature_type="OPENBAO",
                    submit_payload_signature_pki_signature=None,
                    submit_payload_signature_pki_algorithm=None,
                    submit_payload_signature_pki_author_x509_certificate=None,
                    submit_payload_signature_openbao_signature=openbao_sig.signature,
                    submit_payload_signature_openbao_author_entity_id=openbao_sig.author_openbao_entity_id,
                )
            )

    await send_signal(
        conn,
        EventAsyncEnrollment(
            organization_id=organization_id,
        ),
    )
