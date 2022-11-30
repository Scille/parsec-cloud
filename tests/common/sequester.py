# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass

from parsec._parsec import (
    DateTime,
    SequesterPrivateKeyDer,
    SequesterPublicKeyDer,
    SequesterSigningKeyDer,
    SequesterVerifyKeyDer,
    SigningKey,
)
from parsec.api.data import SequesterAuthorityCertificate, SequesterServiceCertificate
from parsec.api.protocol import SequesterServiceID
from parsec.backend.sequester import (
    BaseSequesterService,
    SequesterServiceType,
    StorageSequesterService,
    WebhookSequesterService,
)


@dataclass
class SequesterAuthorityFullData:
    certif: bytes
    certif_data: SequesterAuthorityCertificate
    signing_key: SequesterSigningKeyDer
    verify_key: SequesterVerifyKeyDer


def sequester_authority_factory(
    organization_root_signing_key: SigningKey, timestamp: DateTime | None = None
) -> SequesterAuthorityFullData:
    timestamp = timestamp or DateTime.now()
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    signing_key, verify_key = SequesterSigningKeyDer.generate_pair(1024)
    certif = SequesterAuthorityCertificate(
        timestamp=timestamp,
        verify_key_der=verify_key,
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
    decryption_key: SequesterPrivateKeyDer
    encryption_key: SequesterPublicKeyDer
    backend_service: BaseSequesterService

    @property
    def service_id(self) -> SequesterServiceID:
        return self.certif_data.service_id


def sequester_service_factory(
    label: str,
    authority: SequesterAuthorityFullData,
    timestamp: DateTime | None = None,
    service_type: SequesterServiceType = SequesterServiceType.STORAGE,
    webhook_url: str | None = None,
) -> SequesterServiceFullData:
    timestamp = timestamp or DateTime.now()
    # Don't use such a small key size in real world, this is only for test !
    # (RSA key generation gets ~10x slower between 1024 and 4096)
    decryption_key, encryption_key = SequesterPrivateKeyDer.generate_pair(1024)
    certif_data = SequesterServiceCertificate(
        service_id=SequesterServiceID.new(),
        timestamp=timestamp,
        service_label=label,
        encryption_key_der=encryption_key,
    )
    certif = authority.signing_key.sign(certif_data.dump())
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
