# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Editics ASGI routes (step 0: auth subset).

Two routes per session, both under the existing authenticated ASGI app
(`server/parsec/asgi/`). They reuse the existing organization-id-in-path pattern
but **not** the existing Parsec `PARSEC-SIGN-ED25519` handshake (see todo
step_0 §3.3):

    GET  /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{vlob_id}/join
    POST /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{vlob_id}/send

- `GET .../join` opens the SSE stream (server→client).
- `POST .../send` carries one client event (client→server).

Identity is the lightweight `Authorization: Editics <device_id_hex>.<participant_uuid_hex>`
header (todo step_0 §3.3). It is **not** a secure authorization system.

The SSE transport lives in this module too (the `EditicsStreamingResponse`
subclass and the `iter_editics_sse_events` framing generator), mirroring the
existing `StreamingResponseMiddleware` in `parsec/asgi/rpc.py`: the ASGI route
is the only consumer of the SSE framing, so it is defined next to it. The
component-layer channel handle (`EditicsSseChannel`) stays in
`parsec/components/editics/base.py` (it is passed between the component and
this route, like `ClientBroadcastableEventStream` for the events SSE route).
"""

from __future__ import annotations

import json
from collections.abc import AsyncGenerator
from typing import Any
from uuid import UUID

import anyio
from fastapi import APIRouter, HTTPException, Request
from fastapi.responses import Response, StreamingResponse
from starlette.requests import ClientDisconnect
from starlette.types import Receive

from parsec._parsec import DeviceID, OrganizationID, VlobID
from parsec.backend import Backend
from parsec.components.editics import (
    ClientEventAuth,
    EditicsClientContext,
    EditicsSseChannel,
    ServerEventAuth,
    ServerEventAuthRejected,
)

editics_router = APIRouter(include_in_schema=False)

# Max size for the RPC body. The only client event in step 0 is `auth`, which is
# tiny, but keep a reasonable ceiling consistent with the rest of the API.
MAX_CONTENT_LENGTH = 1 * 1024 * 1024

ACCEPT_TYPE_SSE = "text/event-stream"


# --- SSE transport -----------------------------------------------------------
#
# This mirrors the existing `StreamingResponseMiddleware` in
# `parsec/asgi/rpc.py` (the authenticated events SSE route) but with the
# editics framing (see todo step_0 §3.2):
#
# - Server *data* events are delivered as a single `data:` line whose JSON
#   carries the `"type"` field used for dispatch. There is **no** `event:`
#   line for data events (this keeps a single discriminated union on `"type"`,
#   consistent with the RPC route).
# - The keepalive reuses the existing Parsec SSE keepalive shape
#   (`event:keepalive\\ndata:\\n\\n`), which is the only place an `event:` line
#   is used on the editics SSE route.


async def iter_editics_sse_events(channel: EditicsSseChannel) -> AsyncGenerator[bytes, None]:
    """Async generator yielding framed SSE bytes for a channel.

    Yields one `data: <json>\\n\\n` block per enqueued server event, and
    `event:keepalive\\ndata:\\n\\n` blocks when no event arrives within the
    keepalive interval. Terminates when the channel is closed (client
    disconnect or server cleanup).
    """
    try:
        while True:
            event: dict[str, Any] | None = None
            with anyio.move_on_after(channel.keepalive) as scope:
                try:
                    event = await channel.receive.receive()
                except anyio.EndOfStream:
                    return

            if scope.cancel_called:
                # Keepalive: the only place an `event:` line is used on the
                # editics SSE route. `data` must be present or SSE clients
                # silently ignore the event (see HTML spec).
                yield b"event:keepalive\ndata:\n\n"
            else:
                if event is None:
                    # Should not happen, but guard regardless.
                    continue
                payload = json.dumps(event, separators=(",", ":"))
                yield f"data: {payload}\n\n".encode()
    finally:
        channel.close()


async def _listen_for_disconnect(receive: Receive) -> None:
    """Wait for the ASGI `http.disconnect` message (mirrors starlette's)."""
    while True:
        message = await receive()
        if message["type"] == "http.disconnect":
            break


class EditicsStreamingResponse(StreamingResponse):
    """SSE response for an editics channel.

    Subclasses `StreamingResponse` to keep the response alive for as long as
    the channel produces events, and to reliably clean up the participant when
    the client disconnects: a dedicated task monitors the ASGI `receive`
    channel for `http.disconnect` (mirrors the `spec_version < (2,4)` path of
    starlette's `StreamingResponse.__call__`, but always, so the disconnect is
    detected promptly even with newer ASGI transports) and closes the channel,
    which lets `iter_editics_sse_events` return and the route's `finally` run
    the leave flow (todo step_0 §8).
    """

    def __init__(self, channel: EditicsSseChannel, **kwargs: Any) -> None:
        self._channel = channel
        super().__init__(content=iter_editics_sse_events(channel), **kwargs)

    async def __call__(self, scope, receive, send) -> None:
        async with anyio.create_task_group() as tg:

            async def watch_disconnect() -> None:
                await _listen_for_disconnect(receive)
                # Client gone: close the channel so the event generator returns
                # and the route's `finally` runs the leave flow.
                self._channel.close()
                tg.cancel_scope.cancel()

            tg.start_soon(watch_disconnect)
            try:
                await super().__call__(scope, receive, send)
            except OSError:
                # The client vanished while we were still trying to send; the
                # disconnect watcher takes care of closing the channel.
                pass
            finally:
                self._channel.close()
                tg.cancel_scope.cancel()


# --- Request helpers ---------------------------------------------------------


def _parse_editics_auth_header(headers, request: Request) -> EditicsClientContext:
    """Parse the `Authorization: Editics <device_id_hex>.<participant_uuid_hex>` header.

    Returns 401 on missing/malformed header. No real auth check in step 0
    (todo step_0 §3.3, §9.1.5).

    The SSE route also accepts the identity as an `authorization` query parameter,
    because the browser's `EventSource` API cannot set custom request headers
    (todo step_0 §9.2). The RPC route always uses the real header.
    """
    raw = headers.get("Authorization")
    if not raw:
        raw = request.query_params.get("authorization")
    if not raw:
        raise HTTPException(status_code=401, detail="Missing Editics authorization")
    expected_scheme, _, rest = raw.partition(" ")
    if expected_scheme != "Editics" or not rest:
        raise HTTPException(status_code=401, detail="Missing Editics authorization")
    device_hex, sep, participant_hex = rest.partition(".")
    if not sep:
        raise HTTPException(status_code=401, detail="Missing Editics authorization")
    try:
        device_id = DeviceID.from_hex(device_hex)
    except ValueError:
        raise HTTPException(status_code=401, detail="Bad Editics authorization")
    try:
        participant_uuid = UUID(hex=participant_hex)
    except ValueError:
        raise HTTPException(status_code=401, detail="Bad Editics authorization")
    return EditicsClientContext(device_id=device_id, participant_uuid=participant_uuid)


def _parse_organization_id(raw_organization_id: str) -> OrganizationID:
    try:
        return OrganizationID(raw_organization_id)
    except ValueError:
        raise HTTPException(status_code=404, detail="Organization not found")


def _parse_vlob_id(raw: str) -> VlobID:
    try:
        return VlobID.from_hex(raw)
    except ValueError:
        raise HTTPException(status_code=404, detail="Bad vlob id")


async def _read_body(request: Request) -> bytes:
    try:
        content_length = int(request.headers["Content-Length"])
    except (ValueError, KeyError):
        content_length = MAX_CONTENT_LENGTH
    else:
        if content_length > MAX_CONTENT_LENGTH:
            raise HTTPException(status_code=413)

    chunks: list[bytes] = []
    try:
        async for chunk in request.stream():
            chunks.append(chunk)
            if sum(len(c) for c in chunks) > content_length:
                raise HTTPException(status_code=413)
    except ClientDisconnect:
        raise HTTPException(status_code=413)
    return b"".join(chunks)


# --- Routes ------------------------------------------------------------------


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/join"
)
async def editics_join(
    raw_organization_id: str, raw_realm_id: str, raw_document_id: str, request: Request
):
    """Open the editics SSE stream for a session (todo step_0 §3.1, §6).

    Registers a *pending* SSE connection for `(session, participant_uuid)`. The
    connection stays pending (and receives nothing but keepalives) until the
    matching `auth` RPC promotes it to a full participant over `POST .../send`.
    """
    backend: Backend = request.app.state.backend

    if request.headers.get("Accept") != ACCEPT_TYPE_SSE:
        raise HTTPException(status_code=406, detail="Expected text/event-stream")

    org_id = _parse_organization_id(raw_organization_id)
    realm_id = _parse_vlob_id(raw_realm_id)
    document_id = _parse_vlob_id(raw_document_id)
    client_ctx = _parse_editics_auth_header(request.headers, request)

    channel = EditicsSseChannel(
        participant_uuid=client_ctx.participant_uuid,
        keepalive=backend.config.sse_keepalive,
    )
    await backend.editics.join_sse(org_id, realm_id, document_id, client_ctx, channel)

    async def stream():
        try:
            async for chunk in iter_editics_sse_events(channel):
                yield chunk
        finally:
            # SSE disconnect (todo step_0 §8): remove the participant and
            # broadcast the updated participant set to the remaining ones.
            await backend.editics.leave(realm_id, document_id, client_ctx)

    return EditicsStreamingResponse(
        channel=channel,
        status_code=200,
        headers={
            "Cache-Control": "no-cache",
            "Connection": "keep-alive",
        },
        media_type=ACCEPT_TYPE_SSE,
    )


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/send"
)
async def editics_send(
    raw_organization_id: str, raw_realm_id: str, raw_document_id: str, request: Request
):
    """Carry one client→server event (todo step_0 §3.1, §3.4, §6).

    Step 0 only implements `auth`. The RPC returns the `auth` server event as
    JSON (200) on success, or an `AuthRejected`-shaped reply on rejection. When
    the client event triggers no reply to the sender, returns 204 No Content
    (not used in step 0 but defined per §3.4).
    """
    backend: Backend = request.app.state.backend

    org_id = _parse_organization_id(raw_organization_id)
    realm_id = _parse_vlob_id(raw_realm_id)
    document_id = _parse_vlob_id(raw_document_id)
    client_ctx = _parse_editics_auth_header(request.headers, request)

    body = await _read_body(request)

    # Strict server-side validation of the client event (Pydantic). The server
    # IS strictly validated on what it accepts from clients (todo step_0 §2).
    try:
        event = ClientEventAuth.model_validate_json(body)
    except ValueError:
        raise HTTPException(status_code=400, detail="Invalid client event")

    reply = await backend.editics.handle_client_event(
        org_id, realm_id, document_id, client_ctx, event
    )

    match reply:
        case ServerEventAuth() | ServerEventAuthRejected():
            return Response(content=reply.model_dump_json(), media_type="application/json")
        case None:
            return Response(status_code=204)
        case _:  # pragma: no cover
            raise HTTPException(status_code=500, detail="Unexpected editics reply")
