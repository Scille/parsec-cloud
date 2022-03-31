# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Iterable, Tuple, List, Optional
from pathlib import Path
from hashlib import sha1
from uuid import UUID
from importlib import import_module
from pendulum import DateTime
import attr
from parsec.core.types.backend_address import BackendPkiEnrollmentAddr

from parsec.crypto import PrivateKey, PublicKey, SigningKey, VerifyKey
from parsec.api.data import PkiEnrollmentSubmitPayload, PkiEnrollmentAcceptPayload
from parsec.api.protocol import HumanHandle, DeviceLabel, UserProfile
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.invite.greeter import _create_new_user_certificates
from parsec.core.backend_connection import BackendAuthenticatedCmds
from parsec.core.local_device import LocalDeviceError


def _load_smartcard_extension():
    try:
        return import_module("parsec_ext.smartcard")
    except ModuleNotFoundError as exc:
        raise ModuleNotFoundError("Parsec smartcard extension not available") from exc


@attr.s(slots=True, frozen=True, auto_attribs=True)
class X509Certificate:
    # TODO: better names ?
    issuer_label: str
    issuer_email: str
    der_x509_certificate: bytes
    certificate_sha1: bytes
    certificate_id: str


def is_pki_enrollment_available() -> bool:
    try:
        _load_smartcard_extension()
        return True
    except ModuleNotFoundError:
        return False


def pki_enrollment_select_certificate(owner_hint: Optional[LocalDevice] = None) -> X509Certificate:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_select_certificate(owner_hint=owner_hint)


def pki_enrollment_sign_payload(payload: bytes, x509_certificate: X509Certificate) -> bytes:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_sign_payload(
        payload=payload, x509_certificate=x509_certificate
    )


def pki_enrollment_save_local_pending(
    config_dir: Path,
    x509_certificate: X509Certificate,
    addr: BackendPkiEnrollmentAddr,
    enrollment_id: UUID,
    submitted_on: DateTime,
    submit_payload: PkiEnrollmentSubmitPayload,
    signing_key: SigningKey,
    private_key: PrivateKey,
) -> None:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_save_local_pending(
        config_dir=config_dir,
        x509_certificate=x509_certificate,
        addr=addr,
        enrollment_id=enrollment_id,
        submitted_on=submitted_on,
        submit_payload=submit_payload,
        signing_key=signing_key,
        private_key=private_key,
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalPendingEnrollment:
    x509_certificate: X509Certificate
    addr: BackendPkiEnrollmentAddr
    # This `submitted_on` is not strictly in line with the submitted_on stored on the backend
    submitted_on: DateTime
    enrollment_id: UUID
    submit_payload: PkiEnrollmentSubmitPayload
    signing_key: SigningKey
    private_key: PrivateKey


def pki_enrollment_list_local_pendings(config_dir: Path) -> List[LocalPendingEnrollment]:
    return _load_smartcard_extension().pki_enrollment_list_local_pendings(config_dir=config_dir)


def pki_enrollment_remove_local_pending(config_dir: Path, enrollment_id: UUID) -> None:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_remove_local_pending(
        config_dir=config_dir, enrollment_id=enrollment_id
    )


def pki_enrollment_load_submit_payload(
    config: CoreConfig,
    der_x509_certificate: bytes,
    payload_signature: bytes,
    payload: bytes,
    extra_trust_roots: Iterable[Path] = (),
) -> Tuple[X509Certificate, PkiEnrollmentSubmitPayload]:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_load_submit_payload(
        config=config,
        der_x509_certificate=der_x509_certificate,
        payload_signature=payload_signature,
        payload=payload,
        extra_trust_roots=extra_trust_roots,
    )


def pki_enrollment_load_accept_payload(
    config: CoreConfig,
    der_x509_certificate: bytes,
    payload_signature: bytes,
    payload: bytes,
    extra_trust_roots: Iterable[Path] = (),
) -> Tuple[X509Certificate, PkiEnrollmentAcceptPayload]:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_load_accept_payload(
        config=config,
        der_x509_certificate=der_x509_certificate,
        payload_signature=payload_signature,
        payload=payload,
        extra_trust_roots=extra_trust_roots,
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RemotePendingEnrollment:
    enrollment_id: UUID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    raw_submit_payload: bytes

    @property
    def certificate_sha1(self) -> bytes:
        return sha1(self.submitter_der_x509_certificate).digest()


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ValidRemotePendingEnrollment(RemotePendingEnrollment):
    submitter_x509_certif: X509Certificate
    submit_payload: PkiEnrollmentSubmitPayload


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ClaqueAuSolRemotePendingEnrollment(RemotePendingEnrollment):
    error: str


async def pki_enrollment_remote_pendings_list(
    config: CoreConfig, cmds: BackendAuthenticatedCmds, extra_trust_roots: Iterable[Path] = ()
) -> List[RemotePendingEnrollment]:
    # TODO: document exceptions !
    try:
        rep = await cmds.pki_enrollment_list()
    except Exception:
        # TODO: wrap exceptions !
        raise

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
                config=config,
                extra_trust_roots=extra_trust_roots,
                der_x509_certificate=submitter_der_x509_certificate,
                payload_signature=submit_payload_signature,
                payload=raw_submit_payload,
            )
            pendings.append(
                ValidRemotePendingEnrollment(
                    enrollment_id=enrollment_id,
                    submitted_on=submitted_on,
                    submitter_der_x509_certificate=submitter_der_x509_certificate,
                    submit_payload_signature=submit_payload_signature,
                    raw_submit_payload=raw_submit_payload,
                    submitter_x509_certif=submitter_x509_certif,
                    submit_payload=submit_payload,
                )
            )

        # Verification failed
        except LocalDeviceError as exc:
            pendings.append(
                ClaqueAuSolRemotePendingEnrollment(
                    enrollment_id=enrollment_id,
                    submitted_on=submitted_on,
                    submitter_der_x509_certificate=submitter_der_x509_certificate,
                    submit_payload_signature=submit_payload_signature,
                    raw_submit_payload=raw_submit_payload,
                    error=str(exc),
                )
            )

    return pendings


async def pki_enrollment_accept_remote_pending(
    cmds: BackendAuthenticatedCmds,
    enrollment_id: UUID,
    author: LocalDevice,
    device_label: DeviceLabel,
    human_handle: HumanHandle,
    profile: UserProfile,
    public_key: PublicKey,
    verify_key: VerifyKey,
    extra_trust_roots: Iterable[Path] = (),
):
    # TODO: document exceptions !
    # Create the certificate for the new user
    user_certificate, redacted_user_certificate, device_certificate, redacted_device_certificate, user_confirmation = _create_new_user_certificates(
        author=author,
        device_label=device_label,
        human_handle=human_handle,
        profile=profile,
        public_key=public_key,
        verify_key=verify_key,
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
    rep = await cmds.pki_enrollment_accept(
        enrollment_id=enrollment_id,
        accepter_der_x509_certificate=accepter_x509_certificate.der_x509_certificate,
        accept_payload_signature=accept_payload_signature,
        accept_payload=accept_payload,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    if rep["status"] != "ok":
        # TODO: better exception !
        raise RuntimeError(f"Could not accept enrollment {enrollment_id.hex}: {rep}")
