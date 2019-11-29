# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.backend.config import BackendConfig
from parsec.backend.events import EventsComponent
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.memory.organization import MemoryOrganizationComponent
from parsec.backend.memory.ping import MemoryPingComponent
from parsec.backend.memory.user import MemoryUserComponent
from parsec.backend.memory.message import MemoryMessageComponent
from parsec.backend.memory.realm import MemoryRealmComponent
from parsec.backend.memory.vlob import MemoryVlobComponent
from parsec.backend.memory.block import MemoryBlockComponent


@asynccontextmanager
async def components_factory(config: BackendConfig, event_bus: EventBus):
    user = MemoryUserComponent(event_bus)
    message = MemoryMessageComponent(event_bus)
    realm = MemoryRealmComponent(event_bus, user, message)
    vlob = MemoryVlobComponent(event_bus, realm)
    realm._register_maintenance_reencryption_hooks(
        vlob._maintenance_reencryption_start_hook, vlob._maintenance_reencryption_is_finished_hook
    )
    ping = MemoryPingComponent(event_bus)
    blockstore = blockstore_factory(config.blockstore_config)
    block = MemoryBlockComponent(blockstore, realm)
    organization = MemoryOrganizationComponent(
        user_component=user, vlob_component=vlob, block_component=block
    )
    events = EventsComponent(event_bus, realm)

    yield {
        "user": user,
        "message": message,
        "realm": realm,
        "vlob": vlob,
        "ping": ping,
        "blockstore": blockstore,
        "block": block,
        "organization": organization,
        "events": events,
    }
