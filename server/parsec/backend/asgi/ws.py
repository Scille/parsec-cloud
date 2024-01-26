# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from functools import partial
from typing import Any, Awaitable, Callable, NoReturn, Type, TypeVar, Union, cast

import trio
from quart import Blueprint, Websocket, g, websocket
from structlog import get_logger
from wsproto.utilities import LocalProtocolError

from parsec._parsec import (
    BackendEvent,
    BackendEventInviteStatusChanged,
    BackendEventOrganizationExpired,
    BackendEventUserUpdatedOrRevoked,
    InvitationStatus,
    OrganizationID,
    ProtocolError,
)
from parsec.backend.app import BackendApp
from parsec.backend.asgi.rpc import (
    AUTHENTICATED_CMDS_LOAD_FN,
    INVITED_CMDS_LOAD_FN,
)
from parsec.backend.client_context import (
    AuthenticatedClientContext,
    BaseClientContext,
    InvitedClientContext,
)
from parsec.backend.handshake import do_handshake
from parsec.backend.invite import CloseInviteConnection
from parsec.backend.utils import CancelledByNewCmd, run_with_cancel_on_client_sending_new_cmd
from parsec.serde import packb

Ctx = TypeVar("Ctx", bound=BaseClientContext)
R = dict[str, object]

logger = get_logger()


ws_bp = Blueprint("ws_api", __name__)


@ws_bp.websocket("/ws")
async def handle_ws() -> None:
    backend: BackendApp = g.backend
    selected_logger = logger

    # 1) Handshake

    client_ctx, error_infos = await do_handshake(backend, websocket)
    if not client_ctx:
        # Invalid handshake
        # TODO Fragile test based on reason, make it more robust
        if error_infos and error_infos.get("reason", "") == "Expired organization":
            organization_id = error_infos["organization_id"]
            assert isinstance(organization_id, OrganizationID)
            await backend.events.send(
                BackendEventOrganizationExpired(organization_id=organization_id)
            )
        selected_logger.info("Connection dropped: bad handshake", **error_infos)
        return

    selected_logger = client_ctx.logger
    selected_logger.info(
        f"Connection established (client/server API version: {client_ctx.client_api_version}/{client_ctx.api_version})"
    )

    # 2) Setup events listener according to client type

    if isinstance(client_ctx, AuthenticatedClientContext):
        with trio.CancelScope() as cancel_scope:
            client_ctx.cancel_scope = cancel_scope
            with backend.event_bus.connection_context() as client_ctx.event_bus_ctx:

                def _on_updated_or_revoked(
                    client_ctx: AuthenticatedClientContext,
                    event: Type[BackendEvent],
                    event_id: str,
                    payload: BackendEventUserUpdatedOrRevoked,
                ) -> None:
                    if (
                        payload.organization_id == client_ctx.organization_id
                        and payload.user_id == client_ctx.user_id
                    ):
                        # We close the connection even if the event is about a profile
                        # change and not a revocation given `client_ctx` is outdated
                        client_ctx.close_connection_asap()

                def _on_expired(
                    client_ctx: AuthenticatedClientContext,
                    event: Type[BackendEvent],
                    event_id: str,
                    payload: BackendEventOrganizationExpired,
                ) -> None:
                    if payload.organization_id == client_ctx.organization_id:
                        client_ctx.close_connection_asap()

                client_ctx.event_bus_ctx.connect(
                    BackendEventUserUpdatedOrRevoked,
                    partial(_on_updated_or_revoked, client_ctx),
                )
                client_ctx.event_bus_ctx.connect(
                    BackendEventOrganizationExpired,
                    partial(_on_expired, client_ctx),
                )

                # 3) Serve commands
                load_fn = AUTHENTICATED_CMDS_LOAD_FN[client_ctx.api_version.version]
                await _handle_client_websocket_loop(backend, load_fn, websocket, client_ctx)

    else:
        assert isinstance(client_ctx, InvitedClientContext)

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
                        event: Type[BackendEvent],
                        event_id: str,
                        payload: BackendEventInviteStatusChanged,
                    ) -> None:
                        if (
                            payload.status == InvitationStatus.DELETED
                            and payload.organization_id == client_ctx.organization_id
                            and payload.token == client_ctx.invitation.token
                        ):
                            client_ctx.close_connection_asap()

                    event_bus_ctx.connect(
                        BackendEventInviteStatusChanged,
                        partial(_on_invite_status_changed, client_ctx),
                    )

                    # 3) Serve commands
                    load_fn = INVITED_CMDS_LOAD_FN[client_ctx.api_version.version]
                    await _handle_client_websocket_loop(backend, load_fn, websocket, client_ctx)

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


async def _handle_client_websocket_loop(
    backend: BackendApp,
    load_req_fn: Callable[[bytes], Any],
    websocket: Websocket,
    client_ctx: Ctx,
) -> NoReturn:
    raw_req: Union[None, bytes, str] = None
    while True:
        # raw_req can be already defined if we received a new request
        # while processing a command
        raw_req = raw_req or await websocket.receive()
        try:
            # `WebSocket` can return both bytes or utf8-string messages, we only accept the former
            if not isinstance(raw_req, bytes):
                raise ProtocolError
            req = load_req_fn(raw_req)

        except ProtocolError:
            raw_rep = packb({"status": "invalid_msg_format", "reason": "Invalid message format"})

        else:
            cmd_func = cast(Callable[[Any, Any], Awaitable[Any]], backend.apis[type(req)])

            try:
                if cmd_func._api_info["cancel_on_client_sending_new_cmd"]:  # type: ignore[attr-defined]
                    rep = await run_with_cancel_on_client_sending_new_cmd(
                        websocket, cmd_func, client_ctx, req
                    )
                else:
                    rep = await cmd_func(client_ctx, req)

                raw_rep = rep.dump()

            except CancelledByNewCmd as exc:
                # Long command handling such as message_get can be cancelled
                # when the peer send a new request
                raw_req = exc.new_raw_req
                continue

            # TODO: cmd/response status should be in snakecase...
            client_ctx.logger.info("Request", cmd=type(req).__name__, status=type(rep).__name__)

        try:
            await websocket.send(raw_rep)
        except LocalProtocolError:
            # Ignore exception if the websocket is closed
            # This used to be the behavior with wsproto < 1.2.0
            pass
        raw_req = None
