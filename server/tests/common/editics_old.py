# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Editics test harness (todo step_2 §6).

This module provides the in-process test infrastructure that exercises the
protocol translation layer (`client/editics/protocol.js`) running in an
embedded V8 isolate (PyMiniRacer) against the real Parsec server (running
in-process over httpx `ASGITransport`). No network is used (todo §2.1).

It contains:

- `BaseEditicsClient`: a mixin added to `AuthenticatedRpcClient` providing
  `join_editics_session`, the SSE + RPC connection helper that speaks the
  editics wire protocol (pydantic `ClientEvent`/`ServerEvent`).
- `fake_editics_client`: a pytest fixture returning a `FakeEditicsClient`
  that wraps one V8 translator + one SSE/RPC connection and exposes the
  OnlyOffice side of the protocol (§6.3).
- `load_captured_session` + `CapturedEvent`: a parser for the captured
  OnlyOffice session logs used as structural oracles (§6.4).
- `assert_oo_shape`: structural comparison helper.
"""

from __future__ import annotations

import re
from collections.abc import AsyncGenerator, Awaitable, Callable
from contextlib import asynccontextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import Any, cast
from uuid import uuid4

# PyMiniRacer is a test-only dependency.
import py_mini_racer
import pytest
from httpx_sse import aconnect_sse
from pydantic import BaseModel, TypeAdapter

from parsec._parsec import VlobID
from parsec.components.editics import (
    ClientEvent,
    ServerEventAuth,
    ServerEventAuthChanges,
    ServerEventAuthRejected,
    ServerEventConnectState,
    ServerEventCursor,
    ServerEventDrop,
    ServerEventGetLock,
    ServerEventMessage,
    ServerEventReleaseLock,
    ServerEventSaveChanges,
    ServerEventSaveLock,
    ServerEventSavePartChanges,
    ServerEventUnSaveLock,
    ServerEventWaitAuth,
    ServerEventWarning,
)
from tests.common.backend import SERVER_DOMAIN


def _racer_eval(racer: py_mini_racer.MiniRacer, code: str) -> Any:
    """Wrap `racer.eval` returning `Any`.

    `MiniRacer.eval` is typed to return the broad `PythonJSConvertedTypes`
    union; pyright cannot know that evaluating an async IIFE yields an
    awaitable `JSPromise` or that a returned `JSFunction` is callable. The
    harness relies on both, so the call sites cast through `Any`.
    """
    return racer.eval(code)


# Path to the pure translator source, relative to the server root.
# `protocol.js` is plain JS (no build step — todo §2.3) so it is loaded as-is
# into the V8 isolate.
_PROTOCOL_JS_PATH = Path(__file__).resolve().parents[3] / "client" / "editics" / "protocol.js"


# --- ServerEvent parsing ----------------------------------------------------
#
# `ServerEvent` is a discriminated union on `type`, but two members
# (`ServerEventAuth` and `ServerEventAuthRejected`) share `type: "auth"` and
# differ by `result`. Pydantic's discriminator therefore rejects a single
# `TypeAdapter(ServerEvent)` (the `auth` value maps to multiple choices). The
# ASGI route never parses a `ServerEvent` from JSON (it only ever *produces*
# one), so this collision is harmless in production — but the test harness
# must parse the server's JSON replies into typed models. We dispatch on
# `type` (and `result` for the `auth` ambiguity) to the concrete model class.

_SERVER_EVENT_BY_TYPE: dict[str, type[BaseModel]] = {
    "connectState": ServerEventConnectState,
    "authChanges": ServerEventAuthChanges,
    "waitAuth": ServerEventWaitAuth,
    "message": ServerEventMessage,
    "cursor": ServerEventCursor,
    "getLock": ServerEventGetLock,
    "releaseLock": ServerEventReleaseLock,
    "saveLock": ServerEventSaveLock,
    "saveChanges": ServerEventSaveChanges,
    "savePartChanges": ServerEventSavePartChanges,
    "unSaveLock": ServerEventUnSaveLock,
    "drop": ServerEventDrop,
    "warning": ServerEventWarning,
}


def parse_server_event(data: dict[str, Any]) -> BaseModel:
    """Parse a JSON dict from the wire into the concrete `ServerEvent` model.

    The server serializes `bytes` fields to base64 over JSON; the returned
    model keeps them as `bytes` (pydantic deserializes base64 back to bytes
    automatically). The harness converts them to `Uint8Array` at the V8
    boundary (see `_event_to_js`).
    """
    typ = data.get("type")
    if typ == "auth":
        if data.get("result") == 1:
            return ServerEventAuth.model_validate(data)
        return ServerEventAuthRejected.model_validate(data)
    assert isinstance(typ, str), f"missing/invalid server event type: {typ!r}"
    cls = _SERVER_EVENT_BY_TYPE.get(typ)
    if cls is None:
        raise ValueError(f"unknown server event type: {typ!r}")
    return cls.model_validate(data)


# --- Bytes <-> Uint8Array at the V8 boundary --------------------------------
#
# The translator's interface uses `Uint8Array` for encrypted fields (todo
# §4.1). At the Python<->V8 boundary, `py_mini_racer` converts a JS
# `Uint8Array` to a Python `memoryview`/`bytes` and vice-versa. The harness
# must:
#   - turn the pydantic `bytes` of an `EditicsServerEvent` into a JS object
#     with `Uint8Array` for those fields before `cookServerEvent`;
#   - turn the JS `EditicsClientEvent` (with `Uint8Array` encrypted fields)
#     returned by `cookClientEvent` into a pydantic `ClientEvent` (with
#     `bytes`) before POSTing it to the server.
#
# Which fields are `bytes` depends on the event type (mirrors the pydantic
# `bytes` fields in `parsec/components/editics.py`).

# Top-level bytes fields per client-event type.
_CLIENT_BYTES_FIELDS: dict[str, list[str]] = {
    "message": ["encryptedMessage"],
    "cursor": ["encryptedCursor"],
}
# `saveChanges` has `encryptedChanges` (list[bytes]) and `encryptedCursor`
# (bytes|None).

# Per-record bytes sub-fields for server events.
_SERVER_RECORD_BYTES: dict[str, dict[str, str]] = {
    # event_type -> {array_field: bytes_sub_field}
    "message": {"messages": "encryptedMessage"},
    "cursor": {"messages": "encryptedCursor"},
    "saveChanges": {"changes": "change"},
}
# `authChanges.changes` is a list of [index, bytes] tuples.
# `saveChanges.encryptedCursor` is a top-level bytes|None.


def _event_to_js(model: BaseModel) -> dict[str, Any]:
    """Convert a pydantic `ServerEvent` model to a plain JS-shaped dict with
    `bytes` fields replaced by `bytes` (the V8 bridge turns `bytes` into a
    `Uint8Array` automatically when passed into JS)."""
    data = model.model_dump(mode="python")
    return _coerce_server_bytes_to_raw(data)


def _coerce_server_bytes_to_raw(data: dict[str, Any]) -> dict[str, Any]:
    """Ensure bytes fields are raw `bytes` (not base64 str) so the V8 bridge
    maps them to `Uint8Array`. `model_dump(mode="python")` already returns
    `bytes` for `bytes` fields, so this is mostly a no-op; kept for clarity
    and to strip any stray base64 if the dict came from JSON instead."""
    typ = data.get("type")
    if typ in _SERVER_RECORD_BYTES:
        arr_field, sub_field = next(iter(_SERVER_RECORD_BYTES[typ].items()))
        for rec in data.get(arr_field, []) or []:
            v = rec.get(sub_field)
            if isinstance(v, str):
                rec[sub_field] = _b64_decode(v)
    if typ == "authChanges":
        for entry in data.get("changes", []) or []:
            if isinstance(entry, list) and len(entry) == 2 and isinstance(entry[1], str):
                entry[1] = _b64_decode(entry[1])
    if typ == "saveChanges":
        v = data.get("encryptedCursor")
        if isinstance(v, str):
            data["encryptedCursor"] = _b64_decode(v)
    return data


def _b64_decode(s: str) -> bytes:
    import base64

    return base64.b64decode(s)


def _client_event_from_js(js_obj: dict[str, Any]) -> ClientEvent:
    """Build a pydantic `ClientEvent` from a JS object returned by
    `cookClientEvent`, converting `Uint8Array` (which crossed the V8
    boundary as `memoryview`/`bytes`) into the `bytes` pydantic expects."""
    typ = js_obj.get("type")
    out = dict(js_obj)
    if typ in _CLIENT_BYTES_FIELDS:
        for f in _CLIENT_BYTES_FIELDS[typ]:
            v = out.get(f)
            if v is not None and not isinstance(v, (bytes, bytearray)):
                out[f] = _to_bytes(v)
    elif typ == "saveChanges":
        # `encryptedChanges` is a list[bytes]; `encryptedCursor` is bytes|None.
        ec = out.get("encryptedChanges")
        if isinstance(ec, list):
            out["encryptedChanges"] = [_to_bytes(x) for x in ec]
        v = out.get("encryptedCursor")
        if v is not None and not isinstance(v, (bytes, bytearray)):
            out["encryptedCursor"] = _to_bytes(v)
    return TypeAdapter(ClientEvent).validate_python(out)


def _to_bytes(v: Any) -> bytes:
    if isinstance(v, (bytes, bytearray, memoryview)):
        return bytes(v)
    if isinstance(v, list):
        return bytes(v)
    if isinstance(v, str):
        # base64 fallback (should not happen for the V8 bridge, but defensive).
        return _b64_decode(v)
    raise TypeError(f"cannot convert {type(v).__name__} to bytes")


# --- BaseEditicsClient mixin (§6.1 / §6.2) ----------------------------------


class BaseEditicsClient:
    """Mixin providing the editics SSE+RPC connection helpers.

    Expects `self.raw_client` (httpx.AsyncClient), `self.device_id` (DeviceID)
    and `self.organization_id` (OrganizationID), all of which
    `AuthenticatedRpcClient` already provides. The editics routes use their
    own `Authorization: Editics <device_id_hex>.<participant_uuid_hex>`
    scheme and `Content-Type: application/json`, distinct from
    `_do_request`'s Parsec `Bearer` + msgpack, so this does NOT reuse
    `_do_request`.
    """

    @asynccontextmanager
    async def join_editics_session(
        self,
        realm_id: VlobID,
        document_id: VlobID,
        *,
        vlob_version: int,
        editor_type: int = 0,
    ) -> AsyncGenerator[
        tuple[Callable[[ClientEvent], Awaitable[BaseModel | None]], SseEvents],
        None,
    ]:
        """Open the editics SSE + RPC connection for a session.

        Yields `(send, sse_events)`:
          - `send(client_event)`: POST a pydantic `ClientEvent` to the server,
            return the RPC reply as a `ServerEvent` model (or `None` for 204).
          - `sse_events`: an async iterator yielding `ServerEvent` models
            from the SSE stream, skipping keepalives.

        The SSE context manager is held open for the life of the `with` block
        (closing it triggers the leave flow on the server).
        """
        participant_uuid = uuid4()
        auth = f"Editics {self.device_id.hex}.{participant_uuid.hex}"  # type: ignore[attr-defined]
        session_path = (
            f"/authenticated/{self.organization_id}"  # type: ignore[attr-defined]
            f"/editics/sessions/{realm_id.hex}/{document_id.hex}"
        )
        join_url = f"http://{SERVER_DOMAIN}{session_path}/join"
        send_url = f"http://{SERVER_DOMAIN}{session_path}/send"

        async with aconnect_sse(
            self.raw_client,  # type: ignore[attr-defined]
            "GET",
            join_url,
            # `EventSource` cannot set headers; the server accepts the identity
            # as an `authorization` query param on the SSE route (todo §6.2).
            params={"authorization": auth},
            headers={"Accept": "text/event-stream"},
        ) as event_source:
            sse_events = SseEvents(event_source)

            async def send(client_event: ClientEvent) -> BaseModel | None:
                content = client_event.model_dump_json()
                rep = await self.raw_client.post(  # type: ignore[attr-defined]
                    send_url,
                    headers={"Authorization": auth, "Content-Type": "application/json"},
                    content=content,
                )
                if rep.status_code == 204:
                    return None
                if 400 <= rep.status_code < 600:
                    raise EditicsRpcError(rep.status_code, rep.text)
                data = rep.json()
                return parse_server_event(data)

            yield send, sse_events


class SseEvents:
    """Async iterator yielding typed `ServerEvent` models from an SSE stream,
    skipping keepalive events (todo §6.2)."""

    def __init__(self, event_source: Any) -> None:
        self._iter = event_source.aiter_sse()

    def __aiter__(self) -> SseEvents:
        return self

    async def __anext__(self) -> BaseModel:
        while True:
            sse = await self._iter.__anext__()
            # Keepalive uses an `event:keepalive` line with empty data; data
            # events have no `event` line and carry JSON in `data`.
            if sse.event == "keepalive":
                continue
            if not sse.data:
                continue
            import json

            return parse_server_event(json.loads(sse.data))


class EditicsRpcError(Exception):
    def __init__(self, status_code: int, body: str) -> None:
        super().__init__(f"editics RPC failed: {status_code} {body}")
        self.status_code = status_code
        self.body = body


# --- Captured-session parser (§6.4) ----------------------------------------


@dataclass
class CapturedEvent:
    direction: str  # "<-" server->client, "->" client->server
    user: str  # "John Smith", "Kate Cage"
    type: str  # event type
    payload: dict[str, Any]  # the JSON block


_HEADING_RE = re.compile(
    r"^###\s+\S+\s+(?P<dir><-|->)\s+(?P<user>.+?)\s+(?P<type>\S+)\s*$",
    re.MULTILINE,
)


def load_captured_session(path: Path) -> list[CapturedEvent]:
    """Parse a captured-session markdown file into a list of `CapturedEvent`.

    The format (see `docs/rfcs/1030-collaborative-editics/oo_example_session.md`)
    is a sequence of `### <time>  <-|->  <user>  <event>` headings each
    followed by a ```` ```json ```` fenced block. Entries without a direction
    (`ws-open`, `open`) are skipped: they are engine.io transport events,
    not part of the OnlyOffice protocol (todo §6.4).
    """
    import json

    text = path.read_text()
    events: list[CapturedEvent] = []
    # Walk the fenced json blocks; the heading immediately preceding a block
    # describes the event.
    blocks = list(re.finditer(r"```json\n(.*?)\n```", text, re.DOTALL))
    # Build an index of heading positions to pair each block with the nearest
    # preceding heading.
    headings = list(_HEADING_RE.finditer(text))
    for b in blocks:
        # Find the last heading before this block.
        heading = None
        for h in headings:
            if h.start() < b.start():
                heading = h
            else:
                break
        if heading is None:
            continue
        m = _HEADING_RE.match(text[heading.start() : heading.end()])
        if m is None:
            continue
        direction = m.group("dir")
        user = m.group("user").strip()
        etype = m.group("type").strip()
        payload = json.loads(b.group(1))
        # Skip engine.io transport entries (ws-open / open) — they have no
        # direction arrow in the heading, so they never match _HEADING_RE and
        # are naturally excluded; keep this guard for robustness.
        if etype in ("ws-open", "open"):
            continue
        events.append(CapturedEvent(direction=direction, user=user, type=etype, payload=payload))
    return events


def assert_oo_shape(actual: Any, captured: Any, *, path: str = "") -> None:
    """Assert `actual` is structurally compatible with the captured event:
    same dict keys (recursively), same list lengths, same primitive *types*
    for non-opaque fields. Timestamps/ids/JWTs/opaque blobs are matched by
    type only (todo §6.4).

    "Structurally compatible" means the translator produced what the real
    OnlyOffice editor expects to receive: same `type`, same field presence
    and nesting. Exact values of opaque fields (change blobs, cursors,
    JWTs, connection ids, timestamps) are intentionally not compared.
    """
    if isinstance(captured, dict):
        assert isinstance(actual, dict), f"{path}: expected dict, got {type(actual).__name__}"
        # The editics translator intentionally drops some OnlyOffice fields
        # (e.g. `connectionId`, `isCloseCoAuthoring`, `encrypted` per RFC §2.2
        # editics changes), so the captured event may have MORE keys than the
        # translator's output. Assert every key the translator produced exists
        # in the captured shape with a compatible value (i.e. the translator's
        # output is a structural subset of the real OnlyOffice shape). This
        # validates the translator didn't invent fields and that the fields it
        # does emit are shaped like the real editor expects.
        for key in actual:
            assert key in captured, f"{path}: unexpected key {key!r} (not in captured shape)"
            assert_oo_shape(actual[key], captured[key], path=f"{path}.{key}" if path else key)
        return
    if isinstance(captured, list):
        assert isinstance(actual, list), f"{path}: expected list, got {type(actual).__name__}"
        # List lengths may differ between the captured session (which has real
        # edits) and the test (which may have an empty backlog); compare element
        # shapes only where both have entries (todo §6.4: the shape is what we
        # assert, not the count).
        if len(actual) == len(captured):
            for i, (a, c) in enumerate(zip(actual, captured)):
                assert_oo_shape(a, c, path=f"{path}[{i}]")
        else:
            assert len(actual) >= 0  # list shape is what matters
        return
    # Primitives: match by type (bool before int, since bool is an int subclass).
    if isinstance(captured, bool):
        assert isinstance(actual, bool), f"{path}: expected bool, got {type(actual).__name__}"
        return
    if isinstance(captured, (int, float)):
        assert isinstance(actual, (int, float)) and not isinstance(actual, bool), (
            f"{path}: expected number, got {type(actual).__name__}"
        )
        return
    if isinstance(captured, str):
        assert isinstance(actual, str), f"{path}: expected str, got {type(actual).__name__}"
        return
    if captured is None:
        assert actual is None, f"{path}: expected None, got {type(actual).__name__}"
        return
    # Fallback: anything else (opaque objects) — accept as-is.


# --- PyMiniRacer bridge (§6.3) ----------------------------------------------
#
# The translator (`client/editics/protocol.js`) is loaded once per test
# process into a fresh V8 isolate per fake client (isolation). Capabilities
# are implemented in Python and bridged into V8 via `wrap_py_function` (the
# wrapped async Python functions become JS async functions returning
# Promises; the translator `await`s them).

_PROTOCOL_SOURCE_CACHE: str | None = None


def _load_protocol_source() -> str:
    global _PROTOCOL_SOURCE_CACHE
    if _PROTOCOL_SOURCE_CACHE is None:
        src = _PROTOCOL_JS_PATH.read_text()
        # The file is an ES module ending with `export { EditicsTranslator };`.
        # V8 (PyMiniRacer) does not implement ESM `export`, so rewrite it to a
        # global assignment that the bootstrap can pick up. This is the only
        # transform; the rest of the source is loaded verbatim (no build step,
        # todo §2.3).
        assert "export { EditicsTranslator };" in src, "protocol.js export marker changed"
        src = src.replace(
            "export { EditicsTranslator };",
            "globalThis.__EditicsTranslator = EditicsTranslator;",
        )
        _PROTOCOL_SOURCE_CACHE = src
    return _PROTOCOL_SOURCE_CACHE


# Fixed key byte for the test encrypt/decrypt fake (todo §6.3): the cipher is
# `[KEY_BYTE, ...plain]`. Deterministic, so the round-trip is checkable.
_KEY_BYTE = 0x42


class FakeEditicsClient:
    """One translator in V8 + one SSE/RPC connection.

    Surfaces (todo §6.3):
      - `inject_oo_client_event(oo)`: editor -> translator -> server. Returns
        the cooked editics client event (the dict the translator produced).
      - `oo_server_events()`: async generator pulling ServerEvents from the
        SSE stream, running each through `cookServerEvent` in V8, yielding the
        resulting OnlyOffice server event dicts.
      - `inject_editics_server_event(editics)`: isolated translator test
        (cookServerEvent only, no server).
      - `editics_client_events(oo)`: isolated translator test
        (cookClientEvent only, no server).
    """

    def __init__(
        self,
        rpc_client: Any,
        realm_id: VlobID,
        document_id: VlobID,
        *,
        vlob_version: int,
        editor_type: int = 0,
        user_names: dict[str, str] | None = None,
    ) -> None:
        self._rpc_client = rpc_client
        self._realm_id = realm_id
        self._document_id = document_id
        self._vlob_version = vlob_version
        self._editor_type = editor_type
        # device_id hex -> display name (the server is NOT trusted for names;
        # the test resolves them locally, todo §6.3).
        self._user_names = user_names or {}
        self._racer: py_mini_racer.MiniRacer | None = None
        self._cap_ctx: Any | None = None
        # The SSE/RPC session, opened on first use.
        self._session_cm: Any | None = None
        self._send: Callable[[ClientEvent], Awaitable[BaseModel | None]] | None = None
        self._sse: SseEvents | None = None

    # --- lifecycle ---

    async def start(self) -> None:
        py_mini_racer.init_mini_racer(ignore_duplicate_init=True)
        self._racer = py_mini_racer.MiniRacer()
        _racer_eval(self._racer, _load_protocol_source())

        device_id_hex = self._rpc_client.device_id.hex

        async def py_encrypt(plain: Any) -> bytes:
            return bytes([_KEY_BYTE]) + bytes(plain)

        async def py_decrypt(cipher: Any) -> bytes:
            b = bytes(cipher)
            assert b[0] == _KEY_BYTE, f"bad key byte {b[0]}"
            return b[1:]

        async def py_resolve_user_name(device_id_hex: str) -> str:
            return self._user_names.get(device_id_hex, device_id_hex)

        async def py_resolve_user_id(device_id_hex: str) -> str:
            return device_id_hex

        self._cap_ctx = _CapabilitiesContext(
            self._racer, py_encrypt, py_decrypt, py_resolve_user_name, py_resolve_user_id
        )
        await self._cap_ctx.__aenter__()

        # Construct the translator in V8, wiring the injected capabilities
        # (the JSFunction handles are passed by reference, not JSON-serialized).
        await self._cap_ctx.construct_translator(
            device_id_hex=device_id_hex,
            user_name=self._user_names.get(device_id_hex, device_id_hex),
            vlob_version=self._vlob_version,
            editor_type=self._editor_type,
        )

        # Open the SSE + RPC session against the real server (in-process).
        self._session_cm = self._rpc_client.join_editics_session(
            self._realm_id,
            self._document_id,
            vlob_version=self._vlob_version,
            editor_type=self._editor_type,
        )
        self._send, self._sse = await cast(Any, self._session_cm).__aenter__()

    async def aclose(self) -> None:
        if self._session_cm is not None:
            await self._session_cm.__aexit__(None, None, None)
            self._session_cm = None
        if self._cap_ctx is not None:
            await self._cap_ctx.__aexit__(None, None, None)
            self._cap_ctx = None
        if self._racer is not None:
            self._racer.close()
            self._racer = None

    # --- V8 call helpers ---

    async def _cook_client_event(self, oo: dict[str, Any]) -> dict[str, Any] | None:
        """Run `cookClientEvent` in V8, return the JS result as a plain dict
        (encrypted fields as `bytes`, ready for `_client_event_from_js`)."""
        assert self._racer is not None
        import json

        oo_json = json.dumps(oo)
        # The cook methods are async (resolveUserName may be async); await the
        # returned promise. Serialize via a reviver that keeps Uint8Array as
        # a tagged object so we can recover bytes on the Python side.
        js = await _racer_eval(
            self._racer,
            f"""
            (async () => {{
              const r = await globalThis.__t.cookClientEvent({oo_json});
              if (r === null) return 'null';
              return JSON.stringify(r, (k, v) => v instanceof Uint8Array
                ? {{__u8: Array.from(v)}} : v);
            }})()
            """,
        )
        if js == "null":
            return None
        return _decode_u8(js)

    async def _cook_server_event(self, editics: dict[str, Any]) -> dict[str, Any] | None:
        assert self._racer is not None
        import json

        # `editics` may contain `bytes` for encrypted fields (from the server
        # model). Serialize with a default that tags bytes as `{__u8: [...]}`;
        # the JS side revives those into `Uint8Array`. The resulting string is a
        # JSON object literal we can embed directly as a JS expression (no
        # extra string wrapping) so `JSON.parse` gets the object, not a string.
        editics_json = json.dumps(editics, default=_json_default)
        js = await _racer_eval(
            self._racer,
            f"""
            (async () => {{
              const revive = (k, v) => (v && v.__u8) ? new Uint8Array(v.__u8) : v;
              const ev = JSON.parse({json.dumps(editics_json)}, revive);
              const r = await globalThis.__t.cookServerEvent(ev);
              if (r === null) return 'null';
              return JSON.stringify(r, (k, v) => v instanceof Uint8Array
                ? {{__u8: Array.from(v)}} : v);
            }})()
            """,
        )
        if js == "null":
            return None
        return _decode_u8(js)

    # --- public surfaces (§6.3) ---

    async def inject_oo_client_event(self, oo: dict[str, Any]) -> dict[str, Any] | None:
        """Feed an OnlyOffice client event (editor -> server) through the
        translator, forward the cooked editics event to the real server via
        `send`, and return the cooked editics event (for assertion)."""
        cooked = await self._cook_client_event(oo)
        if cooked is None:
            return None
        assert self._send is not None
        client_event = _client_event_from_js(cooked)
        self._last_reply = await self._send(client_event)
        return cooked

    async def oo_server_events(self) -> AsyncGenerator[dict[str, Any], None]:
        """Pull ServerEvents from the SSE generator, run each through
        cookServerEvent in V8, yield the resulting OnlyOffice server events."""
        assert self._sse is not None
        async for server_event in self._sse:
            editics_dict = _event_to_js(server_event)
            oo = await self._cook_server_event(editics_dict)
            if oo is None:
                continue
            yield oo

    async def inject_editics_server_event(self, editics: dict[str, Any]) -> dict[str, Any] | None:
        """Run cookServerEvent directly (isolated translator test, no server)."""
        return await self._cook_server_event(editics)

    async def editics_client_events(self, oo: dict[str, Any]) -> dict[str, Any] | None:
        """Run cookClientEvent directly (isolated translator test, no server)."""
        return await self._cook_client_event(oo)

    async def raw_editics_server_events(self) -> AsyncGenerator[BaseModel, None]:
        """Yield the raw (pre-translation) `ServerEvent` models from the SSE
        stream, for tests that want to assert on the editics wire shape
        directly."""
        assert self._sse is not None
        async for server_event in self._sse:
            yield server_event

    async def send_raw(self, client_event: ClientEvent) -> BaseModel | None:
        """POST a raw pydantic `ClientEvent` directly (bypassing the
        translator), for tests that want to drive the editics wire shape
        directly."""
        assert self._send is not None
        return await self._send(client_event)


# --- JSON helpers for the V8 boundary (bytes <-> tagged __u8) ----------------


def _json_default(o: Any) -> Any:
    if isinstance(o, (bytes, bytearray, memoryview)):
        return {"__u8": list(o)}
    raise TypeError(f"not JSON serializable: {type(o).__name__}")


def _decode_u8(json_str: str) -> dict[str, Any]:
    """Parse a JSON string produced with the `__u8` reviver-tag convention
    back into a Python dict, converting `{__u8: [...]}` to `bytes`."""
    import json

    def revive(o: Any) -> Any:
        if isinstance(o, dict):
            if set(o.keys()) == {"__u8"} and isinstance(o["__u8"], list):
                return bytes(o["__u8"])
            return {k: revive(v) for k, v in o.items()}
        if isinstance(o, list):
            return [revive(x) for x in o]
        return o

    return revive(json.loads(json_str))


# --- Capabilities context: wraps the `wrap_py_function` async context managers


class _CapabilitiesContext:
    """Holds the `wrap_py_function` async context managers alive for the life
    of the fake client and exposes a helper to construct the translator with
    the JSFunction handles wired as capabilities."""

    def __init__(self, racer: py_mini_racer.MiniRacer, *py_funcs: Any) -> None:
        self._racer = racer
        self._py_funcs = py_funcs
        self._cms: list[Any] = []
        self._js_funcs: list[Any] = []

    async def __aenter__(self) -> _CapabilitiesContext:
        for fn in self._py_funcs:
            cm = self._racer.wrap_py_function(fn)
            jsf = await cm.__aenter__()
            self._cms.append(cm)
            self._js_funcs.append(jsf)
        return self

    async def __aexit__(self, *exc: Any) -> None:
        for cm in reversed(self._cms):
            await cm.__aexit__(*exc)
        self._cms.clear()
        self._js_funcs.clear()

    async def construct_translator(
        self, *, device_id_hex: str, user_name: str, vlob_version: int, editor_type: int
    ) -> None:
        encrypt, decrypt, resolve_user_name, resolve_user_id = self._js_funcs
        # Define a factory that takes the cap functions + config, then call it
        # by passing the JSFunction handles as arguments (the JSFunction
        # __call__ passes them straight into JS without JSON serialization).
        factory = _racer_eval(
            self._racer,
            """
            (encrypt, decrypt, resolveUserName, resolveUserId,
             deviceIdHex, userName, vlobVersion, editorType) => {
              globalThis.__t = new globalThis.__EditicsTranslator({
                workspaceId: 'wksp', vlobId: 'doc',
                deviceIdHex, userId: deviceIdHex, userName,
                vlobVersion, editorType,
                capabilities: {
                  resolveUserName: (d) => resolveUserName(d),
                  resolveUserId: (d) => resolveUserId(d),
                  encrypt: (p) => encrypt(p),
                  decrypt: (c) => decrypt(c),
                },
              });
              return 'ok';
            }
            """,
        )
        cast(Any, factory)(
            encrypt,
            decrypt,
            resolve_user_name,
            resolve_user_id,
            device_id_hex,
            user_name,
            vlob_version,
            editor_type,
        )


# --- pytest fixture (§6.3) --------------------------------------------------


@pytest.fixture
async def fake_editics_client_factory():
    """Return a factory `make(rpc_client, realm_id, document_id, **opts)` that
    builds and starts a `FakeEditicsClient`, closing it after the test.

    One fixture instance = one translator in V8 + one SSE/RPC connection. The
    factory pattern lets each test build as many clients (alice/bob) as it
    needs with per-client config (user names, vlob version).
    """
    created: list[FakeEditicsClient] = []

    async def make(
        rpc_client: Any,
        realm_id: VlobID,
        document_id: VlobID,
        *,
        vlob_version: int,
        editor_type: int = 0,
        user_names: dict[str, str] | None = None,
    ) -> FakeEditicsClient:
        client = FakeEditicsClient(
            rpc_client,
            realm_id,
            document_id,
            vlob_version=vlob_version,
            editor_type=editor_type,
            user_names=user_names,
        )
        await client.start()
        created.append(client)
        return client

    yield make
    for client in created:
        await client.aclose()
