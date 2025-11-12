# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    SecretKey,
    UntrustedPasswordAlgorithmArgon2id,
    anonymous_server_cmds,
)
from tests.common import (
    AnonymousServerRpcClient,
    AuthenticatedAccountRpcClient,
    Backend,
    HttpCommonErrorsTester,
)


async def test_anonymous_server_auth_method_password_get_algorithm_ok_existing(
    anonymous_server: AnonymousServerRpcClient,
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    expected_password_algorithm_1 = UntrustedPasswordAlgorithmArgon2id(
        opslimit=65536,
        memlimit_kb=3,
        parallelism=1,
    )
    rep = await anonymous_server.auth_method_password_get_algorithm(
        email=alice_account.account_email
    )
    assert rep == anonymous_server_cmds.latest.auth_method_password_get_algorithm.RepOk(
        password_algorithm=expected_password_algorithm_1
    )

    # Now replace Alice account password algorithm with a new one to ensure the old one is ignored

    expected_password_algorithm_2 = UntrustedPasswordAlgorithmArgon2id(
        opslimit=131072,
        memlimit_kb=6,
        parallelism=2,
    )

    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()
    await backend.account.vault_key_rotation(
        now=DateTime.now(),
        auth_method_id=alice_account.auth_method_id,
        created_by_ip="",
        created_by_user_agent="",
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_vault_key_access=b"",
        new_auth_method_password_algorithm=expected_password_algorithm_2,
        items={},
    )
    rep = await anonymous_server.auth_method_password_get_algorithm(
        email=alice_account.account_email
    )
    assert rep == anonymous_server_cmds.latest.auth_method_password_get_algorithm.RepOk(
        password_algorithm=expected_password_algorithm_2
    )


async def test_anonymous_server_auth_method_password_get_algorithm_ok_stable_fake(
    anonymous_server: AnonymousServerRpcClient,
    # Use `alice_account` as an unrelated existing account to ensure it is ignored
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    unknown_email = EmailAddress("dummy@example.com")

    rep = await anonymous_server.auth_method_password_get_algorithm(email=unknown_email)
    assert rep == anonymous_server_cmds.latest.auth_method_password_get_algorithm.RepOk(
        password_algorithm=UntrustedPasswordAlgorithmArgon2id(
            opslimit=3,
            memlimit_kb=131072,
            parallelism=1,
        )
    )

    # The result must be stable across multiple queries

    rep2 = await anonymous_server.auth_method_password_get_algorithm(email=unknown_email)
    assert rep2 == rep


async def test_anonymous_server_auth_method_password_get_algorithm_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_server.auth_method_password_get_algorithm(
            email=EmailAddress("zack@example.com")
        )

    await anonymous_server_http_common_errors_tester(do)
