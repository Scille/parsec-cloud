# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto

from parsec._parsec import (
    CryptoError,
    DateTime,
    OrganizationID,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    SequesterServiceID,
    SequesterVerifyKeyDer,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.types import BadOutcomeEnum


class SequesterServiceType(Enum):
    STORAGE = "storage"
    WEBHOOK = "webhook"


@dataclass(slots=True, repr=False)
class BaseSequesterService:
    # Overwritten by child classes
    @property
    def service_type(self) -> SequesterServiceType:
        raise NotImplementedError

    service_id: SequesterServiceID
    service_label: str
    service_certificate: bytes
    created_on: DateTime
    revoked_on: DateTime | None

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.service_id})"

    @property
    def is_revoked(self) -> bool:
        return self.revoked_on is not None


@dataclass(slots=True, repr=False)
class StorageSequesterService(BaseSequesterService):
    @property
    def service_type(self) -> SequesterServiceType:
        return SequesterServiceType.STORAGE


@dataclass(slots=True, repr=False)
class WebhookSequesterService(BaseSequesterService):
    @property
    def service_type(self) -> SequesterServiceType:
        return SequesterServiceType.WEBHOOK

    webhook_url: str


class SequesterCreateServiceValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()


def sequester_create_service_validate(
    now: DateTime,
    sequester_authority_verify_key_der: SequesterVerifyKeyDer,
    service_certificate: bytes,
) -> SequesterServiceCertificate | SequesterCreateServiceValidateBadOutcome:
    try:
        raw = sequester_authority_verify_key_der.verify(service_certificate)
        certif = SequesterServiceCertificate.load(raw)

    except (ValueError, CryptoError):
        return SequesterCreateServiceValidateBadOutcome.INVALID_CERTIFICATE

    # Note that unlike for other signed data, we don't ensure the certificate's
    # timestamp is within now's ballpark.
    # This is because the certificate is allowed to be created long before being
    # inserted (see `generate_service_certificate` CLI command)

    return certif


class SequesterRevokeServiceValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()


def sequester_revoke_service_validate(
    now: DateTime,
    sequester_authority_verify_key_der: SequesterVerifyKeyDer,
    revoked_service_certificate: bytes,
) -> SequesterRevokedServiceCertificate | SequesterRevokeServiceValidateBadOutcome:
    try:
        raw = sequester_authority_verify_key_der.verify(revoked_service_certificate)
        certif = SequesterRevokedServiceCertificate.load(raw)

    except (ValueError, CryptoError):
        return SequesterRevokeServiceValidateBadOutcome.INVALID_CERTIFICATE

    # Note that unlike for other signed data, we don't ensure the certificate's
    # timestamp is within now's ballpark.
    # This is because the certificate is allowed to be created long before being
    # inserted (see `generate_service_certificate` CLI command)

    return certif


class SequesterCreateServiceStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    SEQUESTER_DISABLED = auto()
    SEQUESTER_SERVICE_ALREADY_EXISTS = auto()


class SequesterRevokeServiceStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    SEQUESTER_DISABLED = auto()
    SEQUESTER_SERVICE_NOT_FOUND = auto()
    SEQUESTER_SERVICE_ALREADY_REVOKED = auto()


class SequesterGetServiceBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    SEQUESTER_DISABLED = auto()
    SEQUESTER_SERVICE_NOT_FOUND = auto()


class SequesterGetOrganizationServicesBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    SEQUESTER_DISABLED = auto()


class SequesterDumpRealmBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    SEQUESTER_DISABLED = auto()
    SEQUESTER_SERVICE_NOT_FOUND = auto()
    SEQUESTER_SERVICE_NOT_STORAGE = auto()


class BaseSequesterComponent:
    async def create_storage_service(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        service_certificate: bytes,
    ) -> (
        SequesterServiceCertificate
        | SequesterCreateServiceValidateBadOutcome
        | SequesterCreateServiceStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def create_webhook_service(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        service_certificate: bytes,
        webhook_url: str,
    ) -> (
        SequesterServiceCertificate
        | SequesterCreateServiceValidateBadOutcome
        | SequesterCreateServiceStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def revoke_service(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        revoked_service_certificate: bytes,
    ) -> (
        SequesterRevokedServiceCertificate
        | SequesterRevokeServiceValidateBadOutcome
        | SequesterRevokeServiceStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> BaseSequesterService | SequesterGetServiceBadOutcome:
        raise NotImplementedError

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> list[BaseSequesterService] | SequesterGetOrganizationServicesBadOutcome:
        raise NotImplementedError

    async def dump_realm(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        realm_id: VlobID,
    ) -> list[tuple[VlobID, int, bytes]] | SequesterDumpRealmBadOutcome:
        """
        Dump all vlobs in a given realm.
        This should only be used in tests given it doesn't scale at all !
        """
        raise NotImplementedError
