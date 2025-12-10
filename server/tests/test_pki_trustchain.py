# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec.backend import Backend
from parsec.components.pki import parse_pki_cert
from tests.common.pki import TestPki


async def test_build_trustchain_only_leaf(backend: Backend, test_pki: TestPki) -> None:
    bob_cert = test_pki.cert["bob"].der_certificate
    trustchain = await backend.pki.build_trustchain(bob_cert, [])
    assert trustchain == [parse_pki_cert(bob_cert)]


async def test_build_trustchain_useless_intermediate_certs(
    backend: Backend, test_pki: TestPki
) -> None:
    """
    The returned trustchain should only contain certificates that are part of the chain.
    """
    bob_cert = test_pki.cert["bob"].der_certificate
    trustchain = await backend.pki.build_trustchain(
        bob_cert,
        [test_pki.cert["mallory-sign"].der_certificate, test_pki.cert["old-boby"].der_certificate],
    )
    assert trustchain == [parse_pki_cert(bob_cert)]


async def test_build_trustchain_with_intermediate_certificate(
    backend: Backend, test_pki: TestPki
) -> None:
    mallory_cert = test_pki.cert["mallory-sign"].der_certificate
    glados_dt_cert = test_pki.intermediate["glados_dev_team"].der_certificate
    trustchain = await backend.pki.build_trustchain(
        mallory_cert,
        [glados_dt_cert],
    )
    parsed_glados = parse_pki_cert(glados_dt_cert)
    parsed_mallory = parse_pki_cert(mallory_cert, signed_by=parsed_glados.fingerprint_sha256)
    assert trustchain == [parsed_mallory, parsed_glados]


async def test_build_trustchain_with_intermediate_certificates(
    backend: Backend, test_pki: TestPki
) -> None:
    """
    We test with multiple intermediate certificates to verify if the trustchain list of correctly ordered:
        Leaf -signed_by-> Parent -signed_by-> Parent -signed_by-> ...
    """
    mallory_cert = test_pki.cert["mallory-sign"].der_certificate
    glados_dt_cert = test_pki.intermediate["glados_dev_team"].der_certificate
    aperture_cert = test_pki.root["aperture_science"].der_certificate
    trustchain = await backend.pki.build_trustchain(
        mallory_cert,
        [aperture_cert, glados_dt_cert],
    )
    parsed_aperture = parse_pki_cert(aperture_cert)
    parsed_glados = parse_pki_cert(glados_dt_cert, signed_by=parsed_aperture.fingerprint_sha256)
    parsed_mallory = parse_pki_cert(mallory_cert, signed_by=parsed_glados.fingerprint_sha256)
    assert trustchain == [parsed_mallory, parsed_glados, parsed_aperture]


async def test_build_trustchain_with_intermediate_certificates_and_useless_cert(
    backend: Backend, test_pki: TestPki
) -> None:
    """
    Verify that useless cert does not cause problem when building a trustchain with valid intermediate certificates
    """
    mallory_cert = test_pki.cert["mallory-sign"].der_certificate
    glados_dt_cert = test_pki.intermediate["glados_dev_team"].der_certificate
    aperture_cert = test_pki.root["aperture_science"].der_certificate
    trustchain = await backend.pki.build_trustchain(
        mallory_cert,
        [aperture_cert, test_pki.cert["bob"].der_certificate, glados_dt_cert],
    )
    parsed_aperture = parse_pki_cert(aperture_cert)
    parsed_glados = parse_pki_cert(glados_dt_cert, signed_by=parsed_aperture.fingerprint_sha256)
    parsed_mallory = parse_pki_cert(mallory_cert, signed_by=parsed_glados.fingerprint_sha256)
    assert trustchain == [parsed_mallory, parsed_glados, parsed_aperture]
