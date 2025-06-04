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

from parsec._parsec import AccountAuthMethodID, DateTime, EmailAddress, SecretKey
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
ALICE_ACCOUNT_EMAIL = EmailAddress("alice@example.com")
ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY = SecretKey(
    base64.b64decode("0UdFSrwcYKyfhAkTdorrLM46+cwHh49XelCMdoxI4qA=")
)
ALICE_ACCOUNT_AUTH_METHOD_ID = AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e")
ALICE_ACCOUNT_CREATED_ON = DateTime(2000, 1, 1)


BOB_ACCOUNT_EMAIL = EmailAddress("bob@example.com")
BOB_ACCOUNT_AUTH_METHOD_MAC_KEY = SecretKey(
    base64.b64decode("YtFH3Mr8iA5rrduaQPc5fbr+8rGZIU7SZ8wMqwSubhU=")
)
BOB_ACCOUNT_AUTH_METHOD_ID = AccountAuthMethodID.from_hex("1ce9bfb763ba4674a11fe2b064824d72")
BOB_ACCOUNT_CREATED_ON = DateTime(2000, 1, 2)


@pytest.fixture
async def alice_account(
    backend: Backend,
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    # TODO: Replace this by a proper call to `backend.account.create_account()` once available
    assert isinstance(backend.account, MemoryAccountComponent)
    assert ALICE_ACCOUNT_EMAIL not in backend.account._data.accounts
    backend.account._data.accounts[ALICE_ACCOUNT_EMAIL] = MemoryAccount(
        account_email=ALICE_ACCOUNT_EMAIL,
        human_label="Alicey McAliceFace",
        current_vault=MemoryAccountVault(
            items={},
            authentication_methods={
                ALICE_ACCOUNT_AUTH_METHOD_ID: MemoryAuthenticationMethod(
                    id=ALICE_ACCOUNT_AUTH_METHOD_ID,
                    created_on=ALICE_ACCOUNT_CREATED_ON,
                    created_by_ip="127.0.0.1",
                    created_by_user_agent="Parsec-Client/3.4.0 Linux",
                    mac_key=ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY,
                    vault_key_access=b"<alice_vault_key_access>",
                    password_secret_algorithm=PasswordAlgorithmArgon2ID(
                        salt=b"<alice_dummy_salt>",
                        opslimit=65536,
                        memlimit_kb=3,
                        parallelism=1,
                    ),
                )
            },
        ),
    )
    return AuthenticatedAccountRpcClient(
        client, ALICE_ACCOUNT_EMAIL, ALICE_ACCOUNT_AUTH_METHOD_ID, ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY
    )


@pytest.fixture
async def bob_account(
    backend: Backend,
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    # TODO: Replace this by a proper call to `backend.account.create_account()` once available
    assert isinstance(backend.account, MemoryAccountComponent)
    assert BOB_ACCOUNT_EMAIL not in backend.account._data.accounts
    backend.account._data.accounts[BOB_ACCOUNT_EMAIL] = MemoryAccount(
        account_email=BOB_ACCOUNT_EMAIL,
        human_label="Boby McBobFace",
        current_vault=MemoryAccountVault(
            items={},
            authentication_methods={
                BOB_ACCOUNT_AUTH_METHOD_ID: MemoryAuthenticationMethod(
                    id=BOB_ACCOUNT_AUTH_METHOD_ID,
                    created_on=BOB_ACCOUNT_CREATED_ON,
                    created_by_ip="127.0.0.1",
                    created_by_user_agent="Parsec-Client/3.4.0 Linux",
                    mac_key=BOB_ACCOUNT_AUTH_METHOD_MAC_KEY,
                    vault_key_access=b"<bob_vault_key_access>",
                    password_secret_algorithm=PasswordAlgorithmArgon2ID(
                        salt=b"<bob_dummy_salt>",
                        opslimit=65536,
                        memlimit_kb=3,
                        parallelism=1,
                    ),
                )
            },
        ),
    )
    return AuthenticatedAccountRpcClient(
        client, BOB_ACCOUNT_EMAIL, BOB_ACCOUNT_AUTH_METHOD_ID, BOB_ACCOUNT_AUTH_METHOD_MAC_KEY
    )


@pytest.fixture
async def anonymous_account(client: AsyncClient) -> AnonymousAccountRpcClient:
    return AnonymousAccountRpcClient(client)
