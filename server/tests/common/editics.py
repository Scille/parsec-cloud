# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import json
from collections.abc import AsyncGenerator, Awaitable, Callable
from contextlib import AbstractAsyncContextManager, asynccontextmanager
from pathlib import Path
from struct import pack, unpack
from typing import Any
from uuid import uuid4

import httpx_sse
import py_mini_racer
import pytest
from py_mini_racer import JSPromise
from pydantic import BaseModel

from parsec._parsec import VlobID
from parsec.components.auth import EditicsToken
from parsec.editics_protocol import (
    EDITICS_API_VERSION,
    EditicsClientEvent,
    EditicsParticipantID,
    EditicsServerEvent,
    EditicsServerEventAdapter,
    EditicsServerEventParsecInitialContentUploaded,
)
from tests.common.backend import SERVER_DOMAIN
from tests.common.client import AuthenticatedRpcClient, RpcTransportError

type EditicsSessionSendClientEvent = Callable[[EditicsClientEvent], Awaitable[BaseModel | None]]


class EditicsSessionClient:
    """
    Connect to the server to join an editics session.
    """

    def __init__(
        self,
        who: AuthenticatedRpcClient,
        realm_id: VlobID,
        document_id: VlobID,
        participant_id: EditicsParticipantID,
    ):
        self._who = who

        token = EditicsToken.generate_raw(
            device_id=who.device_id,
            timestamp=who.now_factory(),
            key=who.signing_key,
        )
        authorization = f"Bearer {token.decode()}"

        self._base_url = f"http://{SERVER_DOMAIN}/authenticated/{who.organization_id}/editics/sessions/{realm_id.hex}/{document_id.hex}"
        self._base_params = {"participant_id": participant_id}
        self._base_headers = {
            "Authorization": authorization,
            "Api-Version": str(EDITICS_API_VERSION),
        }

        self._sse_event_source_iter = None

    @asynccontextmanager
    async def listen(
        self,
    ) -> AsyncGenerator[None, None]:
        """
        Open the editics SSE + RPC connection for an editics session.

        Yields `(send_client_event, sse_listen_server_events)`
        """
        assert self._sse_event_source_iter is None

        async with httpx_sse.aconnect_sse(
            self._who.raw_client,
            "GET",
            f"{self._base_url}/listen",
            params=self._base_params,
            headers={
                **self._base_headers,
                "Accept": "text/event-stream",
            },
        ) as sse_event_source:
            self._sse_event_source_iter = sse_event_source.aiter_sse()
            try:
                yield
            finally:
                self._sse_event_source_iter = None

    async def upload_initial_content(
        self, key_index: int, encrypted_initial_content: bytes
    ) -> None:
        rep = await self._who.raw_client.post(
            f"{self._base_url}/initial_content",
            params={
                **self._base_params,
                "key_index": key_index,
            },
            headers={
                **self._base_headers,
                "Content-Type": "application/vnd.parsec.key-index-and-ciphertext",
            },
            content=(pack("!I", key_index), encrypted_initial_content),
        )
        match rep.status_code:
            case 204:
                return None
            case _:
                raise RpcTransportError(rep)

    async def fetch_initial_content(self) -> tuple[int, bytes]:
        rep = await self._who.raw_client.get(
            f"{self._base_url}/initial_content",
            params=self._base_params,
            headers={
                **self._base_headers,
                "Accept": "application/octet-stream",
            },
        )
        match rep.status_code:
            case 200:
                assert (
                    rep.headers["Content-Type"] == "application/vnd.parsec.key-index-and-ciphertext"
                )
                body = await rep.aread()
                (key_index,) = unpack("!I", body[:4])
                encrypted_initial_content = body[4:]
                return (key_index, encrypted_initial_content)
            case _:
                raise RpcTransportError(rep)

    async def send(self, event: EditicsClientEvent) -> EditicsServerEvent | None:
        match await self.send_event_raw(event.model_dump_json()):
            case None:
                return None
            case raw_server_event:
                return EditicsServerEventAdapter.validate_json(raw_server_event)

    async def send_event_raw(self, raw_event: str) -> str | None:
        "Take and return the events in JSON encoded format"

        rep = await self._who.raw_client.post(
            f"{self._base_url}/send",
            params=self._base_params,
            headers={
                **self._base_headers,
                "Content-Type": "application/json",
            },
            content=raw_event,
        )
        match rep.status_code:
            case 200:
                return rep.text
            case 204:
                return None
            case _:
                raise RpcTransportError(rep)

    async def recv(self) -> EditicsServerEvent:
        raw_event = await self.recv_raw()
        return EditicsServerEventAdapter.validate_json(raw_event)

    async def recv_raw(self) -> str:
        "Returns the event in JSON encoded format"
        assert self._sse_event_source_iter is not None
        while True:
            sse = await self._sse_event_source_iter.__anext__()
            if sse.event == "keepalive":
                continue
            if not sse.data:
                continue
            return sse.data  # Expected to be a JSON-encoded ServerEvent


@pytest.fixture(scope="session")
def editics_js_runtime() -> EditicsJSRuntime:
    return EditicsJSRuntime()


class EditicsJSRuntime:
    """
    Start a JavaScript runtime (V8 using PyMiniRacer) and load inside it the
    collaborative editics client (i.e. `client/editics/collaborative_protocol.js`).

    Only a single instance of it is needed for the whole tests run: internally it
    will create independant instances of the protocol object for each test.
    """

    def __init__(self):
        protocol_js_path = (
            Path(__file__).parent / "../../../client/editics/collaborative_protocol.js"
        )

        src = protocol_js_path.read_text()
        # V8 (PyMiniRacer) does not implement ESM `export`, so patch `collaborative_protocol.js`
        # to use a global assignment is instead.
        assert "export { EditicsTranslator };" in src, (
            "`collaborative_protocol.js` export marker changed"
        )
        src = src.replace(
            "export { EditicsTranslator };",
            "globalThis.__EditicsTranslator = EditicsTranslator;",
        )
        # The API version constant is not used by the tests, but V8 would choke
        # on its ESM `export` keyword.
        assert "export const EDITICS_API_VERSION" in src, (
            "`collaborative_protocol.js` export marker changed"
        )
        src = src.replace(
            "export const EDITICS_API_VERSION",
            "const EDITICS_API_VERSION",
        )

        self._js_runtime = py_mini_racer.MiniRacer()
        self._js_runtime.eval(src)

        self._js_runtime.eval(
            """
            globalThis.__editics_instances = {};

            // PyMiniRacer doesn't provide a way to export `console.log` & co,
            // so we roll our own by storing messages in a global array that we
            // manually retrieve and print.
            globalThis.__console_messages = [];
            globalThis.__console_fmt = (v) => typeof v === "string" ? v
                : (() => { try { return JSON.stringify(v) } catch (e) { return String(v) } })();
            globalThis.__console_push = (level) => (...args) =>
                globalThis.__console_messages.push(
                    JSON.stringify({level: level, msg: args.map(globalThis.__console_fmt).join(" ")}));
            for (const level of ["log", "info", "warn", "error", "debug", "trace"]) {
                console[level] = globalThis.__console_push(level);
            }
            globalThis.__console_retrieve_messages = () => {
                const logs = globalThis.__console_messages;
                globalThis.__console_messages = [];
                return logs;
            };

            // The V8 runtime provided by PyMiniRacer comes with none of the standard
            // Web APIs. So `collaborative_protocol.js` only use those APIs in wrapper
            // functions that we are going to mock here.

            // Original `bytesFromStr` relies on `TextEncoder.encode`
            globalThis.bytesFromStr = (s) => {
                // Manual UTF-8 <-> bytes conversion, this is probably overkill
                // since the string we encode are ASCII-only.
                const out = [];
                for (let i = 0; i < s.length; i++) {
                    let c = s.charCodeAt(i);
                    if (c < 0x80) {
                        out.push(c);
                    } else if (c < 0x800) {
                        out.push(0xc0 | (c >> 6));
                        out.push(0x80 | (c & 0x3f));
                    } else if (c >= 0xd800 && c <= 0xdbff) {
                        // Surrogate pair (U+10000..U+10FFFF).
                        const next = s.charCodeAt(++i);
                        c = 0x10000 + ((c - 0xd800) << 10) + (next - 0xdc00);
                        out.push(0xf0 | (c >> 18));
                        out.push(0x80 | ((c >> 12) & 0x3f));
                        out.push(0x80 | ((c >> 6) & 0x3f));
                        out.push(0x80 | (c & 0x3f));
                    } else {
                        out.push(0xe0 | (c >> 12));
                        out.push(0x80 | ((c >> 6) & 0x3f));
                        out.push(0x80 | (c & 0x3f));
                    }
                }
                return new Uint8Array(out);
            };
            //  Original `bytesToStr` relies on `TextDecoder.decode`
            globalThis.bytesToStr = (bytes) => {
                let s = "";
                let i = 0;
                while (i < bytes.length) {
                    const b = bytes[i++];
                    if (b < 0x80) {
                        s += String.fromCharCode(b);
                    } else if (b < 0xe0) {
                        s += String.fromCharCode(((b & 0x1f) << 6) | (bytes[i++] & 0x3f));
                    } else if (b < 0xf0) {
                        s += String.fromCharCode(((b & 0x0f) << 12) | ((bytes[i++] & 0x3f) << 6) | (bytes[i++] & 0x3f));
                    } else {
                        const cp = ((b & 0x07) << 18) | ((bytes[i++] & 0x3f) << 12) | ((bytes[i++] & 0x3f) << 6) | (bytes[i++] & 0x3f);
                        // Convert to a surrogate pair.
                        const adj = cp - 0x10000;
                        s += String.fromCharCode(0xd800 + (adj >> 10), 0xdc00 + (adj & 0x3ff));
                    }
                }
                return s;
            };
            """
        )

    @asynccontextmanager
    async def new_client(
        self,
        who: AuthenticatedRpcClient,
        realm_id: VlobID,
        document_id: VlobID,
        editor_type: int,  # 0=Word, 1=Spreadsheet, 2=Presentation
        vlob_version: int = 1,
        participant_id: EditicsParticipantID | None = None,
    ) -> AsyncGenerator[EditicsJSClient, None]:
        participant_id = participant_id or uuid4().hex

        self._js_runtime.eval(
            f"""
            globalThis.__editics_instances['{participant_id}'] = new globalThis.__EditicsTranslator(
                {{
                    workspaceId: '{realm_id.hex}',
                    vlobId: '{document_id.hex}',
                    deviceIdHex: '{who.device_id.hex}',
                    vlobVersion: {vlob_version},
                    editorType: {editor_type},
                }},
                {{
                    getHumanLabelFromDeviceId: async (deviceId) => {{
                        // Hardcoded common values used for the test
                        switch (deviceId) {{
                            case "de10a11cec0010000000000000000000":
                                return "Alice";
                            case "de10808c001000000000000000000000":
                                return "Bob";
                            case "de103a11031c00100000000000000000":
                                return "Mallory";
                            default:
                                throw new Error(`Unknown device ID (you may want to patch the test code): ${{ deviceId }}`)
                        }}
                    }},
                    // Simulate encryption
                    encrypt: async (cleartext) => {{
                        const keyIndex = 3;  // Dummy but realistic-looking value
                        const ciphertext = bytesFromStr(JSON.stringify({{"keyIndex": keyIndex, "cleartext": bytesToStr(cleartext)}}));
                        return [keyIndex, ciphertext];
                    }},
                    decrypt: async (keyIndex, ciphertext) => {{
                        let content;
                        try {{
                            content = JSON.parse(bytesToStr(ciphertext));
                        }} catch(err) {{
                            throw new Error(`Decryption failed: ${{err}}`);
                        }}
                        if (content.keyIndex != keyIndex) {{
                            throw new Error("Decryption failed: keyIndex mismatch (expected: ${{keyIndex}}, got: ${{content.keyIndex}}");
                        }}
                        return bytesFromStr(content.cleartext);
                    }},
                }},
            );
            """,
        )
        client = EditicsJSClient(
            js_runtime=self,
            who=who,
            realm_id=realm_id,
            document_id=document_id,
            participant_id=participant_id,
        )
        try:
            yield client
        finally:
            if client.connected:
                await client.server_disconnect()
            self._js_runtime.eval(f"""delete globalThis.__editics_instances['{participant_id}'];""")

    async def async_eval(self, code: str) -> Any:
        """
        MiniRacer runs its own asyncio loop in a background thread (created when the
        session-scoped fixture is initialized) and convert the Javascript promise to an
        asyncio future, so doing `await js_runtime.eval(...)` is theorically possible
        but fails with a `Future <Future pending> attached to a different loop` error.

        Hence this helper that allows awaiting the future from our own asyncio event loop.
        """
        assert self._js_runtime._ctx is not None
        loop = self._js_runtime._ctx.event_loop

        async def _run() -> Any:
            value = await self._js_runtime.eval_cancelable(code)
            if isinstance(value, JSPromise):
                value = await value
            # It is important to get the console messages after each run, otherwise
            # we may end up with messages from a previous test since we reuse the
            # javascript runtime between tests!
            logs = self._js_runtime.call("__console_retrieve_messages")
            assert isinstance(logs, list)
            for log in logs:
                print(f"js> {log}")
            return value

        future = asyncio.run_coroutine_threadsafe(_run(), loop)
        return future.result()


class EditicsJSClient:
    """
    Combine an (optional) connection to the server (using `EditicsSessionClient`)
    with an instance of the collaborative editics client running in the JavaScript
    runtime.

    The client starts disconnected: `server_connect()` opens the transport-level
    connection (i.e. the SSE + send routes, which in OnlyOffice corresponds to
    the websocket connection) and `server_disconnect()` closes it.

    This should be used to test the OnlyOffice protocol going in and out of the
    OnlyOffice editor (as this validates both our server-reimplementation and
    the client-side OnlyOffice-to-editics-protocol translation layer).
    """

    def __init__(
        self,
        js_runtime: EditicsJSRuntime,
        who: AuthenticatedRpcClient,
        realm_id: VlobID,
        document_id: VlobID,
        participant_id: EditicsParticipantID,
    ):
        self.js_runtime = js_runtime
        self.who = who
        self.realm_id = realm_id
        self.document_id = document_id
        self.participant_id = participant_id
        self.editics_session_client: EditicsSessionClient | None = None
        self._listen_cm: AbstractAsyncContextManager[None] | None = None

    @property
    def connected(self) -> bool:
        return self.editics_session_client is not None

    async def server_connect(self) -> None:
        assert not self.connected

        editics_session_client = EditicsSessionClient(
            who=self.who,
            realm_id=self.realm_id,
            document_id=self.document_id,
            participant_id=self.participant_id,
        )
        listen_cm = editics_session_client.listen()
        await listen_cm.__aenter__()
        self._listen_cm = listen_cm
        self.editics_session_client = editics_session_client

    async def server_disconnect(self) -> None:
        assert self.connected
        assert self._listen_cm is not None

        await self._listen_cm.__aexit__(None, None, None)
        self._listen_cm = None
        self.editics_session_client = None

    async def inject_oo_client_event(
        self, oo_client_event: dict[str, Any]
    ) -> dict[str, Any] | None:
        """
        Simulate the OnlyOffice editor (running on the client) wants to send a new
        event to the server.
        """
        assert self.connected
        assert self.editics_session_client is not None

        raw_editics_client_event: str | None = await self.js_runtime.async_eval(
            f"""
            globalThis.__editics_instances['{self.participant_id}']
                .cookClientEvent({json.dumps(oo_client_event)})
                .then(obj => (obj != null ? JSON.stringify(obj) : null))
            """
        )
        if raw_editics_client_event is None:
            # Ignored OnlyOffice event (e.g. `rpc`)
            return

        maybe_raw_editics_server_event = await self.editics_session_client.send_event_raw(
            raw_editics_client_event
        )
        match maybe_raw_editics_server_event:
            case None:
                return None

            case raw_editics_server_event:
                # Sanity check to detect incorrect server output (since the Javascript
                # part doesn't actually validate the schema)
                EditicsServerEventAdapter.validate_json(raw_editics_server_event)
                raw_oo_client_event = await self.js_runtime.async_eval(
                    f"""
                    globalThis.__editics_instances['{self.participant_id}']
                        .cookServerEvent({raw_editics_server_event})
                        .then(obj => (obj != null ? JSON.stringify(obj) : null))
                    """
                )
                assert raw_oo_client_event is not None, (
                    f"Editics server event failed to be translated: {raw_editics_server_event}"
                )
                return json.loads(raw_oo_client_event)

    async def listen_oo_server_event(self) -> dict[str, Any]:
        """
        Wait for a new event arriving to the OnlyOffice editor.
        """
        assert self.connected
        assert self.editics_session_client is not None

        raw_editics_server_event = await self.editics_session_client.recv_raw()
        # Sanity check to detect incorrect server output (since the Javascript
        # part doesn't actually validate the schema)
        event = EditicsServerEventAdapter.validate_json(raw_editics_server_event)

        # `parsecInitialContentUploaded` event is not supposed to get translated:
        # instead it should trigger a full reload of the OnlyOffice editor!
        # So just ignore it and look for the next received event.
        if isinstance(event, EditicsServerEventParsecInitialContentUploaded):
            return await self.listen_oo_server_event()

        oo_server_event = await self.js_runtime.async_eval(
            f"""
            globalThis.__editics_instances['{self.participant_id}'].cookServerEvent({raw_editics_server_event})
                .then(obj => (obj != null ? JSON.stringify(obj) : null))
            """
        )
        assert oo_server_event is not None, (
            f"Editics server event failed to be translated: {raw_editics_server_event}"
        )
        return json.loads(oo_server_event)
