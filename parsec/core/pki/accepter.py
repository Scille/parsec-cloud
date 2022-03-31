# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from typing import Iterable, List, Union
from uuid import UUID
from pathlib import Path
from hashlib import sha1
from pendulum import DateTime

from parsec.api.data import PkiEnrollmentSubmitPayload, PkiEnrollmentAcceptPayload
from parsec.api.protocol import HumanHandle, DeviceLabel, UserProfile
from parsec.core.backend_connection.authenticated import BackendAuthenticatedCmds
from parsec.core.invite.greeter import _create_new_user_certificates
from parsec.core.local_device import LocalDeviceError
from parsec.core.pki.plumbing import (
    X509Certificate,
    pki_enrollment_load_submit_payload,
    pki_enrollment_select_certificate,
    pki_enrollment_sign_payload,
)
from parsec.core.types import LocalDevice


async def accepter_list_submitted_from_backend(
    cmds: BackendAuthenticatedCmds, extra_trust_roots: Iterable[Path] = ()
) -> List[
    Union["PkiEnrollementAccepterValidSubmittedCtx", "PkiEnrollementAccepterInvalidSubmittedCtx"]
]:
    # TODO: document exceptions !
    try:
        rep = await cmds.pki_enrollment_list()
    except Exception:
        # TODO: exception handling !
        raise
    if rep["status"] != "ok":
        # TODO: exception handling !
        raise RuntimeError()

    pendings = []

    for enrollment in rep["enrollments"]:

        enrollment_id: UUID = enrollment["enrollment_id"]
        submitted_on: DateTime = enrollment["submitted_on"]
        submitter_der_x509_certificate: bytes = enrollment["submitter_der_x509_certificate"]
        submit_payload_signature: bytes = enrollment["submit_payload_signature"]
        raw_submit_payload: bytes = enrollment["submit_payload"]

        # Verify the enrollment request
        try:
            (submitter_x509_certif, submit_payload) = pki_enrollment_load_submit_payload(
                extra_trust_roots=extra_trust_roots,
                der_x509_certificate=submitter_der_x509_certificate,
                payload_signature=submit_payload_signature,
                payload=raw_submit_payload,
            )
            pending = PkiEnrollementAccepterValidSubmittedCtx(
                cmds=cmds,
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submit_payload_signature=submit_payload_signature,
                raw_submit_payload=raw_submit_payload,
                submitter_x509_certif=submitter_x509_certif,
                submit_payload=submit_payload,
            )

        # Verification failed
        except LocalDeviceError as exc:
            submitter_x509_certif = submit_payload = None
            pending = PkiEnrollementAccepterValidSubmittedCtx(
                cmds=cmds,
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submit_payload_signature=submit_payload_signature,
                raw_submit_payload=raw_submit_payload,
                error=str(exc),
            )

        pendings.append(pending)

    return pendings


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollementAccepterValidSubmittedCtx:
    _cmds: BackendAuthenticatedCmds
    enrollment_id: UUID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    raw_submit_payload: bytes

    submitter_x509_certif: X509Certificate
    submit_payload: PkiEnrollmentSubmitPayload

    @property
    def submitter_x509_certificate_sha1(self) -> bytes:
        return self.submitter_x509_certif.certificate_sha1

    async def reject(self) -> None:
        # TODO: document exceptions !
        try:
            rep = await self._cmds.pki_enrollment_reject(self.enrollment_id)
        except Exception:
            # TODO: exception handling !
            raise
        if rep["status"] != "ok":
            # TODO: exception handling !
            raise RuntimeError()

    async def accept(
        self,
        author: LocalDevice,
        device_label: DeviceLabel,
        human_handle: HumanHandle,
        profile: UserProfile,
    ):
        # TODO: document exceptions !
        # Create the certificate for the new user
        user_certificate, redacted_user_certificate, device_certificate, redacted_device_certificate, user_confirmation = _create_new_user_certificates(
            author=author,
            device_label=device_label,
            human_handle=human_handle,
            profile=profile,
            public_key=self.submit_payload.public_key,
            verify_key=self.submit_payload.verify_key,
        )

        # Build accept payload
        accept_payload = PkiEnrollmentAcceptPayload(
            device_id=user_confirmation.device_id,
            device_label=user_confirmation.device_label,
            human_handle=user_confirmation.human_handle,
            profile=user_confirmation.profile,
            root_verify_key=user_confirmation.root_verify_key,
        ).dump()

        accepter_x509_certificate = pki_enrollment_select_certificate(owner_hint=author)
        accept_payload_signature = pki_enrollment_sign_payload(
            payload=accept_payload, x509_certificate=accepter_x509_certificate
        )

        # Do the actual accept
        try:
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
        except Exception:
            # TODO: exception handling !
            raise
        if rep["status"] != "ok":
            # TODO: exception handling !
            raise RuntimeError()


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollementAccepterInvalidSubmittedCtx:
    _cmds: BackendAuthenticatedCmds
    enrollment_id: UUID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    raw_submit_payload: bytes

    @property
    def submitter_x509_certificate_sha1(self) -> bytes:
        return sha1(self.submitter_der_x509_certificate).digest()

    error: str

    async def reject(self) -> None:
        # TODO: document exceptions !
        try:
            rep = await self._cmds.pki_enrollment_reject(self.enrollment_id)
        except Exception:
            # TODO: exception handling !
            raise
        if rep["status"] != "ok":
            # TODO: exception handling !
            raise RuntimeError()
