# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import triopg
from async_generator import asynccontextmanager
from typing import AsyncGenerator, Optional

from parsec.event_bus import EventBus
from parsec.utils import open_service_nursery
from parsec.backend.config import BackendConfig
from parsec.backend.events import EventsComponent
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.webhooks import WebhooksComponent
from parsec.backend.http import HTTPComponent
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.organization import PGOrganizationComponent
from parsec.backend.postgresql.ping import PGPingComponent
from parsec.backend.postgresql.user import PGUserComponent
from parsec.backend.postgresql.invite import PGInviteComponent
from parsec.backend.postgresql.message import PGMessageComponent
from parsec.backend.postgresql.realm import PGRealmComponent
from parsec.backend.postgresql.vlob import PGVlobComponent
from parsec.backend.postgresql.pki import PGCertificateComponent
from parsec.backend.postgresql.block import PGBlockComponent
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
    http = HTTPComponent(config)
    organization = PGOrganizationComponent(dbh, webhooks, config)
    user = PGUserComponent(dbh, event_bus)
    invite = PGInviteComponent(dbh, event_bus, config)
    message = PGMessageComponent(dbh)
    realm = PGRealmComponent(dbh)
    vlob = PGVlobComponent(dbh)
    ping = PGPingComponent(dbh)
    pki = PGCertificateComponent(dbh)
    blockstore = blockstore_factory(config.blockstore_config, postgresql_dbh=dbh)
    block = PGBlockComponent(dbh, blockstore, vlob)
    events = EventsComponent(realm, send_event=_send_event)

    components = {
        "events": events,
        "webhooks": webhooks,
        "http": http,
        "organization": organization,
        "user": user,
        "invite": invite,
        "message": message,
        "realm": realm,
        "vlob": vlob,
        "ping": ping,
        "pki": pki,
        "block": block,
        "blockstore": blockstore,
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
