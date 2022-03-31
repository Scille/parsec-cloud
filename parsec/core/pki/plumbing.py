# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Iterable, Tuple, List, Optional, Dict
from pathlib import Path
from hashlib import sha1
from uuid import UUID
from importlib import import_module
from pendulum import DateTime
import attr
from parsec.core.types.backend_address import BackendPkiEnrollmentAddr

from parsec.crypto import PrivateKey, SigningKey
from parsec.api.data import PkiEnrollmentSubmitPayload, PkiEnrollmentAcceptPayload
from parsec.core.types import LocalDevice


def _load_smartcard_extension():
    try:
        return import_module("parsec_ext.smartcard")
    except ModuleNotFoundError as exc:
        raise ModuleNotFoundError("Parsec smartcard extension not available") from exc


@attr.s(slots=True, frozen=True, auto_attribs=True)
class X509Certificate:
    issuer: Dict[str, str]
    subject: Dict[str, str]
    der_x509_certificate: bytes
    certificate_sha1: bytes
    certificate_id: str

    @property
    def subject_common_name(self) -> Optional[str]:
        return self.subject.get("common_name")

    @property
    def subject_email_address(self) -> Optional[str]:
        return self.subject.get("email_address")

    @property
    def issuer_common_name(self) -> Optional[str]:
        return self.issuer.get("common_name")


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


def pki_enrollment_load_local_pending_secret_part(
    config_dir: Path, enrollment_id: UUID
) -> Tuple[SigningKey, PrivateKey]:
    """
    This will prompt PIN dialog
    """
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_load_local_pending_secret_part(
        config_dir=config_dir, enrollment_id=enrollment_id
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalPendingEnrollment:
    x509_certificate: X509Certificate
    addr: BackendPkiEnrollmentAddr
    # This `submitted_on` is not strictly in line with the submitted_on stored on the backend
    submitted_on: DateTime
    enrollment_id: UUID
    submit_payload: PkiEnrollmentSubmitPayload


def pki_enrollment_list_local_pendings(config_dir: Path) -> List[LocalPendingEnrollment]:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_list_local_pendings(config_dir=config_dir)


def pki_enrollment_remove_local_pending(config_dir: Path, enrollment_id: UUID) -> None:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_remove_local_pending(
        config_dir=config_dir, enrollment_id=enrollment_id
    )


def pki_enrollment_load_submit_payload(
    der_x509_certificate: bytes,
    payload_signature: bytes,
    payload: bytes,
    extra_trust_roots: Iterable[Path] = (),
) -> Tuple[X509Certificate, PkiEnrollmentSubmitPayload]:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_load_submit_payload(
        der_x509_certificate=der_x509_certificate,
        payload_signature=payload_signature,
        payload=payload,
        extra_trust_roots=extra_trust_roots,
    )


def pki_enrollment_load_accept_payload(
    der_x509_certificate: bytes,
    payload_signature: bytes,
    payload: bytes,
    extra_trust_roots: Iterable[Path] = (),
) -> Tuple[X509Certificate, PkiEnrollmentAcceptPayload]:
    # TODO: document exceptions !
    return _load_smartcard_extension().pki_enrollment_load_accept_payload(
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
