# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    HashDigest,
    SecretKey,
    authenticated_account_cmds,
)
from parsec.backend import Backend
from parsec.components.account import UntrustedPasswordAlgorithmArgon2id
from tests.common import AuthenticatedAccountRpcClient, HttpCommonErrorsTester


async def test_authenticated_account_vault_item_recovery_list_ok(
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    rep = await alice_account.vault_item_recovery_list()
    assert rep == authenticated_account_cmds.latest.vault_item_recovery_list.RepOk(
        current_vault=authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault(
            auth_methods=[
                authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethod(
                    auth_method_id=alice_account.auth_method_id,
                    created_on=DateTime(2000, 1, 1),
                    disabled_on=None,
                    created_by_ip="127.0.0.1",
                    created_by_user_agent="Parsec-Client/3.4.0 Linux",
                    vault_key_access=b"<alice_vault_key_access>",
                    password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                        opslimit=65536,
                        memlimit_kb=3,
                        parallelism=1,
                    ),
                )
            ],
            vault_items={},
        ),
        previous_vaults=[],
    )

    # TODO: test with another authentication method added

    # Add items to the vault and create a new vault

    outcome = await backend.account.vault_item_upload(
        auth_method_id=alice_account.auth_method_id,
        item=b"<item-1-enc-1>",
        item_fingerprint=HashDigest.from_data(b"<fingerprint-1>"),
    )
    assert outcome is None

    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()
    new_vault_created_on = DateTime(2000, 1, 31)
    outcome = await backend.account.vault_key_rotation(
        now=new_vault_created_on,
        auth_method_id=alice_account.auth_method_id,
        created_by_ip="",
        created_by_user_agent="dummy-user-agent",
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
            opslimit=42,
            memlimit_kb=4,
            parallelism=2,
        ),
        new_vault_key_access=b"<alice_new_vault_key_access>",
        items={
            HashDigest.from_data(b"<fingerprint-1>"): b"<item-1-enc-2>",
        },
    )
    assert outcome is None

    outcome = await backend.account.vault_item_upload(
        auth_method_id=new_auth_method_id,
        item=b"<item-2-enc-1>",
        item_fingerprint=HashDigest.from_data(b"<fingerprint-2>"),
    )
    assert outcome is None

    old_auth_method_id = alice_account.auth_method_id
    alice_account.auth_method_id = new_auth_method_id
    alice_account.auth_method_mac_key = new_auth_method_mac_key
    rep = await alice_account.vault_item_recovery_list()
    expected = authenticated_account_cmds.latest.vault_item_recovery_list.RepOk(
        current_vault=authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault(
            auth_methods=[
                authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethod(
                    auth_method_id=new_auth_method_id,
                    created_on=DateTime(2000, 1, 31),
                    disabled_on=None,
                    created_by_ip="",
                    created_by_user_agent="dummy-user-agent",
                    vault_key_access=b"<alice_new_vault_key_access>",
                    password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                        opslimit=42,
                        memlimit_kb=4,
                        parallelism=2,
                    ),
                )
            ],
            vault_items={
                HashDigest.from_data(b"<fingerprint-2>"): b"<item-2-enc-1>",
                HashDigest.from_data(b"<fingerprint-1>"): b"<item-1-enc-2>",
            },
        ),
        previous_vaults=[
            authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault(
                auth_methods=[
                    authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethod(
                        auth_method_id=old_auth_method_id,
                        created_on=DateTime(2000, 1, 1),
                        disabled_on=DateTime(2000, 1, 31),
                        created_by_ip="127.0.0.1",
                        created_by_user_agent="Parsec-Client/3.4.0 Linux",
                        vault_key_access=b"<alice_vault_key_access>",
                        password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                            opslimit=65536,
                            memlimit_kb=3,
                            parallelism=1,
                        ),
                    )
                ],
                vault_items={
                    HashDigest.from_data(b"<fingerprint-1>"): b"<item-1-enc-1>",
                },
            ),
        ],
    )
    assert rep == expected


async def test_authenticated_account_vault_item_recovery_list_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.vault_item_recovery_list()

    await authenticated_account_http_common_errors_tester(do)
