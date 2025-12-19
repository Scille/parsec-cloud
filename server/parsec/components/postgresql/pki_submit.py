# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    HumanHandle,
    OrganizationID,
    PKIEnrollmentID,
    PkiInvalidCertificateDER,
    PkiInvalidSignature,
    PkiSignatureAlgorithm,
    PkiUntrusted,
    SignedMessage,
    load_submit_payload,
)
from parsec.components.pki import (
    PkiCertificate,
    PkiEnrollmentSubmitBadOutcome,
    PkiEnrollmentSubmitX509CertificateAlreadySubmitted,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.pki_trustchain import save_trustchain
from parsec.components.postgresql.queries.q_lock_common import lock_common_read
from parsec.components.postgresql.utils import Q
from parsec.config import BackendConfig
from parsec.events import EventPkiEnrollment
from parsec.locks import AdvisoryLock

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


_q_check_enrollment_id = Q("""
SELECT TRUE
FROM pki_enrollment
WHERE
    organization = $organization_internal_id
    AND enrollment_id = $enrollment_id
LIMIT 1
""")


_q_get_previous_enrollment = Q("""
SELECT
    _id AS enrollment_internal_id,
    submitted_on,
    enrollment_state,
    (
        SELECT user_.revoked_on IS NOT NULL
        FROM device
        INNER JOIN user_
            ON device.user_ = user_._id
        WHERE device._id = pki_enrollment.submitter_accepted_device
    ) AS accepted_device_is_revoked
FROM pki_enrollment
WHERE
    organization = $organization_internal_id
    AND submitter_x509_cert_sha256_fingerprint = $submitter_x509_cert_sha256_fingerprint
ORDER BY submitted_on DESC
LIMIT 1
""")


_q_check_email_enrolled = Q("""
SELECT TRUE
FROM user_
WHERE
    organization = $organization_internal_id
    AND human = (
        SELECT human._id
        FROM human
        WHERE
            human.organization = $organization_internal_id
            AND human.email = $email
        LIMIT 1
    )
    AND revoked_on IS NULL
LIMIT 1
""")


_q_cancel_previous_enrollment = Q("""
UPDATE pki_enrollment
SET
    enrollment_state = 'CANCELLED',
    info_cancelled.cancelled_on = $now
WHERE
    _id = $previous_enrollment_internal_id
""")


_q_insert_enrollment = Q("""
INSERT INTO pki_enrollment (
    organization,
    enrollment_id,
    submitter_x509_cert_sha256_fingerprint,
    submit_payload_signature,
    submit_payload_signature_algorithm,
    submit_payload,
    submitted_on,
    enrollment_state
)
VALUES (
    $organization_internal_id,
    $enrollment_id,
    $submitter_x509_cert_sha256_fingerprint,
    $submit_payload_signature,
    $submit_payload_signature_algorithm,
    $submit_payload,
    $submitted_on,
    'SUBMITTED'
)
""")


_q_lock = Q(f"""
SELECT PG_ADVISORY_XACT_LOCK({AdvisoryLock.PKIEnrollmentCreation}, _id)
FROM organization
WHERE organization_id = $organization_id
""")


async def q_take_invitation_create_write_lock(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> None:
    """
    Only a single active PKI enrollment for a given x509 certificate is allowed.

    However we cannot enforce this purely in PostgreSQL (e.g. with a unique index)
    since PKI enrollments are never removed but only marked cancelled/removed/accepted.

    So the easy way to solve this is to get rid of the concurrency altogether
    (considering PKI enrollment creation is far from being performance intensive !)
    by requesting a per-organization PostgreSQL Advisory Lock to be held before
    the PKI enrollment creation procedure starts any check.
    """
    await conn.execute(*_q_lock(organization_id=organization_id.str))


async def pki_submit(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    enrollment_id: PKIEnrollmentID,
    force: bool,
    submitter_human_handle: HumanHandle,
    submitter_trustchain: list[PkiCertificate],
    submit_payload_signature: bytes,
    submit_payload_signature_algorithm: PkiSignatureAlgorithm,
    submit_payload: bytes,
    config: BackendConfig,
) -> None | PkiEnrollmentSubmitBadOutcome | PkiEnrollmentSubmitX509CertificateAlreadySubmitted:
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
            return PkiEnrollmentSubmitBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, org_row

    match org_row["is_expired"]:
        case False:
            pass
        case True:
            return PkiEnrollmentSubmitBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, org_row

    # 2) Validate payload

    try:
        load_submit_payload(
            SignedMessage(
                submit_payload_signature_algorithm, submit_payload_signature, submit_payload
            ),
            submitter_trustchain[0].content,
            list(map(lambda v: v.content, submitter_trustchain[1:])),
            config.x509_trust_anchor,
            now,
        )
    except PkiUntrusted:
        return PkiEnrollmentSubmitBadOutcome.INVALID_X509_TRUSTCHAIN
    except PkiInvalidCertificateDER:
        return PkiEnrollmentSubmitBadOutcome.INVALID_DER_X509_CERTIFICATE
    except PkiInvalidSignature:
        return PkiEnrollmentSubmitBadOutcome.INVALID_PAYLOAD_SIGNATURE
    except ValueError:
        return PkiEnrollmentSubmitBadOutcome.INVALID_SUBMIT_PAYLOAD
    submitter_email = str(submitter_human_handle.email)

    # 3) Take lock to prevent any concurrent PKI enrollment creation
    # We also take the common topic lock since PKI enrollment is not
    # allowed for existing users!

    await q_take_invitation_create_write_lock(conn, organization_id)
    await lock_common_read(conn, organization_internal_id)

    # 4) Check enrollment_id is not already used

    enrollment_exists = await conn.fetchrow(
        *_q_check_enrollment_id(
            organization_internal_id=organization_internal_id,
            enrollment_id=enrollment_id,
        )
    )

    if enrollment_exists is not None:
        return PkiEnrollmentSubmitBadOutcome.ENROLLMENT_ID_ALREADY_USED

    # 5) Check for previous enrollment with same x509 certificate

    submitter_der_x509_certificate = submitter_trustchain[0]
    previous_enrollment_row = await conn.fetchrow(
        *_q_get_previous_enrollment(
            organization_internal_id=organization_internal_id,
            submitter_x509_cert_sha256_fingerprint=submitter_der_x509_certificate.fingerprint_sha256,
        )
    )

    await save_trustchain(conn, submitter_trustchain)

    if previous_enrollment_row is not None:
        match previous_enrollment_row["enrollment_internal_id"]:
            case int() as previous_enrollment_internal_id:
                pass
            case _:
                assert False, previous_enrollment_row

        match previous_enrollment_row["submitted_on"]:
            case DateTime() as previous_enrollment_submitted_on:
                pass
            case _:
                assert False, previous_enrollment_row

        match previous_enrollment_row["enrollment_state"]:
            case "SUBMITTED":
                # Previous attempt is still pending
                if force:
                    # Cancel the previous enrollment
                    await conn.execute(
                        *_q_cancel_previous_enrollment(
                            previous_enrollment_internal_id=previous_enrollment_internal_id,
                            now=now,
                        )
                    )
                    # Note we don't send a `EventPkiEnrollment` event related
                    # to the cancelled enrollment here.
                    # This is because this function already sends a `EventPkiEnrollment`
                    # no matter what, and the type of event doesn't specify the
                    # enrollment ID as its role is only to inform the client
                    # that something has changed (so that the client knows it
                    # should re-fetch the list of PKI enrollments from the
                    # server).
                else:
                    # Cannot submit, previous enrollment still pending
                    return PkiEnrollmentSubmitX509CertificateAlreadySubmitted(
                        submitted_on=previous_enrollment_submitted_on,
                    )

            case "REJECTED" | "CANCELLED":
                # Previous attempt was unsuccessful, we can submit a new attempt
                pass

            case "ACCEPTED":
                # Previous attempt ended successfully, check if user is revoked

                match previous_enrollment_row["accepted_device_is_revoked"]:
                    case True:
                        pass
                    case False:
                        # User is not revoked, cannot submit new enrollment
                        return PkiEnrollmentSubmitBadOutcome.X509_CERTIFICATE_ALREADY_ENROLLED
                    case _:
                        assert False, previous_enrollment_row

            case _:
                assert False, previous_enrollment_row

    # 6) Check that the email is not already enrolled

    email_enrolled = await conn.fetchrow(
        *_q_check_email_enrolled(
            organization_internal_id=organization_internal_id,
            email=submitter_email,
        )
    )

    if email_enrolled is not None:
        return PkiEnrollmentSubmitBadOutcome.USER_EMAIL_ALREADY_ENROLLED

    # 7) All checks are good, now we do the actual insertion

    await conn.execute(
        *_q_insert_enrollment(
            organization_internal_id=organization_internal_id,
            enrollment_id=enrollment_id,
            submitter_x509_cert_sha256_fingerprint=submitter_der_x509_certificate.fingerprint_sha256,
            submit_payload_signature=submit_payload_signature,
            submit_payload_signature_algorithm=submit_payload_signature_algorithm.str,
            submit_payload=submit_payload,
            submitted_on=now,
        )
    )

    await send_signal(
        conn,
        EventPkiEnrollment(
            organization_id=organization_id,
        ),
    )

    return None
