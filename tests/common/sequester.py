# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

import oscrypto.asymmetric

from parsec._parsec import DateTime
from parsec.api.data import SequesterAuthorityCertificate, SequesterServiceCertificate
from parsec.api.protocol import SequesterServiceID
from parsec.backend.sequester import (
    BaseSequesterService,
    SequesterServiceType,
    StorageSequesterService,
    WebhookSequesterService,
)
from parsec.crypto import SigningKey
from parsec.sequester_crypto import (
    SequesterEncryptionKeyDer,
    SequesterVerifyKeyDer,
    sequester_authority_sign,
)


@dataclass
class SequesterAuthorityFullData:
    certif: bytes
    certif_data: SequesterAuthorityCertificate
    signing_key: oscrypto.asymmetric.PrivateKey
    verify_key: oscrypto.asymmetric.PublicKey


def sequester_authority_factory(
    organization_root_signing_key: SigningKey, timestamp: Optional[DateTime] = None
) -> SequesterAuthorityFullData:
    timestamp = timestamp or DateTime.now()
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
    backend_service: BaseSequesterService

    @property
    def service_id(self) -> SequesterServiceID:
        return self.certif_data.service_id


def sequester_service_factory(
    label: str,
    authority: SequesterAuthorityFullData,
    timestamp: Optional[DateTime] = None,
    service_type: SequesterServiceType = SequesterServiceType.STORAGE,
    webhook_url: Optional[str] = None,
) -> SequesterServiceFullData:
    timestamp = timestamp or DateTime.now()
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
    if service_type == SequesterServiceType.STORAGE:
        assert webhook_url is None
        backend_service = StorageSequesterService(
            service_id=certif_data.service_id,
            service_label=certif_data.service_label,
            service_certificate=certif,
        )
    else:
        assert service_type == SequesterServiceType.WEBHOOK
        assert webhook_url is not None
        backend_service = WebhookSequesterService(
            service_id=certif_data.service_id,
            service_label=certif_data.service_label,
            service_certificate=certif,
            webhook_url=webhook_url,
        )
    return SequesterServiceFullData(
        certif=certif,
        certif_data=certif_data,
        decryption_key=decryption_key,
        encryption_key=encryption_key,
        backend_service=backend_service,
    )
