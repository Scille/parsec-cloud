# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    OrganizationID,
    SequesterServiceCertificate,
    SequesterServiceID,
    SequesterRevokedServiceCertificate,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.sequester_create_service import sequester_create_service
from parsec.components.postgresql.sequester_get_service import (
    sequester_get_organization_services,
    sequester_get_service,
)
from parsec.components.postgresql.sequester_revoke_service import sequester_revoke_service
from parsec.components.postgresql.sequester_update_config_for_service import (
    sequester_update_config_for_service,
)
from parsec.components.postgresql.utils import no_transaction, transaction
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
    SequesterUpdateConfigForServiceStoreBadOutcome,
)


class PGSequesterComponent(BaseSequesterComponent):
    def __init__(self, pool: AsyncpgPool):
        self.pool = pool

    @override
    @transaction
    async def create_service(
        self,
        conn: AsyncpgConnection,
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
        return await sequester_create_service(
            conn,
            now,
            organization_id,
            service_certificate,
            config,
        )

    @override
    @transaction
    async def update_config_for_service(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        config: SequesterServiceConfig,
    ) -> None | SequesterUpdateConfigForServiceStoreBadOutcome:
        return await sequester_update_config_for_service(
            conn,
            organization_id,
            service_id,
            config,
        )

    @override
    @transaction
    async def revoke_service(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        revoked_service_certificate: bytes,
    ) -> (
        SequesterRevokedServiceCertificate
        | SequesterRevokeServiceValidateBadOutcome
        | SequesterRevokeServiceStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        return await sequester_revoke_service(
            conn,
            now,
            organization_id,
            revoked_service_certificate,
        )

    @override
    @no_transaction
    async def get_service(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
    ) -> BaseSequesterService | SequesterGetServiceBadOutcome:
        return await sequester_get_service(
            conn,
            organization_id,
            service_id,
        )

    @override
    @no_transaction
    async def get_organization_services(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> list[BaseSequesterService] | SequesterGetOrganizationServicesBadOutcome:
        return await sequester_get_organization_services(
            conn,
            organization_id,
        )
