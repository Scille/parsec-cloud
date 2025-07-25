# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import AccountAuthMethodID, HashDigest
from parsec.components.account import AccountVaultItemListBadOutcome, VaultItems
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_vault_key_access = Q("""
SELECT
    vault._id AS vault_internal_id,
    vault_authentication_method.vault_key_access
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    vault_authentication_method.auth_method_id = $auth_method_id
    AND vault_authentication_method.disabled_on IS NULL
    -- Extra safety check since a deleted account should not have any active authentication method
    AND account.deleted_on IS NULL
LIMIT 1
""")

_q_get_vault_items = Q("""
SELECT
    fingerprint,
    data
FROM vault_item
WHERE vault = $vault_internal_id
""")


async def vault_item_list(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
) -> VaultItems | AccountVaultItemListBadOutcome:
    # 1) Get the vault key access for this auth method

    row = await conn.fetchrow(*_q_get_vault_key_access(auth_method_id=auth_method_id))
    if not row:
        return AccountVaultItemListBadOutcome.ACCOUNT_NOT_FOUND

    match row["vault_internal_id"]:
        case int() as vault_internal_id:
            pass
        case _:
            assert False, row

    match row["vault_key_access"]:
        case bytes() as vault_key_access:
            pass
        case _:
            assert False, row

    # 2) Get all items for this vault

    rows = await conn.fetch(*_q_get_vault_items(vault_internal_id=vault_internal_id))

    items = {}
    for row in rows:
        match row["fingerprint"]:
            case bytes() as raw_fingerprint:
                fingerprint = HashDigest(raw_fingerprint)
            case _:
                assert False, row

        match row["data"]:
            case bytes() as data:
                pass
            case _:
                assert False, row

        items[fingerprint] = data

    return VaultItems(
        key_access=vault_key_access,
        items=items,
    )
