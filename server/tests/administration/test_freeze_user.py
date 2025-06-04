# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
from typing import Any

import httpx
import pytest

from parsec._parsec import authenticated_cmds
from parsec.events import EventUserRevokedOrFrozen, EventUserUnfrozen
from tests.common import Backend, CoolorgRpcClients, RpcTransportError


async def test_bad_auth(
    client: httpx.AsyncClient, backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"
    # No Authorization header
    response = await client.post(url)
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.post(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.post(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403, response.content


@pytest.mark.parametrize(
    "kind", ("invalid_json", "bad_type_user_id", "bad_value_user_id", "bad_type_frozen")
)
async def test_bad_data(
    client: httpx.AsyncClient, backend: Backend, coolorg: CoolorgRpcClients, kind: str
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    body_args: dict[str, Any]
    match kind:
        case "invalid_json":
            body_args = {"content": b"<dummy>"}
        case "bad_type_user_id":
            body_args = {"json": {"frozen": False, "user_id": 42}}
        case "bad_value_user_id":
            body_args = {"json": {"frozen": False, "user_id": "<>"}}
        case "bad_type_user_email":
            body_args = {"json": {"frozen": False, "user_email": 42}}
        case "bad_value_user_email":
            body_args = {"json": {"frozen": False, "user_email": "<>"}}
        case "bad_type_frozen":
            body_args = {"json": {"user_email": "zack@example.com", "frozen": 42}}
        case unknown:
            assert False, unknown

    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        **body_args,
    )
    assert response.status_code == 422, response.content


async def test_bad_method(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"user_id": coolorg.alice.user_id.hex, "frozen": True},
    )
    assert response.status_code == 405, response.content


async def test_disconnect_sse(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    async with coolorg.alice.events_listen() as alice_sse:
        _ = await alice_sse.next_event()  # Server always starts by returning a `ServerConfig` event

        response = await client.post(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
            json={"user_id": coolorg.alice.user_id.hex, "frozen": True},
        )
        assert response.status_code == 200, response.content

        # Now the server should disconnect us...

        with pytest.raises(StopAsyncIteration):
            # Loop given the server might have send us some events before the freeze
            while True:
                async with asyncio.timeout(1):
                    await alice_sse.next_event()

        # ...and we cannot reconnect !

    async with coolorg.alice.raw_sse_connection() as rep:
        assert rep.status_code == 462


async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    # 1) Search by user ID and freeze

    for _ in range(2):  # Re-freeze is idempotent
        with backend.event_bus.spy() as spy:
            response = await client.post(
                url,
                headers={"Authorization": f"Bearer {backend.config.administration_token}"},
                json={"user_id": coolorg.alice.user_id.hex, "frozen": True},
            )
            assert response.status_code == 200, response.content
            assert response.json() == {
                "user_id": "a11cec00100000000000000000000000",
                "user_email": "alice@example.com",
                "user_name": "Alicey McAliceFace",
                "frozen": True,
            }

            await spy.wait_event_occurred(
                EventUserRevokedOrFrozen(
                    organization_id=coolorg.organization_id,
                    user_id=coolorg.alice.user_id,
                )
            )

    # 2) Ensure the user is frozen

    with pytest.raises(RpcTransportError) as exc:
        await coolorg.alice.ping(ping="hello")
    assert exc.value.rep.status_code == 462

    # 3) Search by email and un-freeze

    for _ in range(2):
        with backend.event_bus.spy() as spy:
            response = await client.post(
                url,
                headers={"Authorization": f"Bearer {backend.config.administration_token}"},
                json={"user_email": str(coolorg.alice.human_handle.email), "frozen": False},
            )
            assert response.status_code == 200, response.content
            assert response.json() == {
                "user_id": "a11cec00100000000000000000000000",
                "user_email": "alice@example.com",
                "user_name": "Alicey McAliceFace",
                "frozen": False,
            }

            await spy.wait_event_occurred(
                EventUserUnfrozen(
                    organization_id=coolorg.organization_id,
                    user_id=coolorg.alice.user_id,
                )
            )

    # 4) Finally ensure the user is no longer frozen

    rep = await coolorg.alice.ping(ping="hello")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="hello")


async def test_unknown_organization(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = "http://parsec.invalid/administration/organizations/DummyOrg/users/freeze"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"user_id": coolorg.alice.user_id.hex, "frozen": True},
    )
    assert response.status_code == 404, response.content


async def test_unknown_user_id(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"user_id": "d51589e233c0451e9d2fa1c7b9a8b08b", "frozen": True},
    )
    assert response.status_code == 404, response.content


async def test_unknown_email(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"user_email": "dummy@example.invalid", "frozen": True},
    )
    assert response.status_code == 404, response.content
