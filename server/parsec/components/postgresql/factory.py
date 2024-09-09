# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import ssl
from contextlib import asynccontextmanager
from typing import Any, AsyncGenerator

import httpx

from parsec.components.blockstore import blockstore_factory
from parsec.components.postgresql.auth import PGAuthComponent
from parsec.components.postgresql.block import PGBlockComponent
from parsec.components.postgresql.events import PGEventsComponent, event_bus_factory
from parsec.components.postgresql.handler import asyncpg_pool_factory
from parsec.components.postgresql.invite import PGInviteComponent
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.ping import PGPingComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.shamir import PGShamirComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.vlob import PGVlobComponent
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
        async with event_bus_factory(db_url=config.db_config.url) as event_bus:
            async with httpx.AsyncClient(verify=SSL_CONTEXT) as http_client:
                webhooks = WebhooksComponent(config, http_client)
                events = PGEventsComponent(pool=pool, config=config, event_bus=event_bus)
                ping = PGPingComponent(pool=pool)
                organization = PGOrganizationComponent(
                    pool=pool, webhooks=webhooks, config=config, event_bus=event_bus
                )
                auth = PGAuthComponent(pool=pool, event_bus=event_bus, config=config)
                invite = PGInviteComponent(pool=pool, event_bus=event_bus, config=config)
                user = PGUserComponent(pool=pool, event_bus=event_bus)
                vlob = PGVlobComponent(pool=pool, event_bus=event_bus)
                realm = PGRealmComponent(pool=pool, event_bus=event_bus)
                blockstore = blockstore_factory(
                    config=config.blockstore_config, postgresql_pool=pool
                )
                block = PGBlockComponent(pool=pool, blockstore=blockstore)
                shamir = PGShamirComponent()

                pki = None
                sequester = None

                components = {
                    "event_bus": event_bus,
                    "events": events,
                    "webhooks": webhooks,
                    "organization": organization,
                    "user": user,
                    "auth": auth,
                    "invite": invite,
                    "realm": realm,
                    "vlob": vlob,
                    "ping": ping,
                    "block": block,
                    "blockstore": blockstore,
                    "pki": pki,
                    "sequester": sequester,
                    "shamir": shamir,
                }
                for component in components.values():
                    method = getattr(component, "register_components", None)
                    if method is not None:
                        method(**components)

                yield components
