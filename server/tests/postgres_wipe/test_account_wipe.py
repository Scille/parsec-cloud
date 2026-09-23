# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import itertools
from collections.abc import Iterable

import pytest
from asyncpg import Record
from asyncpg.pool import PoolConnectionProxy

from parsec._parsec import AccountAuthMethodID, DateTime, ValidationCode
from parsec.backend import Backend
from parsec.components.postgresql.account import PGAccountComponent
from tests.common.client import AuthenticatedAccountRpcClient

type Conn = PoolConnectionProxy[Record]


@pytest.mark.postgresql
async def test_account_cascade_wipe(alice_account: AuthenticatedAccountRpcClient, backend: Backend):
    account = backend.account
    now = DateTime.now()

    # Alice start the process to delete its account so `account_delete_validation_code` is populated
    # NOTE: We do not proceed to complete the deletion as it would remove the entry in the table
    code = await account.delete_send_validation_email(now, alice_account.auth_method_id)
    assert isinstance(code, ValidationCode)

    # Alice start the process to delete its account so `account_recover_validation_code` is populated
    code = await account.recover_send_validation_email(now, alice_account.account_email)
    assert isinstance(code, ValidationCode)

    assert isinstance(account, PGAccountComponent)

    # 1. Acquire related info about the account
    async with account.pool.acquire() as conn:
        account_id = await fetch_account_id_from_auth_method_id(conn, alice_account.auth_method_id)

        vault_ids = await fetch_many_row_ids_from_references(
            conn, "vault", "account", (account_id,)
        )
        assert vault_ids

        vault_item_ids = await fetch_many_row_ids_from_references(
            conn, "vault_item", "vault", vault_ids
        )
        # TODO: Alice does not have vault items for now
        assert not vault_item_ids

        vault_authentication_method_ids = await fetch_many_row_ids_from_references(
            conn, "vault_authentication_method", "vault", vault_ids
        )
        assert vault_authentication_method_ids

        for table in ("account_delete_validation_code", "account_recover_validation_code"):
            row_id = await conn.fetchval(
                f"SELECT account FROM {table} WHERE account = $1", account_id
            )
            assert row_id is not None, (
                f"Table {table} is missing entry linked to account#{account_id}"
            )

    # 2. Wipe the account
    async with account.pool.acquire() as conn:
        await conn.execute("DELETE FROM account WHERE _id = $1", account_id)

    # 3. Verify that the wipe has cascaded
    async with account.pool.acquire() as conn:
        for table, id in itertools.chain(
            (("account", account_id),),
            (("vault", id) for id in vault_ids),
            (("vault_item", id) for id in vault_item_ids),
            (("vault_authentication_method", id) for id in vault_authentication_method_ids),
        ):
            row_id = await conn.fetchval(f"SELECT _id FROM {table} WHERE _id = $1", id)
            assert row_id is None, f"Table {table} still has a row linked to the deleted account"

        for table in ("account_delete_validation_code", "account_recover_validation_code"):
            row_id = await conn.fetchval(
                f"SELECT account FROM {table} WHERE account = $1", account_id
            )
            assert row_id is None, f"Table {table} still has a row linked to the deleted account"


async def fetch_account_id_from_auth_method_id(conn: Conn, method_id: AccountAuthMethodID) -> int:
    vault_id = await conn.fetchval(
        "SELECT vault FROM vault_authentication_method WHERE auth_method_id = $1", method_id
    )
    account_id = await conn.fetchval("SELECT account FROM vault WHERE _id = $1", vault_id)
    assert isinstance(account_id, int)
    return account_id


async def fetch_many_row_ids_from_references(
    conn: Conn, table: str, fkey_col_name: str, references: Iterable[int]
) -> list[int]:
    rows = await conn.fetchmany(
        f"SELECT _id as id FROM {table} WHERE {fkey_col_name} = $1",
        ((id,) for id in references),
    )
    return [int(row["id"]) for row in rows]
