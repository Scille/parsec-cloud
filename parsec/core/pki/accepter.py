# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from hashlib import sha1
from pathlib import Path
from typing import Iterable, cast

import attr

from parsec._parsec import (
    AuthenticatedCmds,
    DateTime,
    EnrollmentID,
    PkiEnrollmentAcceptRepActiveUsersLimitReached,
    PkiEnrollmentAcceptRepAlreadyExists,
    PkiEnrollmentAcceptRepInvalidCertification,
    PkiEnrollmentAcceptRepInvalidData,
    PkiEnrollmentAcceptRepInvalidPayloadData,
    PkiEnrollmentAcceptRepNoLongerAvailable,
    PkiEnrollmentAcceptRepNotAllowed,
    PkiEnrollmentAcceptRepNotFound,
    PkiEnrollmentAcceptRepOk,
    PkiEnrollmentAcceptRepUnknownStatus,
    PkiEnrollmentListRepNotAllowed,
    PkiEnrollmentListRepOk,
    PkiEnrollmentRejectRepNoLongerAvailable,
    PkiEnrollmentRejectRepNotAllowed,
    PkiEnrollmentRejectRepNotFound,
    PkiEnrollmentRejectRepOk,
)
from parsec.api.data import PkiEnrollmentAnswerPayload, PkiEnrollmentSubmitPayload
from parsec.api.protocol import DeviceLabel, HumanHandle, UserProfile
from parsec.core.backend_connection.authenticated import BackendAuthenticatedCmds
from parsec.core.invite.greeter import _create_new_user_certificates
from parsec.core.pki.exceptions import (
    PkiEnrollmentAcceptActiveUsersLimitReachedError,
    PkiEnrollmentAcceptAlreadyExistsError,
    PkiEnrollmentAcceptError,
    PkiEnrollmentAcceptInvalidCertificationError,
    PkiEnrollmentAcceptInvalidDataError,
    PkiEnrollmentAcceptInvalidPayloadDataError,
    PkiEnrollmentAcceptNoLongerAvailableError,
    PkiEnrollmentAcceptNotAllowedError,
    PkiEnrollmentAcceptNotFoundError,
    PkiEnrollmentError,
    PkiEnrollmentListError,
    PkiEnrollmentListNotAllowedError,
    PkiEnrollmentRejectError,
    PkiEnrollmentRejectNoLongerAvailableError,
    PkiEnrollmentRejectNotAllowedError,
    PkiEnrollmentRejectNotFoundError,
)
from parsec.core.pki.plumbing import (
    X509Certificate,
    pki_enrollment_load_peer_certificate,
    pki_enrollment_load_submit_payload,
    pki_enrollment_select_certificate,
    pki_enrollment_sign_payload,
)
from parsec.core.types import LocalDevice


async def accepter_list_submitted_from_backend(
    cmds: BackendAuthenticatedCmds | AuthenticatedCmds, extra_trust_roots: Iterable[Path] = ()
) -> list[PkiEnrollmentAccepterValidSubmittedCtx | PkiEnrollmentAccepterInvalidSubmittedCtx]:
    """
    Raises:
        BackendNotAvailable
        BackendProtocolError

        PkiEnrollmentListError
        PkiEnrollmentListNotAllowedError
    """
    rep = await cmds.pki_enrollment_list()

    if isinstance(rep, PkiEnrollmentListRepNotAllowed):
        raise PkiEnrollmentListNotAllowedError(f"Listing enrollments is not allowed: {rep}")
    if not isinstance(rep, PkiEnrollmentListRepOk):
        raise PkiEnrollmentListError(f"Backend refused to list enrollments: {rep}")

    pendings: list[
        PkiEnrollmentAccepterInvalidSubmittedCtx | PkiEnrollmentAccepterValidSubmittedCtx
    ] = []

    for enrollment in rep.enrollments:

        enrollment_id = enrollment.enrollment_id
        submitted_on = cast(DateTime, enrollment.submitted_on)
        submitter_der_x509_certificate = enrollment.submitter_der_x509_certificate
        submit_payload_signature = enrollment.submit_payload_signature
        raw_submit_payload = enrollment.submit_payload

        # Load the submitter certificate
        try:
            submitter_x509_certificate = pki_enrollment_load_peer_certificate(
                submitter_der_x509_certificate
            )

        # Could not load the submitter certificate
        except PkiEnrollmentError as exc:
            invalid_submitted_pending = PkiEnrollmentAccepterInvalidSubmittedCtx(
                cmds=cmds,
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submitter_x509_certificate=None,
                submit_payload_signature=submit_payload_signature,
                raw_submit_payload=raw_submit_payload,
                error=exc,
            )
            pendings.append(invalid_submitted_pending)
            continue

        # Verify the enrollment request
        try:
            submit_payload = pki_enrollment_load_submit_payload(
                extra_trust_roots=extra_trust_roots,
                der_x509_certificate=submitter_der_x509_certificate,
                payload_signature=submit_payload_signature,
                payload=raw_submit_payload,
            )

        # Verification failed
        except PkiEnrollmentError as exc:
            invalid_submitted_pending = PkiEnrollmentAccepterInvalidSubmittedCtx(
                cmds=cmds,
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submitter_x509_certificate=submitter_x509_certificate,
                submit_payload_signature=submit_payload_signature,
                raw_submit_payload=raw_submit_payload,
                error=exc,
            )
            pendings.append(invalid_submitted_pending)
            continue

        # Verification succeed
        valid_submitted_pending = PkiEnrollmentAccepterValidSubmittedCtx(
            cmds=cmds,
            enrollment_id=enrollment_id,
            submitted_on=submitted_on,
            submitter_der_x509_certificate=submitter_der_x509_certificate,
            submit_payload_signature=submit_payload_signature,
            raw_submit_payload=raw_submit_payload,
            submitter_x509_certificate=submitter_x509_certificate,
            submit_payload=submit_payload,
        )
        pendings.append(valid_submitted_pending)

    return pendings


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentAccepterValidSubmittedCtx:
    _cmds: BackendAuthenticatedCmds | AuthenticatedCmds
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    raw_submit_payload: bytes

    submitter_x509_certificate: X509Certificate
    submit_payload: PkiEnrollmentSubmitPayload

    @property
    def submitter_x509_certificate_sha1(self) -> bytes:
        return self.submitter_x509_certificate.certificate_sha1

    async def reject(self) -> None:
        """
        Raises:
            BackendNotAvailable
            BackendProtocolError

            PkiEnrollmentRejectError
            PkiEnrollmentRejectNotFoundError
            PkiEnrollmentRejectNotAllowedError
            PkiEnrollmentRejectNoLongerAvailableError
        """
        await _reject(self._cmds, self.enrollment_id)

    async def accept(
        self,
        author: LocalDevice,
        device_label: DeviceLabel,
        human_handle: HumanHandle,
        profile: UserProfile,
    ) -> None:
        """
        Raises:
            BackendNotAvailable
            BackendProtocolError

            PkiEnrollmentAcceptError
            PkiEnrollmentAcceptNotAllowedError
            PkiEnrollmentAcceptInvalidPayloadDataError
            PkiEnrollmentAcceptInvalidDataError
            PkiEnrollmentAcceptInvalidCertificationError
            PkiEnrollmentAcceptNotFoundError
            PkiEnrollmentAcceptNoLongerAvailableError
            PkiEnrollmentAcceptAlreadyExistsError
            PkiEnrollmentAcceptActiveUsersLimitReachedError


            PkiEnrollmentCertificateError
            PkiEnrollmentCertificateCryptoError
            PkiEnrollmentCertificateNotFoundError
            PkiEnrollmentCertificatePinCodeUnavailableError
        """
        # Create the certificate for the new user
        (
            user_certificate,
            redacted_user_certificate,
            device_certificate,
            redacted_device_certificate,
            user_confirmation,
        ) = _create_new_user_certificates(
            author=author,
            device_label=device_label,
            human_handle=human_handle,
            profile=profile,
            public_key=self.submit_payload.public_key,
            verify_key=self.submit_payload.verify_key,
        )

        # Build accept payload
        accept_payload = PkiEnrollmentAnswerPayload(
            device_id=user_confirmation.device_id,
            device_label=user_confirmation.device_label,
            human_handle=user_confirmation.human_handle,
            profile=user_confirmation.profile,
            root_verify_key=user_confirmation.root_verify_key,
        ).dump()

        accepter_x509_certificate = await pki_enrollment_select_certificate(owner_hint=author)
        accept_payload_signature = await pki_enrollment_sign_payload(
            payload=accept_payload, x509_certificate=accepter_x509_certificate
        )

        # Do the actual accept
        rep = await self._cmds.pki_enrollment_accept(
            enrollment_id=self.enrollment_id,
            accepter_der_x509_certificate=accepter_x509_certificate.der_x509_certificate,
            accept_payload_signature=accept_payload_signature,
            accept_payload=accept_payload,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )

        if isinstance(rep, PkiEnrollmentAcceptRepNotAllowed):
            raise PkiEnrollmentAcceptNotAllowedError(
                f"Accepting the enrollment is not allowed: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepInvalidPayloadData):
            raise PkiEnrollmentAcceptInvalidPayloadDataError(
                f"The provided payload is invalid: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepInvalidData):
            raise PkiEnrollmentAcceptInvalidDataError(
                f"The provided user data is invalid: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepInvalidCertification):
            raise PkiEnrollmentAcceptInvalidCertificationError(
                f"The provided user certification is invalid: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepNotFound):
            raise PkiEnrollmentAcceptNotFoundError(
                f"The enrollment cannot be found: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepNoLongerAvailable):
            raise PkiEnrollmentAcceptNoLongerAvailableError(
                f"The enrollment is not longer available: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepAlreadyExists):
            raise PkiEnrollmentAcceptAlreadyExistsError(
                f"This user already exists: {rep.reason}", rep
            )
        if isinstance(rep, PkiEnrollmentAcceptRepActiveUsersLimitReached):
            raise PkiEnrollmentAcceptActiveUsersLimitReachedError(
                f"The active users limit has been reached.", rep
            )
        if not isinstance(rep, PkiEnrollmentAcceptRepOk):
            raise PkiEnrollmentAcceptError(
                f"Backend refused to accept the enrollment: {cast(PkiEnrollmentAcceptRepUnknownStatus, rep).status}",
                rep,
            )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentAccepterInvalidSubmittedCtx:
    """
    The `error` attribute can be one of the following:
    - PkiEnrollmentCertificateCryptoError: when any of the required certificate-replated crypto operation fails
    - PkiEnrollmentCertificateSignatureError: when the provided signature does not correspond to the certificate public key
    - PkiEnrollmentCertificateValidationError: when the provided certificate cannot be validated using the configured trust roots
    - PkiEnrollmentCertificateError: an generic certificate-related errors
    - PkiEnrollmentPayloadValidationError: when some enrollment information cannot be properly loaded

    The `submitter_x509_certificate` is optional depending on whether the certificate information could be successfully extracted
    before the error or not.
    """

    _cmds: BackendAuthenticatedCmds | AuthenticatedCmds
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submitter_x509_certificate: X509Certificate | None
    submit_payload_signature: bytes
    raw_submit_payload: bytes
    error: PkiEnrollmentError

    @property
    def submitter_x509_certificate_sha1(self) -> bytes:
        return sha1(self.submitter_der_x509_certificate).digest()

    async def reject(self) -> None:
        """
        Raises:
            BackendNotAvailable
            BackendProtocolError

            PkiEnrollmentRejectError
            PkiEnrollmentRejectNotFoundError
            PkiEnrollmentRejectNotAllowedError
            PkiEnrollmentRejectNoLongerAvailableError
        """
        await _reject(self._cmds, self.enrollment_id)


async def _reject(
    cmds: BackendAuthenticatedCmds | AuthenticatedCmds, enrollment_id: EnrollmentID
) -> None:
    """
    Raises:
        BackendNotAvailable
        BackendProtocolError

        PkiEnrollmentRejectError
        PkiEnrollmentRejectNotFoundError
        PkiEnrollmentRejectNotAllowedError
        PkiEnrollmentRejectNoLongerAvailableError
    """
    rep = await cmds.pki_enrollment_reject(enrollment_id)

    if isinstance(rep, PkiEnrollmentRejectRepNotAllowed):
        raise PkiEnrollmentRejectNotAllowedError(
            f"Rejecting the enrollment is not allowed: {rep.reason}", rep
        )
    if isinstance(rep, PkiEnrollmentRejectRepNotFound):
        raise PkiEnrollmentRejectNotFoundError(f"The enrollment cannot be found: {rep.reason}", rep)
    if isinstance(rep, PkiEnrollmentRejectRepNoLongerAvailable):
        raise PkiEnrollmentRejectNoLongerAvailableError(
            f"The enrollment is not longer available: {rep.reason}", rep
        )
    if not isinstance(rep, PkiEnrollmentRejectRepOk):
        raise PkiEnrollmentRejectError(f"Backend refused to reject the enrollment: {rep}", rep)
