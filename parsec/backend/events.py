# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio

from parsec.event_bus import EventBus
from parsec.api.protocole import events_subscribe_serializer, events_listen_serializer
from parsec.backend.utils import catch_protocole_errors
from parsec.backend.realm import BaseRealmComponent


class EventsComponent:
    def __init__(self, event_bus: EventBus, realm_component: BaseRealmComponent):
        self.event_bus = event_bus
        self.realm_component = realm_component

    @catch_protocole_errors
    async def api_events_subscribe(self, client_ctx, msg):
        msg = events_subscribe_serializer.req_load(msg)

        def _on_roles_updated(event, organization_id, author, realm_id, user, role):
            if organization_id != client_ctx.organization_id or user != client_ctx.user_id:
                return

            if role is None:
                client_ctx.realms.discard(realm_id)
            else:
                client_ctx.realms.add(realm_id)

            # Note for this event we don't filter out the ones sent by the client's
            # device, there is two reason for this:
            # 1) A user cannot change it own role, so this case should never occur
            # 2) Returning this event inform the peer we are ready to send it
            #    `realm.vlobs_updated` events on this realm (especially useful during tests)
            try:
                client_ctx.send_events_channel.send_nowait(
                    {"event": event, "realm_id": realm_id, "role": role}
                )
            except trio.WouldBlock:
                client_ctx.logger.warning(f"event queue is full for {client_ctx}")

        def _on_pinged(event, organization_id, author, ping):
            if organization_id != client_ctx.organization_id or author == client_ctx.device_id:
                return

            try:
                client_ctx.send_events_channel.send_nowait({"event": event, "ping": ping})
            except trio.WouldBlock:
                client_ctx.logger.warning(f"event queue is full for {client_ctx}")

        def _on_realm_events(event, organization_id, author, realm_id, **kwargs):
            if (
                organization_id != client_ctx.organization_id
                or author == client_ctx.device_id
                or realm_id not in client_ctx.realms
            ):
                return

            msg = {"event": event, "realm_id": realm_id, **kwargs}
            try:
                client_ctx.send_events_channel.send_nowait(msg)
            except trio.WouldBlock:
                client_ctx.logger.warning(f"event queue is full for {client_ctx}")

        def _on_message_received(event, organization_id, author, recipient, index):
            if organization_id != client_ctx.organization_id or recipient != client_ctx.user_id:
                return

            try:
                client_ctx.send_events_channel.send_nowait({"event": event, "index": index})
            except trio.WouldBlock:
                client_ctx.logger.warning(f"event queue is full for {client_ctx}")

        # Drop previous event callbacks if any
        client_ctx.event_bus_ctx.clear()

        # Connect the new callbacks
        client_ctx.event_bus_ctx.connect("pinged", _on_pinged)
        client_ctx.event_bus_ctx.connect("realm.vlobs_updated", _on_realm_events)
        client_ctx.event_bus_ctx.connect("realm.maintenance_started", _on_realm_events)
        client_ctx.event_bus_ctx.connect("realm.maintenance_finished", _on_realm_events)
        client_ctx.event_bus_ctx.connect("message.received", _on_message_received)

        # Final event to keep up to date the list of realm we should listen on
        client_ctx.event_bus_ctx.connect("realm.roles_updated", _on_roles_updated)

        # Finally populate the list of realm we should listen on
        realms_for_user = await self.realm_component.get_realms_for_user(
            client_ctx.organization_id, client_ctx.user_id
        )
        client_ctx.realms = set(realms_for_user.keys())

        return events_subscribe_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_events_listen(self, client_ctx, msg):
        msg = events_listen_serializer.req_load(msg)

        if msg["wait"]:
            # This is kind of a special case here:
            # unlike other requests this one is going to (potentially) take
            # a long time to complete. In the meantime we must monitor the
            # connection with the client in order to make sure it is still
            # online and handles websocket pings
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
