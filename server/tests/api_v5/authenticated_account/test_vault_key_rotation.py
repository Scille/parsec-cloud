# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    HashDigest,
    SecretKey,
    authenticated_account_cmds,
)
from parsec.backend import Backend
from parsec.components.account import PasswordAlgorithmArgon2id, VaultItems
from tests.common import AuthenticatedAccountRpcClient, HttpCommonErrorsTester


async def test_authenticated_account_vault_key_rotation_ok(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    outcome = await backend.account.vault_item_upload(
        auth_method_id=alice_account.auth_method_id,
        item=b"<item-1-enc-1>",
        item_fingerprint=HashDigest.from_data(b"<fingerprint-1>"),
    )
    assert outcome is None

    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()
    new_auth_method_key_access = b"<vault_key_access>"
    new_auth_method_items = {
        HashDigest.from_data(b"<fingerprint-1>"): b"<item-1-enc-2>",
    }

    rep = await alice_account.vault_key_rotation(
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"<new salt>",
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
        ),
        new_vault_key_access=new_auth_method_key_access,
        items=new_auth_method_items,
    )
    assert rep == authenticated_account_cmds.latest.vault_key_rotation.RepOk()

    outcome = await backend.account.vault_item_list(auth_method_id=new_auth_method_id)
    assert isinstance(outcome, VaultItems)
    assert outcome.key_access == new_auth_method_key_access
    assert outcome.items == new_auth_method_items


@pytest.mark.parametrize("kind", ("too-few", "too-many", "different"))
async def test_authenticated_account_vault_key_rotation_items_mismatch(
    xfail_if_postgresql: None,
    kind: str,
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    outcome = await backend.account.vault_item_upload(
        auth_method_id=alice_account.auth_method_id,
        item=b"<item-1-enc-1>",
        item_fingerprint=HashDigest.from_data(b"<fingerprint-1>"),
    )
    assert outcome is None

    match kind:
        case "too-few":
            bad_items = {}

        case "too-many":
            bad_items = {
                HashDigest.from_data(b"<fingerprint-1>"): b"<item-1-enc-2>",
                HashDigest.from_data(b"<fingerprint-2>"): b"<dummy>",
            }

        case "different":
            bad_items = {
                HashDigest.from_data(b"<fingerprint-2>"): b"<dummy>",
            }

        case unknown:
            assert False, unknown

    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()

    rep = await alice_account.vault_key_rotation(
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"<new salt>",
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
        ),
        new_vault_key_access=b"<vault_key_access>",
        items=bad_items,
    )
    assert rep == authenticated_account_cmds.latest.vault_key_rotation.RepItemsMismatch()


@pytest.mark.parametrize(
    "kind", ("reuse_from_current_vault", "reuse_from_old_vault", "reuse_from_other_account")
)
async def test_authenticated_account_vault_key_rotation_new_auth_method_id_already_exists(
    xfail_if_postgresql: None,
    kind: str,
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    match kind:
        case "reuse_from_current_vault":
            bad_new_auth_method_id = alice_account.auth_method_id

        case "reuse_from_old_vault":
            bad_new_auth_method_id = alice_account.auth_method_id
            new_auth_method_id = AccountAuthMethodID.new()
            new_key = SecretKey.generate()
            outcome = await backend.account.vault_key_rotation(
                now=DateTime.now(),
                auth_method_id=alice_account.auth_method_id,
                created_by_ip="",
                created_by_user_agent="test-user-agent",
                new_auth_method_id=new_auth_method_id,
                new_auth_method_mac_key=new_key,
                new_auth_method_password_algorithm=PasswordAlgorithmArgon2id(
                    salt=b"<new salt>",
                    opslimit=65536,
                    memlimit_kb=3,
                    parallelism=1,
                ),
                new_vault_key_access=b"<vault_key_access>",
                items={},
            )
            assert outcome is None
            alice_account.auth_method_id = new_auth_method_id
            alice_account.auth_method_mac_key = new_key

        case "reuse_from_other_account":
            bad_new_auth_method_id = bob_account.auth_method_id

        case unknown:
            assert False, unknown

    new_auth_method_mac_key = SecretKey.generate()

    rep = await alice_account.vault_key_rotation(
        new_auth_method_id=bad_new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=PasswordAlgorithmArgon2id(
            salt=b"<new salt>",
            opslimit=65536,
            memlimit_kb=3,
            parallelism=1,
        ),
        new_vault_key_access=b"<vault_key_access>",
        items={},
    )
    assert (
        rep
        == authenticated_account_cmds.latest.vault_key_rotation.RepNewAuthMethodIdAlreadyExists()
    )


async def test_authenticated_account_vault_key_rotation_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()

    async def do():
        await alice_account.vault_key_rotation(
            new_auth_method_id=new_auth_method_id,
            new_auth_method_mac_key=new_auth_method_mac_key,
            new_auth_method_password_algorithm=PasswordAlgorithmArgon2id(
                salt=b"<new salt>",
                opslimit=65536,
                memlimit_kb=3,
                parallelism=1,
            ),
            new_vault_key_access=b"<vault_key_access>",
            items={},
        )

    await authenticated_account_http_common_errors_tester(do)
