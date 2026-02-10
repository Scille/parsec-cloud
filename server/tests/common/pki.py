# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import os
from dataclasses import dataclass, field
from pathlib import Path

import pytest
import pytest_asyncio
from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey
from cryptography.hazmat.primitives.asymmetric.types import PrivateKeyTypes

from parsec._parsec import (
    DeviceLabel,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    SigningKey,
    X509CertificateInformation,
)
from tests.common.postgresql import clear_postgresql_pki_certificate_data


@dataclass
class TestPki:
    root: dict[str, PkiCertificate]
    intermediate: dict[str, PkiCertificate]
    cert: dict[str, PkiCertificate]

    def find_cert_by_cert_der(self, der: bytes) -> PkiCertificate:
        for cert in self.cert.values():
            if cert.certificate.der == der:
                return cert
        for cert in self.intermediate.values():
            if cert.certificate.der == der:
                return cert
        for cert in self.root.values():
            if cert.certificate.der == der:
                return cert
        raise KeyError


@dataclass
class PkiCertificate:
    name: str
    cert_info: X509CertificateInformation
    certificate: Cert = field(repr=False)
    key: Key = field(repr=False)

    @classmethod
    def from_partial(cls, partial: PkiPartialCertificate) -> PkiCertificate:
        assert partial.certificate is not None
        assert partial.key is not None
        cert_info = X509CertificateInformation.load_der(partial.certificate.der)
        return PkiCertificate(
            name=partial.name,
            certificate=partial.certificate,
            key=partial.key,
            cert_info=cert_info,
        )


@dataclass
class Cert:
    der: bytes
    path: Path
    obj: x509.Certificate


@dataclass
class Key:
    der: bytes
    path: Path
    obj: PrivateKeyTypes


@dataclass
class PkiPartialCertificate:
    name: str
    certificate: Cert | None = field(repr=False, default=None)
    key: Key | None = field(repr=False, default=None)

    @classmethod
    def load_pem_certificate(cls, path: Path, name: str, pem_cert: bytes) -> PkiPartialCertificate:
        return PkiPartialCertificate(name).with_pem_certificate(path, pem_cert)

    def with_pem_certificate(self, path: Path, pem_cert: bytes) -> PkiPartialCertificate:
        cert = x509.load_pem_x509_certificate(pem_cert)
        self.certificate = Cert(cert.public_bytes(serialization.Encoding.DER), path, cert)

        return self

    @classmethod
    def load_pem_key(cls, path: Path, name: str, pem_key: bytes) -> PkiPartialCertificate:
        return PkiPartialCertificate(name).with_pem_key(path, pem_key)

    def with_pem_key(self, path: Path, pem_key: bytes) -> PkiPartialCertificate:
        key = serialization.load_pem_private_key(pem_key, password=None)
        self.key = Key(
            key.private_bytes(
                encoding=serialization.Encoding.DER,
                format=serialization.PrivateFormat.PKCS8,
                encryption_algorithm=serialization.NoEncryption(),
            ),
            path,
            key,
        )
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


async def list_files(dir: Path) -> dict[str, PkiCertificate]:
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
                    store[name] = PkiPartialCertificate.load_pem_key(path, name, content)
            case ".crt":
                if name in store:
                    store[name].with_pem_certificate(path, content)
                else:
                    store[name] = PkiPartialCertificate.load_pem_certificate(path, name, content)
            case _:
                pass  # Skip unknown suffix

    return {k: PkiCertificate.from_partial(partial) for k, partial in store.items()}


async def read_file(file: Path) -> tuple[Path, bytes]:
    content = await asyncio.to_thread(Path.read_bytes, file)
    return (file, content)


@pytest.fixture
async def clear_pki_certificate(request: pytest.FixtureRequest) -> None:
    if request.config.getoption("--postgresql"):
        await clear_postgresql_pki_certificate_data()


# @pytest.fixture
# def backend_with_test_pki_roots(test_pki: TestPki, backend: Backend):
#     trust_anchor = [TrustAnchor.from_der(cert.certificate.der) for cert in test_pki.root.values()]
#     backend.pki._config.x509_trust_anchor = trust_anchor


def sign_message(key: Key, message: bytes) -> tuple[PkiSignatureAlgorithm, bytes]:
    match key.obj:
        case RSAPrivateKey() as k:
            signature = k.sign(
                message,
                padding.PSS(
                    mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH
                ),
                hashes.SHA256(),
            )
            return (PkiSignatureAlgorithm.RSASSA_PSS_SHA256, signature)
        case _:
            raise ValueError("Unsupported key")


# @dataclass
# class PkiEnrollment:
#     enrollment_id: PKIEnrollmentID
#     submitter_trustchain: list[ParsedPkiCertificate]
#     submitter_human_handle: HumanHandle
#     submit_payload_signature: bytes
#     submit_payload_signature_algorithm: PkiSignatureAlgorithm
#     submit_payload: bytes
#     submitted_on: DateTime


@pytest.fixture(scope="session")
def submit_payload() -> bytes:
    return PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    ).dump()


# async def start_pki_enrollment(
#     now: DateTime,
#     backend: Backend,
#     org_id: OrganizationID,
#     client_cert: PkiCertificate,
#     client_intermediate_certs: list[bytes],
#     submit_payload: bytes,
#     force: bool = False,
# ) -> PkiEnrollment:
#     enrollment_id = PKIEnrollmentID.new()
#     human_handle = client_cert.cert_info.human_handle()
#     sign_algo, submit_payload_signature = sign_message(client_cert.key, submit_payload)
#     trustchain = await backend.pki.build_trustchain(
#         client_cert.certificate.der, client_intermediate_certs
#     )
#     assert isinstance(trustchain, list)

#     with backend.event_bus.spy() as spy:
#         rep = await backend.pki.submit(
#             now=now,
#             organization_id=org_id,
#             enrollment_id=enrollment_id,
#             force=force,
#             submitter_human_handle=human_handle,
#             submitter_trustchain=trustchain,
#             submit_payload_signature=submit_payload_signature,
#             submit_payload_signature_algorithm=sign_algo,
#             submit_payload=submit_payload,
#         )
#         assert rep is None, repr(rep)

#         await spy.wait_event_occurred(
#             EventPkiEnrollment(
#                 organization_id=org_id,
#             )
#         )
#     return PkiEnrollment(
#         enrollment_id=enrollment_id,
#         submitter_trustchain=trustchain,
#         submitter_human_handle=human_handle,
#         submit_payload_signature=submit_payload_signature,
#         submit_payload_signature_algorithm=sign_algo,
#         submit_payload=submit_payload,
#         submitted_on=now,
#     )


# @pytest.fixture
# async def existing_pki_enrollment(
#     coolorg: CoolorgRpcClients,
#     backend: Backend,
#     submit_payload: bytes,
#     test_pki: TestPki,
#     backend_with_test_pki_roots,
# ) -> PkiEnrollment:
#     submitter_intermediate_der_x509_certificates = []

#     return await start_pki_enrollment(
#         DateTime.now(),
#         backend,
#         coolorg.organization_id,
#         test_pki.cert["bob"],
#         submitter_intermediate_der_x509_certificates,
#         submit_payload,
#     )


# @dataclass
# class PkiAcceptedEnrollment:
#     accepted_on: DateTime
#     enrollment_id: PKIEnrollmentID
#     accepter_payload: bytes
#     accepter_payload_signature: bytes
#     accepter_payload_signature_algorithm: PkiSignatureAlgorithm
#     accepter_trustchain: list[ParsedPkiCertificate]


# async def accept_pki_enrollment(
#     now: DateTime,
#     backend: Backend,
#     org_id: OrganizationID,
#     client: AuthenticatedRpcClient,
#     client_cert: PkiCertificate,
#     client_intermediate_certs: list[bytes],
#     root_verify_key: VerifyKey,
#     enrollment: PkiEnrollment,
# ) -> PkiAcceptedEnrollment:
#     user_certificates = generate_new_user_certificates(
#         timestamp=now,
#         human_handle=enrollment.submitter_human_handle,
#         author_device_id=client.device_id,
#         author_signing_key=client.signing_key,
#     )

#     device_certificates = generate_new_device_certificates(
#         timestamp=now,
#         user_id=user_certificates.certificate.user_id,
#         device_label=DeviceLabel("Dev1"),
#         author_device_id=client.device_id,
#         author_signing_key=client.signing_key,
#     )

#     payload = PkiEnrollmentAnswerPayload(
#         user_id=user_certificates.certificate.user_id,
#         device_id=device_certificates.certificate.device_id,
#         device_label=device_certificates.certificate.device_label,
#         profile=user_certificates.certificate.profile,
#         root_verify_key=root_verify_key,
#     ).dump()
#     sign_algo, signature = sign_message(client_cert.key, payload)
#     trustchain = await backend.pki.build_trustchain(
#         client_cert.certificate.der, client_intermediate_certs
#     )
#     assert isinstance(trustchain, list)

#     outcome = await backend.pki.accept(
#         now=now,
#         organization_id=org_id,
#         author=client.device_id,
#         author_verify_key=client.signing_key.verify_key,
#         enrollment_id=enrollment.enrollment_id,
#         payload=payload,
#         payload_signature=signature,
#         payload_signature_algorithm=sign_algo,
#         accepter_trustchain=trustchain,
#         submitter_user_certificate=user_certificates.signed_certificate,
#         submitter_redacted_user_certificate=user_certificates.signed_redacted_certificate,
#         submitter_device_certificate=device_certificates.signed_certificate,
#         submitter_redacted_device_certificate=device_certificates.signed_redacted_certificate,
#     )

#     assert isinstance(outcome, tuple), repr(outcome)

#     return PkiAcceptedEnrollment(
#         enrollment_id=enrollment.enrollment_id,
#         accepted_on=now,
#         accepter_trustchain=trustchain,
#         accepter_payload=payload,
#         accepter_payload_signature=signature,
#         accepter_payload_signature_algorithm=sign_algo,
#     )
