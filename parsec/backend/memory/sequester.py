# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from copy import deepcopy
from typing import TYPE_CHECKING, Any, Dict, List, Tuple

from parsec._parsec import CryptoError, DateTime
from parsec.api.data import DataError, SequesterServiceCertificate
from parsec.api.protocol import OrganizationID, RealmID, SequesterServiceID, VlobID
from parsec.backend.sequester import (
    BaseSequesterComponent,
    BaseSequesterService,
    SequesterCertificateValidationError,
    SequesterDisabledError,
    SequesterOrganizationNotFoundError,
    SequesterServiceAlreadyDisabledError,
    SequesterServiceAlreadyEnabledError,
    SequesterServiceAlreadyExists,
    SequesterServiceNotFoundError,
    SequesterServiceType,
    SequesterWrongServiceTypeError,
)

if TYPE_CHECKING:
    from parsec.backend.memory.organization import MemoryOrganizationComponent
    from parsec.backend.memory.vlob import MemoryVlobComponent


class MemorySequesterComponent(BaseSequesterComponent):
    def __init__(self) -> None:
        self._organization_component: MemoryOrganizationComponent | None = None
        self._vlob_component: MemoryVlobComponent | None = None
        self._services: Dict[
            OrganizationID, Dict[SequesterServiceID, BaseSequesterService]
        ] = defaultdict(dict)

    def register_components(
        self,
        organization: MemoryOrganizationComponent,
        vlob: MemoryVlobComponent,
        **other_components: Any,
    ) -> None:
        self._organization_component = organization
        self._vlob_component = vlob

    def _enabled_services(self, organization_id: OrganizationID) -> List[BaseSequesterService]:
        return [s for s in self._services[organization_id].values() if s.is_enabled]

    def _refresh_services_in_organization_component(self, organization_id: OrganizationID) -> None:
        # Organization objects in the organization component contains the list of active services
        # (typically returned in `organization_config` API command), so it must be refreshed
        # every time we create/disable/re-enable a service !
        assert self._organization_component is not None

        organization = self._organization_component._organizations[organization_id]
        sequester_services_certificates = tuple(
            s.service_certificate for s in self._services[organization_id].values() if s.is_enabled
        )
        self._organization_component._organizations[organization_id] = organization.evolve(
            sequester_services_certificates=sequester_services_certificates
        )

    async def create_service(
        self,
        organization_id: OrganizationID,
        service: BaseSequesterService,
    ) -> None:
        assert self._organization_component is not None

        try:
            organization = self._organization_component._organizations[organization_id]
        except KeyError as exc:
            raise SequesterOrganizationNotFoundError from exc
        if organization.sequester_authority is None:
            raise SequesterDisabledError

        try:
            certif_dumped = organization.sequester_authority.verify_key_der.verify(
                service.service_certificate
            )
        except CryptoError as exc:
            raise SequesterCertificateValidationError(
                f"Invalid certification data ({exc})."
            ) from exc

        try:
            SequesterServiceCertificate.load(certif_dumped)

        except DataError as exc:
            raise SequesterCertificateValidationError(
                f"Invalid certification data ({exc})."
            ) from exc

        org_services = self._services[organization_id]
        if service.service_id in org_services:
            raise SequesterServiceAlreadyExists
        org_services[service.service_id] = service
        # Also don't forget to update Organization structure in organization component
        self._refresh_services_in_organization_component(organization_id)

    async def disable_service(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        disabled_on: DateTime | None = None,
    ) -> None:
        disabled_on = disabled_on or DateTime.now()
        service = self._get_service(organization_id=organization_id, service_id=service_id)
        if not service.is_enabled:
            raise SequesterServiceAlreadyDisabledError
        self._services[organization_id][service_id] = service.evolve(disabled_on=disabled_on)
        # Also don't forget to update Organization structure in organization component
        self._refresh_services_in_organization_component(organization_id)

    async def enable_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> None:
        service = self._get_service(organization_id=organization_id, service_id=service_id)
        if service.is_enabled:
            raise SequesterServiceAlreadyEnabledError
        self._services[organization_id][service_id] = service.evolve(disabled_on=None)
        # Also don't forget to update Organization structure in organization component
        self._refresh_services_in_organization_component(organization_id)

    def _get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> BaseSequesterService:
        assert self._organization_component is not None

        try:
            organization = self._organization_component._organizations[organization_id]
        except KeyError as exc:
            raise SequesterOrganizationNotFoundError from exc
        if organization.sequester_authority is None:
            raise SequesterDisabledError
        try:
            return self._services[organization_id][service_id]
        except KeyError as exc:
            raise SequesterServiceNotFoundError from exc

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> BaseSequesterService:
        return self._get_service(organization_id=organization_id, service_id=service_id)

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[BaseSequesterService]:
        assert self._organization_component is not None

        try:
            organization = self._organization_component._organizations[organization_id]
        except KeyError as exc:
            raise SequesterOrganizationNotFoundError from exc
        if organization.sequester_authority is None:
            raise SequesterDisabledError
        return list(self._services[organization_id].values())

    async def dump_realm(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        realm_id: RealmID,
    ) -> List[Tuple[VlobID, int, bytes]]:
        assert self._vlob_component is not None

        dump: List[Tuple[VlobID, int, bytes]] = []
        # Check orga and service exists
        service = self._get_service(organization_id=organization_id, service_id=service_id)
        if service.service_type != SequesterServiceType.STORAGE:
            raise SequesterWrongServiceTypeError(
                f"Service type {service.service_type} is not compatible with export"
            )
        # Do the actual dump
        for (vlob_org, vlob_id), vlob in self._vlob_component._vlobs.items():
            if vlob_org != organization_id or vlob.realm_id != realm_id:
                continue
            assert vlob.sequestered_data is not None
            for version, sequestered_version in enumerate(vlob.sequestered_data, start=1):
                try:
                    dump.append((vlob_id, version, sequestered_version[service_id]))
                except KeyError:
                    pass
        return dump

    def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        self._services[new_id] = deepcopy(self._services[id])

    def test_drop_organization(self, id: OrganizationID) -> None:
        self._services.pop(id, None)
