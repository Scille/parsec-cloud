# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path
from typing import Iterable, List, Union

import attr

from parsec._parsec import (
    DateTime,
    EnrollmentID,
    PkiEnrollmentInfoRepNotFound,
    PkiEnrollmentInfoRepOk,
    PkiEnrollmentStatus,
    PkiEnrollmentSubmitRepAlreadyEnrolled,
    PkiEnrollmentSubmitRepAlreadySubmitted,
    PkiEnrollmentSubmitRepEmailAlreadyUsed,
    PkiEnrollmentSubmitRepIdAlreadyUsed,
    PkiEnrollmentSubmitRepOk,
)
from parsec.api.data import PkiEnrollmentSubmitPayload
from parsec.api.protocol import DeviceLabel
from parsec.core.backend_connection import pki_enrollment_info as cmd_pki_enrollment_info
from parsec.core.backend_connection import pki_enrollment_submit as cmd_pki_enrollment_submit
from parsec.core.local_device import generate_new_device
from parsec.core.pki.exceptions import (
    PkiEnrollmentError,
    PkiEnrollmentInfoError,
    PkiEnrollmentInfoNotFoundError,
    PkiEnrollmentSubmitCertificateAlreadyEnrolledError,
    PkiEnrollmentSubmitCertificateAlreadySubmittedError,
    PkiEnrollmentSubmitCertificateEmailAlreadyUsedError,
    PkiEnrollmentSubmitEnrollmentIdAlreadyUsedError,
    PkiEnrollmentSubmitError,
)
from parsec.core.pki.plumbing import (
    PkiEnrollmentAnswerPayload,
    X509Certificate,
    pki_enrollment_create_local_pending,
    pki_enrollment_load_accept_payload,
    pki_enrollment_load_local_pending_secret_part,
    pki_enrollment_load_peer_certificate,
    pki_enrollment_select_certificate,
    pki_enrollment_sign_payload,
)
from parsec.core.types import (
    BackendAddr,
    BackendOrganizationAddr,
    BackendPkiEnrollmentAddr,
    LocalDevice,
)
from parsec.core.types.pki import LocalPendingEnrollment
from parsec.crypto import PrivateKey, SigningKey


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterInitialCtx:
    addr: BackendPkiEnrollmentAddr
    enrollment_id: EnrollmentID
    signing_key: SigningKey
    private_key: PrivateKey
    x509_certificate: X509Certificate

    @classmethod
    async def new(cls, addr: BackendPkiEnrollmentAddr) -> PkiEnrollmentSubmitterInitialCtx:
        """
        Raises:
            PkiEnrollmentCertificateError
            PkiEnrollmentCertificateCryptoError
            PkiEnrollmentCertificateNotFoundError
        """
        enrollment_id = EnrollmentID.new()
        signing_key = SigningKey.generate()
        private_key = PrivateKey.generate()

        x509_certificate = await pki_enrollment_select_certificate()

        return cls(
            addr=addr,
            enrollment_id=enrollment_id,
            signing_key=signing_key,
            private_key=private_key,
            x509_certificate=x509_certificate,
        )

    async def submit(
        self, config_dir: Path, requested_device_label: DeviceLabel, force: bool
    ) -> PkiEnrollmentSubmitterSubmittedStatusCtx:
        """
        Raises:
            BackendNotAvailable

            PkiEnrollmentSubmitError
            PkiEnrollmentSubmitEnrollmentIdAlreadyUsedError
            PkiEnrollmentSubmitCertificateAlreadySubmittedError
            PkiEnrollmentSubmitCertificateAlreadyEnrolledError

            PkiEnrollmentCertificateError
            PkiEnrollmentCertificateCryptoError
            PkiEnrollmentCertificateNotFoundError
            PkiEnrollmentCertificatePinCodeUnavailableError

            PkiEnrollmentLocalPendingError
            PkiEnrollmentLocalPendingCryptoError
            PkiEnrollmentLocalPendingPackingError
        """

        # Build submit payload
        cooked_submit_payload = PkiEnrollmentSubmitPayload(
            verify_key=self.signing_key.verify_key,
            public_key=self.private_key.public_key,
            requested_device_label=requested_device_label,
        )

        raw_submit_payload = cooked_submit_payload.dump()
        payload_signature = await pki_enrollment_sign_payload(
            payload=raw_submit_payload, x509_certificate=self.x509_certificate
        )

        assert (
            self.x509_certificate.subject_email_address is not None
        ), "Subject email address is `None` before enrollment submit"
        rep = await cmd_pki_enrollment_submit(
            addr=self.addr,
            enrollment_id=self.enrollment_id,
            force=force,
            submitter_der_x509_certificate=self.x509_certificate.der_x509_certificate,
            submitter_der_x509_certificate_email=self.x509_certificate.subject_email_address,
            submit_payload_signature=payload_signature,
            submit_payload=raw_submit_payload,
        )

        if isinstance(rep, PkiEnrollmentSubmitRepAlreadySubmitted):
            submitted_on = rep.submitted_on
            raise PkiEnrollmentSubmitCertificateAlreadySubmittedError(
                f"The certificate `{self.x509_certificate.certificate_sha1.hex()}` has already been submitted on {submitted_on}",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )

        if isinstance(rep, PkiEnrollmentSubmitRepIdAlreadyUsed):
            raise PkiEnrollmentSubmitEnrollmentIdAlreadyUsedError(
                f"The enrollment ID {self.enrollment_id.hex} is already used",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )

        if isinstance(rep, PkiEnrollmentSubmitRepAlreadyEnrolled):
            raise PkiEnrollmentSubmitCertificateAlreadyEnrolledError(
                f"The certificate `{self.x509_certificate.certificate_sha1.hex()}` has already been enrolled",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )

        if isinstance(rep, PkiEnrollmentSubmitRepEmailAlreadyUsed):
            raise PkiEnrollmentSubmitCertificateEmailAlreadyUsedError(
                f"The email address `{self.x509_certificate.subject_email_address}` is already used by an active user",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )

        if not isinstance(rep, PkiEnrollmentSubmitRepOk):
            raise PkiEnrollmentSubmitError(
                f"Backend refused to create enrollment: {type(rep).__name__}",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )
        submitted_on = rep.submitted_on

        # Save the enrollment request on disk.
        # Note there is not atomicity with the request to the backend, but it's
        # considered fine:
        # - if the pending enrollment is not saved, CLI will display an error message (unless
        #   the whole machine has crashed ^^) so user is expected to retry the submit command
        # - in case the enrollment is accepted by a ninja-fast admin before the submit can be
        #   retried, it's no big deal to revoke the newly enrolled user and restart from scratch
        local_pending = pki_enrollment_create_local_pending(
            config_dir=config_dir,
            x509_certificate=self.x509_certificate,
            addr=self.addr,
            enrollment_id=self.enrollment_id,
            submitted_on=submitted_on,
            submit_payload=cooked_submit_payload,
            signing_key=self.signing_key,
            private_key=self.private_key,
        )
        local_pending.save(config_dir)

        return PkiEnrollmentSubmitterSubmittedStatusCtx(
            config_dir=config_dir,
            x509_certificate=self.x509_certificate,
            addr=self.addr,
            submitted_on=submitted_on,
            enrollment_id=self.enrollment_id,
            submit_payload=cooked_submit_payload,
        )


T_PkiEnrollmentSubmitterStatusCtx = Union[
    "PkiEnrollmentSubmitterSubmittedStatusCtx",
    "PkiEnrollmentSubmitterCancelledStatusCtx",
    "PkiEnrollmentSubmitterRejectedStatusCtx",
    "PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx",
    "PkiEnrollmentSubmitterAcceptedStatusCtx",
]


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterSubmittedCtx:
    config_dir: Path
    x509_certificate: X509Certificate
    addr: BackendPkiEnrollmentAddr
    submitted_on: DateTime
    enrollment_id: EnrollmentID
    submit_payload: PkiEnrollmentSubmitPayload

    @classmethod
    def list_from_disk(cls, config_dir: Path) -> List["PkiEnrollmentSubmitterSubmittedCtx"]:
        """Raises: None"""
        ctxs = []
        for pending in LocalPendingEnrollment.list(config_dir):
            ctx = PkiEnrollmentSubmitterSubmittedCtx(
                config_dir=config_dir,
                x509_certificate=pending.x509_certificate,
                addr=pending.addr,
                submitted_on=pending.submitted_on,
                enrollment_id=pending.enrollment_id,
                submit_payload=pending.submit_payload,
            )
            ctxs.append(ctx)

        return ctxs

    async def poll(
        self, extra_trust_roots: Iterable[Path] = ()
    ) -> T_PkiEnrollmentSubmitterStatusCtx:
        """
        Raises:
            BackendNotAvailable

            PkiEnrollmentInfoError
            PkiEnrollmentInfoNotFoundError
        """
        #  Tuple[PkiEnrollmentStatus, DateTime, LocalDevice | None]:
        rep = await cmd_pki_enrollment_info(addr=self.addr, enrollment_id=self.enrollment_id)

        if isinstance(rep, PkiEnrollmentInfoRepNotFound):
            raise PkiEnrollmentInfoNotFoundError(
                f"The provided enrollment could not be found: {rep.reason}",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )

        if not isinstance(rep, PkiEnrollmentInfoRepOk):
            raise PkiEnrollmentInfoError(
                f"Backend refused to provide the enrollment info: {rep}",
                rep,
                self.enrollment_id,
                self.x509_certificate,
            )

        enrollment_status = rep.enrollment_status

        if enrollment_status.status == PkiEnrollmentStatus.SUBMITTED:
            return PkiEnrollmentSubmitterSubmittedStatusCtx(
                config_dir=self.config_dir,
                x509_certificate=self.x509_certificate,
                addr=self.addr,
                submitted_on=self.submitted_on,
                enrollment_id=self.enrollment_id,
                submit_payload=self.submit_payload,
            )

        elif enrollment_status.status == PkiEnrollmentStatus.CANCELLED:
            cancelled_on = rep.cancelled_on
            return PkiEnrollmentSubmitterCancelledStatusCtx(
                config_dir=self.config_dir,
                x509_certificate=self.x509_certificate,
                addr=self.addr,
                submitted_on=self.submitted_on,
                enrollment_id=self.enrollment_id,
                submit_payload=self.submit_payload,
                cancelled_on=cancelled_on,
            )

        elif enrollment_status.status == PkiEnrollmentStatus.REJECTED:
            rejected_on = rep.rejected_on
            return PkiEnrollmentSubmitterRejectedStatusCtx(
                config_dir=self.config_dir,
                x509_certificate=self.x509_certificate,
                addr=self.addr,
                submitted_on=self.submitted_on,
                enrollment_id=self.enrollment_id,
                submit_payload=self.submit_payload,
                rejected_on=rejected_on,
            )

        else:
            assert enrollment_status.status == PkiEnrollmentStatus.ACCEPTED
            accepter_der_x509_certificate = rep.accepter_der_x509_certificate
            payload_signature = rep.accept_payload_signature
            payload = rep.accept_payload
            accepted_on = rep.accepted_on

            # Load peer certificate
            try:
                accepter_x509_certificate = pki_enrollment_load_peer_certificate(
                    accepter_der_x509_certificate
                )

            # Could not load peer certificate
            except PkiEnrollmentError as exc:
                return PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx(
                    config_dir=self.config_dir,
                    x509_certificate=self.x509_certificate,
                    addr=self.addr,
                    submitted_on=self.submitted_on,
                    enrollment_id=self.enrollment_id,
                    submit_payload=self.submit_payload,
                    accepted_on=accepted_on,
                    accepter_x509_certificate=None,
                    error=exc,
                )

            # Load accept payload
            try:
                accept_payload = pki_enrollment_load_accept_payload(
                    extra_trust_roots=extra_trust_roots,
                    der_x509_certificate=accepter_der_x509_certificate,
                    payload_signature=payload_signature,
                    payload=payload,
                )

            # Verification failed
            except PkiEnrollmentError as exc:
                return PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx(
                    config_dir=self.config_dir,
                    x509_certificate=self.x509_certificate,
                    addr=self.addr,
                    submitted_on=self.submitted_on,
                    enrollment_id=self.enrollment_id,
                    submit_payload=self.submit_payload,
                    accepted_on=accepted_on,
                    accepter_x509_certificate=accepter_x509_certificate,
                    error=exc,
                )

            # Verification succeed
            return PkiEnrollmentSubmitterAcceptedStatusCtx(
                config_dir=self.config_dir,
                x509_certificate=self.x509_certificate,
                addr=self.addr,
                submitted_on=self.submitted_on,
                enrollment_id=self.enrollment_id,
                submit_payload=self.submit_payload,
                accepted_on=accepted_on,
                accepter_x509_certificate=accepter_x509_certificate,
                accept_payload=accept_payload,
            )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BasePkiEnrollmentSubmitterStatusCtx:
    config_dir: Path
    x509_certificate: X509Certificate
    addr: BackendPkiEnrollmentAddr
    submitted_on: DateTime
    enrollment_id: EnrollmentID
    submit_payload: PkiEnrollmentSubmitPayload


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterSubmittedStatusCtx(BasePkiEnrollmentSubmitterStatusCtx):
    submit_payload: PkiEnrollmentSubmitPayload

    async def remove_from_disk(self) -> None:
        """
        Raises:
            PkiEnrollmentLocalPendingError
            PkiEnrollmentLocalPendingNotFoundError
            PkiEnrollmentLocalPendingValidationError
        """
        LocalPendingEnrollment.remove_from_enrollment_id(
            config_dir=self.config_dir, enrollment_id=self.enrollment_id
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterCancelledStatusCtx(BasePkiEnrollmentSubmitterStatusCtx):
    cancelled_on: DateTime

    async def remove_from_disk(self) -> None:
        """
        Raises:
            PkiEnrollmentLocalPendingError
            PkiEnrollmentLocalPendingNotFoundError
            PkiEnrollmentLocalPendingValidationError
        """
        LocalPendingEnrollment.remove_from_enrollment_id(
            config_dir=self.config_dir, enrollment_id=self.enrollment_id
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterRejectedStatusCtx(BasePkiEnrollmentSubmitterStatusCtx):
    rejected_on: DateTime

    async def remove_from_disk(self) -> None:
        """
        Raises:
            PkiEnrollmentLocalPendingError
            PkiEnrollmentLocalPendingNotFoundError
            PkiEnrollmentLocalPendingValidationError
        """
        LocalPendingEnrollment.remove_from_enrollment_id(
            config_dir=self.config_dir, enrollment_id=self.enrollment_id
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx(BasePkiEnrollmentSubmitterStatusCtx):
    """
    The `error` attribute can be one of the following:
    - PkiEnrollmentCertificateCryptoError: when any of the required certificate-replated crypto operation fails
    - PkiEnrollmentCertificateSignatureError: when the provided signature does not correspond to the certificate public key
    - PkiEnrollmentCertificateValidationError: when the provided certificate cannot be validated using the configured trust roots
    - PkiEnrollmentCertificateError: an generic certificate-related errors
    - PkiEnrollmentPayloadValidationError: when some enrollment information cannot be properly loaded

    The `accepter_x509_certificate` is optional depending on whether the certificate information could be successfully extracted
    before the error or not.
    """

    accepted_on: DateTime
    accepter_x509_certificate: X509Certificate | None
    error: PkiEnrollmentError

    async def remove_from_disk(self) -> None:
        """
        Raises:
            PkiEnrollmentLocalPendingError
            PkiEnrollmentLocalPendingNotFoundError
            PkiEnrollmentLocalPendingValidationError
        """
        LocalPendingEnrollment.remove_from_enrollment_id(
            config_dir=self.config_dir, enrollment_id=self.enrollment_id
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmitterAcceptedStatusCtx(BasePkiEnrollmentSubmitterStatusCtx):
    accepted_on: DateTime
    accepter_x509_certificate: X509Certificate
    accept_payload: PkiEnrollmentAnswerPayload

    async def finalize(self) -> "PkiEnrollmentFinalizedCtx":
        """
        Raises:
            PkiEnrollmentCertificateNotFoundError
            PkiEnrollmentCertificateCryptoError
            PkiEnrollmentCertificateError
            PkiEnrollmentCertificatePinCodeUnavailableError
            PkiEnrollmentLocalPendingCryptoError
        """
        signing_key, private_key = await pki_enrollment_load_local_pending_secret_part(
            config_dir=self.config_dir, enrollment_id=self.enrollment_id
        )

        # Create the local device
        organization_addr = BackendOrganizationAddr.build(
            backend_addr=BackendAddr(self.addr.hostname, self.addr.port, self.addr.use_ssl),
            organization_id=self.addr.organization_id,
            root_verify_key=self.accept_payload.root_verify_key,
        )
        new_device = generate_new_device(
            organization_addr=organization_addr,
            device_id=self.accept_payload.device_id,
            profile=self.accept_payload.profile,
            human_handle=self.accept_payload.human_handle,
            device_label=self.accept_payload.device_label,
            signing_key=signing_key,
            private_key=private_key,
        )

        return PkiEnrollmentFinalizedCtx(
            config_dir=self.config_dir,
            enrollment_id=self.enrollment_id,
            new_device=new_device,
            x509_certificate=self.x509_certificate,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentFinalizedCtx:
    config_dir: Path
    enrollment_id: EnrollmentID
    new_device: LocalDevice
    x509_certificate: X509Certificate

    async def remove_from_disk(self) -> None:
        """
        Raises:
            PkiEnrollmentLocalPendingError
            PkiEnrollmentLocalPendingNotFoundError
            PkiEnrollmentLocalPendingValidationError
        """
        LocalPendingEnrollment.remove_from_enrollment_id(
            config_dir=self.config_dir, enrollment_id=self.enrollment_id
        )
