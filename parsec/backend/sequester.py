# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import Optional, List, Tuple
import attr
from pendulum import DateTime, now as pendulum_now

from parsec.api.protocol import OrganizationID, SequesterServiceID, RealmID, VlobID


class SequesterError(Exception):
    pass


class SequesterOrganizationNotFoundError(SequesterError):
    pass


class SequesterNotFoundError(SequesterError):
    pass


class SequesterDisabledError(SequesterError):
    pass


class SequesterServiceAlreadyExists(SequesterError):
    pass


class SequesterSignatureError(SequesterError):
    pass


class SequesterCertificateValidationError(SequesterError):
    pass


class SequesterCertificateOutOfBallparkError(SequesterError):
    pass


class SequesterServiceNotFoundError(SequesterError):
    pass


class SequesterServiceAlreadyDeletedError(SequesterError):
    pass


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class SequesterService:
    service_id: SequesterServiceID
    service_label: str
    service_certificate: bytes
    created_on: DateTime = attr.ib(factory=pendulum_now)
    deleted_on: Optional[DateTime] = None

    def __repr__(self):
        return f"{self.__class__.__name__}({self.service_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def is_deleted(self) -> bool:
        return self.deleted_on is not None


class BaseSequesterComponent:
    async def create_service(
        self, organization_id: OrganizationID, service: SequesterService
    ) -> None:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterSignatureError
            SequesterServiceAlreadyExists
        """
        raise NotImplementedError()

    async def delete_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> None:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
            SequesterServiceAlreadyDeletedError
        """
        raise NotImplementedError()

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> SequesterService:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
        """
        raise NotImplementedError()

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[SequesterService]:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
        """
        raise NotImplementedError()

    async def dump_realm(
        self, organization_id: OrganizationID, service_id: SequesterServiceID, realm_id: RealmID
    ) -> List[Tuple[VlobID, int, bytes]]:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
        """
        raise NotImplementedError
