# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import HashDigest, authenticated_account_cmds
from parsec.backend import Backend
from tests.common import AuthenticatedAccountRpcClient, HttpCommonErrorsTester


async def test_authenticated_account_vault_item_list_ok(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    rep = await alice_account.vault_item_list()
    assert rep == authenticated_account_cmds.latest.vault_item_list.RepOk(
        items={},
        key_access=b"<alice_vault_key_access>",
    )

    expected_items = {}
    for i in range(3):
        item = f"item-{i}".encode()
        item_fingerprint = HashDigest.from_data(f"fingerprint-{i}".encode())
        outcome = await backend.account.vault_item_upload(
            auth_method_id=alice_account.auth_method_id,
            item_fingerprint=item_fingerprint,
            item=item,
        )
        assert outcome is None
        expected_items[item_fingerprint] = item

    rep = await alice_account.vault_item_list()
    assert rep == authenticated_account_cmds.latest.vault_item_list.RepOk(
        items=expected_items,
        key_access=b"<alice_vault_key_access>",
    )


async def test_authenticated_account_vault_item_list_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.vault_item_list()

    await authenticated_account_http_common_errors_tester(do)
