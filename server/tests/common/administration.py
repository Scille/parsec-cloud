# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import AsyncGenerator, Callable, Coroutine, Generator

import pytest
from httpx import AsyncClient, Auth, Request, Response

from parsec.backend import Backend


class AdministrationTokenAuth(Auth):
    def __init__(self, token: str) -> None:
        self.set_token(token)

    def set_token(self, token: str) -> None:
        self.authorization_header = f"Bearer {token}"

    def auth_flow(self, request: Request) -> Generator[Request, Response, None]:
        request.headers["Authorization"] = self.authorization_header
        yield request


@pytest.fixture
def administration_token_auth(backend: Backend) -> AdministrationTokenAuth:
    return AdministrationTokenAuth(backend.config.administration_token)


@pytest.fixture
def administration_client(
    client: AsyncClient, administration_token_auth: AdministrationTokenAuth
) -> AsyncClient:
    client.auth = administration_token_auth
    return client


type AdminUnauthErrorsTesterDoCallback = Callable[[AsyncClient], Coroutine[None, None, Response]]
type AdminUnauthErrorsTester = Callable[
    [AdminUnauthErrorsTesterDoCallback], Coroutine[None, None, None]
]


@pytest.fixture(params=("no_authentication", "invalid_authorization_header", "bad_bearer_token"))
async def administration_route_unauth_errors_tester(
    request: pytest.FixtureRequest, client: AsyncClient
) -> AsyncGenerator[AdminUnauthErrorsTester, None]:
    tester_called = False

    async def _administration_route_unauth_error_tester(do: AdminUnauthErrorsTesterDoCallback):
        nonlocal tester_called
        tester_called = True

        match request.param:
            case "no_authentication":
                pass
            case "invalid_authorization_header":
                client.headers["Authorization"] = "DUMMY"
            case "bad_bearer_token":
                client.auth = AdministrationTokenAuth("BADTOKEN")
            case param:
                assert False, param

        response = await do(client)
        assert response.status_code == 403, response.content

    yield _administration_route_unauth_error_tester
    assert tester_called
