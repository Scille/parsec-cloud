import trio

from parsec.event_bus import EventBus
from parsec.api.protocole import events_subscribe_serializer, events_listen_serializer
from parsec.backend.utils import catch_protocole_errors


def _pinged_callback_factory(client_ctx, pings):
    pings = set(pings)

    def _on_pinged(event, author, ping):
        if author == client_ctx.device_id or ping not in pings:
            return

        try:
            client_ctx.events.put_nowait({"event": event, "ping": ping})
        except trio.WouldBlock:
            client_ctx.logger.warning("event queue is full")

    return _on_pinged


def _beacon_updated_callback_factory(client_ctx, beacons_ids):
    beacons_ids = set(beacons_ids)

    def _on_beacon_updated(event, author, beacon_id, index, src_id, src_version):
        if author == client_ctx.device_id or beacon_id not in beacons_ids:
            return

        msg = {
            "event": event,
            "beacon_id": beacon_id,
            "index": index,
            "src_id": src_id,
            "src_version": src_version,
        }
        try:
            client_ctx.events.put_nowait(msg)
        except trio.WouldBlock:
            client_ctx.logger.warning("event queue is full")

    return _on_beacon_updated


def _message_received_callback_factory(client_ctx):
    def _on_message_received(event, author, recipient, index):
        if author == client_ctx.device_id or recipient != client_ctx.user_id:
            return

        try:
            client_ctx.events.put_nowait({"event": event, "index": index})
        except trio.WouldBlock:
            client_ctx.logger.warning("event queue is full")

    return _on_message_received


class EventsComponent:
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus

    @catch_protocole_errors
    async def api_events_subscribe(self, client_ctx, msg):
        msg = events_subscribe_serializer.req_load(msg)

        new_subscribed_events = []

        if msg["pinged"]:
            on_pinged = _pinged_callback_factory(client_ctx, msg["pinged"])
            new_subscribed_events.append(on_pinged)
            self.event_bus.connect("pinged", on_pinged, weak=True)

        if msg["beacon_updated"]:
            on_beacon_updated = _beacon_updated_callback_factory(client_ctx, msg["beacon_updated"])
            new_subscribed_events.append(on_beacon_updated)
            self.event_bus.connect("beacon.updated", on_beacon_updated, weak=True)

        if msg["message_received"]:
            on_message_received = _message_received_callback_factory(client_ctx)
            new_subscribed_events.append(on_message_received)
            self.event_bus.connect("message.received", on_message_received, weak=True)

        # Note: previous subscribed events should be disconnected by doing this
        client_ctx.subscribed_events = tuple(new_subscribed_events)

        return events_subscribe_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_events_listen(self, client_ctx, msg):
        msg = events_listen_serializer.req_load(msg)

        if msg["wait"]:
            event_data = await client_ctx.events.get()
        else:
            try:
                event_data = client_ctx.events.get_nowait()
            except trio.WouldBlock:
                return {"status": "no_events"}

        return events_listen_serializer.rep_dump({"status": "ok", **event_data})
