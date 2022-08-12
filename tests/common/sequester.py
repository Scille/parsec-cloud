# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Optional
from dataclasses import dataclass
from pendulum import now as pendulum_now, DateTime
import oscrypto.asymmetric

from parsec.crypto import SigningKey
from parsec.sequester_crypto import (
    sequester_authority_sign,
    SequesterVerifyKeyDer,
    SequesterEncryptionKeyDer,
)
from parsec.api.data import SequesterAuthorityCertificate, SequesterServiceCertificate
from parsec.api.protocol import SequesterServiceID
from parsec.backend.sequester import SequesterService, SequesterServiceType


@dataclass
class SequesterAuthorityFullData:
    certif: bytes
    certif_data: SequesterAuthorityCertificate
    signing_key: oscrypto.asymmetric.PrivateKey
    verify_key: oscrypto.asymmetric.PublicKey


def sequester_authority_factory(
    organization_root_signing_key: SigningKey, timestamp: Optional[DateTime] = None
) -> SequesterAuthorityFullData:
    timestamp = timestamp or pendulum_now()
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    verify_key, signing_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=1024)
    certif = SequesterAuthorityCertificate(
        author=None,
        timestamp=timestamp,
        verify_key_der=SequesterVerifyKeyDer(
            oscrypto.asymmetric.dump_public_key(verify_key, encoding="der")
        ),
    )
    return SequesterAuthorityFullData(
        certif=certif.dump_and_sign(organization_root_signing_key),
        certif_data=certif,
        signing_key=signing_key,
        verify_key=verify_key,
    )


@dataclass
class SequesterServiceFullData:
    certif: bytes
    certif_data: SequesterServiceCertificate
    decryption_key: oscrypto.asymmetric.PrivateKey
    encryption_key: oscrypto.asymmetric.PublicKey
    backend_service: SequesterService

    @property
    def service_id(self) -> SequesterServiceID:
        return self.certif_data.service_id


def sequester_service_factory(
    label: str, authority: SequesterAuthorityFullData, timestamp: Optional[DateTime] = None
) -> SequesterServiceFullData:
    timestamp = timestamp or pendulum_now()
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    encryption_key, decryption_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=1024)
    certif_data = SequesterServiceCertificate(
        service_id=SequesterServiceID.new(),
        timestamp=timestamp,
        service_label=label,
        encryption_key_der=SequesterEncryptionKeyDer(
            oscrypto.asymmetric.dump_public_key(encryption_key, encoding="der")
        ),
    )
    certif = sequester_authority_sign(signing_key=authority.signing_key, data=certif_data.dump())
    return SequesterServiceFullData(
        certif=certif,
        certif_data=certif_data,
        decryption_key=decryption_key,
        encryption_key=encryption_key,
        backend_service=SequesterService(
            service_id=certif_data.service_id,
            service_label=certif_data.service_label,
            service_certificate=certif,
            service_type=SequesterServiceType.STORAGE,
        ),
    )
