# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from parsec._parsec import RevokedUserCertificate
from parsec.components.user import UserInfo
from tests.common import Backend, CoolorgRpcClients, alice_gives_profile


async def test_bad_auth(client: httpx.AsyncClient, coolorg: CoolorgRpcClients) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"
    # No Authorization header
    response = await client.get(url)
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.get(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.get(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403, response.content


async def test_bad_method(
    client: httpx.AsyncClient, backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 405, response.content


async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
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
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
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
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy/users"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 404, response.content
