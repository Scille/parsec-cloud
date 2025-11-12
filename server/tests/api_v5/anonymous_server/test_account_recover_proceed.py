# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    HashDigest,
    SecretKey,
    UntrustedPasswordAlgorithmArgon2id,
    ValidationCode,
    anonymous_server_cmds,
)
from parsec.components.account import (
    VALIDATION_CODE_MAX_FAILED_ATTEMPTS,
    VALIDATION_CODE_VALIDITY_DURATION_SECONDS,
    VaultItemRecoveryList,
    VaultItems,
)
from parsec.config import MockedEmailConfig
from tests.common import (
    ALICE_ACCOUNT_HUMAN_HANDLE,
    AnonymousServerRpcClient,
    AsyncClient,
    AuthenticatedAccountRpcClient,
    Backend,
    HttpCommonErrorsTester,
    RpcTransportError,
    generate_different_validation_code,
)


@pytest.fixture
async def alice_validation_code(
    backend: Backend, alice_account: AuthenticatedAccountRpcClient
) -> ValidationCode:
    assert isinstance(backend.config.email_config, MockedEmailConfig)
    assert len(backend.config.email_config.sent_emails) == 0  # Sanity check

    validation_code = await backend.account.recover_send_validation_email(
        DateTime.now(), ALICE_ACCOUNT_HUMAN_HANDLE.email
    )
    assert isinstance(validation_code, ValidationCode)
    backend.config.email_config.sent_emails.clear()

    return validation_code


@pytest.mark.parametrize("kind", ("with_password", "without_password"))
async def test_anonymous_server_account_recover_proceed_ok(
    kind: str,
    alice_account: AuthenticatedAccountRpcClient,
    anonymous_server: AnonymousServerRpcClient,
    alice_validation_code: ValidationCode,
    backend: Backend,
    client: AsyncClient,
) -> None:
    new_auth_method_id = AccountAuthMethodID.from_hex("a11a285306c546bf89e2d59c8d7deafa")
    new_auth_method_mac_key = SecretKey.generate()
    match kind:
        case "with_password":
            auth_method_password_algorithm = UntrustedPasswordAlgorithmArgon2id(
                opslimit=1, memlimit_kb=2, parallelism=3
            )

        case "without_password":
            auth_method_password_algorithm = None

        case unknown:
            assert False, unknown

    # Add an item to the current vault to ensure it is no longer returned when a new vault is created

    outcome = await backend.account.vault_item_upload(
        auth_method_id=alice_account.auth_method_id,
        item_fingerprint=HashDigest.from_data(b"item"),
        item=b"<item>",
    )
    assert outcome is None

    # Do the actual recovery

    rep = await anonymous_server.account_recover_proceed(
        validation_code=alice_validation_code,
        email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
        new_vault_key_access=b"<new_vault_key_access>",
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=auth_method_password_algorithm,
    )
    assert rep == anonymous_server_cmds.latest.account_recover_proceed.RepOk()

    # The reset can only be used once for a given validation code!

    rep = await anonymous_server.account_recover_proceed(
        validation_code=alice_validation_code,
        email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
        new_vault_key_access=b"<other_vault_key_access>",
        new_auth_method_id=AccountAuthMethodID.new(),
        new_auth_method_mac_key=SecretKey.generate(),
        new_auth_method_password_algorithm=auth_method_password_algorithm,
    )
    assert (
        rep == anonymous_server_cmds.latest.account_recover_proceed.RepSendValidationEmailRequired()
    )

    # Now ensure that the old auth method is no longer usable...

    with pytest.raises(RpcTransportError) as ctx:
        await alice_account.ping(ping="ping")
    assert ctx.value.rep.status_code == 403

    # ...and the new one is!

    account = AuthenticatedAccountRpcClient(
        client,
        ALICE_ACCOUNT_HUMAN_HANDLE.email,
        new_auth_method_id,
        new_auth_method_mac_key,
    )
    await account.ping(ping="ping")

    # Finally ensure the new vault has no items...

    outcome = await backend.account.vault_item_list(auth_method_id=new_auth_method_id)
    assert isinstance(outcome, VaultItems)
    assert len(outcome.items) == 0
    assert outcome.key_access == b"<new_vault_key_access>"

    # ...and the previous vault is accessible from the recovery list

    outcome = await backend.account.vault_item_recovery_list(auth_method_id=new_auth_method_id)
    assert isinstance(outcome, VaultItemRecoveryList)
    assert len(outcome.current_vault.auth_methods) == 1
    assert outcome.current_vault.auth_methods[0].vault_key_access == b"<new_vault_key_access>"
    assert len(outcome.previous_vaults) == 1
    assert len(outcome.previous_vaults[0].auth_methods) == 1
    assert (
        outcome.previous_vaults[0].auth_methods[0].vault_key_access == b"<alice_vault_key_access>"
    )
    assert outcome.previous_vaults[0].auth_methods[0].disabled_on is not None
    assert outcome.previous_vaults[0].vault_items == {HashDigest.from_data(b"item"): b"<item>"}


async def test_anonymous_server_account_recover_proceed_invalid_validation_code(
    anonymous_server: AnonymousServerRpcClient,
    alice_validation_code: ValidationCode,
):
    bad_validation_code = generate_different_validation_code(alice_validation_code)

    async def do_request(
        validation_code: ValidationCode,
    ) -> anonymous_server_cmds.latest.account_recover_proceed.Rep:
        return await anonymous_server.account_recover_proceed(
            validation_code=bad_validation_code,
            email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
            new_vault_key_access=b"vault_key_access",
            new_auth_method_id=AccountAuthMethodID.new(),
            new_auth_method_password_algorithm=None,
            new_auth_method_mac_key=SecretKey.generate(),
        )

    # Multiple bad attempts
    for _ in range(VALIDATION_CODE_MAX_FAILED_ATTEMPTS):
        rep = await do_request(bad_validation_code)
        assert (
            rep == anonymous_server_cmds.latest.account_recover_proceed.RepInvalidValidationCode()
        )

    # Further attempts are no longer considered even if the good validation code is provided
    for validation_code in (bad_validation_code, alice_validation_code):
        rep = await do_request(validation_code)
        assert (
            rep
            == anonymous_server_cmds.latest.account_recover_proceed.RepSendValidationEmailRequired()
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
async def test_anonymous_server_account_recover_proceed_send_validation_email_required(
    kind: str,
    anonymous_server: AnonymousServerRpcClient,
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
):
    validation_code = None
    match kind:
        case "validation_code_already_used":
            # Nothing to do: this has already been testbed at the end of
            # `test_anonymous_server_account_recover_proceed_ok`
            return

        case "validation_code_too_many_attemps":
            # Nothing to do: this has already been testbed at the end of
            # `test_anonymous_server_account_recover_proceed_invalid_validation_code`
            return

        case "validation_code_too_old":
            timestamp_too_old = DateTime.now().add(
                seconds=-VALIDATION_CODE_VALIDITY_DURATION_SECONDS
            )
            validation_code = await backend.account.recover_send_validation_email(
                timestamp_too_old, alice_account.account_email
            )

        case "wrong_email_address":
            validation_code = await backend.account.recover_send_validation_email(
                DateTime.now(), bob_account.account_email
            )

        case unknown:
            assert False, unknown

    assert isinstance(validation_code, ValidationCode)

    rep = await anonymous_server.account_recover_proceed(
        validation_code=validation_code,
        email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
        new_vault_key_access=b"vault_key_access",
        new_auth_method_id=bob_account.auth_method_id,
        new_auth_method_password_algorithm=None,
        new_auth_method_mac_key=SecretKey.generate(),
    )
    assert (
        rep == anonymous_server_cmds.latest.account_recover_proceed.RepSendValidationEmailRequired()
    )


async def test_anonymous_server_account_recover_proceed_auth_method_id_already_exists(
    anonymous_server: AnonymousServerRpcClient,
    alice_validation_code: ValidationCode,
    bob_account: AuthenticatedAccountRpcClient,
):
    rep = await anonymous_server.account_recover_proceed(
        validation_code=alice_validation_code,
        email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
        new_vault_key_access=b"vault_key_access",
        new_auth_method_id=bob_account.auth_method_id,
        new_auth_method_password_algorithm=None,
        new_auth_method_mac_key=SecretKey.generate(),
    )
    assert (
        rep == anonymous_server_cmds.latest.account_recover_proceed.RepAuthMethodIdAlreadyExists()
    )


async def test_anonymous_server_account_recover_proceed_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
    alice_validation_code: ValidationCode,
) -> None:
    async def do():
        await anonymous_server.account_recover_proceed(
            validation_code=alice_validation_code,
            email=ALICE_ACCOUNT_HUMAN_HANDLE.email,
            new_vault_key_access=b"vault_key_access",
            new_auth_method_id=AccountAuthMethodID.new(),
            new_auth_method_password_algorithm=None,
            new_auth_method_mac_key=SecretKey.generate(),
        )

    await anonymous_server_http_common_errors_tester(do)
