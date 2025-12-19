# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    PKIEnrollmentID,
    PkiInvalidCertificateDER,
    PkiInvalidSignature,
    PkiSignatureAlgorithm,
    PkiUntrusted,
    SignedMessage,
    UserCertificate,
    UserProfile,
    VerifyKey,
    load_accept_payload,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.pki import (
    PkiCertificate,
    PkiEnrollmentAcceptBadOutcome,
    PkiEnrollmentAcceptValidateBadOutcome,
    pki_enrollment_accept_validate,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.pki_trustchain import save_trustchain
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    auth_and_lock_common_write,
)
from parsec.components.postgresql.user_create_user import _q_insert_user_and_device
from parsec.components.postgresql.utils import Q
from parsec.config import BackendConfig
from parsec.events import EventCommonCertificate, EventPkiEnrollment

_q_get_enrollment = Q("""
SELECT
    _id AS enrollment_internal_id,
    enrollment_state = 'SUBMITTED' AS in_submitted_state
FROM pki_enrollment
WHERE
    organization = $organization_internal_id
    AND enrollment_id = $enrollment_id
LIMIT 1
FOR UPDATE
""")


_q_update_enrollment = Q("""
WITH my_update AS (
    UPDATE pki_enrollment
    SET
        enrollment_state = 'ACCEPTED',
        accepter = $author_internal_id,
        submitter_accepted_device = $new_device_internal_id,
        info_accepted.accepted_on = $now,
        info_accepted.accepter_x509_cert_sha256_fingerprint = $accepter_x509_cert_sha256_fingerprint,
        info_accepted.accept_payload_signature = $accept_payload_signature,
        info_accepted.accept_payload = $accept_payload,
        info_accepted.accept_payload_signature_algorithm = $accept_payload_signature_algorithm
    WHERE _id = $enrollment_internal_id
    RETURNING TRUE
)

SELECT (SELECT * FROM my_update) AS enrollment_update_ok
""")


async def pki_accept(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    enrollment_id: PKIEnrollmentID,
    payload: bytes,
    payload_signature: bytes,
    payload_signature_algorithm: PkiSignatureAlgorithm,
    accepter_trustchain: list[PkiCertificate],
    submitter_user_certificate: bytes,
    submitter_redacted_user_certificate: bytes,
    submitter_device_certificate: bytes,
    submitter_redacted_device_certificate: bytes,
    config: BackendConfig,
) -> (
    tuple[UserCertificate, DeviceCertificate]
    | PkiEnrollmentAcceptValidateBadOutcome
    | PkiEnrollmentAcceptBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
):
    # 1) Write lock common topic

    # We lock the common topic to ensure the user doesn't change its role
    # after we checked it.
    # This is required since we are going to insert new certificates.
    match await auth_and_lock_common_write(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return PkiEnrollmentAcceptBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return PkiEnrollmentAcceptBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return PkiEnrollmentAcceptBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return PkiEnrollmentAcceptBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile != UserProfile.ADMIN:
        return PkiEnrollmentAcceptBadOutcome.AUTHOR_NOT_ALLOWED

    # 2) Validate certificates

    try:
        load_accept_payload(
            SignedMessage(payload_signature_algorithm, payload_signature, payload),
            accepter_trustchain[0].content,
            list(map(lambda v: v.content, accepter_trustchain[1:])),
            config.x509_trust_anchor,
            now,
        )
    except PkiUntrusted:
        return PkiEnrollmentAcceptBadOutcome.INVALID_X509_TRUSTCHAIN
    except PkiInvalidCertificateDER:
        return PkiEnrollmentAcceptBadOutcome.INVALID_DER_X509_CERTIFICATE
    except PkiInvalidSignature:
        return PkiEnrollmentAcceptBadOutcome.INVALID_PAYLOAD_SIGNATURE
    except ValueError:
        return PkiEnrollmentAcceptBadOutcome.INVALID_ACCEPT_PAYLOAD

    match pki_enrollment_accept_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        user_certificate=submitter_user_certificate,
        device_certificate=submitter_device_certificate,
        redacted_user_certificate=submitter_redacted_user_certificate,
        redacted_device_certificate=submitter_redacted_device_certificate,
    ):
        case (user_certif, device_certif):
            pass
        case error:
            return error

    # 3) Ensure we are not breaking causality by adding a newer timestamp.

    # We already ensured user and device certificates' timestamps are consistent,
    # so only need to check one of them here
    if user_certif.timestamp <= db_common.last_common_certificate_timestamp:
        return RequireGreaterTimestamp(
            strictly_greater_than=db_common.last_common_certificate_timestamp
        )

    # 4) Retrieve the enrollment

    enrollment_row = await conn.fetchrow(
        *_q_get_enrollment(
            organization_internal_id=db_common.organization_internal_id,
            enrollment_id=enrollment_id,
        )
    )
    if enrollment_row is None:
        return PkiEnrollmentAcceptBadOutcome.ENROLLMENT_NOT_FOUND

    match enrollment_row["enrollment_internal_id"]:
        case int() as enrollment_internal_id:
            pass
        case _:
            assert False, enrollment_row

    match enrollment_row["in_submitted_state"]:
        case True:
            pass
        case False:
            return PkiEnrollmentAcceptBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE
        case _:
            assert False, enrollment_row

    # 5) (Almost) all checks are good, now we do the actual insertion
    # Almost because the cases user_id/device_id already exists and human_handle
    # already taken are handled in the insert query

    row = await conn.fetchrow(
        *_q_insert_user_and_device(
            organization_internal_id=db_common.organization_internal_id,
            author_internal_id=db_common.device_internal_id,
            created_on=user_certif.timestamp,
            email=str(user_certif.human_handle.email),
            label=user_certif.human_handle.label,
            user_id=user_certif.user_id,
            initial_profile=user_certif.profile.str,
            user_certificate=submitter_user_certificate,
            redacted_user_certificate=submitter_redacted_user_certificate,
            device_id=device_certif.device_id,
            device_label=device_certif.device_label.str,
            verify_key=device_certif.verify_key.encode(),
            device_certificate=submitter_device_certificate,
            redacted_device_certificate=submitter_redacted_device_certificate,
        )
    )
    assert row is not None

    match row["human_already_taken"]:
        case False:
            pass
        case True:
            return PkiEnrollmentAcceptBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN
        case _:
            assert False, row

    match row["active_users_limit_reached"]:
        case False:
            pass
        case True:
            return PkiEnrollmentAcceptBadOutcome.ACTIVE_USERS_LIMIT_REACHED
        case _:
            assert False, row

    match row["new_user_internal_id"]:
        case int():
            pass
        case None:
            return PkiEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS
        case _:
            assert False, row

    match row["new_device_internal_id"]:
        case int() as new_device_internal_id:
            pass
        case None:
            return PkiEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS
        case _:
            assert False, row

    match row["update_common_topic_ok"]:
        case True:
            pass
        case _:
            assert False, row

    await send_signal(
        conn,
        EventCommonCertificate(
            organization_id=organization_id,
            timestamp=user_certif.timestamp,
        ),
    )

    await send_signal(
        conn,
        EventPkiEnrollment(
            organization_id=organization_id,
        ),
    )

    await save_trustchain(conn, accepter_trustchain)

    # 6) Finally update the enrollment as accepted

    enrollment_update_row = await conn.fetchrow(
        *_q_update_enrollment(
            enrollment_internal_id=enrollment_internal_id,
            author_internal_id=db_common.device_internal_id,
            new_device_internal_id=new_device_internal_id,
            now=now,
            accepter_x509_cert_sha256_fingerprint=accepter_trustchain[0].fingerprint_sha256,
            accept_payload_signature=payload_signature,
            accept_payload_signature_algorithm=payload_signature_algorithm.str,
            accept_payload=payload,
        )
    )
    assert enrollment_update_row is not None

    match enrollment_update_row["enrollment_update_ok"]:
        case True:
            pass
        case _:
            assert False, enrollment_update_row

    return user_certif, device_certif
