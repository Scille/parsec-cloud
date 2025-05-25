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

from parsec._parsec import DateTime, SecretKey
from parsec.backend import Backend
from parsec.components.account import PasswordAlgorithmArgon2ID
from parsec.components.memory.account import MemoryAccountComponent
from parsec.components.memory.datamodel import (
    MemoryAccount,
    MemoryAccountVault,
    MemoryAuthenticationMethod,
)
from tests.common.client import AnonymousAccountRpcClient, AuthenticatedAccountRpcClient

# Note `alice@invalid.com` is Alice's email in `CoolOrg`, `MinimalOrg` etc.
ALICE_ACCOUNT_EMAIL = "alice@invalid.com"
ALICE_ACCOUNT_MAC_KEY = SecretKey(base64.b64decode("0UdFSrwcYKyfhAkTdorrLM46+cwHh49XelCMdoxI4qA="))


@pytest.fixture
async def alice_account(
    backend: Backend,
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    # TODO: Replace this by a proper call to `backend.account.create_account()` once available
    assert isinstance(backend.account, MemoryAccountComponent)
    backend.account._data.accounts[ALICE_ACCOUNT_EMAIL] = MemoryAccount(
        account_email=ALICE_ACCOUNT_EMAIL,
        human_label="Alicey McAliceFace",
        current_vault=MemoryAccountVault(
            items={},
            authentication_methods=[
                MemoryAuthenticationMethod(
                    created_on=DateTime.now(),
                    created_by_ip="127.0.0.1",
                    created_by_user_agent="Parsec-Client/3.4.0 Linux",
                    mac_key=ALICE_ACCOUNT_MAC_KEY,
                    vault_key_access=b"<vault_key_access>",
                    password_secret_algorithm=PasswordAlgorithmArgon2ID(
                        salt=b"<dummy salt>",
                        opslimit=65536,
                        memlimit_kb=3,
                        parallelism=1,
                    ),
                )
            ],
        ),
    )
    return AuthenticatedAccountRpcClient(client, ALICE_ACCOUNT_EMAIL, ALICE_ACCOUNT_MAC_KEY)


@pytest.fixture
async def anonymous_account(client: AsyncClient) -> AnonymousAccountRpcClient:
    return AnonymousAccountRpcClient(client)
