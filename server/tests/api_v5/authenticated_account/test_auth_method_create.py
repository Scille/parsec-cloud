# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AccountAuthMethodID,
    SecretKey,
    UntrustedPasswordAlgorithmArgon2id,
    authenticated_account_cmds,
)
from tests.common import (
    AsyncClient,
    AuthenticatedAccountRpcClient,
    HttpCommonErrorsTester,
    RpcTransportError,
)


async def test_authenticated_account_auth_method_create_ok(
    alice_account: AuthenticatedAccountRpcClient,
    client: AsyncClient,
) -> None:
    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()

    rep = await alice_account.auth_method_create(
        auth_method_id=new_auth_method_id,
        auth_method_mac_key=new_auth_method_mac_key,
        auth_method_password_algorithm=None,
        vault_key_access=b"<vault_key_access_data>",
    )
    assert rep == authenticated_account_cmds.latest.auth_method_create.RepOk()

    # Finally ensure we can connect to the new account...

    new_alice_account = AuthenticatedAccountRpcClient(
        client,
        alice_account.account_email,
        new_auth_method_id,
        new_auth_method_mac_key,
    )
    await new_alice_account.ping(ping="ping")

    # ...and we can still use the old one

    await alice_account.ping(ping="ping")


async def test_authenticated_account_auth_method_create_with_password_algorithm(
    # Use `bob_acount` here since its auth method has no password (while `alice_account` does)
    bob_account: AuthenticatedAccountRpcClient,
    client: AsyncClient,
) -> None:
    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()
    new_auth_method_password_algorithm = UntrustedPasswordAlgorithmArgon2id(
        opslimit=1, memlimit_kb=2, parallelism=3
    )

    rep = await bob_account.auth_method_create(
        auth_method_id=new_auth_method_id,
        auth_method_mac_key=new_auth_method_mac_key,
        auth_method_password_algorithm=new_auth_method_password_algorithm,
        vault_key_access=b"<vault_key_access_data>",
    )
    assert rep == authenticated_account_cmds.latest.auth_method_create.RepOk()

    # Ensure we can connect to the new account...

    new_bob_account = AuthenticatedAccountRpcClient(
        client,
        bob_account.account_email,
        new_auth_method_id,
        new_auth_method_mac_key,
    )
    await new_bob_account.ping(ping="ping")

    # ...and we can still use the old one

    await bob_account.ping(ping="ping")

    # And now re-create another password auth method, this should overwrite the previous one!

    newest_auth_method_id = AccountAuthMethodID.new()
    newest_auth_method_mac_key = SecretKey.generate()
    newest_auth_method_password_algorithm = UntrustedPasswordAlgorithmArgon2id(
        opslimit=2, memlimit_kb=4, parallelism=6
    )

    rep = await bob_account.auth_method_create(
        auth_method_id=newest_auth_method_id,
        auth_method_mac_key=newest_auth_method_mac_key,
        auth_method_password_algorithm=newest_auth_method_password_algorithm,
        vault_key_access=b"<newest_vault_key_access_data>",
    )
    assert rep == authenticated_account_cmds.latest.auth_method_create.RepOk()

    # The initial auth method is still valid (since it is not a password one)

    await bob_account.ping(ping="ping")

    # The newest auth method is valid...

    newest_bob_account = AuthenticatedAccountRpcClient(
        client,
        bob_account.account_email,
        newest_auth_method_id,
        newest_auth_method_mac_key,
    )
    await newest_bob_account.ping(ping="ping")

    # ...but not the previous one

    with pytest.raises(RpcTransportError) as ctx:
        await new_bob_account.ping(ping="ping")
    assert ctx.value.rep.status_code == 403


async def test_authenticated_account_auth_method_create_auth_method_id_already_exists(
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
) -> None:
    # Use the same auth method ID as the existing alice account
    existing_auth_method_id = bob_account.auth_method_id
    new_auth_method_mac_key = SecretKey.generate()

    rep = await alice_account.auth_method_create(
        auth_method_id=existing_auth_method_id,
        auth_method_mac_key=new_auth_method_mac_key,
        auth_method_password_algorithm=None,
        vault_key_access=b"<vault_key_access_data>",
    )
    assert (
        rep == authenticated_account_cmds.latest.auth_method_create.RepAuthMethodIdAlreadyExists()
    )


async def test_authenticated_account_auth_method_create_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        auth_method_id = AccountAuthMethodID.new()
        auth_method_mac_key = SecretKey.generate()

        await alice_account.auth_method_create(
            auth_method_id=auth_method_id,
            auth_method_mac_key=auth_method_mac_key,
            auth_method_password_algorithm=None,
            vault_key_access=b"<vault_key_access_data>",
        )

    await authenticated_account_http_common_errors_tester(do)
