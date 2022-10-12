# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import trio
from typing import Callable, Union, Type
from functools import partial

from parsec._parsec import (
    EventsListenRep,
    EventsListenRepNoEvents,
    EventsListenRepOkInviteStatusChanged,
    EventsListenRepOkMessageReceived,
    EventsListenRepOkPinged,
    EventsListenRepOkPkiEnrollmentUpdated,
    EventsListenRepOkRealmMaintenanceFinished,
    EventsListenRepOkRealmMaintenanceStarted,
    EventsListenRepOkRealmRolesUpdated,
    EventsListenRepOkRealmVlobsUpdated,
    EventsListenReq,
    EventsSubscribeRep,
    EventsSubscribeRepOk,
    EventsSubscribeReq,
)
from parsec.api.protocol import (
    OrganizationID,
    DeviceID,
    UserID,
    RealmID,
    RealmRole,
    InvitationStatus,
    InvitationToken,
)
from parsec.api.protocol.base import api_typed_msg_adapter
from parsec.api.protocol.types import UserProfile
from parsec.backend.utils import catch_protocol_errors, api
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.realm import BaseRealmComponent
from parsec.backend.backend_events import BackendEvent


class EventsComponent:
    def __init__(self, realm_component: BaseRealmComponent, send_event: Callable):
        self._realm_component = realm_component
        self.send = send_event

    @api("events_subscribe")
    @catch_protocol_errors
    @api_typed_msg_adapter(EventsSubscribeReq, EventsSubscribeRep)
    async def api_events_subscribe(self, client_ctx: AuthenticatedClientContext, msg):
        def _on_roles_updated(
            backend_event: BackendEvent,
            organization_id: OrganizationID,
            author: DeviceID,
            realm_id: RealmID,
            user: UserID,
            role: RealmRole,
        ) -> None:
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
                    EventsListenRepOkRealmRolesUpdated(realm_id, role)
                )
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_pinged(
            backend_event: BackendEvent,
            organization_id: OrganizationID,
            author: DeviceID,
            ping: str,
        ) -> None:
            if organization_id != client_ctx.organization_id or author == client_ctx.device_id:
                return

            try:
                client_ctx.send_events_channel.send_nowait(EventsListenRepOkPinged(ping))
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_realm_events(
            events_listen_rep_cls: Union[
                Type[EventsListenRepOkRealmVlobsUpdated],
                Type[EventsListenRepOkRealmMaintenanceStarted],
                Type[EventsListenRepOkRealmMaintenanceFinished],
            ],
            backend_event: BackendEvent,
            organization_id: OrganizationID,
            author: DeviceID,
            realm_id: RealmID,
            **kwargs,
        ) -> None:
            if (
                organization_id != client_ctx.organization_id
                or author == client_ctx.device_id
                or realm_id not in client_ctx.realms
            ):
                return
            try:
                client_ctx.send_events_channel.send_nowait(
                    events_listen_rep_cls(realm_id, **kwargs)
                )
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_message_received(
            backend_event: BackendEvent,
            organization_id: OrganizationID,
            author: DeviceID,
            recipient: UserID,
            index: int,
        ) -> None:
            if organization_id != client_ctx.organization_id or recipient != client_ctx.user_id:
                return

            try:
                client_ctx.send_events_channel.send_nowait(EventsListenRepOkMessageReceived(index))
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_invite_status_changed(
            backend_event: BackendEvent,
            organization_id: OrganizationID,
            greeter: UserID,
            token: InvitationToken,
            status: InvitationStatus,
        ) -> None:
            if organization_id != client_ctx.organization_id or greeter != client_ctx.user_id:
                return

            try:
                client_ctx.send_events_channel.send_nowait(
                    EventsListenRepOkInviteStatusChanged(token, status)
                )
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_pki_enrollment_updated(
            backend_event: BackendEvent,
            organization_id: OrganizationID,
        ) -> None:
            if (
                organization_id != client_ctx.organization_id
                and client_ctx.profile != UserProfile.ADMIN
            ):
                return
            try:
                client_ctx.send_events_channel.send_nowait(EventsListenRepOkPkiEnrollmentUpdated())
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        # Command should be idempotent
        if not client_ctx.events_subscribed:
            # Connect the new callbacks
            client_ctx.event_bus_ctx.connect(
                BackendEvent.PINGED, _on_pinged  # type: ignore[arg-type]
            )
            client_ctx.event_bus_ctx.connect(
                BackendEvent.REALM_VLOBS_UPDATED,
                partial(_on_realm_events, EventsListenRepOkRealmVlobsUpdated),
            )
            client_ctx.event_bus_ctx.connect(
                BackendEvent.REALM_MAINTENANCE_STARTED,
                partial(_on_realm_events, EventsListenRepOkRealmMaintenanceStarted),
            )
            client_ctx.event_bus_ctx.connect(
                BackendEvent.REALM_MAINTENANCE_FINISHED,
                partial(_on_realm_events, EventsListenRepOkRealmMaintenanceFinished),
            )
            client_ctx.event_bus_ctx.connect(
                BackendEvent.MESSAGE_RECEIVED,
                _on_message_received,  # type: ignore[arg-type]
            )
            client_ctx.event_bus_ctx.connect(
                BackendEvent.INVITE_STATUS_CHANGED,
                _on_invite_status_changed,  # type: ignore[arg-type]
            )

            client_ctx.event_bus_ctx.connect(
                BackendEvent.PKI_ENROLLMENTS_UPDATED,
                _on_pki_enrollment_updated,  # type: ignore[arg-type]
            )

            # Final event to keep up to date the list of realm we should listen on
            client_ctx.event_bus_ctx.connect(
                BackendEvent.REALM_ROLES_UPDATED,
                _on_roles_updated,  # type: ignore[arg-type]
            )

            # Finally populate the list of realm we should listen on
            realms_for_user = await self._realm_component.get_realms_for_user(
                client_ctx.organization_id, client_ctx.user_id
            )
            client_ctx.realms = set(realms_for_user.keys())
            client_ctx.events_subscribed = True

        return EventsSubscribeRepOk()

    @api("events_listen", cancel_on_client_sending_new_cmd=True)
    @catch_protocol_errors
    @api_typed_msg_adapter(EventsListenReq, EventsListenRep)
    async def api_events_listen(self, client_ctx, msg: EventsListenReq):
        if msg.wait:
            event_rep = await client_ctx.receive_events_channel.receive()

        else:
            try:
                event_rep = client_ctx.receive_events_channel.receive_nowait()
            except trio.WouldBlock:
                return EventsListenRepNoEvents()

        assert isinstance(event_rep, EventsListenRep), event_rep
        return event_rep
