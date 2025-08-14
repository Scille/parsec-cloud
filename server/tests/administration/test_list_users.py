# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from parsec._parsec import RevokedUserCertificate
from parsec.components.user import UserInfo
from tests.common import AdminUnauthErrorsTester, Backend, CoolorgRpcClients, alice_gives_profile


async def test_bad_auth(
    client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"

    async def do(client: httpx.AsyncClient):
        return await client.get(url)

    await administration_route_unauth_errors_tester(do)


async def test_bad_method(
    administration_client: httpx.AsyncClient, coolorg: CoolorgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"
    response = await administration_client.post(
        url,
    )
    assert response.status_code == 405, response.content


async def test_ok(
    administration_client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"
    response = await administration_client.get(
        url,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "users": [
            {
                "user_id": "a11cec00100000000000000000000000",
                "user_email": "alice@example.com",
                "user_name": "Alicey McAliceFace",
                "frozen": False,
            },
            {
                "user_id": "808c0010000000000000000000000000",
                "user_email": "bob@example.com",
                "user_name": "Boby McBobFace",
                "frozen": False,
            },
            {
                "user_id": "3a11031c001000000000000000000000",
                "user_email": "mallory@example.com",
                "user_name": "Malloryy McMalloryFace",
                "frozen": False,
            },
        ],
    }

    # Revoked user are not listed
    outcome = await alice_gives_profile(coolorg, backend, coolorg.mallory.user_id, None)
    assert isinstance(outcome, RevokedUserCertificate)

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.alice.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"
    response = await administration_client.get(
        url,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "users": [
            {
                "user_id": "a11cec00100000000000000000000000",
                "user_email": "alice@example.com",
                "user_name": "Alicey McAliceFace",
                "frozen": True,
            },
            {
                "user_id": "808c0010000000000000000000000000",
                "user_email": "bob@example.com",
                "user_name": "Boby McBobFace",
                "frozen": False,
            },
        ],
    }


async def test_unknown_organization(
    administration_client: httpx.AsyncClient,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy/users"
    response = await administration_client.get(
        url,
    )
    assert response.status_code == 404, response.content
