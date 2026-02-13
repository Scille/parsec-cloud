# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
from typing import Any

import httpx
import pytest

from parsec._parsec import authenticated_cmds
from parsec.events import EventUserRevokedOrFrozen, EventUserUnfrozen
from tests.common import (
    AdminUnauthErrorsTester,
    Backend,
    CoolorgRpcClients,
    RpcTransportError,
    alice_gives_profile,
)


async def test_bad_auth(
    coolorg: CoolorgRpcClients,
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    async def do(client: httpx.AsyncClient):
        return await client.post(url)

    await administration_route_unauth_errors_tester(do)


@pytest.mark.parametrize(
    "kind", ("invalid_json", "bad_type_user_id", "bad_value_user_id", "bad_type_frozen")
)
async def test_bad_data(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
    kind: str,
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

    response = await administration_client.post(
        url,
        **body_args,
    )
    assert response.status_code == 422, response.content


async def test_bad_method(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    response = await administration_client.patch(
        url,
        json={"user_id": coolorg.alice.user_id.hex, "frozen": True},
    )
    assert response.status_code == 405, response.content


async def test_disconnect_sse(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    async with coolorg.alice.events_listen() as alice_sse:
        _ = await alice_sse.next_event()  # Server always starts by returning a `ServerConfig` event

        response = await administration_client.post(
            url,
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
    administration_client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    # 1) Search by user ID and freeze

    for _ in range(2):  # Re-freeze is idempotent
        with backend.event_bus.spy() as spy:
            response = await administration_client.post(
                url,
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
            response = await administration_client.post(
                url,
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
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
) -> None:
    url = "http://parsec.invalid/administration/organizations/DummyOrg/users/freeze"
    response = await administration_client.post(
        url,
        json={"user_id": coolorg.alice.user_id.hex, "frozen": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}


async def test_unknown_user_id(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"
    response = await administration_client.post(
        url,
        json={"user_id": "d51589e233c0451e9d2fa1c7b9a8b08b", "frozen": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User not found"}


async def test_unknown_email(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"
    response = await administration_client.post(
        url,
        json={"user_email": "dummy@example.invalid", "frozen": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User not found"}


async def test_revoked_user(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    await alice_gives_profile(coolorg, backend, recipient=coolorg.bob.user_id, new_profile=None)
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/freeze"

    # Access by user ID
    response = await administration_client.post(
        url,
        json={"user_id": coolorg.bob.user_id.hex, "frozen": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User has been revoked"}

    # Access by email
    response = await administration_client.post(
        url,
        json={"user_email": coolorg.bob.human_handle.email.str, "frozen": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User not found"}
