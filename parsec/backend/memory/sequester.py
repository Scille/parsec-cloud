# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import TYPE_CHECKING, Optional, Dict, List, Tuple
from collections import defaultdict
from pendulum import DateTime, now as pendulum_now

from parsec.utils import timestamps_in_the_ballpark
from parsec.crypto import CryptoError
from parsec.api.data import SequesterServiceCertificate, DataError
from parsec.api.protocol import OrganizationID, SequesterServiceID, RealmID, VlobID
from parsec.backend.sequester import (
    BaseSequesterComponent,
    SequesterService,
    SequesterServiceNotFoundError,
    SequesterOrganizationNotFoundError,
    SequesterDisabledError,
    SequesterServiceAlreadyExists,
    SequesterServiceAlreadyDeletedError,
    SequesterCertificateValidationError,
    SequesterCertificateOutOfBallparkError,
)

if TYPE_CHECKING:
    from parsec.backend.memory.organization import MemoryOrganizationComponent
    from parsec.backend.memory.vlob import MemoryVlobComponent


class MemorySequesterComponent(BaseSequesterComponent):
    def __init__(self):
        self._organization_component: "MemoryOrganizationComponent" = None
        self._vlob_component: "MemoryVlobComponent" = None
        self._services: Dict[
            OrganizationID, Dict[SequesterServiceID, SequesterService]
        ] = defaultdict(dict)

    def register_components(
        self,
        organization: "MemoryOrganizationComponent",
        vlob: "MemoryVlobComponent",
        **other_components,
    ):
        self._organization_component = organization
        self._vlob_component = vlob

    def _active_services(self, organization_id: OrganizationID) -> List[SequesterService]:
        return [s for s in self._services[organization_id].values() if not s.is_deleted]

    async def create_service(
        self, organization_id: OrganizationID, service: SequesterService
    ) -> None:
        try:
            organization = self._organization_component._organizations[organization_id]
        except KeyError as exc:
            raise SequesterOrganizationNotFoundError from exc
        if organization.sequester is None:
            raise SequesterDisabledError

        try:
            certif_dumped = organization.sequester.authority_verify_key_der.verify(
                service.service_certificate
            )
        except CryptoError as exc:
            raise SequesterCertificateValidationError(
                f"Invalid certification data ({exc})."
            ) from exc

        try:
            certif_data = SequesterServiceCertificate.load(certif_dumped)

        except DataError as exc:
            raise SequesterCertificateValidationError(
                f"Invalid certification data ({exc})."
            ) from exc

        now = pendulum_now()
        if not timestamps_in_the_ballpark(certif_data.timestamp, now):
            raise SequesterCertificateOutOfBallparkError(
                f"Invalid certification data (timestamp out of ballpark)."
            )

        org_services = self._services[organization_id]
        if service.service_id in org_services:
            raise SequesterServiceAlreadyExists
        org_services[service.service_id] = service

    async def delete_service(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        deleted_on: Optional[DateTime] = None,
    ) -> None:
        deleted_on = deleted_on or pendulum_now()
        service = self._get_service(organization_id=organization_id, service_id=service_id)
        if service.is_deleted:
            raise SequesterServiceAlreadyDeletedError
        self._services[organization_id][service_id] = service.evolve(deleted_on=deleted_on)

    def _get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> SequesterService:
        try:
            organization = self._organization_component._organizations[organization_id]
        except KeyError as exc:
            raise SequesterOrganizationNotFoundError from exc
        if organization.sequester is None:
            raise SequesterDisabledError
        try:
            return self._services[organization_id][service_id]
        except KeyError as exc:
            raise SequesterServiceNotFoundError from exc

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> SequesterService:
        return self._get_service(organization_id=organization_id, service_id=service_id)

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[SequesterService]:
        try:
            organization = self._organization_component._organizations[organization_id]
        except KeyError as exc:
            raise SequesterOrganizationNotFoundError from exc
        if organization.sequester is None:
            raise SequesterDisabledError
        return list(self._services[organization_id].values())

    async def dump_realm(
        self, organization_id: OrganizationID, service_id: SequesterServiceID, realm_id: RealmID
    ) -> List[Tuple[VlobID, int, bytes]]:
        dump: List[Tuple[VlobID, int, bytes]] = []
        # Check orga and service exists
        self._get_service(organization_id=organization_id, service_id=service_id)
        # Do the actual dump
        for (vorg, vid), vlob in self._vlob_component._vlobs.items():
            if vorg != organization_id or vlob.realm_id != realm_id:
                continue
            assert vlob.sequestered_data is not None
            for version, sequestered_version in enumerate(vlob.sequestered_data, start=1):
                try:
                    dump.append((vid, version, sequestered_version[service_id]))
                except KeyError:
                    pass
        return dump
