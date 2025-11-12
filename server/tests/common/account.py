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
    HumanHandle,
    SecretKey,
    ValidationCode,
)
from parsec.backend import Backend
from parsec.components.account import UntrustedPasswordAlgorithmArgon2id
from parsec.config import MockedEmailConfig
from tests.common.client import AnonymousServerRpcClient, AuthenticatedAccountRpcClient
from tests.common.postgresql import clear_postgresql_account_data

# Note `alice@invalid.com` is Alice's email in `CoolOrg`, `MinimalOrg` etc.
ALICE_ACCOUNT_HUMAN_HANDLE = HumanHandle(EmailAddress("alice@example.com"), "Alicey McAliceFace")
ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY = SecretKey(
    base64.b64decode("0UdFSrwcYKyfhAkTdorrLM46+cwHh49XelCMdoxI4qA=")
)
ALICE_ACCOUNT_AUTH_METHOD_ID = AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e")
ALICE_ACCOUNT_CREATED_ON = DateTime(2000, 1, 1)


BOB_ACCOUNT_HUMAN_HANDLE = HumanHandle(EmailAddress("bob@example.com"), "Boby McBobFace")
BOB_ACCOUNT_AUTH_METHOD_MAC_KEY = SecretKey(
    base64.b64decode("YtFH3Mr8iA5rrduaQPc5fbr+8rGZIU7SZ8wMqwSubhU=")
)
BOB_ACCOUNT_AUTH_METHOD_ID = AccountAuthMethodID.from_hex("1ce9bfb763ba4674a11fe2b064824d72")
BOB_ACCOUNT_CREATED_ON = DateTime(2000, 1, 2)


async def create_account(
    backend: Backend,
    auth_method_id: AccountAuthMethodID,
    created_by_ip: str,
    created_by_user_agent: str,
    created_on: DateTime,
    human_handle: HumanHandle,
    auth_method_mac_key: SecretKey,
    auth_method_password_algorithm: UntrustedPasswordAlgorithmArgon2id | None,
    vault_key_access: bytes,
):
    assert isinstance(backend.config.email_config, MockedEmailConfig)
    assert len(backend.config.email_config.sent_emails) == 0  # Sanity check

    validation_code = await backend.account.create_send_validation_email(
        created_on, human_handle.email
    )
    assert isinstance(validation_code, ValidationCode), validation_code
    backend.config.email_config.sent_emails.clear()

    res = await backend.account.create_proceed(
        now=created_on,
        validation_code=validation_code,
        vault_key_access=vault_key_access,
        human_handle=human_handle,
        created_by_user_agent=created_by_user_agent,
        created_by_ip=created_by_ip,
        auth_method_id=auth_method_id,
        auth_method_mac_key=auth_method_mac_key,
        auth_method_password_algorithm=auth_method_password_algorithm,
    )
    assert res is None


@pytest.fixture
async def clear_account_data(
    request: pytest.FixtureRequest,
) -> None:
    """
    Unlike for organizations, accounts are global across the whole database,
    hence each test involving account need to start by clearing those data.

    This fixture is typically not directly used by the test, but instead
    by the `alice_account`/`bob_account`/`anonymous_server` fixtures the test
    itself relies on.
    """
    if request.config.getoption("--postgresql"):
        await clear_postgresql_account_data()


@pytest.fixture
async def alice_account(
    clear_account_data: None,
    backend: Backend,
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    await create_account(
        backend,
        auth_method_id=ALICE_ACCOUNT_AUTH_METHOD_ID,
        created_by_ip="127.0.0.1",
        created_by_user_agent="Parsec-Client/3.4.0 Linux",
        created_on=ALICE_ACCOUNT_CREATED_ON,
        human_handle=ALICE_ACCOUNT_HUMAN_HANDLE,
        vault_key_access=b"<alice_vault_key_access>",
        auth_method_mac_key=ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY,
        auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
        ),
    )
    return AuthenticatedAccountRpcClient(
        client,
        ALICE_ACCOUNT_HUMAN_HANDLE.email,
        ALICE_ACCOUNT_AUTH_METHOD_ID,
        ALICE_ACCOUNT_AUTH_METHOD_MAC_KEY,
    )


@pytest.fixture
async def bob_account(
    clear_account_data: None,
    backend: Backend,
    client: AsyncClient,
) -> AuthenticatedAccountRpcClient:
    await create_account(
        backend,
        auth_method_id=BOB_ACCOUNT_AUTH_METHOD_ID,
        created_by_ip="127.0.0.1",
        created_by_user_agent="Parsec-Client/3.4.0 Linux",
        created_on=BOB_ACCOUNT_CREATED_ON,
        human_handle=BOB_ACCOUNT_HUMAN_HANDLE,
        vault_key_access=b"<bob_vault_key_access>",
        auth_method_mac_key=BOB_ACCOUNT_AUTH_METHOD_MAC_KEY,
        auth_method_password_algorithm=None,
    )
    return AuthenticatedAccountRpcClient(
        client,
        BOB_ACCOUNT_HUMAN_HANDLE.email,
        BOB_ACCOUNT_AUTH_METHOD_ID,
        BOB_ACCOUNT_AUTH_METHOD_MAC_KEY,
    )


@pytest.fixture
async def anonymous_server(
    clear_account_data: None, client: AsyncClient
) -> AnonymousServerRpcClient:
    return AnonymousServerRpcClient(client)
