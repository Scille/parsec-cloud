# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

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
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentAcceptBadOutcome,
    AsyncEnrollmentAcceptValidateBadOutcome,
    AsyncEnrollmentEmailAlreadySubmitted,
    AsyncEnrollmentInfo,
    AsyncEnrollmentInfoAccepted,
    AsyncEnrollmentInfoBadOutcome,
    AsyncEnrollmentInfoCancelled,
    AsyncEnrollmentInfoRejected,
    AsyncEnrollmentInfoSubmitted,
    AsyncEnrollmentListBadOutcome,
    AsyncEnrollmentListItem,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
    AsyncEnrollmentRejectBadOutcome,
    AsyncEnrollmentSubmitBadOutcome,
    AsyncEnrollmentSubmitValidateBadOutcome,
    BaseAsyncEnrollmentComponent,
    async_enrollment_accept_validate,
    async_enrollment_submit_validate,
)
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryAsyncEnrollment,
    MemoryAsyncEnrollmentState,
    MemoryDatamodel,
    MemoryDevice,
    MemoryUser,
)
from parsec.events import EventAsyncEnrollment, EventCommonCertificate
from parsec.locks import AdvisoryLock


class MemoryAsyncEnrollmentComponent(BaseAsyncEnrollmentComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        event_bus: EventBus,
    ) -> None:
        self._data = data
        self._event_bus = event_bus

    @override
    async def submit(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AsyncEnrollmentSubmitBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return AsyncEnrollmentSubmitBadOutcome.ORGANIZATION_EXPIRED

        match async_enrollment_submit_validate(
            now=now,
            submit_payload=submit_payload,
            submit_payload_signature=submit_payload_signature,
        ):
            case (s_payload, _):
                pass
            case error:
                return error

        async with (
            org.topics_lock(read=["common"]),
            org.advisory_lock_exclusive(AdvisoryLock.AsyncEnrollmentCreation),
        ):
            email = s_payload.requested_human_handle.email

            if any(u.cooked.human_handle.email == email for u in org.active_users()):
                return AsyncEnrollmentSubmitBadOutcome.EMAIL_ALREADY_ENROLLED

            if enrollment_id in org.async_enrollments:
                return AsyncEnrollmentSubmitBadOutcome.ID_ALREADY_USED

            for enrollment in org.async_enrollments.values():
                if enrollment.state != MemoryAsyncEnrollmentState.SUBMITTED:
                    continue

                if enrollment.cooked_submit_payload.requested_human_handle.email == email:
                    if force:
                        enrollment.state = MemoryAsyncEnrollmentState.CANCELLED
                        enrollment.cancelled_on = now
                    else:
                        return AsyncEnrollmentEmailAlreadySubmitted(
                            submitted_on=enrollment.submitted_on
                        )

            # All checks are good, now we do the actual insertion

            org.async_enrollments[enrollment_id] = MemoryAsyncEnrollment(
                id=enrollment_id,
                submitted_on=now,
                submit_payload=submit_payload,
                cooked_submit_payload=s_payload,
                submit_payload_signature=submit_payload_signature,
            )

            await self._event_bus.send(EventAsyncEnrollment(organization_id=organization_id))

    @override
    async def info(
        self,
        organization_id: OrganizationID,
        enrollment_id: AsyncEnrollmentID,
    ) -> AsyncEnrollmentInfo | AsyncEnrollmentInfoBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AsyncEnrollmentInfoBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return AsyncEnrollmentInfoBadOutcome.ORGANIZATION_EXPIRED

        try:
            enrollment = org.async_enrollments[enrollment_id]
        except KeyError:
            return AsyncEnrollmentInfoBadOutcome.ENROLLMENT_NOT_FOUND

        match enrollment.state:
            case MemoryAsyncEnrollmentState.ACCEPTED:
                assert enrollment.accepted_on is not None
                assert enrollment.accept_payload is not None
                assert enrollment.accept_payload_signature is not None
                return AsyncEnrollmentInfoAccepted(
                    submitted_on=enrollment.submitted_on,
                    accepted_on=enrollment.accepted_on,
                    accept_payload=enrollment.accept_payload,
                    accept_payload_signature=enrollment.accept_payload_signature,
                )

            case MemoryAsyncEnrollmentState.SUBMITTED:
                return AsyncEnrollmentInfoSubmitted(submitted_on=enrollment.submitted_on)

            case MemoryAsyncEnrollmentState.REJECTED:
                assert enrollment.rejected_on is not None
                return AsyncEnrollmentInfoRejected(
                    submitted_on=enrollment.submitted_on,
                    rejected_on=enrollment.rejected_on,
                )

            case MemoryAsyncEnrollmentState.CANCELLED:
                assert enrollment.cancelled_on is not None
                return AsyncEnrollmentInfoCancelled(
                    submitted_on=enrollment.submitted_on,
                    cancelled_on=enrollment.cancelled_on,
                )

    @override
    async def list(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> list[AsyncEnrollmentListItem] | AsyncEnrollmentListBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AsyncEnrollmentListBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return AsyncEnrollmentListBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return AsyncEnrollmentListBadOutcome.AUTHOR_NOT_FOUND

        author_user = org.users[author_device.cooked.user_id]
        if author_user.is_revoked:
            return AsyncEnrollmentListBadOutcome.AUTHOR_REVOKED
        if author_user.current_profile != UserProfile.ADMIN:
            return AsyncEnrollmentListBadOutcome.AUTHOR_NOT_ALLOWED

        return sorted(
            [
                AsyncEnrollmentListItem(
                    enrollment_id=enrollment.id,
                    submitted_on=enrollment.submitted_on,
                    submit_payload=enrollment.submit_payload,
                    submit_payload_signature=enrollment.submit_payload_signature,
                )
                for enrollment in org.async_enrollments.values()
                if enrollment.state == MemoryAsyncEnrollmentState.SUBMITTED
            ],
            key=lambda e: e.submitted_on,
        )

    @override
    async def reject(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        enrollment_id: AsyncEnrollmentID,
    ) -> None | AsyncEnrollmentRejectBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AsyncEnrollmentRejectBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return AsyncEnrollmentRejectBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(write=["common"]):
            try:
                author_device = org.devices[author]
            except KeyError:
                return AsyncEnrollmentRejectBadOutcome.AUTHOR_NOT_FOUND

            author_user = org.users[author_device.cooked.user_id]
            if author_user.is_revoked:
                return AsyncEnrollmentRejectBadOutcome.AUTHOR_REVOKED
            if author_user.current_profile != UserProfile.ADMIN:
                return AsyncEnrollmentRejectBadOutcome.AUTHOR_NOT_ALLOWED

            try:
                enrollment = org.async_enrollments[enrollment_id]
            except KeyError:
                return AsyncEnrollmentRejectBadOutcome.ENROLLMENT_NOT_FOUND

            if enrollment.state != MemoryAsyncEnrollmentState.SUBMITTED:
                return AsyncEnrollmentRejectBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

            enrollment.state = MemoryAsyncEnrollmentState.REJECTED
            enrollment.rejected_on = now

            await self._event_bus.send(EventAsyncEnrollment(organization_id=organization_id))

    @override
    async def accept(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AsyncEnrollmentAcceptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return AsyncEnrollmentAcceptBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(write=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return AsyncEnrollmentAcceptBadOutcome.AUTHOR_NOT_FOUND

            author_user = org.users[author_device.cooked.user_id]
            if author_user.is_revoked:
                return AsyncEnrollmentAcceptBadOutcome.AUTHOR_REVOKED
            if author_user.current_profile != UserProfile.ADMIN:
                return AsyncEnrollmentAcceptBadOutcome.AUTHOR_NOT_ALLOWED

            try:
                enrollment = org.async_enrollments[enrollment_id]
            except KeyError:
                return AsyncEnrollmentAcceptBadOutcome.ENROLLMENT_NOT_FOUND

            if enrollment.state != MemoryAsyncEnrollmentState.SUBMITTED:
                return AsyncEnrollmentAcceptBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

            match (enrollment.submit_payload_signature, accept_payload_signature):
                case (
                    AsyncEnrollmentPayloadSignaturePKI(),
                    AsyncEnrollmentPayloadSignaturePKI(),
                ) | (
                    AsyncEnrollmentPayloadSignatureOpenBao(),
                    AsyncEnrollmentPayloadSignatureOpenBao(),
                ):
                    pass
                case _:
                    return (
                        AsyncEnrollmentAcceptBadOutcome.SUBMIT_AND_ACCEPT_IDENTITY_SYSTEMS_MISMATCH
                    )

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
                case (a_payload, u_certif, d_certif, _):
                    pass
                case error:
                    return error

            if org.active_user_limit_reached():
                return AsyncEnrollmentAcceptBadOutcome.ACTIVE_USERS_LIMIT_REACHED

            if u_certif.user_id in org.users:
                return AsyncEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS

            if d_certif.device_id in org.devices:
                return AsyncEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS

            if any(
                True for u in org.active_users() if u.cooked.human_handle == u_certif.human_handle
            ):
                return AsyncEnrollmentAcceptBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN

            # Ensure we are not breaking causality by adding a newer timestamp.

            # We already ensured user and device certificates' timestamps are consistent,
            # so only need to check one of them here
            if common_topic_last_timestamp >= u_certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=common_topic_last_timestamp)

            # All checks are good, now we do the actual insertion

            enrollment.state = MemoryAsyncEnrollmentState.ACCEPTED
            enrollment.accepted_on = now
            enrollment.accept_payload = accept_payload
            enrollment.cooked_accept_payload = a_payload
            enrollment.accept_payload_signature = accept_payload_signature

            org.per_topic_last_timestamp["common"] = u_certif.timestamp

            org.users[u_certif.user_id] = MemoryUser(
                cooked=u_certif,
                user_certificate=submitter_user_certificate,
                redacted_user_certificate=submitter_redacted_user_certificate,
            )

            org.devices[d_certif.device_id] = MemoryDevice(
                cooked=d_certif,
                device_certificate=submitter_device_certificate,
                redacted_device_certificate=submitter_redacted_device_certificate,
            )

            await self._event_bus.send(
                EventCommonCertificate(
                    organization_id=organization_id, timestamp=u_certif.timestamp
                )
            )

            await self._event_bus.send(EventAsyncEnrollment(organization_id=organization_id))

            return u_certif, d_certif
