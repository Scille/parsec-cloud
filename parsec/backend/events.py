import trio

from parsec.event_bus import EventBus
from parsec.api.protocole import events_subscribe_serializer, events_listen_serializer
from parsec.backend.utils import catch_protocole_errors


def _pinged_callback_factory(client_ctx, pings):
    pings = set(pings)

    def _on_pinged(event, organization_id, author, ping):
        if (
            organization_id != client_ctx.organization_id
            or author == client_ctx.device_id
            or ping not in pings
        ):
            return

        try:
            client_ctx.send_events_channel.send_nowait({"event": event, "ping": ping})
        except trio.WouldBlock:
            client_ctx.logger.warning("event queue is full")

    return _on_pinged


def _beacon_updated_callback_factory(client_ctx, beacons_ids):
    beacons_ids = set(beacons_ids)

    def _on_beacon_updated(event, organization_id, author, beacon_id, index, src_id, src_version):
        if (
            organization_id != client_ctx.organization_id
            or author == client_ctx.device_id
            or beacon_id not in beacons_ids
        ):
            return

        msg = {
            "event": event,
            "beacon_id": beacon_id,
            "index": index,
            "src_id": src_id,
            "src_version": src_version,
        }
        try:
            client_ctx.send_events_channel.send_nowait(msg)
        except trio.WouldBlock:
            client_ctx.logger.warning("event queue is full")

    return _on_beacon_updated


def _message_received_callback_factory(client_ctx):
    def _on_message_received(event, organization_id, author, recipient, index):
        if (
            organization_id != client_ctx.organization_id
            or author == client_ctx.device_id
            or recipient != client_ctx.user_id
        ):
            return

        try:
            client_ctx.send_events_channel.send_nowait({"event": event, "index": index})
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
            # This is kind of a special case here:
            # unlike other requests this one is going to (potentially) take
            # a long time to complete. In the meantime we must monitor the
            # connection with the client in order to make sure it is still
            # online and handle websocket pings
            event_data = None

            async def _get_event(cancel_scope):
                nonlocal event_data
                event_data = await client_ctx.receive_events_channel.receive()
                cancel_scope.cancel()

            async def _keep_transport_breathing(cancel_scope):
                # If a command is received, the client is violating the
                # request/reply pattern. We consider this as an order to stop
                # listening events.
                await client_ctx.transport.recv()
                cancel_scope.cancel()

            async with trio.open_nursery() as nursery:
                nursery.start_soon(_get_event, nursery.cancel_scope)
                nursery.start_soon(_keep_transport_breathing, nursery.cancel_scope)

            if not event_data:
                return {"status": "cancelled", "reason": "Client cancelled the listening"}

        else:
            try:
                event_data = client_ctx.receive_events_channel.receive_nowait()
            except trio.WouldBlock:
                return {"status": "no_events"}

        return events_listen_serializer.rep_dump({"status": "ok", **event_data})
