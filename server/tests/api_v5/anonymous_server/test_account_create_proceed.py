# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    SecretKey,
    UntrustedPasswordAlgorithmArgon2id,
    ValidationCode,
    anonymous_server_cmds,
)
from parsec.components.account import (
    VALIDATION_CODE_MAX_FAILED_ATTEMPTS,
    VALIDATION_CODE_VALIDITY_DURATION_SECONDS,
)
from parsec.config import MockedEmailConfig
from tests.common import (
    ALICE_ACCOUNT_HUMAN_HANDLE,
    AnonymousServerRpcClient,
    AsyncClient,
    AuthenticatedAccountRpcClient,
    Backend,
    HttpCommonErrorsTester,
    generate_different_validation_code,
)


@pytest.fixture
async def alice_validation_code(backend: Backend) -> ValidationCode:
    assert isinstance(backend.config.email_config, MockedEmailConfig)
    assert len(backend.config.email_config.sent_emails) == 0  # Sanity check

    validation_code = await backend.account.create_send_validation_email(
        DateTime.now(), ALICE_ACCOUNT_HUMAN_HANDLE.email
    )
    assert isinstance(validation_code, ValidationCode)
    backend.config.email_config.sent_emails.clear()

    return validation_code


async def test_anonymous_server_account_create_proceed_ok(
    anonymous_server: AnonymousServerRpcClient,
    alice_validation_code: ValidationCode,
    client: AsyncClient,
) -> None:
    new_auth_method_id = AccountAuthMethodID.from_hex("a11a285306c546bf89e2d59c8d7deafa")
    new_auth_method_mac_key = SecretKey.generate()

    # Once step 1 has been done once, the steps can obviously no longer be retried
    for attempt in range(2):
        # Optional step 0

        rep = await anonymous_server.account_create_proceed(
            account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber0CheckCode(
                validation_code=alice_validation_code,
                email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
            )
        )

        if attempt == 0:
            assert rep == anonymous_server_cmds.latest.account_create_proceed.RepOk()
        else:
            assert (
                rep
                == anonymous_server_cmds.latest.account_create_proceed.RepSendValidationEmailRequired()
            )

        # Step 1

        rep = await anonymous_server.account_create_proceed(
            account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber1Create(
                validation_code=alice_validation_code,
                human_handle=ALICE_ACCOUNT_HUMAN_HANDLE,
                auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                    opslimit=1, memlimit_kb=2, parallelism=3
                ),
                auth_method_mac_key=new_auth_method_mac_key,
                vault_key_access=b"vault_key_access",
                auth_method_id=new_auth_method_id,
            )
        )
        if attempt == 0:
            assert rep == anonymous_server_cmds.latest.account_create_proceed.RepOk()
        else:
            assert (
                rep
                == anonymous_server_cmds.latest.account_create_proceed.RepSendValidationEmailRequired()
            )

    # Finally ensure we can connect to the new account

    account = AuthenticatedAccountRpcClient(
        client,
        ALICE_ACCOUNT_HUMAN_HANDLE.email,
        new_auth_method_id,
        new_auth_method_mac_key,
    )
    await account.ping(ping="ping")


@pytest.mark.parametrize("kind", ("step_0", "step_1"))
async def test_anonymous_server_account_create_proceed_invalid_validation_code(
    kind: str,
    anonymous_server: AnonymousServerRpcClient,
    alice_validation_code: ValidationCode,
):
    bad_validation_code = generate_different_validation_code(alice_validation_code)

    match kind:
        case "step_0":

            async def do_request(
                validation_code: ValidationCode,
            ) -> anonymous_server_cmds.latest.account_create_proceed.Rep:
                return await anonymous_server.account_create_proceed(
                    account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber0CheckCode(
                        validation_code=bad_validation_code,
                        email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
                    )
                )

        case "step_1":

            async def do_request(
                validation_code: ValidationCode,
            ) -> anonymous_server_cmds.latest.account_create_proceed.Rep:
                return await anonymous_server.account_create_proceed(
                    account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber1Create(
                        validation_code=bad_validation_code,
                        human_handle=ALICE_ACCOUNT_HUMAN_HANDLE,
                        auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                            opslimit=1, memlimit_kb=2, parallelism=3
                        ),
                        auth_method_mac_key=SecretKey.generate(),
                        vault_key_access=b"vault_key_access",
                        auth_method_id=AccountAuthMethodID.from_hex(
                            "9aae259f748045cc9fe7146eab0b132e"
                        ),
                    )
                )

        case unkown:
            assert False, unkown

    # Multiple bad attempts
    for _ in range(VALIDATION_CODE_MAX_FAILED_ATTEMPTS):
        rep = await do_request(bad_validation_code)
        assert rep == anonymous_server_cmds.latest.account_create_proceed.RepInvalidValidationCode()

    # Further attempts are no longer considered even if the good validation code is provided
    for validation_code in (bad_validation_code, alice_validation_code):
        rep = await do_request(validation_code)
        assert (
            rep
            == anonymous_server_cmds.latest.account_create_proceed.RepSendValidationEmailRequired()
        )


@pytest.mark.parametrize(
    "kind",
    (
        "validation_code_already_used",
        "validation_code_too_many_attemps",
        "validation_code_too_old",
        "wrong_email_address",
    ),
)
async def test_anonymous_server_account_create_proceed_send_validation_email_required(
    kind: str,
    anonymous_server: AnonymousServerRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
):
    validation_code = None
    match kind:
        case "validation_code_already_used":
            # Nothing to do: this has already been testbed at the end of
            # `test_anonymous_server_account_create_proceed_ok`
            return

        case "validation_code_too_many_attemps":
            # Nothing to do: this has already been testbed at the end of
            # `test_anonymous_server_account_create_proceed_invalid_validation_code`
            return

        case "validation_code_too_old":
            timestamp_too_old = DateTime.now().add(
                seconds=-VALIDATION_CODE_VALIDITY_DURATION_SECONDS
            )
            validation_code = await backend.account.create_send_validation_email(
                timestamp_too_old, ALICE_ACCOUNT_HUMAN_HANDLE.email
            )

        case "wrong_email_address":
            validation_code = await backend.account.create_send_validation_email(
                DateTime.now(), EmailAddress("dummy@example.com")
            )

        case unknown:
            assert False, unknown

    assert isinstance(validation_code, ValidationCode)

    rep = await anonymous_server.account_create_proceed(
        account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber1Create(
            validation_code=validation_code,
            human_handle=ALICE_ACCOUNT_HUMAN_HANDLE,
            auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                opslimit=1, memlimit_kb=2, parallelism=3
            ),
            auth_method_mac_key=SecretKey.generate(),
            vault_key_access=b"vault_key_access",
            auth_method_id=AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e"),
        )
    )
    assert (
        rep == anonymous_server_cmds.latest.account_create_proceed.RepSendValidationEmailRequired()
    )


async def test_anonymous_server_account_create_proceed_auth_method_id_already_exists(
    anonymous_server: AnonymousServerRpcClient,
    alice_validation_code: ValidationCode,
    bob_account: AuthenticatedAccountRpcClient,
):
    rep = await anonymous_server.account_create_proceed(
        account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber1Create(
            validation_code=alice_validation_code,
            human_handle=ALICE_ACCOUNT_HUMAN_HANDLE,
            auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                opslimit=1, memlimit_kb=2, parallelism=3
            ),
            auth_method_mac_key=SecretKey.generate(),
            vault_key_access=b"vault_key_access",
            auth_method_id=bob_account.auth_method_id,
        )
    )
    assert rep == anonymous_server_cmds.latest.account_create_proceed.RepAuthMethodIdAlreadyExists()


async def test_anonymous_server_account_create_proceed_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
    alice_validation_code: ValidationCode,
) -> None:
    async def do():
        await anonymous_server.account_create_proceed(
            account_create_step=anonymous_server_cmds.latest.account_create_proceed.AccountCreateStepNumber0CheckCode(
                validation_code=alice_validation_code,
                email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
            )
        )

    await anonymous_server_http_common_errors_tester(do)
