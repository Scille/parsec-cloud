# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import UUID, uuid4

import attr
from parsec.api.data.sequester import EncryptionKeyFormat, SequesterServiceEncryptionKey

from parsec.api.protocol.types import OrganizationID
from parsec.event_bus import EventBus
from parsec.api.protocol.sequester import SequesterServiceSchema, SequesterServiceType

from parsec.sequester_crypto import (
    SequesterPublicKey,
    SequesterCryptoSignatureError,
    load_sequester_public_key,
    verify_sequester,
)
from parsec.types import UUID4


class SequesterError(Exception):
    pass


class SequesterSignatureError(SequesterError):
    pass


class SequesterKeyFormatError(SequesterError):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class SequesterServiceRequest:
    service_type: SequesterServiceType
    service_id: UUID4
    sequester_encryption_key: bytes
    sequester_encryption_key_signature: bytes


class BaseSequesterComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    async def register_service(
        self, organization_id: OrganizationID, sequester_register_service: SequesterServiceRequest
    ):
        """
        Raises:
            SequesterKeyForamatError
            SequesterSignatureError
        """
        sequester_encryption_key = SequesterServiceEncryptionKey.load(
            sequester_register_service.sequester_encryption_key
        )
        # Assert encryption key is rsa and loadable
        if not sequester_encryption_key.encryption_key_format != EncryptionKeyFormat.RSA:
            raise SequesterKeyFormatError(
                f"Key format {sequester_encryption_key.encryption_key_format} is not supported"
            )

        load_sequester_public_key(sequester_encryption_key.encryption_key)
        self._register_service(
            organization_id=organization_id,
            service_id=uuid4(),
            service_type=sequester_register_service.service_type,
            sequester_encryption_key=sequester_register_service.sequester_encryption_key,
            sequester_encryption_key_signature=sequester_register_service.sequester_encryption_key_signature,
        )

    async def get(self, organization_id: OrganizationID) -> SequesterServiceSchema:
        pass

    async def _register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: SequesterServiceType,
        sequester_encryption_key: bytes,
        sequester_encryption_key_signature: bytes,
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
