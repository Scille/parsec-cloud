# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    OrganizationID,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    SequesterServiceID,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemorySequesterService,
)
from parsec.components.sequester import (
    BaseSequesterComponent,
    BaseSequesterService,
    SequesterCreateServiceStoreBadOutcome,
    SequesterCreateServiceValidateBadOutcome,
    SequesterGetOrganizationServicesBadOutcome,
    SequesterGetServiceBadOutcome,
    SequesterRevokeServiceStoreBadOutcome,
    SequesterRevokeServiceValidateBadOutcome,
    SequesterServiceConfig,
    SequesterServiceType,
    SequesterUpdateConfigForServiceStoreBadOutcome,
    StorageSequesterService,
    WebhookSequesterService,
    sequester_create_service_validate,
    sequester_revoke_service_validate,
)
from parsec.events import EventSequesterCertificate


def _cook_service(service: MemorySequesterService) -> BaseSequesterService:
    match service.service_type:
        case SequesterServiceType.STORAGE:
            return StorageSequesterService(
                service_id=service.cooked.service_id,
                service_label=service.cooked.service_label,
                service_certificate=service.sequester_service_certificate,
                created_on=service.cooked.timestamp,
                revoked_on=service.cooked_revoked.timestamp if service.cooked_revoked else None,
            )
        case SequesterServiceType.WEBHOOK:
            assert service.webhook_url is not None
            return WebhookSequesterService(
                service_id=service.cooked.service_id,
                service_label=service.cooked.service_label,
                service_certificate=service.sequester_service_certificate,
                created_on=service.cooked.timestamp,
                revoked_on=service.cooked_revoked.timestamp if service.cooked_revoked else None,
                webhook_url=service.webhook_url,
            )


class MemorySequesterComponent(BaseSequesterComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        self._data = data
        self._event_bus = event_bus

    async def create_service(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        service_certificate: bytes,
        config: SequesterServiceConfig,
    ) -> (
        SequesterServiceCertificate
        | SequesterCreateServiceValidateBadOutcome
        | SequesterCreateServiceStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return SequesterCreateServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.sequester_services is None:
            return SequesterCreateServiceStoreBadOutcome.SEQUESTER_DISABLED

        async with org.topics_lock(write=["sequester"]) as (sequester_topic_last_timestamp,):
            assert org.cooked_sequester_authority is not None
            match sequester_create_service_validate(
                now, org.cooked_sequester_authority.verify_key_der, service_certificate
            ):
                case SequesterServiceCertificate() as certif:
                    pass
                case error:
                    return error

            # Ensure certificate consistency: our certificate must be the very last among
            # the existing sequester (authority & service) certificates.

            if sequester_topic_last_timestamp >= certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=sequester_topic_last_timestamp)

            if certif.service_id in org.sequester_services:
                return SequesterCreateServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_EXISTS

            # All checks are good, now we do the actual insertion

            match config:
                case SequesterServiceType.STORAGE as service_type:
                    webhook_url = None
                case (SequesterServiceType.WEBHOOK as service_type, webhook_url):
                    pass

            org.sequester_services[certif.service_id] = MemorySequesterService(
                cooked=certif,
                sequester_service_certificate=service_certificate,
                service_type=service_type,
                webhook_url=webhook_url,
            )
            org.per_topic_last_timestamp["sequester"] = certif.timestamp

            await self._event_bus.send(
                EventSequesterCertificate(
                    organization_id=organization_id, timestamp=certif.timestamp
                )
            )

            return certif

    @override
    async def update_config_for_service(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        config: SequesterServiceConfig,
    ) -> None | SequesterUpdateConfigForServiceStoreBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return SequesterUpdateConfigForServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.sequester_services is None:
            return SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_DISABLED

        try:
            service = org.sequester_services[service_id]
        except KeyError:
            return SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND

        match config:
            case SequesterServiceType.STORAGE as service_type:
                webhook_url = None
            case (SequesterServiceType.WEBHOOK as service_type, webhook_url):
                pass

        service.service_type = service_type
        service.webhook_url = webhook_url

    @override
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return SequesterRevokeServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.sequester_services is None:
            return SequesterRevokeServiceStoreBadOutcome.SEQUESTER_DISABLED

        async with org.topics_lock(write=["sequester"]) as (sequester_topic_last_timestamp,):
            assert org.cooked_sequester_authority is not None
            match sequester_revoke_service_validate(
                now, org.cooked_sequester_authority.verify_key_der, revoked_service_certificate
            ):
                case SequesterRevokedServiceCertificate() as certif:
                    pass
                case error:
                    return error

            try:
                service = org.sequester_services[certif.service_id]
            except KeyError:
                return SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND

            # Ensure certificate consistency: our certificate must be the very last among
            # the existing sequester (authority & service) certificates.

            if sequester_topic_last_timestamp >= certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=sequester_topic_last_timestamp)

            if service.is_revoked:
                return SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_REVOKED

            # All checks are good, now we do the actual insertion

            service.sequester_revoked_service_certificate = revoked_service_certificate
            service.cooked_revoked = certif
            org.per_topic_last_timestamp["sequester"] = certif.timestamp

            await self._event_bus.send(
                EventSequesterCertificate(
                    organization_id=organization_id, timestamp=certif.timestamp
                )
            )

            return certif

    @override
    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> BaseSequesterService | SequesterGetServiceBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return SequesterGetServiceBadOutcome.ORGANIZATION_NOT_FOUND

        if org.sequester_services is None:
            return SequesterGetServiceBadOutcome.SEQUESTER_DISABLED

        try:
            service = org.sequester_services[service_id]
        except KeyError:
            return SequesterGetServiceBadOutcome.SEQUESTER_SERVICE_NOT_FOUND

        return _cook_service(service)

    @override
    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> list[BaseSequesterService] | SequesterGetOrganizationServicesBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return SequesterGetOrganizationServicesBadOutcome.ORGANIZATION_NOT_FOUND

        if org.sequester_services is None:
            return SequesterGetOrganizationServicesBadOutcome.SEQUESTER_DISABLED

        return [_cook_service(service) for service in org.sequester_services.values()]
