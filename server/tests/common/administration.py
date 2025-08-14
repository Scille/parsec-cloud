from collections.abc import Generator

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
