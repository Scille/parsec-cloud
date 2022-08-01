# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import sys
import subprocess
import pytest
import pendulum
from collections import defaultdict
from typing import Optional, Tuple, Iterable
from hashlib import sha1
from uuid import UUID
from pathlib import Path

from parsec.crypto import SigningKey, PrivateKey
from parsec.api.data import PkiEnrollmentSubmitPayload, PkiEnrollmentAcceptPayload, DataError
from parsec.core.types import LocalDevice, BackendPkiEnrollmentAddr
from parsec.core.local_device import (
    LocalDeviceNotFoundError,
    LocalDeviceCryptoError,
    LocalDevicePackingError,
    DeviceFileType,
    _save_device,
    _load_device,
)
from parsec.core.types.pki import X509Certificate, LocalPendingEnrollment


def create_test_certificates(tmp_path):
    """
    Requires openssl to be installed. Only used on linux when parsec_ext is installed.
    """
    # Create openssl config
    (tmp_path / "openssl.cnf").write_text("keyUsage = digitalSignature")

    # OpenSSL commands for creating the CA key, the CA certificate, the client key and the client certificate.
    commands = """\
openssl req -x509 -newkey rsa:4096 -keyout ca_key.pem -out ca_cert.pem -sha256 -days 365 -subj "/CN=My CA" -addext "keyUsage = digitalSignature" -passout pass:P@ssw0rd
openssl req -out client_request.csr -newkey rsa:4096 -keyout client_key.pem -new -passout pass:P@ssw0rd -subj "/CN=John Doe/emailAddress=john@example.com"
openssl x509 -req -days 365 -in client_request.csr -CA ca_cert.pem -CAkey ca_key.pem -passin pass:P@ssw0rd -CAcreateserial -out client_cert.pem -extfile "./openssl.cnf"
"""

    # Run the commands
    for command in commands.splitlines():
        subprocess.run(command, shell=True, check=True, cwd=tmp_path)

    # Concatenate the key and the certificate
    combined_data = (tmp_path / "client_cert.pem").read_bytes() + (
        tmp_path / "client_key.pem"
    ).read_bytes()
    (tmp_path / "client_combined.pem").write_bytes(combined_data)

    # Return certificates and password
    return tmp_path / "ca_cert.pem", tmp_path / "client_combined.pem", "P@ssw0rd"


def use_actual_parsec_ext_module(monkeypatch, tmp_path):
    """
    All tests using `mocked_parsec_ext_smartcard` can also run with the actual `parsec_ext` module if it is installed.

    It is currently not integrated in the CI but it is useful to run locally.
    """
    # We only generate test certificates for linux, that's enough for the moment
    if sys.platform == "win32":
        return pytest.skip(reason="Linux only")

    # Simply ignore this test if parsec_ext is not installed, which is typicall the case in the CI
    try:
        import parsec_ext.smartcard
    except ModuleNotFoundError:
        return pytest.skip(reason="parsec_ext not installed")

    # Generate tests certificate
    ca_cert, client_cert, password = create_test_certificates(tmp_path)

    # Patch parsec_ext to automatically select those test certificates
    monkeypatch.setattr(
        "parsec_ext.smartcard.linux.prompt_file_selection", lambda: str(client_cert)
    )
    monkeypatch.setattr("parsec_ext.smartcard.linux.prompt_password", lambda _: password)

    # Create the corresponding instance of `X509Certificate``
    der_certificate, certificate_id, certificate_sha1 = (
        parsec_ext.smartcard.get_der_encoded_certificate()
    )
    issuer, subject, _ = parsec_ext.smartcard.get_certificate_info(der_certificate)
    default_x509_certificate = X509Certificate(
        issuer=issuer,
        subject=subject,
        der_x509_certificate=der_certificate,
        certificate_id=certificate_id,
        certificate_sha1=certificate_sha1,
    )

    # Add two attributes to the smartcard modules that are going to be used the tests
    monkeypatch.setattr(
        "parsec_ext.smartcard.default_x509_certificate", default_x509_certificate, raising=False
    )
    monkeypatch.setattr("parsec_ext.smartcard.default_trust_root_path", ca_cert, raising=False)

    # Return the non-mocked module
    return parsec_ext.smartcard


@pytest.fixture(params=(True, False), ids=("mock_parsec_ext", "use_parsec_ext"))
def mocked_parsec_ext_smartcard(monkeypatch, request, tmp_path):
    if not request.param:
        return use_actual_parsec_ext_module(monkeypatch, tmp_path)

    class MockedParsecExtSmartcard:
        def __init__(self):
            self._pending_enrollments = defaultdict(list)
            der_x509_certificate = b"der_X509_certificate:42:john@example.com:John Doe:My CA"
            self.default_trust_root_path = "some_path.pem"
            self.default_x509_certificate = X509Certificate(
                issuer={"common_name": "My CA"},
                subject={"common_name": "John Doe", "email_address": "john@example.com"},
                der_x509_certificate=der_x509_certificate,
                certificate_sha1=sha1(der_x509_certificate).digest(),
                certificate_id=42,
            )

        @staticmethod
        def _compute_signature(der_x509_certificate: bytes, payload: bytes) -> bytes:
            return sha1(payload + der_x509_certificate).digest()  # 100% secure crypto \o/

        def pki_enrollment_select_certificate(
            self, owner_hint: Optional[LocalDevice] = None
        ) -> X509Certificate:
            return self.default_x509_certificate

        def pki_enrollment_sign_payload(
            self, payload: bytes, x509_certificate: X509Certificate
        ) -> bytes:
            return self._compute_signature(x509_certificate.der_x509_certificate, payload)

        def pki_enrollment_create_local_pending(
            self,
            config_dir: Path,
            x509_certificate: X509Certificate,
            addr: BackendPkiEnrollmentAddr,
            enrollment_id: UUID,
            submitted_on: pendulum.DateTime,
            submit_payload: PkiEnrollmentSubmitPayload,
            signing_key: SigningKey,
            private_key: PrivateKey,
        ):
            pending = LocalPendingEnrollment(
                x509_certificate=x509_certificate,
                addr=addr,
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submit_payload=submit_payload,
                encrypted_key=b"123",
                ciphertext=b"123",
            )
            secret_part = (signing_key, private_key)
            self._pending_enrollments[config_dir].append((pending, secret_part))
            return pending

        def pki_enrollment_load_local_pending_secret_part(
            self, config_dir: Path, enrollment_id: UUID
        ) -> Tuple[SigningKey, PrivateKey]:
            for (pending, secret_part) in self._pending_enrollments[config_dir]:
                if pending.enrollment_id == enrollment_id:
                    return secret_part
            else:
                raise LocalDeviceNotFoundError()

        def pki_enrollment_load_submit_payload(
            self,
            der_x509_certificate: bytes,
            payload_signature: bytes,
            payload: bytes,
            extra_trust_roots: Iterable[Path] = (),
        ) -> PkiEnrollmentSubmitPayload:
            computed_signature = self._compute_signature(der_x509_certificate, payload)
            if computed_signature != payload_signature:
                raise LocalDeviceCryptoError()
            try:
                submit_payload = PkiEnrollmentSubmitPayload.load(payload)
            except DataError as exc:
                raise LocalDevicePackingError(str(exc)) from exc

            return submit_payload

        def pki_enrollment_load_accept_payload(
            self,
            der_x509_certificate: bytes,
            payload_signature: bytes,
            payload: bytes,
            extra_trust_roots: Iterable[Path] = (),
        ) -> PkiEnrollmentAcceptPayload:
            computed_signature = self._compute_signature(der_x509_certificate, payload)
            if computed_signature != payload_signature:
                raise LocalDeviceCryptoError()
            try:
                accept_payload = PkiEnrollmentAcceptPayload.load(payload)
            except DataError as exc:
                raise LocalDevicePackingError(str(exc)) from exc

            return accept_payload

        def save_device_with_smartcard(
            self,
            key_file: Path,
            device: LocalDevice,
            force: bool = False,
            certificate_id: Optional[str] = None,
            certificate_sha1: Optional[bytes] = None,
        ) -> None:
            def _encrypt_dump(cleartext: bytes) -> Tuple[DeviceFileType, bytes, dict]:
                extra_args = {
                    "encrypted_key": b"123",
                    "certificate_id": certificate_id,
                    "certificate_sha1": certificate_sha1,
                }
                return DeviceFileType.SMARTCARD, cleartext, extra_args

            _save_device(key_file, device, force, _encrypt_dump)

        def load_device_with_smartcard(self, key_file: Path) -> LocalDevice:
            def _decrypt_ciphertext(data: dict) -> bytes:
                return data["ciphertext"]

            return _load_device(key_file, _decrypt_ciphertext)

        def pki_enrollment_load_peer_certificate(
            self, der_x509_certificate: bytes
        ) -> X509Certificate:
            if not der_x509_certificate.startswith(b"der_X509_certificate:"):
                # Consider the certificate invalid
                raise LocalDeviceCryptoError()

            _, certificate_id, subject_email, subject_cn, issuer_cn = der_x509_certificate.decode(
                "utf8"
            ).split(":")
            return X509Certificate(
                issuer={"common_name": issuer_cn},
                subject={"email_address": subject_email, "common_name": subject_cn},
                der_x509_certificate=der_x509_certificate,
                certificate_sha1=sha1(der_x509_certificate).digest(),
                certificate_id=certificate_id,
            )

    mocked_parsec_ext_smartcard = MockedParsecExtSmartcard()

    def _mocked_load_smartcard_extension():
        return mocked_parsec_ext_smartcard

    monkeypatch.setattr(
        "parsec.core.pki.plumbing._load_smartcard_extension", _mocked_load_smartcard_extension
    )
    monkeypatch.setattr(
        "parsec.core.local_device._load_smartcard_extension", _mocked_load_smartcard_extension
    )
    return mocked_parsec_ext_smartcard
