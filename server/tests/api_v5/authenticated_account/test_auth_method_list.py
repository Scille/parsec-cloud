# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    SecretKey,
    UntrustedPasswordAlgorithmArgon2id,
    authenticated_account_cmds,
)
from tests.common import (
    AuthenticatedAccountRpcClient,
    Backend,
    DateTime,
    HttpCommonErrorsTester,
)


async def test_authenticated_account_auth_method_list_ok(
    alice_account: AuthenticatedAccountRpcClient,
    # Use Bob here to ensure its auth method is ignored
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    auth_method_password_1_id = alice_account.auth_method_id

    # Alice initial auth method is a password one, overwrite it with a new password

    auth_method_password_2_id = AccountAuthMethodID.new()
    auth_method_password_2_mac_key = SecretKey.generate()

    outcome = await backend.account.auth_method_create(
        now=DateTime(2025, 1, 1),
        auth_method_id=auth_method_password_1_id,
        created_by_user_agent="",
        created_by_ip="",
        new_auth_method_id=auth_method_password_2_id,
        new_auth_method_mac_key=auth_method_password_2_mac_key,
        new_auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
            opslimit=1, memlimit_kb=2, parallelism=3
        ),
        new_vault_key_access=b"<password_2_vault_key_access_data>",
    )
    assert outcome is None

    # Also add a non-password auth method...

    auth_method_master_secret_1_id = AccountAuthMethodID.new()
    auth_method_master_secret_1_mac_key = SecretKey.generate()

    outcome = await backend.account.auth_method_create(
        now=DateTime(2025, 1, 2),
        auth_method_id=auth_method_password_2_id,
        created_by_user_agent="",
        created_by_ip="",
        new_auth_method_id=auth_method_master_secret_1_id,
        new_auth_method_mac_key=auth_method_master_secret_1_mac_key,
        new_auth_method_password_algorithm=None,
        new_vault_key_access=b"<master_secret_1_vault_key_access_data>",
    )
    assert outcome is None

    # ...and another one that got disabled

    auth_method_master_secret_2_id = AccountAuthMethodID.new()
    auth_method_master_secret_2_mac_key = SecretKey.generate()

    outcome = await backend.account.auth_method_create(
        now=DateTime(2025, 1, 3),
        auth_method_id=auth_method_password_2_id,
        created_by_user_agent="",
        created_by_ip="",
        new_auth_method_id=auth_method_master_secret_2_id,
        new_auth_method_mac_key=auth_method_master_secret_2_mac_key,
        new_auth_method_password_algorithm=None,
        new_vault_key_access=b"<master_secret_2_vault_key_access_data>",
    )
    assert outcome is None

    outcome = await backend.account.auth_method_disable(
        now=DateTime(2025, 1, 4),
        auth_method_id=auth_method_password_2_id,
        to_disable_auth_method_id=auth_method_master_secret_2_id,
    )
    assert outcome is None

    # Finally do the actual listing!

    alice_account.auth_method_id = auth_method_password_2_id
    alice_account.auth_method_mac_key = auth_method_password_2_mac_key
    rep = await alice_account.auth_method_list()
    assert rep == authenticated_account_cmds.latest.auth_method_list.RepOk(
        [
            authenticated_account_cmds.latest.auth_method_list.AuthMethod(
                auth_method_id=auth_method_password_2_id,
                created_on=DateTime(2025, 1, 1),
                created_by_ip="",
                created_by_user_agent="",
                vault_key_access=b"<password_2_vault_key_access_data>",
                password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                    opslimit=1, memlimit_kb=2, parallelism=3
                ),
            ),
            authenticated_account_cmds.latest.auth_method_list.AuthMethod(
                auth_method_id=auth_method_master_secret_1_id,
                created_on=DateTime(2025, 1, 2),
                created_by_ip="",
                created_by_user_agent="",
                vault_key_access=b"<master_secret_1_vault_key_access_data>",
                password_algorithm=None,
            ),
        ]
    )


async def test_authenticated_account_auth_method_list_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.auth_method_list()

    await authenticated_account_http_common_errors_tester(do)
