# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import ssl
from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from typing import Any

import httpx

from parsec.components.blockstore import blockstore_factory
from parsec.components.postgresql.account import PGAccountComponent
from parsec.components.postgresql.async_enrollment import PGAsyncEnrollmentComponent
from parsec.components.postgresql.auth import PGAuthComponent
from parsec.components.postgresql.block import PGBlockComponent
from parsec.components.postgresql.events import PGEventsComponent, event_bus_factory
from parsec.components.postgresql.handler import asyncpg_pool_factory
from parsec.components.postgresql.invite import PGInviteComponent
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.ping import PGPingComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.sequester import PGSequesterComponent
from parsec.components.postgresql.shamir import PGShamirComponent
from parsec.components.postgresql.totp import PGTOTPComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.vlob import PGVlobComponent
from parsec.components.scws import ScwsComponent
from parsec.config import BackendConfig, PostgreSQLDatabaseConfig
from parsec.webhooks import WebhooksComponent

# SSL context is costly to setup, so cache it instead of having `httpx.AsyncClient`
# re-creating it for each test.
SSL_CONTEXT = ssl.create_default_context()


@asynccontextmanager
async def components_factory(
    config: BackendConfig,
) -> AsyncGenerator[dict[str, Any], None]:
    assert isinstance(config.db_config, PostgreSQLDatabaseConfig)
    async with asyncpg_pool_factory(
        url=config.db_config.url,
        min_connections=config.db_config.min_connections,
        max_connections=config.db_config.max_connections,
    ) as pool:
        async with event_bus_factory(pool) as event_bus:
            async with httpx.AsyncClient(verify=SSL_CONTEXT) as http_client:
                webhooks = WebhooksComponent(config, http_client)
                blockstore = blockstore_factory(
                    config=config.blockstore_config, postgresql_pool=pool
                )

                account = PGAccountComponent(pool=pool, config=config)
                async_enrollment = PGAsyncEnrollmentComponent(pool=pool, config=config)
                auth = PGAuthComponent(pool=pool, event_bus=event_bus, config=config)
                block = PGBlockComponent(pool=pool, blockstore=blockstore)
                events = PGEventsComponent(pool=pool, config=config, event_bus=event_bus)
                invite = PGInviteComponent(pool=pool, config=config)
                organization = PGOrganizationComponent(pool=pool, webhooks=webhooks, config=config)
                ping = PGPingComponent(pool=pool)
                realm = PGRealmComponent(pool=pool, webhooks=webhooks)
                scws = ScwsComponent(config)
                sequester = PGSequesterComponent(pool=pool)
                shamir = PGShamirComponent(pool=pool)
                totp = PGTOTPComponent(pool=pool, config=config)
                user = PGUserComponent(pool=pool)
                vlob = PGVlobComponent(pool=pool, webhooks=webhooks)

                components = {
                    "account": account,
                    "async_enrollment": async_enrollment,
                    "auth": auth,
                    "block": block,
                    "blockstore": blockstore,
                    "event_bus": event_bus,
                    "events": events,
                    "invite": invite,
                    "organization": organization,
                    "ping": ping,
                    "realm": realm,
                    "scws": scws,
                    "sequester": sequester,
                    "shamir": shamir,
                    "totp": totp,
                    "user": user,
                    "vlob": vlob,
                    "webhooks": webhooks,
                }
                for component in components.values():
                    method = getattr(component, "register_components", None)
                    if method is not None:
                        method(**components)

                yield components
