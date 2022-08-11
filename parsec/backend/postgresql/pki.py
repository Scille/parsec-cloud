# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import hashlib
from typing import List
from uuid import UUID
from asyncpg import UniqueViolationError
from parsec._parsec import DateTime

from parsec.api.protocol import OrganizationID
from parsec.api.protocol.pki import PkiEnrollmentStatus
from parsec.api.protocol.types import UserID, UserProfile
from parsec.backend.backend_events import BackendEvent
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.user import UserActiveUsersLimitReached, UserAlreadyExistsError
from parsec.backend.user_type import User, Device
from parsec.backend.pki import (
    PkiEnrollementEmailAlreadyUsedError,
    PkiEnrollmentActiveUsersLimitReached,
    PkiEnrollmentAlreadyEnrolledError,
    PkiEnrollmentAlreadyExistError,
    PkiEnrollmentCertificateAlreadySubmittedError,
    PkiEnrollmentError,
    PkiEnrollmentIdAlreadyUsedError,
    PkiEnrollmentInfo,
    PkiEnrollmentInfoAccepted,
    PkiEnrollmentInfoCancelled,
    PkiEnrollmentInfoRejected,
    PkiEnrollmentInfoSubmitted,
    PkiEnrollmentListItem,
    BasePkiEnrollmentComponent,
    PkiEnrollmentNoLongerAvailableError,
    PkiEnrollmentNotFoundError,
)
from parsec.backend.postgresql import PGHandler
from parsec.backend.postgresql.utils import Q, q_organization_internal_id, q_device_internal_id
from parsec.backend.postgresql.user_queries.create import (
    q_create_user,
    q_take_user_device_write_lock,
)

_q_get_last_pki_enrollment_from_certificate_sha1_for_update = Q(
    f"""
    SELECT
        enrollment_id,
        enrollment_state,
        submitted_on,
        submitter_accepted_device,
        accepter
    FROM
        pki_enrollment
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND submitter_der_x509_certificate_sha1=$submitter_der_x509_certificate_sha1
    )
    ORDER BY _id DESC LIMIT 1
    FOR UPDATE
    """
)

_q_get_pki_enrollment_from_enrollment_id = Q(
    f"""
    SELECT
        enrollment_id,
        enrollment_state,
        submitted_on,
        info_cancelled,
        info_accepted,
        info_rejected
    FROM
        pki_enrollment
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND enrollment_id=$enrollment_id
    )
    """
)

_q_get_pki_enrollment_for_update = Q(
    f"""
    SELECT
        enrollment_id,
        enrollment_state,
        submitted_on,
        accepter,
        submitter_accepted_device
    FROM
        pki_enrollment
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND enrollment_id=$enrollment_id
    )
    FOR UPDATE
    """
)

_q_get_pki_enrollment_from_state = Q(
    f"""
    SELECT
        enrollment_id,
        submitted_on,
        submitter_der_x509_certificate,
        submit_payload_signature,
        submit_payload
    FROM
        pki_enrollment
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND enrollment_state=$state
    )
    ORDER BY _id ASC
    """
)


_q_submit_pki_enrollment = Q(
    f"""
    INSERT INTO pki_enrollment(
        organization,
        enrollment_id,
        submitter_der_x509_certificate,
        submitter_der_x509_certificate_sha1,
        submit_payload_signature,
        submit_payload,
        enrollment_state,
        submitted_on
    )
    VALUES(
        { q_organization_internal_id("$organization_id") },
        $enrollment_id,
        $submitter_der_x509_certificate,
        $submitter_der_x509_certificate_sha1,
        $submit_payload_signature,
        $submit_payload,
        $enrollment_state,
        $submitted_on
    )

    """
)

_q_cancel_pki_enrollment = Q(
    f"""
    UPDATE pki_enrollment
    SET
        enrollment_state=$enrollment_state,
        info_cancelled.cancelled_on=$cancelled_on
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND enrollment_id=$enrollment_id
    )
    """
)


_q_reject_pki_enrollment = Q(
    f"""
    UPDATE pki_enrollment
    SET
        enrollment_state=$enrollment_state,
        info_rejected.rejected_on=$rejected_on
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND enrollment_id=$enrollment_id
    )
    """
)


_q_accept_pki_enrollment = Q(
    f"""
    UPDATE pki_enrollment
    SET
        enrollment_state=$enrollment_state,
        info_accepted.accepted_on=$accepted_on,
        info_accepted.accepter_der_x509_certificate=$accepter_der_x509_certificate,
        info_accepted.accept_payload_signature=$accept_payload_signature,
        info_accepted.accept_payload=$accept_payload,
        accepter={q_device_internal_id(organization_id="$organization_id", device_id="$accepter")},
        submitter_accepted_device={q_device_internal_id(organization_id="$organization_id", device_id="$accepted")}
    WHERE (
        organization = { q_organization_internal_id("$organization_id") }
        AND enrollment_id=$enrollment_id
    )
    """
)


_q_get_user_from_device_id = Q(
    f"""
    SELECT *
    FROM user_
    WHERE
        user_.organization = { q_organization_internal_id("$organization_id") }
        AND user_._id=(
            SELECT user_
            FROM device
            WHERE device._id=$device_id
            AND device.organization = { q_organization_internal_id("$organization_id") }
        )
    FOR UPDATE
    LIMIT 1
    """
)
_q_retrieve_active_human_by_email_for_update = Q(
    f"""
    SELECT
        user_.user_id
    FROM user_ LEFT JOIN human ON user_.human=human._id
    WHERE
        user_.organization = { q_organization_internal_id("$organization_id") }
        AND human.email = $email
        AND user_.revoked_on IS NULL
    FOR UPDATE
    LIMIT 1
"""
)


def _build_enrollment_info(entry) -> PkiEnrollmentInfo:
    if entry["enrollment_state"] == PkiEnrollmentStatus.SUBMITTED.value:
        return PkiEnrollmentInfoSubmitted(
            enrollment_id=entry["enrollment_id"], submitted_on=entry["submitted_on"]
        )
    elif entry["enrollment_state"] == PkiEnrollmentStatus.CANCELLED.value:
        return PkiEnrollmentInfoCancelled(
            enrollment_id=entry["enrollment_id"],
            submitted_on=entry["submitted_on"],
            cancelled_on=entry["info_cancelled"]["cancelled_on"],
        )
    elif entry["enrollment_state"] == PkiEnrollmentStatus.REJECTED.value:
        return PkiEnrollmentInfoRejected(
            enrollment_id=entry["enrollment_id"],
            submitted_on=entry["submitted_on"],
            rejected_on=entry["info_rejected"]["rejected_on"],
        )
    elif entry["enrollment_state"] == PkiEnrollmentStatus.ACCEPTED.value:
        return PkiEnrollmentInfoAccepted(
            enrollment_id=entry["enrollment_id"],
            submitted_on=entry["submitted_on"],
            accepted_on=entry["info_accepted"]["accepted_on"],
            accepter_der_x509_certificate=entry["info_accepted"]["accepter_der_x509_certificate"],
            accept_payload_signature=entry["info_accepted"]["accept_payload_signature"],
            accept_payload=entry["info_accepted"]["accept_payload"],
        )
    else:
        assert False


class PGPkiEnrollmentComponent(BasePkiEnrollmentComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def submit(
        self,
        organization_id: OrganizationID,
        enrollment_id: UUID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submitter_der_x509_certificate_email: str,
        submit_payload_signature: bytes,
        submit_payload: bytes,
        submitted_on: DateTime,
    ) -> None:
        """
        Raises:
            PkiEnrollmentCertificateAlreadySubmittedError
            PkiEnrollmentAlreadyEnrolledError
            PkiEnrollmentIdAlreadyUsedError
        """
        submitter_der_x509_certificate_sha1 = hashlib.sha1(submitter_der_x509_certificate).digest()
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            # Hold the user/device write lock before any check in the enrollment
            # table to ensure it is going to be done without concurrency issues
            await q_take_user_device_write_lock(conn, organization_id)

            # Assert enrollment_id not used
            row = await conn.fetchrow(
                *_q_get_pki_enrollment_for_update(
                    organization_id=organization_id.str, enrollment_id=enrollment_id
                )
            )
            if row:
                raise PkiEnrollmentIdAlreadyUsedError()

            # Try to retrieve the last attempt with this x509 certificate
            row = await conn.fetchrow(
                *_q_get_last_pki_enrollment_from_certificate_sha1_for_update(
                    organization_id=organization_id.str,
                    submitter_der_x509_certificate_sha1=submitter_der_x509_certificate_sha1,
                )
            )
            if row:
                enrollment_state = row["enrollment_state"]
                if enrollment_state == PkiEnrollmentStatus.SUBMITTED.value:
                    if force:
                        await conn.execute(
                            *_q_cancel_pki_enrollment(
                                organization_id=organization_id.str,
                                enrollment_id=row["enrollment_id"],
                                enrollment_state=PkiEnrollmentStatus.CANCELLED.value,
                                cancelled_on=submitted_on,
                            )
                        )
                        await send_signal(
                            conn,
                            BackendEvent.PKI_ENROLLMENTS_UPDATED,
                            organization_id=organization_id,
                        )

                    else:
                        raise PkiEnrollmentCertificateAlreadySubmittedError(
                            submitted_on=row["submitted_on"]
                        )
                elif enrollment_state in [
                    PkiEnrollmentStatus.REJECTED.value,
                    PkiEnrollmentStatus.CANCELLED.value,
                ]:
                    # Previous attempt was unsuccessful, so we are clear to submit a new attempt !
                    pass
                elif enrollment_state == PkiEnrollmentStatus.ACCEPTED.value:
                    # Previous attempt end successfully, we are not allowed to submit
                    # unless the created user has been revoked
                    assert (
                        row["submitter_accepted_device"] is not None and row["accepter"] is not None
                    )

                    row = await conn.fetchrow(
                        *_q_get_user_from_device_id(
                            organization_id=organization_id.str,
                            device_id=row["submitter_accepted_device"],
                        )
                    )
                    user = User(
                        user_id=UserID(row["user_id"]),
                        human_handle=None,
                        profile=UserProfile(row["profile"]),
                        user_certificate=row["user_certificate"],
                        redacted_user_certificate=row["redacted_user_certificate"],
                        user_certifier=None,
                        created_on=row["created_on"],
                        revoked_on=row["revoked_on"] if row["revoked_on"] else None,
                        revoked_user_certificate=row["revoked_user_certificate"],
                        revoked_user_certifier=None,
                    )
                    if not user.is_revoked():
                        raise PkiEnrollmentAlreadyEnrolledError(submitted_on)
                else:
                    assert False

            # Optional check for client compatibility with version < 2.8.3
            if submitter_der_x509_certificate_email:
                # Assert email not already used by active human
                row = await conn.fetchrow(
                    *_q_retrieve_active_human_by_email_for_update(
                        organization_id=organization_id.str,
                        email=submitter_der_x509_certificate_email,
                    )
                )
                if row:
                    raise PkiEnrollementEmailAlreadyUsedError()

            try:
                result = await conn.execute(
                    *_q_submit_pki_enrollment(
                        organization_id=organization_id.str,
                        enrollment_id=enrollment_id,
                        submitter_der_x509_certificate=submitter_der_x509_certificate,
                        submitter_der_x509_certificate_sha1=submitter_der_x509_certificate_sha1,
                        submit_payload_signature=submit_payload_signature,
                        submit_payload=submit_payload,
                        enrollment_state=PkiEnrollmentStatus.SUBMITTED.value,
                        submitted_on=submitted_on,
                    )
                )
            except UniqueViolationError:
                raise PkiEnrollmentIdAlreadyUsedError()

            if result != "INSERT 0 1":
                raise PkiEnrollmentError(f"Insertion error: {result}")

            await send_signal(
                conn, BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
            )

    async def info(self, organization_id: OrganizationID, enrollment_id: UUID) -> PkiEnrollmentInfo:
        """
        Raises:
            PkiEnrollmentNotFoundError
        """
        async with self.dbh.pool.acquire() as conn:
            row = await conn.fetchrow(
                *_q_get_pki_enrollment_from_enrollment_id(
                    organization_id=organization_id.str, enrollment_id=enrollment_id
                )
            )
            if not row:
                raise PkiEnrollmentNotFoundError()
            else:
                return _build_enrollment_info(row)

    async def list(self, organization_id: OrganizationID) -> List[PkiEnrollmentListItem]:
        """
        Raises: Nothing !
        """

        async with self.dbh.pool.acquire() as conn:

            entries = await conn.fetch(
                *_q_get_pki_enrollment_from_state(
                    organization_id=organization_id.str, state=PkiEnrollmentStatus.SUBMITTED.value
                )
            )
            return [
                PkiEnrollmentListItem(
                    enrollment_id=entry["enrollment_id"],
                    submitted_on=entry["submitted_on"],
                    submitter_der_x509_certificate=entry["submitter_der_x509_certificate"],
                    submit_payload_signature=entry["submit_payload_signature"],
                    submit_payload=entry["submit_payload"],
                )
                for entry in entries
            ]

    async def reject(
        self, organization_id: OrganizationID, enrollment_id: UUID, rejected_on: DateTime
    ) -> None:
        """
        Raises:
            PkiEnrollmentNotFoundError
            PkiEnrollmentNoLongerAvailableError
        """

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            # Enrollment submit depend on the data we are going to modify,
            # hence by transitivity we also should hold the user/device write lock
            await q_take_user_device_write_lock(conn, organization_id)

            row = await conn.fetchrow(
                *_q_get_pki_enrollment_for_update(
                    organization_id=organization_id.str, enrollment_id=enrollment_id
                )
            )
            if not row:
                raise PkiEnrollmentNotFoundError()
            if row["enrollment_state"] != PkiEnrollmentStatus.SUBMITTED.value:
                raise PkiEnrollmentNoLongerAvailableError()

            await conn.execute(
                *_q_reject_pki_enrollment(
                    organization_id=organization_id.str,
                    enrollment_id=enrollment_id,
                    enrollment_state=PkiEnrollmentStatus.REJECTED.value,
                    rejected_on=rejected_on,
                )
            )
            await send_signal(
                conn, BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
            )

    async def accept(
        self,
        organization_id: OrganizationID,
        enrollment_id: UUID,
        accepter_der_x509_certificate: bytes,
        accept_payload_signature: bytes,
        accept_payload: bytes,
        accepted_on: DateTime,
        user: User,
        first_device: Device,
    ) -> None:
        """
        Raises:
            PkiEnrollmentNotFoundError
            PkiEnrollmentNoLongerAvailableError
            PkiEnrollmentAlreadyExistError
            PkiEnrollmentActiveUsersLimitReached
        """
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            # Enrollment submit depend on the data we are going to modify,
            # hence by transitivity we also should hold the user/device write lock
            await q_take_user_device_write_lock(conn, organization_id)

            row = await conn.fetchrow(
                *_q_get_pki_enrollment_for_update(
                    organization_id=organization_id.str, enrollment_id=enrollment_id
                )
            )
            if not row:
                raise PkiEnrollmentNotFoundError()
            if row["enrollment_state"] != PkiEnrollmentStatus.SUBMITTED.value:
                raise PkiEnrollmentNoLongerAvailableError()

            try:
                await q_create_user(
                    conn=conn,
                    organization_id=organization_id,
                    user=user,
                    first_device=first_device,
                    lock_already_held=True,
                )

            except UserAlreadyExistsError as exc:
                raise PkiEnrollmentAlreadyExistError from exc

            except UserActiveUsersLimitReached as exc:
                raise PkiEnrollmentActiveUsersLimitReached from exc

            # Only the user created during organization bootstrap has no certifier
            assert user.user_certifier is not None
            await conn.execute(
                *_q_accept_pki_enrollment(
                    enrollment_state=PkiEnrollmentStatus.ACCEPTED.value,
                    organization_id=organization_id.str,
                    enrollment_id=enrollment_id,
                    accepted_on=accepted_on,
                    accepter_der_x509_certificate=accepter_der_x509_certificate,
                    accept_payload_signature=accept_payload_signature,
                    accept_payload=accept_payload,
                    accepter=user.user_certifier.str,
                    accepted=first_device.device_id.str,
                )
            )
            await send_signal(
                conn, BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
            )
