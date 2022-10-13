# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from base64 import b64encode
import pytest
from typing import Dict, Optional, Callable
from quart.typing import TestClientProtocol
from werkzeug.datastructures import Headers

from parsec._parsec import DateTime, OrganizationID
from parsec.api.version import API_VERSION
from parsec.core.types import LocalDevice
from parsec.api.protocol import authenticated_ping_serializer, invited_ping_serializer

from tests.common import RunningBackend, OrganizationFullData


class AuthenticatedRpcApiClient:
    def __init__(self, client: TestClientProtocol, device: LocalDevice):
        self.client = client
        self.device = device
        self.base_headers = Headers(
            {
                "Content-Type": "application/msgpack",
                "Api-Version": str(API_VERSION),
                "Authorization": "PARSEC-SIGN-ED25519",
                "Author": b64encode(device.device_id.str.encode("utf8")),
            }
        )

    async def send_ping(self, ping: str = "foo", **kwargs):
        return await self.send(
            {"cmd": "ping", "ping": ping}, authenticated_ping_serializer, **kwargs
        )

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


class AnonymousRpcApiClient:
    def __init__(self, organization_id: OrganizationID, client: TestClientProtocol):
        self.organization_id = organization_id
        self.client = client
        self.base_headers = Headers(
            {
                "Content-Type": "application/msgpack",
                "Api-Version": str(API_VERSION),
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
def alice_rpc(alice: LocalDevice, running_backend: RunningBackend) -> AuthenticatedRpcApiClient:
    test_client = running_backend.test_client()
    return AuthenticatedRpcApiClient(test_client, alice)


@pytest.fixture
def anonymous_rpc(
    coolorg: OrganizationFullData, running_backend: RunningBackend
) -> AuthenticatedRpcApiClient:
    test_client = running_backend.test_client()
    return AnonymousRpcApiClient(coolorg.organization_id, test_client)
