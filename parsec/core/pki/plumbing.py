# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Iterable, Tuple, Optional
from pathlib import Path
from uuid import UUID
from importlib import import_module

import trio
from pendulum import DateTime

from parsec.core.types.backend_address import BackendPkiEnrollmentAddr
from parsec.crypto import PrivateKey, SigningKey
from parsec.api.data import PkiEnrollmentSubmitPayload, PkiEnrollmentAcceptPayload
from parsec.core.types import LocalDevice
from parsec.core.types.pki import X509Certificate, LocalPendingEnrollment


def _load_smartcard_extension():
    try:
        return import_module("parsec_ext.smartcard")
    except ModuleNotFoundError as exc:
        raise ModuleNotFoundError("Parsec smartcard extension not available") from exc


def is_pki_enrollment_available() -> bool:
    try:
        _load_smartcard_extension()
        return True
    except ModuleNotFoundError:
        return False


async def pki_enrollment_select_certificate(
    owner_hint: Optional[LocalDevice] = None
) -> X509Certificate:
    """
    Raises:
        PkiEnrollmentCertificateNotFoundError
        PkiEnrollmentCertificatePinCodeUnavailableError
        PkiEnrollmentCertificateCryptoError
        PkiEnrollmentCertificateError
    """
    extension = _load_smartcard_extension()
    # Selecting a certificate require a prompt, so run the call in a thread
    return await trio.to_thread.run_sync(
        lambda: extension.pki_enrollment_select_certificate(owner_hint=owner_hint)
    )


async def pki_enrollment_sign_payload(payload: bytes, x509_certificate: X509Certificate) -> bytes:
    """
    Raises:
        PkiEnrollmentCertificateNotFoundError
        PkiEnrollmentCertificatePinCodeUnavailableError
        PkiEnrollmentCertificateCryptoError
        PkiEnrollmentCertificateError
    """
    extension = _load_smartcard_extension()
    # Signing require a private key, so a prompt is likely to be used for unlocking it
    return await trio.to_thread.run_sync(
        lambda: extension.pki_enrollment_sign_payload(
            payload=payload, x509_certificate=x509_certificate
        )
    )


def pki_enrollment_create_local_pending(
    config_dir: Path,
    x509_certificate: X509Certificate,
    addr: BackendPkiEnrollmentAddr,
    enrollment_id: UUID,
    submitted_on: DateTime,
    submit_payload: PkiEnrollmentSubmitPayload,
    signing_key: SigningKey,
    private_key: PrivateKey,
) -> LocalPendingEnrollment:
    """
    Raises:
        PkiEnrollmentCertificateNotFoundError
        PkiEnrollmentCertificateCryptoError
        PkiEnrollmentCertificateError
        PkiEnrollmentLocalPendingCryptoError
    """
    # Encrypting the private keys is done using the certificate public key, so this should not block
    return _load_smartcard_extension().pki_enrollment_create_local_pending(
        config_dir=config_dir,
        x509_certificate=x509_certificate,
        addr=addr,
        enrollment_id=enrollment_id,
        submitted_on=submitted_on,
        submit_payload=submit_payload,
        signing_key=signing_key,
        private_key=private_key,
    )


async def pki_enrollment_load_local_pending_secret_part(
    config_dir: Path, enrollment_id: UUID
) -> Tuple[SigningKey, PrivateKey]:
    """
    Raises:
        PkiEnrollmentCertificateNotFoundError
        PkiEnrollmentCertificateCryptoError
        PkiEnrollmentCertificateError
        PkiEnrollmentCertificatePinCodeUnavailableError
        PkiEnrollmentLocalPendingCryptoError
    """
    extension = _load_smartcard_extension()
    # TODO: document exceptions !
    # Retreiving the private keys require the certificate private keys, so a pin prompt is likely to block
    return await trio.to_thread.run_sync(
        lambda: extension.pki_enrollment_load_local_pending_secret_part(
            config_dir=config_dir, enrollment_id=enrollment_id
        )
    )


def pki_enrollment_load_peer_certificate(der_x509_certificate: bytes,) -> X509Certificate:
    """
    Raises:
        PkiEnrollmentCertificateError
        PkiEnrollmentCertificateCryptoError
   """
    return _load_smartcard_extension().pki_enrollment_load_peer_certificate(der_x509_certificate)


def pki_enrollment_load_submit_payload(
    der_x509_certificate: bytes,
    payload_signature: bytes,
    payload: bytes,
    extra_trust_roots: Iterable[Path] = (),
) -> PkiEnrollmentSubmitPayload:
    """
    Raises:
        PkiEnrollmentCertificateError
        PkiEnrollmentCertificateCryptoError
        PkiEnrollmentCertificateSignatureError
        PkiEnrollmentCertificateValidationError
        PkiEnrollmentPayloadValidationError
    """
    # Verifying a payload only requires public key operations, so no blocking here
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
) -> PkiEnrollmentAcceptPayload:
    """
    Raises:
        PkiEnrollmentCertificateError
        PkiEnrollmentCertificateCryptoError
        PkiEnrollmentCertificateSignatureError
        PkiEnrollmentCertificateValidationError
        PkiEnrollmentPayloadValidationError
    """
    # Verifying a payload only requires public key operations, so no blocking here
    return _load_smartcard_extension().pki_enrollment_load_accept_payload(
        der_x509_certificate=der_x509_certificate,
        payload_signature=payload_signature,
        payload=payload,
        extra_trust_roots=extra_trust_roots,
    )
