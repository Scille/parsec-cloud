# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import triopg
from contextlib import asynccontextmanager
from typing import AsyncGenerator, Optional
from parsec.backend.postgresql.sequester import PGPSequesterComponent

from parsec.event_bus import EventBus
from parsec.utils import open_service_nursery
from parsec.backend.config import BackendConfig
from parsec.backend.events import EventsComponent
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.webhooks import WebhooksComponent
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.organization import PGOrganizationComponent
from parsec.backend.postgresql.ping import PGPingComponent
from parsec.backend.postgresql.user import PGUserComponent
from parsec.backend.postgresql.invite import PGInviteComponent
from parsec.backend.postgresql.message import PGMessageComponent
from parsec.backend.postgresql.realm import PGRealmComponent
from parsec.backend.postgresql.vlob import PGVlobComponent
from parsec.backend.postgresql.block import PGBlockComponent
from parsec.backend.postgresql.pki import PGPkiEnrollmentComponent
from parsec.backend.backend_events import BackendEvent


@asynccontextmanager
async def components_factory(
    config: BackendConfig, event_bus: EventBus
) -> AsyncGenerator[dict, None]:
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)

    async def _send_event(
        event: BackendEvent, conn: Optional[triopg._triopg.TrioConnectionProxy] = None, **kwargs
    ) -> None:
        if conn is None:
            async with dbh.pool.acquire() as conn:
                await send_signal(conn, event, **kwargs)
        else:
            await send_signal(conn, event, **kwargs)

    webhooks = WebhooksComponent(config)
    organization = PGOrganizationComponent(dbh=dbh, webhooks=webhooks, config=config)
    user = PGUserComponent(dbh=dbh, event_bus=event_bus)
    invite = PGInviteComponent(dbh=dbh, event_bus=event_bus, config=config)
    message = PGMessageComponent(dbh)
    realm = PGRealmComponent(dbh)
    vlob = PGVlobComponent(dbh)
    ping = PGPingComponent(dbh)
    blockstore = blockstore_factory(config=config.blockstore_config, postgresql_dbh=dbh)
    block = PGBlockComponent(dbh=dbh, blockstore_component=blockstore)
    pki = PGPkiEnrollmentComponent(dbh)
    sequester = PGPSequesterComponent(dbh)
    events = EventsComponent(realm_component=realm, send_event=_send_event)

    components = {
        "events": events,
        "webhooks": webhooks,
        "organization": organization,
        "user": user,
        "invite": invite,
        "message": message,
        "realm": realm,
        "vlob": vlob,
        "ping": ping,
        "block": block,
        "blockstore": blockstore,
        "pki": pki,
        "sequester": sequester,
    }
    for component in components.values():
        method = getattr(component, "register_components", None)
        if method is not None:
            method(**components)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            yield components

        finally:
            await dbh.teardown()
