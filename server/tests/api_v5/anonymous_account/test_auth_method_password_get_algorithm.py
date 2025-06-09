# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    EmailAddress,
    PasswordAlgorithmArgon2id,
    anonymous_account_cmds,
)
from tests.common import (
    AnonymousAccountRpcClient,
    AuthenticatedAccountRpcClient,
    Backend,
    HttpCommonErrorsTester,
)


async def test_anonymous_account_auth_method_password_get_algorithm_ok_existing(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    rep = await anonymous_account.auth_method_password_get_algorithm(
        email=alice_account.account_email
    )
    assert rep == anonymous_account_cmds.latest.auth_method_password_get_algorithm.RepOk(
        password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"<alice_dummy_salt>",
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
        )
    )


async def test_anonymous_account_auth_method_password_get_algorithm_ok_stable_fake(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
) -> None:
    unknown_email = EmailAddress("dummy@example.com")

    rep = await anonymous_account.auth_method_password_get_algorithm(email=unknown_email)
    assert rep == anonymous_account_cmds.latest.auth_method_password_get_algorithm.RepOk(
        password_algorithm=PasswordAlgorithmArgon2id(
            salt=b'\xac\x07iw\xa6\xf9\xb5\x99\xe8\x92\xcd\xd9\xb5\xd5"b',
            opslimit=3,
            memlimit_kb=131072,
            parallelism=1,
        )
    )

    # The result must be stable across multiple queries

    rep2 = await anonymous_account.auth_method_password_get_algorithm(email=unknown_email)
    assert rep2 == rep


async def test_anonymous_account_auth_method_password_get_algorithm_http_common_errors(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_account.auth_method_password_get_algorithm(
            email=EmailAddress("zack@example.com")
        )

    await anonymous_account_http_common_errors_tester(do)
