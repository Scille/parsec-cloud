# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from collections import defaultdict
import pendulum
from parsec.api.protocol.sequester import SequesterServiceType
from parsec.api.protocol.types import OrganizationID
from parsec.backend.memory.organization import MemoryOrganizationComponent
from parsec.backend.sequester import (
    BaseSequesterComponent,
    SequesterService,
    SequesterServiceNotFound,
)

from uuid import UUID
from typing import Dict, List

from parsec.sequester_crypto import SequesterCryptoError


class MemorySequesterComponent(BaseSequesterComponent):
    def __init__(self, send_event):
        self._send_event = send_event
        self._organization_component: MemoryOrganizationComponent = None
        self._services: Dict[OrganizationID, Dict[UUID, SequesterService]] = defaultdict(dict)

    def register_components(self, **other_components):
        self._organization_component = other_components["organization"]

    async def _register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: SequesterServiceType,
        sequester_service_certificate: bytes,
        sequester_service_certificate_signature: bytes,
    ):

        organization = await self._organization_component.get(organization_id)
        if organization.sequester_authority_key_certificate is None:
            raise SequesterCryptoError("Organization does not accept extra services")
        authority_key = organization.sequester_authority_key_certificate.verify_key
        self.verify_sequester_signature(
            authority_key, sequester_service_certificate, sequester_service_certificate_signature
        )

        self._services[organization_id][service_id] = SequesterService(
            service_id=service_id,
            service_type=service_type,
            service_certificate=sequester_service_certificate,
            service_certificate_signature=sequester_service_certificate_signature,
            created_on=pendulum.now(),
            deleted_on=None,
        )

    async def get(self, organization_id, service_id) -> SequesterService:
        try:
            return self._services[organization_id][service_id]
        except KeyError:
            raise SequesterServiceNotFound()

    async def delete(self, organization_id, service_id) -> None:
        try:
            service = self._services[organization_id][service_id]
            service.deleted_on = pendulum.now()
        except KeyError:
            raise SequesterServiceNotFound()

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[SequesterService]:
        try:
            return [
                service
                for service in self._services[organization_id].values()
                if service.deleted_on is None
            ]
        except KeyError:
            raise SequesterServiceNotFound()
