# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    PkiEnrollmentAnswerPayload,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    UserCertificate,
    UserProfile,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryDevice,
    MemoryPkiEnrollment,
    MemoryPkiEnrollmentInfoAccepted,
    MemoryPkiEnrollmentInfoCancelled,
    MemoryPkiEnrollmentInfoRejected,
    MemoryPkiEnrollmentState,
    MemoryUser,
)
from parsec.components.pki import (
    BasePkiEnrollmentComponent,
    PkiEnrollmentAcceptStoreBadOutcome,
    PkiEnrollmentAcceptValidateBadOutcome,
    PkiEnrollmentInfo,
    PkiEnrollmentInfoAccepted,
    PkiEnrollmentInfoBadOutcome,
    PkiEnrollmentInfoCancelled,
    PkiEnrollmentInfoRejected,
    PkiEnrollmentInfoSubmitted,
    PkiEnrollmentListBadOutcome,
    PkiEnrollmentListItem,
    PkiEnrollmentRejectBadOutcome,
    PkiEnrollmentSubmitBadOutcome,
    PkiEnrollmentSubmitX509CertificateAlreadySubmitted,
    pki_enrollment_accept_validate,
)
from parsec.events import EventCommonCertificate, EventPkiEnrollment
from parsec.locks import AdvisoryLock


class MemoryPkiEnrollmentComponent(BasePkiEnrollmentComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        self._data = data
        self._event_bus = event_bus

    @override
    async def submit(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submit_payload_signature: bytes,
        submit_payload_signature_algorithm: PkiSignatureAlgorithm,
        submit_payload: bytes,
    ) -> None | PkiEnrollmentSubmitBadOutcome | PkiEnrollmentSubmitX509CertificateAlreadySubmitted:
        # 1) Check organization exists and is not expired

        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return PkiEnrollmentSubmitBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return PkiEnrollmentSubmitBadOutcome.ORGANIZATION_EXPIRED

        # 2) Validate payload

        try:
            payload = PkiEnrollmentSubmitPayload.load(submit_payload)
        except ValueError:
            return PkiEnrollmentSubmitBadOutcome.INVALID_SUBMIT_PAYLOAD
        submitter_email = payload.human_handle.email

        # 3) Take lock to prevent any concurrent PKI enrollment creation
        # We also take the common topic lock since PKI enrollment is not
        # allowed for existing users!

        async with (
            org.advisory_lock_exclusive(AdvisoryLock.InvitationCreation),
            org.topics_lock(read=["common"]),
        ):
            # 4) Check enrollment_id is not already used

            if enrollment_id in org.pki_enrollments:
                return PkiEnrollmentSubmitBadOutcome.ENROLLMENT_ID_ALREADY_USED

            # 5) Check for previous enrollment with same x509 certificate

            for enrollment in reversed(
                sorted(org.pki_enrollments.values(), key=lambda x: x.submitted_on)
            ):
                if enrollment.submitter_der_x509_certificate != submitter_der_x509_certificate:
                    continue

                match enrollment.enrollment_state:
                    case MemoryPkiEnrollmentState.SUBMITTED:
                        # Previous attempt is still pending, overwrite it if force flag is set...
                        if force:
                            enrollment.enrollment_state = MemoryPkiEnrollmentState.CANCELLED
                            enrollment.info_cancelled = MemoryPkiEnrollmentInfoCancelled(
                                cancelled_on=now
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
                            # ...otherwise nothing we can do
                            return PkiEnrollmentSubmitX509CertificateAlreadySubmitted(
                                submitted_on=enrollment.submitted_on,
                            )

                    case MemoryPkiEnrollmentState.REJECTED | MemoryPkiEnrollmentState.CANCELLED:
                        # Previous attempt was unsuccessful, so we are clear to submit a new attempt !
                        pass

                    case MemoryPkiEnrollmentState.ACCEPTED:
                        # Previous attempt end successfully, we are not allowed to submit
                        # unless the created user has been revoked
                        assert enrollment.submitter_accepted_user_id is not None
                        assert enrollment.submitter_accepted_device_id is not None
                        user = org.users[enrollment.submitter_accepted_user_id]
                        if not user.is_revoked:
                            return PkiEnrollmentSubmitBadOutcome.X509_CERTIFICATE_ALREADY_ENROLLED

                # There is no need looking for older enrollments given the
                # last one represent the current state of this x509 certificate.
                break

            # 6) Check that the email is not already enrolled

            for user in org.users.values():
                if not user.is_revoked and user.cooked.human_handle.email == submitter_email:
                    return PkiEnrollmentSubmitBadOutcome.USER_EMAIL_ALREADY_ENROLLED

            # 7) All checks are good, now we do the actual insertion

            org.pki_enrollments[enrollment_id] = MemoryPkiEnrollment(
                enrollment_id=enrollment_id,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submit_payload_signature=submit_payload_signature,
                submit_payload_signature_algorithm=submit_payload_signature_algorithm,
                submit_payload=submit_payload,
                submitted_on=now,
            )

            await self._event_bus.send(
                EventPkiEnrollment(
                    organization_id=organization_id,
                )
            )

    @override
    async def info(
        self,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
    ) -> PkiEnrollmentInfo | PkiEnrollmentInfoBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return PkiEnrollmentInfoBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return PkiEnrollmentInfoBadOutcome.ORGANIZATION_EXPIRED

        try:
            enrollment = org.pki_enrollments[enrollment_id]
        except KeyError:
            return PkiEnrollmentInfoBadOutcome.ENROLLMENT_NOT_FOUND

        match enrollment.enrollment_state:
            case MemoryPkiEnrollmentState.SUBMITTED:
                return PkiEnrollmentInfoSubmitted(
                    enrollment_id=enrollment_id,
                    submitted_on=enrollment.submitted_on,
                )
            case MemoryPkiEnrollmentState.ACCEPTED:
                assert enrollment.info_accepted is not None
                return PkiEnrollmentInfoAccepted(
                    enrollment_id=enrollment_id,
                    submitted_on=enrollment.submitted_on,
                    accept_payload=enrollment.info_accepted.accept_payload,
                    accept_payload_signature=enrollment.info_accepted.accept_payload_signature,
                    accept_payload_signature_algorithm=enrollment.info_accepted.accept_payload_signature_algorithm,
                    # TODO: https://github.com/Scille/parsec-cloud/issues/11560
                    accepter_intermediate_der_x509_certificates=[],
                    accepted_on=enrollment.info_accepted.accepted_on,
                    accepter_der_x509_certificate=enrollment.info_accepted.accepter_der_x509_certificate,
                )
            case MemoryPkiEnrollmentState.CANCELLED:
                assert enrollment.info_cancelled is not None
                return PkiEnrollmentInfoCancelled(
                    enrollment_id=enrollment_id,
                    submitted_on=enrollment.submitted_on,
                    cancelled_on=enrollment.info_cancelled.cancelled_on,
                )
            case MemoryPkiEnrollmentState.REJECTED:
                assert enrollment.info_rejected is not None
                return PkiEnrollmentInfoRejected(
                    enrollment_id=enrollment_id,
                    submitted_on=enrollment.submitted_on,
                    rejected_on=enrollment.info_rejected.rejected_on,
                )

    @override
    async def list(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> list[PkiEnrollmentListItem] | PkiEnrollmentListBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return PkiEnrollmentListBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return PkiEnrollmentListBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return PkiEnrollmentListBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return PkiEnrollmentListBadOutcome.AUTHOR_NOT_FOUND

        if author_user.is_revoked:
            return PkiEnrollmentListBadOutcome.AUTHOR_REVOKED

        if author_user.current_profile != UserProfile.ADMIN:
            return PkiEnrollmentListBadOutcome.AUTHOR_NOT_ALLOWED

        return sorted(
            [
                PkiEnrollmentListItem(
                    enrollment_id=enrollment.enrollment_id,
                    payload=enrollment.submit_payload,
                    payload_signature=enrollment.submit_payload_signature,
                    payload_signature_algorithm=enrollment.submit_payload_signature_algorithm,
                    submitted_on=enrollment.submitted_on,
                    der_x509_certificate=enrollment.submitter_der_x509_certificate,
                    # TODO: https://github.com/Scille/parsec-cloud/issues/11558
                    intermediate_der_x509_certificates=[],
                )
                for enrollment in org.pki_enrollments.values()
                if enrollment.enrollment_state == MemoryPkiEnrollmentState.SUBMITTED
            ],
            key=lambda x: x.submitted_on,
        )

    @override
    async def reject(
        self,
        now: DateTime,
        author: DeviceID,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
    ) -> None | PkiEnrollmentRejectBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return PkiEnrollmentRejectBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return PkiEnrollmentRejectBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]):
            try:
                author_device = org.devices[author]
            except KeyError:
                return PkiEnrollmentRejectBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return PkiEnrollmentRejectBadOutcome.AUTHOR_REVOKED
            if author_user.current_profile != UserProfile.ADMIN:
                return PkiEnrollmentRejectBadOutcome.AUTHOR_NOT_ALLOWED

            try:
                enrollment = org.pki_enrollments[enrollment_id]
            except KeyError:
                return PkiEnrollmentRejectBadOutcome.ENROLLMENT_NOT_FOUND

            if enrollment.enrollment_state != MemoryPkiEnrollmentState.SUBMITTED:
                return PkiEnrollmentRejectBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

            # All checks are good, now we do the actual insertion

            enrollment.enrollment_state = MemoryPkiEnrollmentState.REJECTED
            enrollment.info_rejected = MemoryPkiEnrollmentInfoRejected(
                rejected_on=now,
            )

            await self._event_bus.send(
                EventPkiEnrollment(
                    organization_id=organization_id,
                )
            )

    @override
    async def accept(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        enrollment_id: PKIEnrollmentID,
        payload: bytes,
        payload_signature: bytes,
        payload_signature_algorithm: PkiSignatureAlgorithm,
        accepter_der_x509_certificate: bytes,
        accepter_intermediate_der_x509_certificates: list[bytes],
        submitter_user_certificate: bytes,
        submitter_redacted_user_certificate: bytes,
        submitter_device_certificate: bytes,
        submitter_redacted_device_certificate: bytes,
    ) -> (
        tuple[UserCertificate, DeviceCertificate]
        | PkiEnrollmentAcceptValidateBadOutcome
        | PkiEnrollmentAcceptStoreBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return PkiEnrollmentAcceptStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return PkiEnrollmentAcceptStoreBadOutcome.ORGANIZATION_EXPIRED

        # 1) Write lock common topic

        async with org.topics_lock(write=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return PkiEnrollmentAcceptStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return PkiEnrollmentAcceptStoreBadOutcome.AUTHOR_REVOKED

            if author_user.current_profile != UserProfile.ADMIN:
                return PkiEnrollmentAcceptStoreBadOutcome.AUTHOR_NOT_ALLOWED

            # 2) Validate certificates

            try:
                PkiEnrollmentAnswerPayload.load(payload)
            except ValueError:
                return PkiEnrollmentAcceptStoreBadOutcome.INVALID_ACCEPT_PAYLOAD

            match pki_enrollment_accept_validate(
                now=now,
                expected_author=author,
                author_verify_key=author_verify_key,
                user_certificate=submitter_user_certificate,
                device_certificate=submitter_device_certificate,
                redacted_user_certificate=submitter_redacted_user_certificate,
                redacted_device_certificate=submitter_redacted_device_certificate,
            ):
                case (u_certif, d_certif):
                    pass
                case error:
                    return error

            # 3) Ensure we are not breaking causality by adding a newer timestamp.

            # We already ensured user and device certificates' timestamps are consistent,
            # so only need to check one of them here
            if common_topic_last_timestamp >= u_certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=common_topic_last_timestamp)

            # 4) Retrieve the enrollment

            try:
                enrollment = org.pki_enrollments[enrollment_id]
            except KeyError:
                return PkiEnrollmentAcceptStoreBadOutcome.ENROLLMENT_NOT_FOUND

            if enrollment.enrollment_state != MemoryPkiEnrollmentState.SUBMITTED:
                return PkiEnrollmentAcceptStoreBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

            # 5) Check the user_id/device_id don't already exists and human_handle
            # is not already taken

            if org.active_user_limit_reached():
                return PkiEnrollmentAcceptStoreBadOutcome.ACTIVE_USERS_LIMIT_REACHED

            if u_certif.user_id in org.users:
                return PkiEnrollmentAcceptStoreBadOutcome.USER_ALREADY_EXISTS
            assert d_certif.device_id not in org.devices

            if any(
                True for u in org.active_users() if u.cooked.human_handle == u_certif.human_handle
            ):
                return PkiEnrollmentAcceptStoreBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN

            # 6) All checks are good, now we do the actual insertion

            org.per_topic_last_timestamp["common"] = u_certif.timestamp

            org.users[u_certif.user_id] = MemoryUser(
                cooked=u_certif,
                user_certificate=submitter_user_certificate,
                redacted_user_certificate=submitter_redacted_user_certificate,
            )

            # Sanity check, should never occurs given user doesn't exist yet !
            assert d_certif.device_id not in org.devices
            org.devices[d_certif.device_id] = MemoryDevice(
                cooked=d_certif,
                device_certificate=submitter_device_certificate,
                redacted_device_certificate=submitter_redacted_device_certificate,
            )

            enrollment.enrollment_state = MemoryPkiEnrollmentState.ACCEPTED
            enrollment.accepter = author
            enrollment.submitter_accepted_user_id = d_certif.user_id
            enrollment.submitter_accepted_device_id = d_certif.device_id
            enrollment.info_accepted = MemoryPkiEnrollmentInfoAccepted(
                accepted_on=now,
                accept_payload=payload,
                accept_payload_signature=payload_signature,
                accept_payload_signature_algorithm=payload_signature_algorithm,
                accepter_der_x509_certificate=accepter_der_x509_certificate,
            )

            await self._event_bus.send(
                EventCommonCertificate(
                    organization_id=organization_id, timestamp=u_certif.timestamp
                )
            )

            await self._event_bus.send(
                EventPkiEnrollment(
                    organization_id=organization_id,
                )
            )

            return u_certif, d_certif
