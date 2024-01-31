# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx
import pytest

from parsec._parsec import ActiveUsersLimit
from parsec.components.organization import OrganizationDump
from tests.common import Backend, CoolorgRpcClients


async def test_get_organization_auth(client: httpx.AsyncClient, coolorg: CoolorgRpcClients) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    # No Authorization header
    response = await client.patch(url, json={"is_expired": True})
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.patch(
        url,
        headers={
            "Authorization": "DUMMY",
        },
        json={"is_expired": True},
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.patch(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
        json={"is_expired": True},
    )
    assert response.status_code == 403, response.content


@pytest.mark.parametrize("kind", ("bad_json", "bad_data"))
async def test_bad_data(
    client: httpx.AsyncClient, backend: Backend, coolorg: CoolorgRpcClients, kind: str
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    body_args: dict
    match kind:
        case "bad_json":
            body_args = {"content": b"<dummy>"}
        case "bad_data":
            body_args = {"json": {"is_expired": "dummy"}}
        case unknown:
            assert False, unknown

    response = await client.patch(
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
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={},
    )
    assert response.status_code == 405, response.content


@pytest.mark.parametrize(
    "params",
    (
        {},
        {"is_expired": True},
        {"active_user_limit": 1},
        {"user_profile_outsider_allowed": False},
        {"is_expired": False, "active_user_limit": None, "user_profile_outsider_allowed": True},
    ),
)
async def test_ok(
    params: dict,
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json=params,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {}

    dump = await backend.organization.test_dump_organizations()
    assert dump == {
        coolorg.organization_id: OrganizationDump(
            organization_id=coolorg.organization_id,
            is_bootstrapped=True,
            is_expired=params.get("is_expired", False),
            active_users_limit=ActiveUsersLimit.from_maybe_int(
                params.get("active_users_limit", None)
            ),
            user_profile_outsider_allowed=params.get("user_profile_outsider_allowed", True),
        )
    }


async def test_404(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"is_expired": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}
