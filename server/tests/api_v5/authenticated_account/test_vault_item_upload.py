# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    HashDigest,
    SecretKey,
    ValidationCode,
    authenticated_account_cmds,
)
from tests.common import AuthenticatedAccountRpcClient, Backend, HttpCommonErrorsTester


async def test_authenticated_account_vault_item_upload_ok(
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    item = b"<item>"
    item_fingerprint = HashDigest.from_data(item)

    # Fingerprint is allowed to have been uploaded in a vault from a different account...
    outcome = await backend.account.vault_item_upload(
        auth_method_id=bob_account.auth_method_id, item=item, item_fingerprint=item_fingerprint
    )
    assert outcome is None

    # ...or even from a previous vault from our account!
    outcome = await backend.account.vault_item_upload(
        auth_method_id=alice_account.auth_method_id, item=item, item_fingerprint=item_fingerprint
    )
    assert outcome is None
    alice_recover_validation_code = await backend.account.recover_send_validation_email(
        now=DateTime.now(), email=alice_account.account_email
    )
    assert isinstance(alice_recover_validation_code, ValidationCode)
    new_auth_method_id = AccountAuthMethodID.from_hex("a11a285306c546bf89e2d59c8d7deafa")
    new_auth_method_mac_key = SecretKey.generate()
    await backend.account.recover_proceed(
        now=DateTime.now(),
        validation_code=alice_recover_validation_code,
        email=alice_account.account_email,
        created_by_user_agent="",
        created_by_ip="",
        new_vault_key_access=b"<new_vault_key_access>",
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=None,
    )

    alice_account.auth_method_id = new_auth_method_id
    alice_account.auth_method_mac_key = new_auth_method_mac_key
    rep = await alice_account.vault_item_upload(item=item, item_fingerprint=item_fingerprint)
    assert rep == authenticated_account_cmds.latest.vault_item_upload.RepOk()


async def test_authenticated_account_vault_item_upload_fingerprint_already_exists(
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    item = b"<item>"
    item_fingerprint = HashDigest.from_data(item)

    ret = await backend.account.vault_item_upload(
        auth_method_id=alice_account.auth_method_id, item=item, item_fingerprint=item_fingerprint
    )
    assert ret is None  # Sanity check

    rep = await alice_account.vault_item_upload(item=item, item_fingerprint=item_fingerprint)
    assert rep == authenticated_account_cmds.latest.vault_item_upload.RepFingerprintAlreadyExists()


async def test_authenticated_account_vault_item_upload_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    item = b"<item>"
    item_fingerprint = HashDigest.from_data(item)

    async def do():
        await alice_account.vault_item_upload(item=item, item_fingerprint=item_fingerprint)

    await authenticated_account_http_common_errors_tester(do)
