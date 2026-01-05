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
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.async_enrollment import (
    AsyncEnrollmentAcceptBadOutcome,
    AsyncEnrollmentAcceptValidateBadOutcome,
    AsyncEnrollmentEmailAlreadySubmitted,
    AsyncEnrollmentInfo,
    AsyncEnrollmentInfoBadOutcome,
    AsyncEnrollmentListBadOutcome,
    AsyncEnrollmentListItem,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentRejectBadOutcome,
    AsyncEnrollmentSubmitBadOutcome,
    AsyncEnrollmentSubmitValidateBadOutcome,
    BaseAsyncEnrollmentComponent,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.async_enrollment_accept import async_enrollment_accept
from parsec.components.postgresql.async_enrollment_info import async_enrollment_info
from parsec.components.postgresql.async_enrollment_list import async_enrollment_list
from parsec.components.postgresql.async_enrollment_reject import async_enrollment_reject
from parsec.components.postgresql.async_enrollment_submit import async_enrollment_submit
from parsec.components.postgresql.utils import no_transaction, transaction


class PGAsyncEnrollmentComponent(BaseAsyncEnrollmentComponent):
    def __init__(self, pool: AsyncpgPool) -> None:
        self.pool = pool

    @override
    @transaction
    async def submit(
        self,
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
        return await async_enrollment_submit(
            conn,
            now,
            organization_id,
            enrollment_id,
            force,
            submit_payload,
            submit_payload_signature,
        )

    @override
    @no_transaction
    async def info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        enrollment_id: AsyncEnrollmentID,
    ) -> AsyncEnrollmentInfo | AsyncEnrollmentInfoBadOutcome:
        return await async_enrollment_info(
            conn,
            organization_id,
            enrollment_id,
        )

    @override
    @no_transaction
    async def list(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> list[AsyncEnrollmentListItem] | AsyncEnrollmentListBadOutcome:
        return await async_enrollment_list(
            conn,
            organization_id,
            author,
        )

    @override
    @transaction
    async def reject(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        enrollment_id: AsyncEnrollmentID,
    ) -> None | AsyncEnrollmentRejectBadOutcome:
        return await async_enrollment_reject(
            conn,
            now,
            organization_id,
            author,
            enrollment_id,
        )

    @override
    @transaction
    async def accept(
        self,
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
        return await async_enrollment_accept(
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            enrollment_id,
            accept_payload,
            accept_payload_signature,
            submitter_user_certificate,
            submitter_redacted_user_certificate,
            submitter_device_certificate,
            submitter_redacted_device_certificate,
        )
