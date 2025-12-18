# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import os
from dataclasses import dataclass, field
from pathlib import Path

import pytest
import pytest_asyncio
from cryptography import x509
from cryptography.hazmat.primitives import serialization

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EmailAddress,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    SigningKey,
    X509CertificateInformation,
    anonymous_cmds,
)
from parsec.events import EventPkiEnrollment
from tests.common import Backend, CoolorgRpcClients
from tests.common.postgresql import clear_postgresql_pki_certificate_data


@dataclass
class TestPki:
    root: dict[str, Certificate]
    intermediate: dict[str, Certificate]
    cert: dict[str, Certificate]


@dataclass
class Certificate:
    name: str
    cert_info: X509CertificateInformation
    der_certificate: bytes = field(repr=False)
    certificate_path: Path
    der_key: bytes = field(repr=False)
    key_path: Path

    @classmethod
    def from_partial(cls, partial: PartialCertificate) -> Certificate:
        assert partial.der_certificate is not None
        assert partial.certificate_path is not None
        assert partial.der_key is not None
        assert partial.key_path is not None
        cert_info = X509CertificateInformation.load_der(partial.der_certificate)
        return Certificate(
            name=partial.name,
            der_certificate=partial.der_certificate,
            certificate_path=partial.certificate_path,
            der_key=partial.der_key,
            key_path=partial.key_path,
            cert_info=cert_info,
        )


@dataclass
class PartialCertificate:
    name: str
    der_certificate: bytes | None = field(repr=False, default=None)
    certificate_path: Path | None = None
    der_key: bytes | None = field(repr=False, default=None)
    key_path: Path | None = None

    @classmethod
    def load_pem_certificate(cls, path: Path, name: str, pem_cert: bytes) -> PartialCertificate:
        return PartialCertificate(name).with_pem_certificate(path, pem_cert)

    def with_pem_certificate(self, path: Path, pem_cert: bytes) -> PartialCertificate:
        cert = x509.load_pem_x509_certificate(pem_cert)
        self.der_certificate = cert.public_bytes(serialization.Encoding.DER)
        self.certificate_path = path

        return self

    @classmethod
    def load_pem_key(cls, path: Path, name: str, pem_key: bytes) -> PartialCertificate:
        return PartialCertificate(name).with_pem_key(path, pem_key)

    def with_pem_key(self, path: Path, pem_key: bytes) -> PartialCertificate:
        key = serialization.load_pem_private_key(pem_key, password=None)
        self.der_key = key.private_bytes(
            encoding=serialization.Encoding.DER,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption(),
        )
        self.key_path = path
        return self


def test_pki_dir() -> Path:
    match os.environ.get("TEST_PKI_DIR"):
        case str() as dir:
            return Path(dir)
        case None:
            return Path(__file__).parent / "../../../libparsec/crates/platform_pki/test-pki"


@pytest_asyncio.fixture(scope="session", loop_scope="session")
async def test_pki() -> TestPki:
    dir = test_pki_dir()

    cert_dir = dir / "Cert"
    intermediate_dir = dir / "Intermediate"
    root_dir = dir / "Root"

    cert, intermediate, root = await asyncio.gather(
        list_files(cert_dir), list_files(intermediate_dir), list_files(root_dir)
    )

    return TestPki(root=root, intermediate=intermediate, cert=cert)


async def list_files(dir: Path) -> dict[str, Certificate]:
    files = map(Path, await asyncio.to_thread(os.listdir, dir))

    file_n_content = await asyncio.gather(*(read_file(dir / file) for file in files))

    store = {}
    for path, content in file_n_content:
        name = path.stem
        match path.suffix:
            case ".key":
                if name in store:
                    store[name].with_pem_key(path, content)
                else:
                    store[name] = PartialCertificate.load_pem_key(path, name, content)
            case ".crt":
                if name in store:
                    store[name].with_pem_certificate(path, content)
                else:
                    store[name] = PartialCertificate.load_pem_certificate(path, name, content)
            case _:
                print(f"ignoring {name}: unknown suffix {path.suffix} ({path=})")
                pass

    return {k: Certificate.from_partial(partial) for k, partial in store.items()}


async def read_file(file: Path) -> tuple[Path, bytes]:
    content = await asyncio.to_thread(Path.read_bytes, file)
    return (file, content)


@pytest.fixture
async def clear_pki_certificate(request: pytest.FixtureRequest) -> None:
    if request.config.getoption("--postgresql"):
        await clear_postgresql_pki_certificate_data()


@pytest.fixture(scope="session")
def submit_payload() -> bytes:
    return PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    ).dump()


@dataclass
class Enrollment:
    enrollment_id: PKIEnrollmentID
    submitter_der_x509_certificate: bytes
    submitter_intermediate_der_x509_certificates: list[bytes]
    submitter_der_x509_certificate_email: EmailAddress
    submit_payload_signature: bytes
    submit_payload: bytes
    submitted_on: DateTime


@pytest.fixture
async def existing_enrollment(
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes, test_pki: TestPki
) -> Enrollment:
    enrollment_id = PKIEnrollmentID.new()
    submitter_der_x509_certificate = test_pki.cert["bob"].der_certificate
    submitter_intermediate_der_x509_certificates = []
    submitter_der_x509_certificate_email = EmailAddress("mike@example.invalid")
    submit_payload_signature = b"<mike submit payload signature>"

    with backend.event_bus.spy() as spy:
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=False,
            der_x509_certificate=submitter_der_x509_certificate,
            intermediate_der_x509_certificates=submitter_intermediate_der_x509_certificates,
            payload_signature=submit_payload_signature,
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )
        assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)

        await spy.wait_event_occurred(
            EventPkiEnrollment(
                organization_id=coolorg.organization_id,
            )
        )
    return Enrollment(
        enrollment_id=enrollment_id,
        submitter_der_x509_certificate=submitter_der_x509_certificate,
        submitter_intermediate_der_x509_certificates=submitter_intermediate_der_x509_certificates,
        submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
        submit_payload_signature=submit_payload_signature,
        submit_payload=submit_payload,
        submitted_on=rep.submitted_on,
    )
