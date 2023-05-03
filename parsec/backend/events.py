# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Awaitable, Callable, Type

import trio

from parsec._parsec import (
    BackendEvent,
    BackendEventCertificatesUpdated,
    BackendEventInviteStatusChanged,
    BackendEventMessageReceived,
    BackendEventPinged,
    BackendEventPkiEnrollmentUpdated,
    BackendEventRealmMaintenanceFinished,
    BackendEventRealmMaintenanceStarted,
    BackendEventRealmRolesUpdated,
    BackendEventRealmVlobsUpdated,
    authenticated_cmds,
)
from parsec.api.protocol.types import UserProfile
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.realm import BaseRealmComponent
from parsec.backend.utils import api, api_ws_cancel_on_client_sending_new_cmd


def internal_to_api_v2_v3_events(
    event: BackendEvent,
) -> authenticated_cmds.v3.events_listen.APIEvent | None:
    event_listen_cmd_mod = authenticated_cmds.v3.events_listen
    if isinstance(event, BackendEventCertificatesUpdated):
        # Ignore this event
        return None
    elif isinstance(event, BackendEventPinged):
        return event_listen_cmd_mod.APIEventPinged(event.ping)
    elif isinstance(event, BackendEventMessageReceived):
        return event_listen_cmd_mod.APIEventMessageReceived(event.index)
    elif isinstance(event, BackendEventInviteStatusChanged):
        return event_listen_cmd_mod.APIEventInviteStatusChanged(
            token=event.token, invitation_status=event.status
        )
    elif isinstance(event, BackendEventRealmMaintenanceStarted):
        return event_listen_cmd_mod.APIEventRealmMaintenanceStarted(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmMaintenanceFinished):
        return event_listen_cmd_mod.APIEventRealmMaintenanceFinished(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmVlobsUpdated):
        return event_listen_cmd_mod.APIEventRealmVlobsUpdated(
            realm_id=event.realm_id,
            checkpoint=event.checkpoint,
            src_id=event.src_id,
            src_version=event.src_version,
        )
    elif isinstance(event, BackendEventRealmRolesUpdated):
        return event_listen_cmd_mod.APIEventRealmRolesUpdated(
            realm_id=event.realm_id,
            role=event.role,
        )
    else:
        assert isinstance(event, BackendEventPkiEnrollmentUpdated)
        return event_listen_cmd_mod.APIEventPkiEnrollmentUpdated()


def internal_to_api_events(
    event: BackendEvent, redacted: bool
) -> authenticated_cmds.latest.events_listen.APIEvent | None:
    event_listen_cmd_mod = authenticated_cmds.latest.events_listen

    if isinstance(event, BackendEventCertificatesUpdated):
        certificate = (
            event.certificate
            if not redacted or event.redacted_certificate is None
            else event.redacted_certificate
        )
        return event_listen_cmd_mod.APIEventCertificatesUpdated(
            certificate=certificate,
        )
    elif isinstance(event, BackendEventPinged):
        return event_listen_cmd_mod.APIEventPinged(event.ping)
    elif isinstance(event, BackendEventMessageReceived):
        return event_listen_cmd_mod.APIEventMessageReceived(event.index, event.message)
    elif isinstance(event, BackendEventInviteStatusChanged):
        return event_listen_cmd_mod.APIEventInviteStatusChanged(
            token=event.token, invitation_status=event.status
        )
    elif isinstance(event, BackendEventRealmMaintenanceStarted):
        return event_listen_cmd_mod.APIEventRealmMaintenanceStarted(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmMaintenanceFinished):
        return event_listen_cmd_mod.APIEventRealmMaintenanceFinished(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmVlobsUpdated):
        return event_listen_cmd_mod.APIEventRealmVlobsUpdated(
            realm_id=event.realm_id,
            checkpoint=event.checkpoint,
            src_id=event.src_id,
            src_version=event.src_version,
        )
    elif isinstance(event, BackendEventRealmRolesUpdated):
        # Ignore this event
        return None
    else:
        assert isinstance(event, BackendEventPkiEnrollmentUpdated)
        return event_listen_cmd_mod.APIEventPkiEnrollmentUpdated()


class EventsComponent:
    def __init__(
        self, realm_component: BaseRealmComponent, send_event: Callable[..., Awaitable[None]]
    ):
        self._realm_component = realm_component
        self.send = send_event

    @api
    async def api_events_subscribe(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.events_subscribe.Req,
    ) -> authenticated_cmds.latest.events_subscribe.Rep:
        # Only Websocket transport need to subscribe events
        if client_ctx.event_bus_ctx:
            await self.connect_events(client_ctx)
        return authenticated_cmds.latest.events_subscribe.RepOk()

    async def connect_events(self, client_ctx: AuthenticatedClientContext) -> None:
        def _on_certificates_updated(
            event: Type[BackendEvent],
            payload: BackendEventCertificatesUpdated,
        ) -> None:
            if payload.organization_id != client_ctx.organization_id:
                return

            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_roles_updated(
            event: Type[BackendEvent],
            payload: BackendEventRealmRolesUpdated,
        ) -> None:
            if (
                payload.organization_id != client_ctx.organization_id
                or payload.user != client_ctx.user_id
            ):
                return

            if payload.role is None:
                client_ctx.realms.discard(payload.realm_id)
            else:
                client_ctx.realms.add(payload.realm_id)

            # Note for this event we don't filter out the ones sent by the client's
            # device, there is two reason for this:
            # 1) A user cannot change it own role, so this case should never occur
            # 2) Returning this event inform the peer we are ready to send it
            #    `realm.vlobs_updated` events on this realm (especially useful during tests)
            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_pinged(
            event: Type[BackendEvent],
            payload: BackendEventPinged,
        ) -> None:
            if (
                payload.organization_id != client_ctx.organization_id
                or payload.author == client_ctx.device_id
            ):
                return

            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_realm_events(
            event: Type[BackendEvent],
            payload: BackendEventRealmVlobsUpdated
            | BackendEventRealmMaintenanceFinished
            | BackendEventRealmMaintenanceStarted,
        ) -> None:
            if (
                payload.organization_id != client_ctx.organization_id
                or payload.author == client_ctx.device_id
                or payload.realm_id not in client_ctx.realms
            ):
                return
            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_message_received(
            event: Type[BackendEvent],
            payload: BackendEventMessageReceived,
        ) -> None:
            if (
                payload.organization_id != client_ctx.organization_id
                or payload.recipient != client_ctx.user_id
            ):
                return

            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_invite_status_changed(
            event: Type[BackendEvent],
            payload: BackendEventInviteStatusChanged,
        ) -> None:
            if (
                payload.organization_id != client_ctx.organization_id
                or payload.greeter != client_ctx.user_id
            ):
                return

            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        def _on_pki_enrollment_updated(
            event: Type[BackendEvent],
            payload: BackendEventPkiEnrollmentUpdated,
        ) -> None:
            if (
                payload.organization_id != client_ctx.organization_id
                and client_ctx.profile != UserProfile.ADMIN
            ):
                return
            try:
                client_ctx.send_events_channel.send_nowait(payload)
            except trio.WouldBlock:
                client_ctx.close_connection_asap()

        # Command should be idempotent
        if not client_ctx.events_subscribed:
            # Connect the new callbacks
            client_ctx.event_bus_ctx.connect(
                BackendEventCertificatesUpdated,
                _on_certificates_updated,  # type: ignore
            )
            client_ctx.event_bus_ctx.connect(
                BackendEventPinged,
                _on_pinged,  # type: ignore
            )
            client_ctx.event_bus_ctx.connect(
                BackendEventRealmVlobsUpdated,
                _on_realm_events,  # type: ignore
            )
            client_ctx.event_bus_ctx.connect(
                BackendEventRealmMaintenanceStarted,
                _on_realm_events,  # type: ignore
            )
            client_ctx.event_bus_ctx.connect(
                BackendEventRealmMaintenanceFinished,
                _on_realm_events,  # type: ignore
            )
            client_ctx.event_bus_ctx.connect(
                BackendEventMessageReceived,
                _on_message_received,  # type: ignore
            )
            client_ctx.event_bus_ctx.connect(
                BackendEventInviteStatusChanged,
                _on_invite_status_changed,  # type: ignore
            )

            client_ctx.event_bus_ctx.connect(
                BackendEventPkiEnrollmentUpdated,
                _on_pki_enrollment_updated,  # type: ignore
            )

            # Final event to keep up to date the list of realm we should listen on
            client_ctx.event_bus_ctx.connect(
                BackendEventRealmRolesUpdated,
                _on_roles_updated,  # type: ignore
            )

            # Finally populate the list of realm we should listen on
            realms_for_user = await self._realm_component.get_realms_for_user(
                client_ctx.organization_id, client_ctx.user_id
            )
            client_ctx.realms = set(realms_for_user.keys())
            client_ctx.events_subscribed = True

    @api_ws_cancel_on_client_sending_new_cmd
    @api
    async def api_v2_v3_events_listen(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.v3.events_listen.Req,
    ) -> authenticated_cmds.v3.events_listen.Rep:
        while True:
            if req.wait:
                event = await client_ctx.receive_events_channel.receive()

            else:
                try:
                    event = client_ctx.receive_events_channel.receive_nowait()
                except trio.WouldBlock:
                    return authenticated_cmds.v3.events_listen.RepNoEvents()

            unit = internal_to_api_v2_v3_events(event)
            if not unit:
                # Ignore the current event
                continue

            return authenticated_cmds.v3.events_listen.RepOk(unit)

    @api_ws_cancel_on_client_sending_new_cmd
    @api
    async def api_events_listen(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.events_listen.Req,
    ) -> authenticated_cmds.latest.events_listen.Rep:
        need_redacted = client_ctx.profile == UserProfile.OUTSIDER

        while True:
            if req.wait:
                event = await client_ctx.receive_events_channel.receive()

            else:
                try:
                    event = client_ctx.receive_events_channel.receive_nowait()
                except trio.WouldBlock:
                    return authenticated_cmds.latest.events_listen.RepNoEvents()

            unit = internal_to_api_events(event, redacted=need_redacted)
            if not unit:
                # Ignore the current event
                continue

            return authenticated_cmds.latest.events_listen.RepOk(unit)
