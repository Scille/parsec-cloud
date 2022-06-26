# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from dataclasses import dataclass
from pendulum import now as pendulum_now
import oscrypto.asymmetric

from parsec.crypto import SigningKey
from parsec.api.data.certif import (
    SequesterAuthorityKeyCertificate,
    SequesterAuthorityKeyFormat,
    SequesterServiceCertificate,
    SequesterServiceKeyFormat,
)
from parsec.sequester_crypto import sign_service_certificate


@dataclass
class SequesterAuthorityFullData:
    signed_certif: bytes
    certif: SequesterAuthorityKeyCertificate
    signing_key: oscrypto.asymmetric.PrivateKey
    verify_key: oscrypto.asymmetric.PublicKey


def sequester_authority_factory(
    organization_root_signing_key: SigningKey
) -> SequesterAuthorityFullData:
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    verify_key, signing_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=1024)
    certif = SequesterAuthorityKeyCertificate(
        author=None,
        timestamp=pendulum_now(),
        verify_key=oscrypto.asymmetric.dump_public_key(verify_key, encoding="der"),
        verify_key_format=SequesterAuthorityKeyFormat.RSA,
    )
    return SequesterAuthorityFullData(
        signed_certif=certif.dump_and_sign(organization_root_signing_key),
        certif=certif,
        signing_key=signing_key,
        verify_key=verify_key,
    )


@dataclass
class SequesterServiceFullData:
    certif_signature: bytes
    certif_dumped: bytes
    certif: SequesterAuthorityKeyCertificate
    decryption_key: oscrypto.asymmetric.PrivateKey
    encryption_key: oscrypto.asymmetric.PublicKey


def sequester_service_factory(
    name: str, authority: SequesterAuthorityFullData
) -> SequesterServiceFullData:
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    encryption_key, decryption_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=1024)
    certif = SequesterServiceCertificate(
        encryption_key=oscrypto.asymmetric.dump_public_key(encryption_key, encoding="der"),
        encryption_key_format=SequesterServiceKeyFormat.RSA,
        timestamp=pendulum_now(),
        service_name=name,
    )
    certif_dumped = certif.dump()
    return SequesterServiceFullData(
        certif_signature=sign_service_certificate(
            certificate=certif_dumped,
            raw_signing_key=oscrypto.asymmetric.dump_private_key(decryption_key, passphrase=None),
        ),
        certif_dumped=certif_dumped,
        certif=certif,
        decryption_key=decryption_key,
        encryption_key=encryption_key,
    )
