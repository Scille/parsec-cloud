# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from enum import Enum
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


class SequesterServiceAlreadyDisabledError(SequesterError):
    pass


class SequesterServiceAlreadyEnabledError(SequesterError):
    pass


class SequesterWrongServiceType(SequesterError):
    pass


class SequesterServiceType(Enum):
    STORAGE = "storage"
    WEBHOOK = "webhook"


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class SequesterService:
    service_id: SequesterServiceID
    service_label: str
    service_certificate: bytes
    created_on: DateTime = attr.ib(factory=pendulum_now)
    disabled_on: Optional[DateTime] = None
    service_type: SequesterServiceType = SequesterServiceType.STORAGE
    webhook_url: Optional[str] = None

    def __repr__(self):
        return f"{self.__class__.__name__}({self.service_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def is_enabled(self) -> bool:
        return self.disabled_on is None


class BaseSequesterComponent:
    async def create_service(
        self,
        organization_id: OrganizationID,
        service: SequesterService,
        now: Optional[DateTime] = None,
    ) -> None:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterSignatureError
            SequesterServiceAlreadyExists
        """
        raise NotImplementedError()

    async def disable_service(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        disabled_on: Optional[DateTime] = None,
    ) -> None:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
            SequesterServiceAlreadyDisabledError
        """
        raise NotImplementedError()

    async def enable_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> None:
        """
        Re-enable a service that has been previously disabled.

        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
            SequesterServiceAlreadyEnableddError
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
        Dump all vlobs in a given realm.
        This should only be used in tests given it doesn't scale at all !

        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
            SequesterWrongServiceType
        """
        raise NotImplementedError
