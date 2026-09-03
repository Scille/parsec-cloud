# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Editics SSE transport.

This mirrors the existing `StreamingResponseMiddleware` in
`parsec/asgi/rpc.py` (the authenticated events SSE route) but with the editics
framing (see todo step_0 §3.2):

- Server *data* events are delivered as a single `data:` line whose JSON carries
  the `"type"` field used for dispatch. There is **no** `event:` line for data
  events (this keeps a single discriminated union on `"type"`, consistent with
  the RPC route).
- The keepalive reuses the existing Parsec SSE keepalive shape
  (`event:keepalive\\ndata:\\n\\n`), which is the only place an `event:` line is
  used on the editics SSE route.
"""

from __future__ import annotations

import json
from collections.abc import AsyncGenerator
from typing import Any
from uuid import UUID

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream
from fastapi.responses import StreamingResponse
from starlette.types import Receive

from parsec.logging import get_logger

logger = get_logger()

# Buffer size for the per-connection SSE event queue. Step 0 produces very few
# events (a single `connectState` on join), so a small buffer is plenty.
SSE_CHANNEL_BUFFER = 16


class EditicsSseChannel:
    """One participant's SSE connection.

    The channel is created when the client opens the `GET .../join` SSE stream
    but is *pending* until the matching `auth` RPC arrives: until then it is
    not a participant of the session and does not receive `connectState`
    broadcasts (todo step_0 §6).

    The server pushes server events by calling `send_nowait` (or `send`); the
    `EditicsStreamingResponse` consumes them from the receive stream and frames
    them as SSE `data:` lines.
    """

    def __init__(self, participant_uuid: UUID, keepalive: float) -> None:
        self.participant_uuid = participant_uuid
        self.keepalive = keepalive
        self._send, self._recv = anyio.create_memory_object_stream[dict[str, Any]](
            max_buffer_size=SSE_CHANNEL_BUFFER
        )
        # `pending` is True from creation until the matching `auth` RPC promotes
        # the channel to a full participant. While pending, the channel is
        # registered in `Session.pending` (not `Session.connections`).
        self.pending: bool = True
        self.closed: bool = False
        # Filled in when the channel is promoted to a full participant by the
        # `auth` RPC (used to build the `AuthServer.sessionTimeConnect` field and
        # to find the participant on leave). They are not part of the SSE
        # protocol itself.
        self.index_user: int | None = None
        self.connect_time_ms: int | None = None

    @property
    def receive(self) -> MemoryObjectReceiveStream[dict[str, Any]]:
        return self._recv

    def send_nowait(self, event: dict[str, Any]) -> None:
        """Enqueue a server event to be delivered over this SSE stream.

        Raises `anyio.WouldBlock` if the buffer is full (backpressure); the
        component closes the channel in that case (mirrors the events SSE
        backpressure handling in `parsec/asgi/rpc.py`).
        """
        if self.closed:
            return
        self._send.send_nowait(event)

    async def send(self, event: dict[str, Any]) -> None:
        if self.closed:
            return
        await self._send.send(event)

    def close(self) -> None:
        if self.closed:
            return
        self.closed = True
        self._send.close()
        self._recv.close()


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
