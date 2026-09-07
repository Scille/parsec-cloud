# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import json
from collections.abc import AsyncGenerator, Awaitable, Callable
from contextlib import asynccontextmanager
from pathlib import Path
from typing import Any
from uuid import UUID, uuid4

import httpx_sse
import py_mini_racer
import pytest
from py_mini_racer import JSPromise
from pydantic import BaseModel

from parsec._parsec import VlobID
from parsec.editics_protocol import (
    EditicsProtocolClientEvent,
    EditicsProtocolServerEvent,
    EditicsProtocolServerEventAdapter,
)
from tests.common.backend import SERVER_DOMAIN
from tests.common.client import AuthenticatedRpcClient, RpcTransportError

type EditicsSessionSendClientEvent = Callable[
    [EditicsProtocolClientEvent], Awaitable[BaseModel | None]
]


class EditicsSessionClient:
    """
    Connect to the server to join an editics session.
    """

    def __init__(
        self,
        sse_event_source: httpx_sse.EventSource,
        send_events_raw: Callable[[str], Awaitable[str | None]],
    ):
        self._sse_event_source = sse_event_source
        self._sse_event_source_iter = self._sse_event_source.aiter_sse()
        self.send_raw = send_events_raw

    @classmethod
    @asynccontextmanager
    async def join(
        cls,
        who: AuthenticatedRpcClient,
        realm_id: VlobID,
        document_id: VlobID,
        participant_id: UUID,
    ) -> AsyncGenerator[EditicsSessionClient, None]:
        """
        Open the editics SSE + RPC connection for an editics session.

        Yields `(send_client_event, sse_listen_server_events)`
        """
        auth = f"Editics {who.device_id.hex}.{participant_id.hex}"
        session_path = f"/authenticated/{who.organization_id}/editics/sessions/{realm_id.hex}/{document_id.hex}"
        join_url = f"http://{SERVER_DOMAIN}{session_path}/join"
        send_url = f"http://{SERVER_DOMAIN}{session_path}/send"

        async def send_events_raw(raw_event: str) -> str | None:
            rep = await who.raw_client.post(
                send_url,
                headers={"Authorization": auth, "Content-Type": "application/json"},
                content=raw_event,
            )
            match rep.status_code:
                case 200:
                    return rep.text
                case 204:
                    return None
                case _:
                    raise RpcTransportError(rep)

        async with httpx_sse.aconnect_sse(
            who.raw_client,
            "GET",
            join_url,
            # `EventSource` cannot set headers; the server accepts the identity
            # as an `authorization` query param on the SSE route (todo §6.2).
            params={"authorization": auth},
            headers={"Accept": "text/event-stream"},
        ) as sse_event_source:
            yield cls(
                send_events_raw=send_events_raw,
                sse_event_source=sse_event_source,
            )

    async def send(self, event: EditicsProtocolClientEvent) -> EditicsProtocolServerEvent | None:
        match await self.send_raw(event.model_dump_json()):
            case None:
                return None
            case raw_server_event:
                return EditicsProtocolServerEventAdapter.validate_json(raw_server_event)

    send_raw: Callable[[str], Awaitable[str | None]]  # Set in `__init__`
    "Take and return the events in JSON encoded format"

    async def recv(self) -> EditicsProtocolServerEvent:
        raw_event = await self.recv_raw()
        return EditicsProtocolServerEventAdapter.validate_json(raw_event)

    async def recv_raw(self) -> str:
        "Returns the event in JSON encoded format"
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
    Start a JavaScript runtime (V8 using PyMiniRacer) and load inside it the editics
    client (i.e. `client/editics/protocol.js`).

    Only a single instance of it is needed for the whole tests run: internally it
    will create independant instances of the protocol object for each test.
    """

    def __init__(self):
        protocol_js_path = Path(__file__).parent / "../../../client/editics/protocol.js"

        src = protocol_js_path.read_text()
        # The file is an ES module ending with `export { EditicsTranslator };`.
        # V8 (PyMiniRacer) does not implement ESM `export`, so rewrite it to a
        # global assignment that the bootstrap can pick up. This is the only
        # transform; the rest of the source is loaded verbatim (no build step,
        # todo §2.3).
        assert "export { EditicsTranslator };" in src, "`protocol.js` export marker changed"
        src = src.replace(
            "export { EditicsTranslator };",
            "globalThis.__EditicsTranslator = EditicsTranslator;",
        )

        self._js_runtime = py_mini_racer.MiniRacer()
        self._js_runtime.eval(src)

        self._js_runtime.eval("globalThis.__editics_instances = {};")

    @asynccontextmanager
    async def new_client(
        self,
        who: AuthenticatedRpcClient,
        realm_id: VlobID,
        document_id: VlobID,
        vlob_version: int = 1,
        editor_type: int = 0,
    ) -> AsyncGenerator[EditicsJSClient, None]:
        participant_id = uuid4()

        # TODO: mock capabilities callbacks
        self._js_runtime.eval(
            f"""
            globalThis.__editics_instances['{participant_id.hex}'] = new globalThis.__EditicsTranslator({{
                workspaceId: '{realm_id.hex}',
                vlobId: '{document_id.hex}',
                deviceIdHex: '{who.device_id.hex}',
                userId: '<dummy userId>',
                userName: '<dummy userName>',
                vlobVersion: {vlob_version},
                editorType: {editor_type},
                capabilities: {{
                    resolveUserName: (d) => console.log('resolveUserName', d),
                    resolveUserId: (d) => console.log('resolveUserId', d),
                    encrypt: (p) => console.log('encrypt', p),
                    decrypt: (c) => console.log('decrypt', c),
                }},
            }});
            """,
        )
        async with EditicsSessionClient.join(
            who=who,
            realm_id=realm_id,
            document_id=document_id,
            participant_id=participant_id,
        ) as editics_session_client:
            client = EditicsJSClient(self, who, participant_id, editics_session_client)
            try:
                yield client
            finally:
                self._js_runtime.eval(
                    f"""delete globalThis.__editics_instances['{participant_id.hex}'];"""
                )

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
            return value

        future = asyncio.run_coroutine_threadsafe(_run(), loop)
        return future.result()


class EditicsJSClient:
    """
    Combine an connection to the server (using `EditicsSessionClient`) with an
    instance of the editics client running in the JavaScript runtime.

    The should be used to test the OnlyOffice protocol going in and out of the
    OnlyOffice editor (as this validate both our server-reimplementation and
    the client-side OnlyOffice-to-editics-protocol translation layer).
    """

    def __init__(
        self,
        js_runtime: EditicsJSRuntime,
        who: AuthenticatedRpcClient,
        participant_id: UUID,
        editics_session_client: EditicsSessionClient,
    ):
        self.js_runtime = js_runtime
        self.who = who
        self.participant_id = participant_id
        self.editics_session_client = editics_session_client

    async def inject_oo_client_event(self, oo_client_event: dict) -> dict | None:
        """
        Simulate the OnlyOffice editor (running on the client) wants to send a new
        event to the server.
        """
        raw_editics_client_event: str | None = await self.js_runtime.async_eval(
            f"""
            globalThis.__editics_instances['{self.participant_id.hex}']
                .cookClientEvent({json.dumps(oo_client_event)})
                .then(obj => (obj != null ? JSON.stringify(obj) : null))
            """
        )
        if raw_editics_client_event is None:
            # Ignored OnlyOffice event (e.g. `rpc`)
            return

        maybe_raw_editics_server_event = await self.editics_session_client.send_raw(
            raw_editics_client_event
        )
        match maybe_raw_editics_server_event:
            case None:
                return None

            case raw_editics_server_event:
                raw_oo_client_event = await self.js_runtime.async_eval(
                    f"""
                    globalThis.__editics_instances['{self.participant_id.hex}']
                        .cookServerEvent({raw_editics_server_event})
                        .then(obj => (obj != null ? JSON.stringify(obj) : null))
                    """
                )
                assert raw_oo_client_event is not None, (
                    f"Server event unsupported on the client: {raw_oo_client_event}"
                )
                return json.loads(raw_oo_client_event)

    async def listen_oo_server_event(self) -> dict:
        """
        Wait for a new event arriving to the OnlyOffice editor.
        """
        raw_editics_server_event = await self.editics_session_client.recv_raw()
        # Sanity check to detect incorrect server output (since the Javascript
        # part doesn't actually validate the schema)
        editics_server_event = EditicsProtocolServerEventAdapter.validate_json(
            raw_editics_server_event
        )
        oo_server_event = await self.js_runtime.async_eval(
            f"""
            globalThis.__editics_instances['{self.participant_id.hex}'].cookServerEvent({raw_editics_server_event})
                .then(obj => (obj != null ? JSON.stringify(obj) : null))
            """
        )
        assert oo_server_event is not None, (
            f"Server event unsupported on the client: {editics_server_event}"
        )
        return json.loads(oo_server_event)
