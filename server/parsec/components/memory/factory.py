# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import ssl
from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from typing import Any

import httpx

from parsec.components.blockstore import blockstore_factory
from parsec.components.memory.account import MemoryAccountComponent
from parsec.components.memory.async_enrollment import MemoryAsyncEnrollmentComponent
from parsec.components.memory.auth import MemoryAuthComponent
from parsec.components.memory.block import MemoryBlockComponent
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.components.memory.events import MemoryEventsComponent, event_bus_factory
from parsec.components.memory.invite import MemoryInviteComponent
from parsec.components.memory.organization import MemoryOrganizationComponent
from parsec.components.memory.ping import MemoryPingComponent
from parsec.components.memory.realm import MemoryRealmComponent
from parsec.components.memory.sequester import MemorySequesterComponent
from parsec.components.memory.shamir import MemoryShamirComponent
from parsec.components.memory.totp import MemoryTOTPComponent
from parsec.components.memory.user import MemoryUserComponent
from parsec.components.memory.vlob import MemoryVlobComponent
from parsec.components.scws import ScwsComponent
from parsec.config import BackendConfig
from parsec.webhooks import WebhooksComponent

# SSL context is costly to setup, so cache it instead of having `httpx.AsyncClient`
# re-creating it for each test.
SSL_CONTEXT = ssl.create_default_context()


@asynccontextmanager
async def components_factory(config: BackendConfig) -> AsyncGenerator[dict[str, Any], None]:
    data = MemoryDatamodel({} if config.backend_mocked_data is None else config.backend_mocked_data)

    async with event_bus_factory() as event_bus:
        async with httpx.AsyncClient(verify=SSL_CONTEXT) as http_client:
            webhooks = WebhooksComponent(config, http_client)
            blockstore = blockstore_factory(config.blockstore_config, mocked_data=data)

            account = MemoryAccountComponent(data, config, event_bus)
            async_enrollment = MemoryAsyncEnrollmentComponent(data, event_bus)
            auth = MemoryAuthComponent(data, event_bus, config)
            block = MemoryBlockComponent(data, blockstore)
            events = MemoryEventsComponent(data, config, event_bus)
            invite = MemoryInviteComponent(data, event_bus, config)
            organization = MemoryOrganizationComponent(data, event_bus, webhooks, config)
            ping = MemoryPingComponent(event_bus)
            realm = MemoryRealmComponent(data, event_bus, webhooks)
            scws = ScwsComponent(config)
            sequester = MemorySequesterComponent(data, event_bus)
            shamir = MemoryShamirComponent(data, event_bus)
            totp = MemoryTOTPComponent(data, config)
            user = MemoryUserComponent(data, event_bus)
            vlob = MemoryVlobComponent(data, event_bus, webhooks)

            components = {
                "account": account,
                "async_enrollment": async_enrollment,
                "auth": auth,
                "block": block,
                "blockstore": blockstore,
                "event_bus": event_bus,
                "events": events,
                "invite": invite,
                "mocked_data": data,
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

            yield components
