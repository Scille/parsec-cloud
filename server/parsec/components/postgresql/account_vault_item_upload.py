# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from asyncpg.exceptions import UniqueViolationError

from parsec._parsec import (
    AccountAuthMethodID,
    HashDigest,
)
from parsec.components.account import (
    AccountVaultItemUploadBadOutcome,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_vault_from_auth_method = Q("""
SELECT vault._id AS vault_internal_id
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    vault_authentication_method.auth_method_id = $auth_method_id
    AND vault_authentication_method.disabled_on IS NULL
    -- Extra safety check since a deleted account should not have any active authentication method
    AND account.deleted_on IS NULL
LIMIT 1
-- All queries modifying the account, its vaults or, its authentication methods must
-- first take a lock on the account row.
-- This is to avoid inconsistent state due to concurrent operations (e.g.
-- deleting an account while creating a new authentication method, leading to
-- a deleted account with a non-disabled authentication method!).
FOR UPDATE
""")

_q_insert_vault_item = Q("""
INSERT INTO vault_item (vault, fingerprint, data)
VALUES ($vault_internal_id, $fingerprint, $data)
""")


async def vault_item_upload(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
    item_fingerprint: HashDigest,
    item: bytes,
) -> None | AccountVaultItemUploadBadOutcome:
    # 1) Find the current vault for this auth method

    row = await conn.fetchrow(*_q_get_vault_from_auth_method(auth_method_id=auth_method_id))
    if not row:
        return AccountVaultItemUploadBadOutcome.ACCOUNT_NOT_FOUND

    match row["vault_internal_id"]:
        case int() as vault_internal_id:
            pass
        case _:
            assert False, row

    # 2) Insert the new item

    try:
        ret = await conn.execute(
            *_q_insert_vault_item(
                vault_internal_id=vault_internal_id,
                fingerprint=item_fingerprint.digest,
                data=item,
            )
        )
        assert ret == "INSERT 0 1", ret

    except UniqueViolationError:
        return AccountVaultItemUploadBadOutcome.FINGERPRINT_ALREADY_EXISTS

    return None
