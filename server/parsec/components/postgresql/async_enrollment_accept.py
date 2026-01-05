# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    UserCertificate,
    UserProfile,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.async_enrollment import (
    AsyncEnrollmentAcceptBadOutcome,
    AsyncEnrollmentAcceptValidateBadOutcome,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
    async_enrollment_accept_validate,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.async_enrollment_submit import (
    _q_insert_pki_x509_certificate,
    _q_take_async_enrollment_create_write_lock,
)
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    auth_and_lock_common_write,
)
from parsec.components.postgresql.user_create_user import _q_insert_user_and_device
from parsec.components.postgresql.utils import Q
from parsec.events import EventAsyncEnrollment, EventCommonCertificate

_q_get_enrollment = Q("""
SELECT
    _id AS enrollment_internal_id,
    state,
    submit_payload_signature_type
FROM async_enrollment
WHERE
    organization = $organization_internal_id
    AND enrollment_id = $enrollment_id
LIMIT 1
""")


_q_update_enrollment_accept = Q("""
UPDATE async_enrollment
SET
    state = 'ACCEPTED',
    accepted_on = $now,
    accept_payload = $accept_payload,
    accept_payload_signature_type = $accept_payload_signature_type,
    accept_payload_signature_pki_signature = $accept_payload_signature_pki_signature,
    accept_payload_signature_pki_algorithm = $accept_payload_signature_pki_algorithm,
    submit_payload_signature_pki_author_x509_certificate = $submit_payload_signature_pki_author_x509_certificate,
    accept_payload_signature_openbao_signature = $accept_payload_signature_openbao_signature,
    accept_payload_signature_openbao_author_entity_id = $accept_payload_signature_openbao_author_entity_id
WHERE
    _id = $enrollment_internal_id
""")


async def async_enrollment_accept(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    enrollment_id: AsyncEnrollmentID,
    accept_payload: bytes,
    accept_payload_signature: AsyncEnrollmentPayloadSignature,
    submitter_user_certificate: bytes,
    submitter_redacted_user_certificate: bytes,
    submitter_device_certificate: bytes,
    submitter_redacted_device_certificate: bytes,
) -> (
    tuple[UserCertificate, DeviceCertificate]
    | AsyncEnrollmentAcceptValidateBadOutcome
    | AsyncEnrollmentAcceptBadOutcome
    | RequireGreaterTimestamp
    | TimestampOutOfBallpark
):
    # Write lock common topic

    # We lock the common topic to ensure the user doesn't change its role
    # after we checked it.
    # This is required since we are going to insert new certificates.
    match await auth_and_lock_common_write(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return AsyncEnrollmentAcceptBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return AsyncEnrollmentAcceptBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return AsyncEnrollmentAcceptBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return AsyncEnrollmentAcceptBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile != UserProfile.ADMIN:
        return AsyncEnrollmentAcceptBadOutcome.AUTHOR_NOT_ALLOWED

    # Take lock to prevent any concurrent async enrollment creation

    await _q_take_async_enrollment_create_write_lock(conn, organization_id)

    # Retrieve the enrollment and check it can be accepted

    enrollment_row = await conn.fetchrow(
        *_q_get_enrollment(
            organization_internal_id=db_common.organization_internal_id,
            enrollment_id=enrollment_id,
        )
    )

    if enrollment_row is None:
        return AsyncEnrollmentAcceptBadOutcome.ENROLLMENT_NOT_FOUND

    match enrollment_row["enrollment_internal_id"]:
        case int() as enrollment_internal_id:
            pass
        case _:
            assert False, enrollment_row

    match enrollment_row["state"]:
        case "SUBMITTED":
            pass
        case _:
            return AsyncEnrollmentAcceptBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

    match enrollment_row["submit_payload_signature_type"]:
        case "PKI":
            submit_signature_type = "PKI"
        case "OPENBAO":
            submit_signature_type = "OPENBAO"
        case _:
            assert False, enrollment_row

    match accept_payload_signature:
        case AsyncEnrollmentPayloadSignaturePKI():
            accept_signature_type = "PKI"
        case AsyncEnrollmentPayloadSignatureOpenBao():
            accept_signature_type = "OPENBAO"

    if submit_signature_type != accept_signature_type:
        return AsyncEnrollmentAcceptBadOutcome.SUBMIT_AND_ACCEPT_IDENTITY_SYSTEMS_MISMATCH

    # Validate accept payload and certificates

    match async_enrollment_accept_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        accept_payload=accept_payload,
        accept_payload_signature=accept_payload_signature,
        user_certificate=submitter_user_certificate,
        device_certificate=submitter_device_certificate,
        redacted_user_certificate=submitter_redacted_user_certificate,
        redacted_device_certificate=submitter_redacted_device_certificate,
    ):
        case (_, u_certif, d_certif, x509_trustchain):
            pass
        case error:
            return error

    # Ensure we are not breaking causality by adding a newer timestamp

    # We already ensured user and device certificates' timestamps are consistent,
    # so only need to check one of them here
    if db_common.last_common_certificate_timestamp >= u_certif.timestamp:
        return RequireGreaterTimestamp(
            strictly_greater_than=db_common.last_common_certificate_timestamp
        )

    # All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_insert_user_and_device(
            organization_internal_id=db_common.organization_internal_id,
            email=u_certif.human_handle.email.str,
            label=u_certif.human_handle.label,
            user_id=u_certif.user_id,
            initial_profile=u_certif.profile.str,
            user_certificate=submitter_user_certificate,
            redacted_user_certificate=submitter_redacted_user_certificate,
            author_internal_id=db_common.device_internal_id,
            created_on=u_certif.timestamp,
            device_id=d_certif.device_id,
            device_label=d_certif.device_label.str if d_certif.device_label else None,
            verify_key=d_certif.verify_key.encode(),
            device_certificate=submitter_device_certificate,
            redacted_device_certificate=submitter_redacted_device_certificate,
        )
    )
    assert row is not None

    match row["active_users_limit_reached"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentAcceptBadOutcome.ACTIVE_USERS_LIMIT_REACHED
        case _:
            assert False, row

    match row["human_already_taken"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentAcceptBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN
        case _:
            assert False, row

    match row["new_user_internal_id"]:
        case int():
            pass
        case None:
            return AsyncEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS
        case _:
            assert False, row

    match row["new_device_internal_id"]:
        case int():
            pass
        case None:
            return AsyncEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS
        case _:
            assert False, row

    match row["update_common_topic_ok"]:
        case True:
            pass
        case _:
            assert False, row

    # Update enrollment to ACCEPTED

    match accept_payload_signature:
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

            # Now update the enrollment record
            await conn.execute(
                *_q_update_enrollment_accept(
                    enrollment_internal_id=enrollment_internal_id,
                    now=now,
                    accept_payload=accept_payload,
                    accept_payload_signature_type="PKI",
                    accept_payload_signature_pki_signature=pki_sig.signature,
                    accept_payload_signature_pki_algorithm=pki_sig.algorithm.str,
                    submit_payload_signature_pki_author_x509_certificate=x509_trustchain[
                        0
                    ].sha256_fingerprint,
                    accept_payload_signature_openbao_signature=None,
                    accept_payload_signature_openbao_author_entity_id=None,
                )
            )

        case AsyncEnrollmentPayloadSignatureOpenBao() as openbao_sig:
            assert x509_trustchain is None, x509_trustchain
            await conn.execute(
                *_q_update_enrollment_accept(
                    enrollment_internal_id=enrollment_internal_id,
                    now=now,
                    accept_payload=accept_payload,
                    accept_payload_signature_type="OPENBAO",
                    accept_payload_signature_pki_signature=None,
                    accept_payload_signature_pki_algorithm=None,
                    submit_payload_signature_pki_author_x509_certificate=None,
                    accept_payload_signature_openbao_signature=openbao_sig.signature,
                    accept_payload_signature_openbao_author_entity_id=openbao_sig.author_openbao_entity_id,
                )
            )

    await send_signal(
        conn,
        EventCommonCertificate(
            organization_id=organization_id,
            timestamp=u_certif.timestamp,
        ),
    )

    await send_signal(
        conn,
        EventAsyncEnrollment(
            organization_id=organization_id,
        ),
    )

    return u_certif, d_certif
