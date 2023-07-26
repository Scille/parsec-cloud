# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncGenerator, Awaitable, Callable, Dict

import attr
from structlog import get_logger

from parsec._parsec import ClientType, OrganizationID
from parsec.backend.block import BaseBlockComponent
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.config import BackendConfig
from parsec.backend.events import EventsComponent
from parsec.backend.invite import BaseInviteComponent
from parsec.backend.memory import components_factory as mocked_components_factory
from parsec.backend.message import BaseMessageComponent
from parsec.backend.organization import BaseOrganizationComponent
from parsec.backend.ping import BasePingComponent
from parsec.backend.pki import BasePkiEnrollmentComponent
from parsec.backend.postgresql import components_factory as postgresql_components_factory
from parsec.backend.realm import BaseRealmComponent
from parsec.backend.sequester import BaseSequesterComponent
from parsec.backend.shamir import BaseShamirComponent
from parsec.backend.user import BaseUserComponent
from parsec.backend.utils import collect_apis
from parsec.backend.vlob import BaseVlobComponent
from parsec.backend.webhooks import WebhooksComponent
from parsec.event_bus import EventBus

logger = get_logger()


@asynccontextmanager
async def backend_app_factory(
    config: BackendConfig, event_bus: EventBus | None = None
) -> AsyncGenerator[BackendApp, None]:
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
            shamir=components["shamir"],
        )


@attr.s(slots=True, auto_attribs=True, kw_only=True, eq=False, repr=False)
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
    shamir: BaseShamirComponent

    apis: Dict[ClientType, Dict[str, Callable[..., Awaitable[dict[str, object]]]]] = attr.field(
        init=False
    )

    def __attrs_post_init__(self) -> None:
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
            self.shamir,
            # Ping command is only used in tests
            include_ping=self.config.debug,
        )

    def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        self.user.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.invite.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.organization.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.message.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.realm.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.vlob.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.ping.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.blockstore.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.block.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.pki.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.sequester.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.shamir.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]

    def test_drop_organization(self, id: OrganizationID) -> None:
        self.user.test_drop_organization(id)  # type: ignore[attr-defined]
        self.invite.test_drop_organization(id)  # type: ignore[attr-defined]
        self.organization.test_drop_organization(id)  # type: ignore[attr-defined]
        self.message.test_drop_organization(id)  # type: ignore[attr-defined]
        self.realm.test_drop_organization(id)  # type: ignore[attr-defined]
        self.vlob.test_drop_organization(id)  # type: ignore[attr-defined]
        self.ping.test_drop_organization(id)  # type: ignore[attr-defined]
        self.blockstore.test_drop_organization(id)  # type: ignore[attr-defined]
        self.block.test_drop_organization(id)  # type: ignore[attr-defined]
        self.pki.test_drop_organization(id)  # type: ignore[attr-defined]
        self.sequester.test_drop_organization(id)  # type: ignore[attr-defined]
        self.shamir.test_drop_organization(id)  # type: ignore[attr-defined]
