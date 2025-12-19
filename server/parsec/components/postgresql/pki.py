# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    HumanHandle,
    OrganizationID,
    PKIEnrollmentID,
    PkiSignatureAlgorithm,
    UserCertificate,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.pki import (
    BasePkiEnrollmentComponent,
    PkiCertificate,
    PkiEnrollmentAcceptBadOutcome,
    PkiEnrollmentAcceptValidateBadOutcome,
    PkiEnrollmentInfo,
    PkiEnrollmentInfoBadOutcome,
    PkiEnrollmentListBadOutcome,
    PkiEnrollmentListItem,
    PkiEnrollmentRejectBadOutcome,
    PkiEnrollmentSubmitBadOutcome,
    PkiEnrollmentSubmitX509CertificateAlreadySubmitted,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.pki_accept import pki_accept
from parsec.components.postgresql.pki_info import pki_info
from parsec.components.postgresql.pki_list import pki_list
from parsec.components.postgresql.pki_reject import pki_reject
from parsec.components.postgresql.pki_submit import pki_submit
from parsec.components.postgresql.utils import no_transaction, transaction
from parsec.config import BackendConfig


class PGPkiEnrollmentComponent(BasePkiEnrollmentComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config)
        self.pool = pool

    @override
    @transaction
    async def submit(
        self,
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
    ) -> None | PkiEnrollmentSubmitBadOutcome | PkiEnrollmentSubmitX509CertificateAlreadySubmitted:
        return await pki_submit(
            conn,
            now,
            organization_id,
            enrollment_id,
            force,
            submitter_human_handle,
            submitter_trustchain,
            submit_payload_signature,
            submit_payload_signature_algorithm,
            submit_payload,
            self._config,
        )

    @override
    @no_transaction
    async def info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
    ) -> PkiEnrollmentInfo | PkiEnrollmentInfoBadOutcome:
        return await pki_info(
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
    ) -> list[PkiEnrollmentListItem] | PkiEnrollmentListBadOutcome:
        return await pki_list(
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
        author: DeviceID,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
    ) -> None | PkiEnrollmentRejectBadOutcome:
        return await pki_reject(
            conn,
            now,
            author,
            organization_id,
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
        enrollment_id: PKIEnrollmentID,
        payload: bytes,
        payload_signature: bytes,
        payload_signature_algorithm: PkiSignatureAlgorithm,
        accepter_trustchain: list[PkiCertificate],
        submitter_user_certificate: bytes,
        submitter_redacted_user_certificate: bytes,
        submitter_device_certificate: bytes,
        submitter_redacted_device_certificate: bytes,
    ) -> (
        tuple[UserCertificate, DeviceCertificate]
        | PkiEnrollmentAcceptValidateBadOutcome
        | PkiEnrollmentAcceptBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        return await pki_accept(
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            enrollment_id,
            payload,
            payload_signature,
            payload_signature_algorithm,
            accepter_trustchain,
            submitter_user_certificate,
            submitter_redacted_user_certificate,
            submitter_device_certificate,
            submitter_redacted_device_certificate,
            self._config,
        )
