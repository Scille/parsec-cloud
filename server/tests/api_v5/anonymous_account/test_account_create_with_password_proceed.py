# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    EmailValidationToken,
    SecretKey,
    anonymous_account_cmds,
)
from tests.common import AnonymousAccountRpcClient, Backend, HttpCommonErrorsTester


async def test_anonymous_account_account_create_with_password_proceed_ok(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
) -> None:
    email = "alice@invalid.com"
    # 1st account creation request

    rep = await anonymous_account.account_create_send_validation_email(email=email)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    # retrieve alice's token that was sent by email
    token = backend.account.test_get_token_by_email(email)
    assert token is not None

    rep = await anonymous_account.account_create_with_password_proceed(
        validation_token=token,
        human_label="Anonymous Alice",
        password_algorithm=anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
    )
    assert rep == anonymous_account_cmds.latest.account_create_with_password_proceed.RepOk()

    # Alice's mail removed from unverified emails
    try:
        token = backend.account.test_get_token_by_email(email)
    except KeyError:
        pass


async def test_anonymous_account_account_create_with_password_proceed_invalid_validation_token(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
):
    email = "alice@invalid.com"
    # 1st account creation request

    rep = await anonymous_account.account_create_send_validation_email(email=email)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    # retrieve alice's token that was sent by email
    token = backend.account.test_get_token_by_email(email)
    assert token is not None

    other_token = EmailValidationToken.new()

    rep = await anonymous_account.account_create_with_password_proceed(
        validation_token=other_token,
        human_label="Anonymous Alice",
        password_algorithm=anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
    )
    assert (
        rep
        == anonymous_account_cmds.latest.account_create_with_password_proceed.RepInvalidValidationToken()
    )

    # Alice's mail kept in unverified emails
    token = backend.account.test_get_token_by_email(email)
    assert token is not None


async def test_anonymous_account_account_create_with_password_proceed_http_common_errors(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        other_token = EmailValidationToken.new()

        await anonymous_account.account_create_with_password_proceed(
            validation_token=other_token,
            human_label="Anonymous Alice",
            password_algorithm=anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithmArgon2id(
                salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
            ),
            auth_method_hmac_key=SecretKey.generate(),
            vault_key_access=b"vault_key_access",
            auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
        )

    await anonymous_account_http_common_errors_tester(do)


async def test_anonymous_account_account_create_with_password_proceed_auth_method_id_already_exists(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
):
    # 1st ok account creation
    email = "alice@invalid.com"
    # 1st account creation request

    rep = await anonymous_account.account_create_send_validation_email(email=email)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    # retrieve alice's token that was sent by email
    token = backend.account.test_get_token_by_email(email)
    assert token is not None

    rep = await anonymous_account.account_create_with_password_proceed(
        validation_token=token,
        human_label="Anonymous Alice",
        password_algorithm=anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
    )

    assert rep == anonymous_account_cmds.latest.account_create_with_password_proceed.RepOk()

    # Second account creation
    email = "bob@invalid.com"
    rep = await anonymous_account.account_create_send_validation_email(email=email)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    # retrieve bob's token that was sent by email
    token = backend.account.test_get_token_by_email(email)
    assert token is not None

    rep = await anonymous_account.account_create_with_password_proceed(
        validation_token=token,
        human_label="Anonymous Bob",
        password_algorithm=anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        # Same auth method id as previous
        auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
    )

    assert (
        rep
        == anonymous_account_cmds.latest.account_create_with_password_proceed.RepAuthMethodIdAlreadyExists()
    )
