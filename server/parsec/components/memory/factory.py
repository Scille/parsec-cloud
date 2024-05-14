# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import ssl
from contextlib import asynccontextmanager
from typing import Any, AsyncGenerator

import httpx

from parsec.components.blockstore import blockstore_factory
from parsec.components.memory.auth import MemoryAuthComponent
from parsec.components.memory.block import MemoryBlockComponent
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.components.memory.events import MemoryEventsComponent, event_bus_factory
from parsec.components.memory.invite import MemoryInviteComponent
from parsec.components.memory.organization import MemoryOrganizationComponent
from parsec.components.memory.ping import MemoryPingComponent
from parsec.components.memory.pki import MemoryPkiEnrollmentComponent
from parsec.components.memory.realm import MemoryRealmComponent
from parsec.components.memory.sequester import MemorySequesterComponent
from parsec.components.memory.shamir import MemoryShamirComponent
from parsec.components.memory.user import MemoryUserComponent
from parsec.components.memory.vlob import MemoryVlobComponent
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
            auth = MemoryAuthComponent(data, event_bus, config)
            organization = MemoryOrganizationComponent(data, event_bus, webhooks, config)
            user = MemoryUserComponent(data, event_bus)
            invite = MemoryInviteComponent(data, event_bus, config)
            realm = MemoryRealmComponent(data, event_bus)
            vlob = MemoryVlobComponent(data, event_bus, http_client)
            ping = MemoryPingComponent(event_bus)
            pki = MemoryPkiEnrollmentComponent(data, event_bus)
            sequester = MemorySequesterComponent(data, event_bus)
            shamir = MemoryShamirComponent(data, event_bus)
            blockstore = blockstore_factory(config.blockstore_config, mocked_data=data)
            block = MemoryBlockComponent(data, blockstore)
            events = MemoryEventsComponent(data, config, event_bus)

            components = {
                "mocked_data": data,
                "event_bus": event_bus,
                "events": events,
                "webhooks": webhooks,
                "auth": auth,
                "organization": organization,
                "user": user,
                "invite": invite,
                "realm": realm,
                "vlob": vlob,
                "ping": ping,
                "pki": pki,
                "sequester": sequester,
                "block": block,
                "blockstore": blockstore,
                "shamir": shamir,
            }

            yield components
