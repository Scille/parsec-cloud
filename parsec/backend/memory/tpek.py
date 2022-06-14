# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import attr
from parsec.api.protocol.tpek import TpekServiceType
from parsec.api.protocol.types import OrganizationID
from parsec.backend.memory.organization import MemoryOrganizationComponent
from parsec.backend.tpek import BaseTpekComponent, verify_tpek_der_signature

from uuid import UUID
from typing import Dict


@attr.s(slots=True, auto_attribs=True)
class TpekService:
    service_id: UUID
    service_type: TpekServiceType
    tpek_encryption_key_payload: bytes
    tpek_encryption_key_payload_signature: bytes


class MemoryTpekComponent(BaseTpekComponent):
    def __init__(self, send_event):
        self._send_event = send_event
        self._organization_component: MemoryOrganizationComponent = None
        self._services: Dict[OrganizationID, Dict[UUID, TpekService]] = {}

    def register_components(self, **other_components):
        self._organization_component = other_components["organization"]

    async def register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: TpekServiceType,
        tpek_encryption_key_payload: bytes,
        tpek_encryption_key_payload_signature: bytes,
    ):
        organization = await self._organization_component.get(organization_id)
        verify_tpek_der_signature(
            organization.tpek_verify_key,
            tpek_encryption_key_payload,
            tpek_encryption_key_payload_signature,
        )

        self._services[organization_id][service_id] = TpekService(
            service_id=service_id,
            service_type=service_type,
            tpek_encryption_key_payload=tpek_encryption_key_payload,
            tpek_encryption_key_payload_signature=tpek_encryption_key_payload_signature,
        )
