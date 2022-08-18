# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Optional, Dict, Callable
import attr
from structlog import get_logger
from contextlib import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.backend.utils import collect_apis, ClientType
from parsec.backend.config import BackendConfig
from parsec.backend.memory import components_factory as mocked_components_factory
from parsec.backend.postgresql import components_factory as postgresql_components_factory
from parsec.backend.events import EventsComponent
from parsec.backend.webhooks import WebhooksComponent
from parsec.backend.user import BaseUserComponent
from parsec.backend.invite import BaseInviteComponent
from parsec.backend.organization import BaseOrganizationComponent
from parsec.backend.message import BaseMessageComponent
from parsec.backend.realm import BaseRealmComponent
from parsec.backend.vlob import BaseVlobComponent
from parsec.backend.ping import BasePingComponent
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import BaseBlockComponent
from parsec.backend.sequester import BaseSequesterComponent
from parsec.backend.pki import BasePkiEnrollmentComponent


logger = get_logger()


@asynccontextmanager
async def backend_app_factory(config: BackendConfig, event_bus: Optional[EventBus] = None):
    event_bus = event_bus or EventBus()

    if config.db_url == "MOCKED":
        components_factory = mocked_components_factory
    else:
        components_factory = postgresql_components_factory

    async with components_factory(config=config, event_bus=event_bus) as components:
        yield BackendApp(
            config=config,
            event_bus=event_bus,
            webhooks=components["webhooks"],
            user=components["user"],
            invite=components["invite"],
            organization=components["organization"],
            message=components["message"],
            realm=components["realm"],
            vlob=components["vlob"],
            ping=components["ping"],
            blockstore=components["blockstore"],
            block=components["block"],
            pki=components["pki"],
            sequester=components["sequester"],
            events=components["events"],
        )


@attr.s(slots=True, auto_attribs=True, kw_only=True, eq=False)
class BackendApp:
    config: BackendConfig
    event_bus: EventBus
    webhooks: WebhooksComponent
    user: BaseUserComponent
    invite: BaseInviteComponent
    organization: BaseOrganizationComponent
    message: BaseMessageComponent
    realm: BaseRealmComponent
    vlob: BaseVlobComponent
    ping: BasePingComponent
    blockstore: BaseBlockStoreComponent
    block: BaseBlockComponent
    pki: BasePkiEnrollmentComponent
    sequester: BaseSequesterComponent
    events: EventsComponent

    apis: Dict[ClientType, Dict[str, Callable]] = attr.field(init=False)

    def __attrs_post_init__(self):
        self.apis = collect_apis(
            self.user,
            self.invite,
            self.organization,
            self.message,
            self.realm,
            self.vlob,
            self.ping,
            self.blockstore,
            self.block,
            self.pki,
            self.events,
        )
