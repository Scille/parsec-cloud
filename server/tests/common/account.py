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

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    EmailValidationToken,
    SecretKey,
)
from parsec.backend import Backend
from parsec.components.account import BaseAccountComponent, PasswordAlgorithmArgon2id
from parsec.components.memory.account import MemoryAccountComponent
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


async def create_account(
    account_component: BaseAccountComponent,
    account_email: EmailAddress,
    auth_method_id: AccountAuthMethodID,
    created_by_ip: str,
    created_by_user_agent: str,
    created_on: DateTime,
    human_label: str,
    mac_key: SecretKey,
    auth_method_password_algorithm: PasswordAlgorithmArgon2id,
    vault_key_access: bytes,
):
    email_token = await account_component.create_email_validation_token(account_email, created_on)
    assert isinstance(email_token, EmailValidationToken), email_token
    res = await account_component.create_account(
        token=email_token,
        now=created_on,
        mac_key=mac_key,
        vault_key_access=vault_key_access,
        human_label=human_label,
        created_by_user_agent=created_by_user_agent,
        created_by_ip=created_by_ip,
        auth_method_id=auth_method_id,
        auth_method_password_algorithm=auth_method_password_algorithm,
    )
    assert res is None


@pytest.fixture
async def alice_account(
    backend: Backend,
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    assert isinstance(backend.account, MemoryAccountComponent)
    await create_account(
        backend.account,
        account_email=ALICE_ACCOUNT_EMAIL,
        auth_method_id=ALICE_ACCOUNT_AUTH_METHOD_ID,
        created_by_ip="127.0.0.1",
        created_by_user_agent="Parsec-Client/3.4.0 Linux",
        created_on=ALICE_ACCOUNT_CREATED_ON,
        human_label="Alicey McAliceFace",
        mac_key=ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY,
        vault_key_access=b"<alice_vault_key_access>",
        auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"<alice_dummy_salt>",
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
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
    assert isinstance(backend.account, MemoryAccountComponent)
    await create_account(
        backend.account,
        account_email=BOB_ACCOUNT_EMAIL,
        auth_method_id=BOB_ACCOUNT_AUTH_METHOD_ID,
        created_by_ip="127.0.0.1",
        created_by_user_agent="Parsec-Client/3.4.0 Linux",
        created_on=BOB_ACCOUNT_CREATED_ON,
        human_label="Boby McBobFace",
        mac_key=BOB_ACCOUNT_AUTH_METHOD_MAC_KEY,
        vault_key_access=b"<bob_vault_key_access>",
        auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"<bob_dummy_salt>",
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
        ),
    )
    return AuthenticatedAccountRpcClient(
        client, BOB_ACCOUNT_EMAIL, BOB_ACCOUNT_AUTH_METHOD_ID, BOB_ACCOUNT_AUTH_METHOD_MAC_KEY
    )


@pytest.fixture
async def anonymous_account(client: AsyncClient) -> AnonymousAccountRpcClient:
    return AnonymousAccountRpcClient(client)
