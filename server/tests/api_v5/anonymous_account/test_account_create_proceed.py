# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    EmailValidationToken,
    PasswordAlgorithmArgon2id,
    SecretKey,
    anonymous_account_cmds,
)
from parsec.components.account import AccountCreateAccountBadOutcome
from tests.common import AnonymousAccountRpcClient, Backend, HttpCommonErrorsTester


async def request_account_validation_email(
    backend: Backend, email: EmailAddress, now: DateTime
) -> EmailValidationToken:
    res = await backend.account.create_email_validation_token(email, now)
    match res:
        case EmailValidationToken() as token:
            return token
        case _:
            raise ValueError(f"Unexpected outcome {res}")


async def test_anonymous_account_account_create_proceed_ok(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
) -> None:
    email = EmailAddress("alice@invalid.com")
    now = DateTime.now()
    # 1st account creation request

    token = await request_account_validation_email(backend, email, now)

    rep = await anonymous_account.account_create_proceed(
        validation_token=token,
        human_label="Anonymous Alice",
        auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
    )
    assert rep == anonymous_account_cmds.latest.account_create_proceed.RepOk()

    # Alice's mail removed from unverified emails
    try:
        token = backend.account.test_get_token_by_email(email)
    except KeyError:
        pass


async def test_anonymous_account_account_create_proceed_invalid_validation_token(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
):
    email = EmailAddress("alice@invalid.com")
    now = DateTime.now()
    # 1st account creation request

    token = await request_account_validation_email(backend, email, now)

    other_token = EmailValidationToken.new()
    assert other_token != token  # Sanity check

    rep = await anonymous_account.account_create_proceed(
        validation_token=other_token,
        human_label="Anonymous Alice",
        auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
    )
    assert rep == anonymous_account_cmds.latest.account_create_proceed.RepInvalidValidationToken()

    # Alice's mail kept in unverified emails
    token = backend.account.test_get_token_by_email(email)
    assert token is not None


async def test_anonymous_account_acount_create_proceed_token_too_old(
    xfail_if_postgresql: None, anonymous_account: AnonymousAccountRpcClient, backend: Backend
):
    import functools

    email = EmailAddress("alice@example.com")
    now = DateTime.now()

    token = await request_account_validation_email(backend, email, now)

    create_account = functools.partial(
        backend.account.create_account,
        token=token,
        mac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        human_label="Alice",
        created_by_user_agent="Test",
        created_by_ip="",
        auth_method_id=AccountAuthMethodID.new(),
        auth_method_password_algorithm=None,
    )

    outcome = await create_account(
        now=now.add(seconds=backend.config.account_config.creation_token_duration + 1)
    )
    assert outcome == AccountCreateAccountBadOutcome.INVALID_TOKEN

    outcome = await create_account(
        now=now.add(seconds=backend.config.account_config.creation_token_duration)
    )
    assert outcome is None


async def test_anonymous_account_account_create_proceed_http_common_errors(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        other_token = EmailValidationToken.new()

        await anonymous_account.account_create_proceed(
            validation_token=other_token,
            human_label="Anonymous Alice",
            auth_method_password_algorithm=PasswordAlgorithmArgon2id(
                salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
            ),
            auth_method_hmac_key=SecretKey.generate(),
            vault_key_access=b"vault_key_access",
            auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
        )

    await anonymous_account_http_common_errors_tester(do)


async def test_anonymous_account_account_create_proceed_auth_method_id_already_exists(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    backend: Backend,
):
    auth_method_id = AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e")
    # 1st ok account creation
    email = EmailAddress("alice@invalid.com")
    now = DateTime.now()
    # 1st account creation request

    token = await request_account_validation_email(backend, email, now)

    rep = await anonymous_account.account_create_proceed(
        validation_token=token,
        human_label="Anonymous Alice",
        auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        auth_method_id=auth_method_id,
    )

    assert rep == anonymous_account_cmds.latest.account_create_proceed.RepOk()

    # Second account creation
    email = EmailAddress("bob@invalid.com")
    token = await request_account_validation_email(backend, email, now)

    rep = await anonymous_account.account_create_proceed(
        validation_token=token,
        human_label="Anonymous Bob",
        auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"pepper", opslimit=1, memlimit_kb=2, parallelism=3
        ),
        auth_method_hmac_key=SecretKey.generate(),
        vault_key_access=b"vault_key_access",
        # Same auth method id as previous
        auth_method_id=auth_method_id,
    )

    assert (
        rep == anonymous_account_cmds.latest.account_create_proceed.RepAuthMethodIdAlreadyExists()
    )
