# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from typing import Optional, List, Tuple
import attr

from parsec._parsec import DateTime
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


class SequesterWrongServiceTypeError(SequesterError):
    pass


class SequesterServiceType(Enum):
    STORAGE = "storage"
    WEBHOOK = "webhook"


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True, kw_only=True)
class BaseSequesterService:
    # Overwritten by child classes
    @property
    def service_type(self) -> SequesterServiceType:
        raise NotImplementedError()

    service_id: SequesterServiceID
    service_label: str
    service_certificate: bytes
    created_on: DateTime = attr.ib(factory=DateTime.now)
    disabled_on: Optional[DateTime] = None

    def __repr__(self):
        return f"{self.__class__.__name__}({self.service_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def is_enabled(self) -> bool:
        return self.disabled_on is None


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True, kw_only=True)
class StorageSequesterService(BaseSequesterService):
    @property
    def service_type(self) -> SequesterServiceType:
        return SequesterServiceType.STORAGE


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True, kw_only=True)
class WebhookSequesterService(BaseSequesterService):
    @property
    def service_type(self) -> SequesterServiceType:
        return SequesterServiceType.WEBHOOK

    webhook_url: str


class BaseSequesterComponent:
    async def create_service(
        self,
        organization_id: OrganizationID,
        service: BaseSequesterService,
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
    ) -> BaseSequesterService:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
        """
        raise NotImplementedError()

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[BaseSequesterService]:
        """
        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
        """
        raise NotImplementedError()

    async def dump_realm(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        realm_id: RealmID,
    ) -> List[Tuple[VlobID, int, bytes]]:
        """
        Dump all vlobs in a given realm.
        This should only be used in tests given it doesn't scale at all !

        Raises:
            SequesterDisabledError
            SequesterOrganizationNotFoundError
            SequesterServiceNotFoundError
            SequesterWrongServiceTypeError
        """
        raise NotImplementedError
