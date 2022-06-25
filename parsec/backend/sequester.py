# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import List, Optional
from uuid import uuid4

import attr
from pendulum import DateTime
from parsec.api.data.certif import SequesterServiceKeyFormat, SequesterServiceCertificate

from parsec.api.protocol.types import OrganizationID
from parsec.event_bus import EventBus
from parsec.api.protocol.sequester import SequesterServiceType

from parsec.sequester_crypto import (
    SequesterPublicKey,
    SequesterCryptoSignatureError,
    load_sequester_public_key,
    verify_sequester,
)
from parsec.types import UUID


class SequesterError(Exception):
    pass


class SequesterSignatureError(SequesterError):
    pass


class SequesterKeyFormatError(SequesterError):
    pass


class SequesterServiceNotFound(SequesterError):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class SequesterService:
    service_type: SequesterServiceType
    service_id: UUID
    sequester_service_certificate: bytes
    sequester_service_certificate_signature: bytes
    created_on: Optional[DateTime] = None
    deleted_on: Optional[DateTime] = None


class BaseSequesterComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    async def create(self, organization_id: OrganizationID, service: SequesterService):
        """
        Raises:
            SequesterKeyForamatError
            SequesterSignatureError
        """
        sequester_encryption_key = SequesterServiceCertificate.load(
            service.sequester_service_certificate
        )
        # Assert encryption key is rsa and loadable
        if sequester_encryption_key.encryption_key_format != SequesterServiceKeyFormat.RSA:
            raise SequesterKeyFormatError(
                f"Key format {sequester_encryption_key.encryption_key_format} is not supported"
            )
        # TODO Handle errors
        load_sequester_public_key(sequester_encryption_key.encryption_key)
        await self._register_service(
            organization_id=organization_id,
            service_id=uuid4(),
            service_type=service.service_type,
            sequester_service_certificate=service.sequester_service_certificate,
            sequester_service_certificate_signature=service.sequester_service_certificate_signature,
        )

    async def update(self, organization_id: OrganizationID, service: SequesterService):
        await self.create(organization_id, service)

    async def get(self, organization_id: OrganizationID, service_id: UUID) -> SequesterService:
        """
        Raises:
            SequesterServiceNotFound
        """
        raise NotImplementedError()

    async def delete(self, organization_id: str, service_id: UUID):
        """
        Raises:
            SequesterServiceNotFound
        """
        raise NotImplementedError()

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[SequesterService]:
        """
        Raises:
            SequesterServiceNotFound
        """
        raise NotImplementedError()

    async def _register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: SequesterServiceType,
        sequester_service_certificate: bytes,
        sequester_service_certificate_signature: bytes,
    ):
        """
        Raises:
            SequesterSignatureError
        """
        raise NotImplementedError()

    def verify_sequester_signature(
        self, sequester_verify_key: SequesterPublicKey, signed_data: bytes, data: bytes
    ):
        """
        Raises:
            SequesterSignatureError
        """
        try:
            verify_sequester(sequester_verify_key, data, signed_data)
        except SequesterCryptoSignatureError:
            raise SequesterSignatureError()
