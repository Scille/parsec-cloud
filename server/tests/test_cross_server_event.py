# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from asyncio import Queue

import pytest

from parsec._parsec import OrganizationID
from parsec.backend import backend_factory
from parsec.config import BackendConfig
from parsec.events import EventPinged


@pytest.mark.postgresql
async def test_cross_server_event(backend_config: BackendConfig) -> None:
    async with (
        backend_factory(config=backend_config) as b1,
        backend_factory(config=backend_config) as b2,
    ):
        b2_received_events = Queue()

        def on_b2_receive_event(event):
            b2_received_events.put_nowait(event)

        event = EventPinged(organization_id=OrganizationID("Org"), ping="hello")
        b2.event_bus.connect(on_b2_receive_event)

        await b1.event_bus.test_send(event)
        b2_event = await b2_received_events.get()
        assert b2_event == event
