# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import HashDigest, authenticated_account_cmds
from tests.common import AuthenticatedAccountRpcClient, Backend, HttpCommonErrorsTester


async def test_authenticated_account_vault_item_upload_ok(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    item = b"<item>"
    item_fingerprint = HashDigest.from_data(item)
    rep = await alice_account.vault_item_upload(item=item, item_fingerprint=item_fingerprint)
    assert rep == authenticated_account_cmds.latest.vault_item_upload.RepOk()


async def test_authenticated_account_vault_item_upload_fingerprint_already_exists(
    xfail_if_postgresql: None,
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
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    item = b"<item>"
    item_fingerprint = HashDigest.from_data(item)

    async def do():
        await alice_account.vault_item_upload(item=item, item_fingerprint=item_fingerprint)

    await authenticated_account_http_common_errors_tester(do)
