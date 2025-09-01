# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
# cspell:words ImReall getfixturevalue

from collections.abc import AsyncGenerator, Callable, Coroutine, Generator

import httpx
import pytest
from httpx import AsyncClient, Auth, BasicAuth, Request, Response

from parsec.backend import Backend


class AdministrationTokenAuth(Auth):
    def __init__(self, token: str) -> None:
        self.set_token(token)

    def set_token(self, token: str) -> None:
        self._auth_header = f"Bearer {token}"

    def auth_flow(self, request: Request) -> Generator[Request, Response, None]:
        request.headers["Authorization"] = self._auth_header
        yield request


@pytest.fixture
def administration_token_auth(backend: Backend) -> AdministrationTokenAuth:
    return AdministrationTokenAuth(backend.config.administration_token)


@pytest.fixture
def administration_basic_auth() -> BasicAuth:
    return BasicAuth(username="admin", password="ImReall!4nAdmin")


@pytest.fixture(
    params=(
        pytest.param(administration_token_auth, id="token_auth"),
        pytest.param(administration_basic_auth, id="basic_auth"),
    )
)
def administration_client(request: pytest.FixtureRequest, client: AsyncClient) -> AsyncClient:
    auth = request.getfixturevalue(request.param.__name__)
    assert isinstance(auth, httpx.Auth)
    client.auth = auth
    return client


type AdminUnauthErrorsTesterDoCallback = Callable[[AsyncClient], Coroutine[None, None, Response]]
type AdminUnauthErrorsTester = Callable[
    [AdminUnauthErrorsTesterDoCallback], Coroutine[None, None, None]
]


@pytest.fixture(
    params=(
        "no_authentication",
        "invalid_authorization_header",
        "bad_bearer_token",
        "invalid_basic_username",
        "invalid_basic_password",
    )
)
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
            case "invalid_basic_username":
                client.auth = BasicAuth(username="foobar", password="ImReall!4nAdmin")
            case "invalid_basic_password":
                client.auth = BasicAuth(username="admin", password="foobar")
            case param:
                assert False, param

        response = await do(client)
        assert response.status_code == 403, response.content

    yield _administration_route_unauth_error_tester
    assert tester_called
