# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio


@pytest.mark.trio
async def test_waiter_on(event_bus):
    async def _trigger(id):
        await trio.sleep(0)
        event_bus.send("foo", id=id)

    async with trio.open_service_nursery() as nursery:
        with event_bus.waiter_on("foo") as waiter:
            nursery.start_soon(_trigger, 1)
            event = await waiter.wait()
            assert event == ("foo", {"id": 1})

            # Event is ignored
            event_bus.send("foo", id=2)
            same_event = await waiter.wait()
            assert same_event == event

            waiter.clear()
            nursery.start_soon(_trigger, 3)
            event = await waiter.wait()
            assert event == ("foo", {"id": 3})

        assert event_bus.stats() == {}


@pytest.mark.trio
async def test_waiter_on_filter(event_bus):
    async def _triggers():
        await trio.sleep(0)
        event_bus.send("foo", id=1)
        event_bus.send("foo", id=42)

    async with trio.open_service_nursery() as nursery:
        with event_bus.waiter_on("foo", filter=lambda event, id: id == 42) as waiter:
            nursery.start_soon(_triggers)
            event = await waiter.wait()
            assert event == ("foo", {"id": 42})


@pytest.mark.trio
async def test_waiter_on_first(event_bus):
    async def _trigger(event, id):
        await trio.sleep(0)
        event_bus.send(event, id=id)

    async with trio.open_service_nursery() as nursery:
        with event_bus.waiter_on_first("foo", "bar") as waiter:
            nursery.start_soon(_trigger, "foo", 1)
            event = await waiter.wait()
            assert event == ("foo", {"id": 1})

            # Event is ignored
            event_bus.send("bar", id=2)
            same_event = await waiter.wait()
            assert same_event == event

            waiter.clear()
            nursery.start_soon(_trigger, "bar", 3)
            event = await waiter.wait()
            assert event == ("bar", {"id": 3})

        assert event_bus.stats() == {}


@pytest.mark.trio
async def test_waiter_on_first_filter(event_bus):
    async def _triggers():
        await trio.sleep(0)
        event_bus.send("foo", id=1)
        event_bus.send("bar", id=42)

    async with trio.open_service_nursery() as nursery:
        with event_bus.waiter_on_first("foo", "bar", filter=lambda event, id: id == 42) as waiter:
            nursery.start_soon(_triggers)
            event = await waiter.wait()
            assert event == ("bar", {"id": 42})


def test_connection_context(event_bus):
    events_received = []

    def _listen_from_global(event):
        events_received.append(("global", event))

    def _listen_from_context(event):
        events_received.append(("ctx", event))

    event_bus.connect("foo", _listen_from_global)

    with event_bus.connection_context() as event_bus_ctx:
        event_bus_ctx.connect("foo", _listen_from_context)
        event_bus_ctx.connect("bar", _listen_from_context)

        assert event_bus.stats() == {"foo": 2, "bar": 1}

        event_bus.send("foo")
        assert events_received == [("global", "foo"), ("ctx", "foo")]

        events_received.clear()
        event_bus_ctx.send("foo")
        assert events_received == [("global", "foo"), ("ctx", "foo")]

        events_received.clear()
        event_bus_ctx.send("bar")
        assert events_received == [("ctx", "bar")]

        event_bus_ctx.disconnect("foo", _listen_from_context)
        events_received.clear()
        event_bus_ctx.send("foo")
        assert events_received == [("global", "foo")]

        assert event_bus.stats() == {"foo": 1, "bar": 1}

    assert event_bus.stats() == {"foo": 1}

    events_received.clear()
    event_bus_ctx.send("foo")
    assert events_received == [("global", "foo")]
