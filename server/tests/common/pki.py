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

from parsec._parsec import X509CertificateInformation
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
    der_key: bytes = field(repr=False)

    @classmethod
    def from_partial(cls, partial: PartialCertificate) -> Certificate:
        assert partial.der_certificate is not None
        assert partial.der_key is not None
        cert_info = X509CertificateInformation.load_der(partial.der_certificate)
        return Certificate(
            name=partial.name,
            der_certificate=partial.der_certificate,
            der_key=partial.der_key,
            cert_info=cert_info,
        )


@dataclass
class PartialCertificate:
    name: str
    der_certificate: bytes | None = field(repr=False, default=None)
    der_key: bytes | None = field(repr=False, default=None)

    @classmethod
    def load_pem_certificate(cls, name: str, pem_cert: bytes) -> PartialCertificate:
        return PartialCertificate(name).with_pem_certificate(pem_cert)

    def with_pem_certificate(self, pem_cert: bytes) -> PartialCertificate:
        cert = x509.load_pem_x509_certificate(pem_cert)
        self.der_certificate = cert.public_bytes(serialization.Encoding.DER)

        return self

    @classmethod
    def load_pem_key(cls, name: str, pem_key: bytes) -> PartialCertificate:
        return PartialCertificate(name).with_pem_key(pem_key)

    def with_pem_key(self, pem_key: bytes) -> PartialCertificate:
        key = serialization.load_pem_private_key(pem_key, password=None)
        self.der_key = key.private_bytes(
            encoding=serialization.Encoding.DER,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption(),
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


async def list_files(dir: Path) -> dict[str, Certificate]:
    files = map(Path, await asyncio.to_thread(os.listdir, dir))

    file_n_content = await asyncio.gather(*(read_file(dir / file) for file in files))

    store = {}
    for path, content in file_n_content:
        name = path.stem
        match path.suffix:
            case ".key":
                if name in store:
                    store[name].with_pem_key(content)
                else:
                    store[name] = PartialCertificate.load_pem_key(name, content)
            case ".crt":
                if name in store:
                    store[name].with_pem_certificate(content)
                else:
                    store[name] = PartialCertificate.load_pem_certificate(name, content)
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
