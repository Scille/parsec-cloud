# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import attr
from parsec.api.protocol.sequester import SequesterServiceType
from parsec.api.protocol.types import OrganizationID
from parsec.backend.memory.organization import MemoryOrganizationComponent
from parsec.backend.sequester import BaseSequesterComponent, verify_sequester_der_signature

from uuid import UUID
from typing import Dict


@attr.s(slots=True, auto_attribs=True)
class SequesterService:
    service_id: UUID
    service_type: SequesterServiceType
    sequester_encryption_key_payload: bytes
    sequester_encryption_key_payload_signature: bytes


class MemorySequesterComponent(BaseSequesterComponent):
    def __init__(self, send_event):
        self._send_event = send_event
        self._organization_component: MemoryOrganizationComponent = None
        self._services: Dict[OrganizationID, Dict[UUID, SequesterService]] = {}

    def register_components(self, **other_components):
        self._organization_component = other_components["organization"]

    async def register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: SequesterServiceType,
        sequester_encryption_key_payload: bytes,
        sequester_encryption_key_payload_signature: bytes,
    ):
        organization = await self._organization_component.get(organization_id)
        verify_sequester_der_signature(
            organization.sequester_verify_key,
            sequester_encryption_key_payload,
            sequester_encryption_key_payload_signature,
        )

        self._services[organization_id][service_id] = SequesterService(
            service_id=service_id,
            service_type=service_type,
            sequester_encryption_key_payload=sequester_encryption_key_payload,
            sequester_encryption_key_payload_signature=sequester_encryption_key_payload_signature,
        )
