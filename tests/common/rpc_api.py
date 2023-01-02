# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from base64 import b64decode, b64encode
from contextlib import asynccontextmanager
from dataclasses import dataclass
from typing import Any, AsyncContextManager, Callable, Dict, Optional

import msgpack
import pytest
import trio
from quart import Quart
from quart.typing import TestClientProtocol, TestHTTPConnectionProtocol
from werkzeug.datastructures import Headers

from parsec._parsec import DateTime, EventsListenRep, OrganizationID
from parsec.api.protocol import authenticated_ping_serializer, invited_ping_serializer
from parsec.api.version import API_VERSION
from parsec.core.types import LocalDevice
from tests.common import OrganizationFullData


class BaseRpcApiClient:
    async def send(
        self,
        req,
        serializer,
        extra_headers: Dict[str, str] = {},
        before_send_hook: Optional[Callable] = None,
        now: Optional[DateTime] = None,
        check_rep: bool = True,
    ):
        raise NotImplementedError


@dataclass
class SSEEventSink:
    connection: TestHTTPConnectionProtocol
    _status_code: int | None = None

    @property
    def status_code(self) -> int:
        # Status code only available after the first event has been received
        assert self.connection.status_code is not None
        return self.connection.status_code

    async def get_next_event(self, raw: bool = False) -> bytes:
        data = await self.connection.receive()
        if self._status_code is None:
            self._status_code = self.connection.status_code
        assert data.startswith(b"data:")
        assert data.endswith(b"\n\n")
        raw_event = b64decode(data[len("data:") : -2])
        # According to SSE spec, `data:test` and `data: test` are identical
        if raw_event.startswith(b" "):
            raw_event = raw_event[1:]
        if raw:
            return raw_event
        else:
            return EventsListenRep.load(raw_event)


class AuthenticatedRpcApiClient(BaseRpcApiClient):
    API_VERSION = API_VERSION

    def __init__(self, client: TestClientProtocol, device: LocalDevice):
        self.client = client
        self.device = device

    @property
    def base_headers(self):
        return Headers(
            {
                "Content-Type": "application/msgpack",
                "Api-Version": str(self.API_VERSION),
                "Authorization": "PARSEC-SIGN-ED25519",
                "Author": b64encode(self.device.device_id.str.encode("utf8")),
            }
        )

    async def send_ping(self, ping: str = "foo", **kwargs):
        return await self.send(
            {"cmd": "ping", "ping": ping}, authenticated_ping_serializer, **kwargs
        )

    @asynccontextmanager
    async def connect_sse_events(
        self,
        before_send_hook: Optional[Callable] = None,
    ) -> AsyncContextManager[SSEEventSink]:
        headers = self.base_headers.copy()
        signature = self.device.signing_key.sign_only_signature(b"")
        headers["Signature"] = b64encode(signature).decode("ascii")
        headers["Accept"] = "text/event-stream"
        args = {
            "path": f"/authenticated/{self.device.organization_id.str}/events",
            "headers": headers,
        }
        # Last chance to customize the request !
        if before_send_hook:
            # Passing as dict allow the hook to event modify the path param
            before_send_hook(args)

        connection = self.client.request(**args)
        # TODO: `connection` async context manager is broken in `quart_trio`
        async with trio.open_nursery() as nursery:
            nursery.start_soon(
                connection.app, connection.scope, connection._asgi_receive, connection._asgi_send
            )

            await connection.send_complete()
            first_data = await connection.receive()
            if connection.status_code == 200:
                assert first_data == b":keepalive\n\n"

            yield SSEEventSink(connection)

            nursery.cancel_scope.cancel()

    async def send(
        self,
        req: Dict[str, Any],
        serializer: Optional[Any] = None,
        extra_headers: Dict[str, str] = {},
        before_send_hook: Optional[Callable] = None,
        now: Optional[DateTime] = None,
        check_rep: bool = True,
    ):
        now = now or DateTime.now()
        if not serializer:
            body = msgpack.packb(req)
        else:
            body = serializer.req_dumps(req)
        headers = self.base_headers.copy()
        signature = self.device.signing_key.sign_only_signature(body)
        headers["Signature"] = b64encode(signature).decode("ascii")

        # Customize headers
        for k, v in extra_headers.items():
            if v is None:
                headers.pop(k, None)
            else:
                headers[k] = v

        args = {
            "path": f"/authenticated/{self.device.organization_id.str}",
            "headers": headers,
            "data": body,
        }
        # Last chance to customize the request !
        if before_send_hook:
            # Passing as dict allow the hook to event modify the path param
            before_send_hook(args)
        rep = await self.client.post(**args)

        if check_rep:
            assert rep.status_code == 200
            rep_body = await rep.get_data()
            return serializer.rep_loads(rep_body)

        else:
            return rep


class AnonymousRpcApiClient(BaseRpcApiClient):
    API_VERSION = API_VERSION

    def __init__(self, organization_id: OrganizationID, client: TestClientProtocol):
        self.organization_id = organization_id
        self.client = client

    @property
    def base_headers(self):
        return Headers(
            {
                "Content-Type": "application/msgpack",
                "Api-Version": str(self.API_VERSION),
            }
        )

    async def send_ping(self, ping: str = "foo", **kwargs):
        return await self.send({"cmd": "ping", "ping": ping}, invited_ping_serializer, **kwargs)

    async def send(
        self,
        req,
        serializer,
        extra_headers: Dict[str, str] = {},
        before_send_hook: Optional[Callable] = None,
        now: Optional[DateTime] = None,
        check_rep: bool = True,
    ):
        now = now or DateTime.now()
        body = serializer.req_dumps(req)
        headers = self.base_headers.copy()

        # Customize headers
        for k, v in extra_headers.items():
            if v is None:
                headers.pop(k, None)
            else:
                headers[k] = v

        args = {"path": f"/anonymous/{self.organization_id.str}", "headers": headers, "data": body}
        # Last chance to customize the request !
        if before_send_hook:
            # Passing as dict allow the hook to event modify the path param
            before_send_hook(args)
        rep = await self.client.post(**args)

        if check_rep:
            assert rep.status_code == 200
            rep_body = await rep.get_data()
            return serializer.rep_loads(rep_body)

        else:
            return rep


@pytest.fixture
def alice_rpc(alice: LocalDevice, backend_asgi_app: Quart) -> AuthenticatedRpcApiClient:
    test_client = backend_asgi_app.test_client()
    return AuthenticatedRpcApiClient(test_client, alice)


@pytest.fixture
def alice2_rpc(alice2: LocalDevice, backend_asgi_app: Quart) -> AuthenticatedRpcApiClient:
    test_client = backend_asgi_app.test_client()
    return AuthenticatedRpcApiClient(test_client, alice2)


@pytest.fixture
def bob_rpc(bob: LocalDevice, backend_asgi_app: Quart) -> AuthenticatedRpcApiClient:
    test_client = backend_asgi_app.test_client()
    return AuthenticatedRpcApiClient(test_client, bob)


@pytest.fixture
def anonymous_rpc(
    coolorg: OrganizationFullData, backend_asgi_app: Quart
) -> AuthenticatedRpcApiClient:
    test_client = backend_asgi_app.test_client()
    return AnonymousRpcApiClient(coolorg.organization_id, test_client)
