# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Account system is an entirely different system that is mostly not related to what is
going on with the organizations, and whose client/server interactions are quite simple.
So for simplicity we don't use the testbed template system for it but instead rely on
fixture-based setup of test data.
"""

import base64

import pytest
from httpx import AsyncClient
from pydantic import EmailStr

from parsec._parsec import DateTime, SecretKey
from parsec.components.auth import AccountPasswordAuthenticationToken
from tests.common.client import AnonymousAccountRpcClient, AuthenticatedAccountRpcClient

# Note `alice@invalid.com` is Alice's email in `CoolOrg`, `MinimalOrg` etc.
ALICE_ACCOUNT_EMAIL = "alice@invalid.com"
ALICE_ACCOUNT_MAC_KEY = SecretKey(base64.b64decode("0UdFSrwcYKyfhAkTdorrLM46+cwHh49XelCMdoxI4qA="))


# TODO: remove me once `MemoryAuthComponent.authenticated_account_auth` is implemented !
@pytest.fixture
def patch_authenticated_account_auth(
    monkeypatch: pytest.MonkeyPatch,
) -> dict[EmailStr, SecretKey]:
    from parsec.components.auth import (
        AuthAuthenticatedAccountAuthBadOutcome,
        AuthenticatedAccountAuthInfo,
    )
    from parsec.components.memory.auth import MemoryAuthComponent

    patched_accounts_auth = {}

    async def authenticated_account_auth(
        self,
        now: DateTime,
        token: AccountPasswordAuthenticationToken,
    ) -> AuthenticatedAccountAuthInfo | AuthAuthenticatedAccountAuthBadOutcome:
        mac_key = patched_accounts_auth.get(token.email)
        if not mac_key:
            return AuthAuthenticatedAccountAuthBadOutcome.ACCOUNT_NOT_FOUND
        return AuthenticatedAccountAuthInfo(mac_key=mac_key)

    monkeypatch.setattr(
        MemoryAuthComponent,
        "authenticated_account_auth",
        authenticated_account_auth,
        raising=False,
    )

    return patched_accounts_auth


@pytest.fixture
async def alice_account(
    patch_authenticated_account_auth: dict[EmailStr, SecretKey],
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    patch_authenticated_account_auth[ALICE_ACCOUNT_EMAIL] = ALICE_ACCOUNT_MAC_KEY
    return AuthenticatedAccountRpcClient(client, ALICE_ACCOUNT_EMAIL, ALICE_ACCOUNT_MAC_KEY)


@pytest.fixture
async def anonymous_account(client: AsyncClient) -> AnonymousAccountRpcClient:
    return AnonymousAccountRpcClient(client)
