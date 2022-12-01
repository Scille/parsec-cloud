# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from functools import partial
from typing import Awaitable, Callable, NoReturn, TypeVar, Union

import trio
from quart import Blueprint, Websocket, g, websocket
from structlog import get_logger

from parsec._parsec import ProtocolError
from parsec.api.protocol import (
    InvalidMessageError,
    InvitationStatus,
    InvitationToken,
    OrganizationID,
    UserID,
    packb,
    unpackb,
)
from parsec.api.protocol.base import MessageSerializationError
from parsec.backend.app import BackendApp
from parsec.backend.backend_events import BackendEvent
from parsec.backend.client_context import (
    APIV1_AnonymousClientContext,
    AuthenticatedClientContext,
    BaseClientContext,
    InvitedClientContext,
)
from parsec.backend.handshake import do_handshake
from parsec.backend.invite import CloseInviteConnection
from parsec.backend.utils import CancelledByNewCmd, run_with_cancel_on_client_sending_new_cmd

Ctx = TypeVar("Ctx", bound=BaseClientContext)
R = dict[str, object]

logger = get_logger()


ws_bp = Blueprint("ws_api", __name__)


@ws_bp.websocket("/ws")
async def handle_ws() -> None:
    backend: BackendApp = g.backend
    selected_logger = logger

    # TODO: try/except on TransportError & MessageSerializationError ?

    # 1) Handshake

    client_ctx, error_infos = await do_handshake(backend, websocket)
    if not client_ctx:
        # Invalid handshake
        # TODO Fragile test based on reason, make it more robust
        if error_infos and error_infos.get("reason", "") == "Expired organization":
            organization_id = error_infos["organization_id"]
            await backend.events.send(
                BackendEvent.ORGANIZATION_EXPIRED, organization_id=organization_id
            )
        selected_logger.info("Connection dropped: bad handshake", **error_infos)
        return

    selected_logger = client_ctx.logger
    selected_logger.info("Connection established")

    # 2) Setup events listener according to client type

    # Retrieve the allowed commands according to api version and auth type
    api_cmds = backend.apis[client_ctx.TYPE]

    if isinstance(client_ctx, AuthenticatedClientContext):
        with trio.CancelScope() as cancel_scope:
            client_ctx.cancel_scope = cancel_scope
            with backend.event_bus.connection_context() as client_ctx.event_bus_ctx:

                def _on_revoked(
                    client_ctx: AuthenticatedClientContext,
                    event: BackendEvent,
                    organization_id: OrganizationID,
                    user_id: UserID,
                ) -> None:
                    if (
                        organization_id == client_ctx.organization_id
                        and user_id == client_ctx.user_id
                    ):
                        client_ctx.close_connection_asap()

                def _on_expired(
                    client_ctx: AuthenticatedClientContext,
                    event: BackendEvent,
                    organization_id: OrganizationID,
                ) -> None:
                    if organization_id == client_ctx.organization_id:
                        client_ctx.close_connection_asap()

                client_ctx.event_bus_ctx.connect(
                    BackendEvent.USER_REVOKED, partial(_on_revoked, client_ctx)
                )
                client_ctx.event_bus_ctx.connect(
                    BackendEvent.ORGANIZATION_EXPIRED, partial(_on_expired, client_ctx)
                )

                # 3) Serve commands
                await _handle_client_websocket_loop(api_cmds, websocket, client_ctx)

    elif isinstance(client_ctx, InvitedClientContext):
        await backend.invite.claimer_joined(
            organization_id=client_ctx.organization_id,
            greeter=client_ctx.invitation.greeter_user_id,
            token=client_ctx.invitation.token,
        )
        try:
            with trio.CancelScope() as cancel_scope:
                client_ctx.cancel_scope = cancel_scope
                with backend.event_bus.connection_context() as event_bus_ctx:

                    def _on_invite_status_changed(
                        client_ctx: InvitedClientContext,
                        event: BackendEvent,
                        organization_id: OrganizationID,
                        greeter: UserID,
                        token: InvitationToken,
                        status: InvitationStatus,
                    ) -> None:
                        if (
                            status == InvitationStatus.DELETED
                            and organization_id == client_ctx.organization_id
                            and token == client_ctx.invitation.token
                        ):
                            client_ctx.close_connection_asap()

                    event_bus_ctx.connect(
                        BackendEvent.INVITE_STATUS_CHANGED,
                        partial(_on_invite_status_changed, client_ctx),
                    )

                    # 3) Serve commands
                    await _handle_client_websocket_loop(api_cmds, websocket, client_ctx)

        except CloseInviteConnection:
            # If the invitation has been deleted after the invited handshake,
            # an invitation commands can raise an InvitationAlreadyDeletedError.
            # This error is then converted by the api route to a
            # CloseInviteConnection and here we are.
            # In most cases the connection is closed from handling the
            # BackendEvent.INVITE_STATUS_CHANGED event, but nothing guarantee
            # that the event will be handled before a command is processed
            # and finish with this error.
            # So if this happens, let's also close the connection.
            pass

        finally:
            with trio.CancelScope(shield=True):
                await backend.invite.claimer_left(
                    organization_id=client_ctx.organization_id,
                    greeter=client_ctx.invitation.greeter_user_id,
                    token=client_ctx.invitation.token,
                )

    # TODO: remove me once APIv1 is deprecated
    else:
        assert isinstance(client_ctx, APIV1_AnonymousClientContext)
        await _handle_client_websocket_loop(api_cmds, websocket, client_ctx)


async def _handle_client_websocket_loop(
    api_cmds: dict[str, Callable[[Ctx, R], Awaitable[R]]],
    websocket: Websocket,
    client_ctx: Ctx,
) -> NoReturn:

    raw_req: Union[None, bytes, str] = None
    while True:
        # raw_req can be already defined if we received a new request
        # while processing a command
        raw_req = raw_req or await websocket.receive()
        rep: R
        try:
            # Wesocket can return both bytes or utf8-string messages, we only accept the former
            if not isinstance(raw_req, bytes):
                raise MessageSerializationError
            req = unpackb(raw_req)

        except MessageSerializationError:
            rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
            cmd = ""  # Dummy placeholder for the log

        else:

            try:
                cmd = req.get("cmd", "<missing>")
                if not isinstance(cmd, str):
                    raise KeyError()

                cmd_func = api_cmds[cmd]

            except KeyError:
                rep = {"status": "unknown_command", "reason": "Unknown command"}

            else:
                try:
                    if cmd_func._api_info[  # type: ignore[attr-defined]
                        "cancel_on_client_sending_new_cmd"
                    ]:
                        rep = await run_with_cancel_on_client_sending_new_cmd(
                            websocket, cmd_func, client_ctx, req
                        )
                    else:
                        rep = await cmd_func(client_ctx, req)

                except InvalidMessageError as exc:
                    rep = {
                        "status": "bad_message",
                        "errors": exc.errors,
                        "reason": "Invalid message.",
                    }

                except ProtocolError as exc:
                    rep = {"status": "bad_message", "reason": str(exc)}

                except CancelledByNewCmd as exc:
                    # Long command handling such as message_get can be cancelled
                    # when the peer send a new request
                    raw_req = exc.new_raw_req
                    continue

            client_ctx.logger.info("Request", cmd=cmd, status=rep["status"])

        raw_rep = packb(rep)
        await websocket.send(raw_rep)
        raw_req = None
