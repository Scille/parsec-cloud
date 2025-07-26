# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    HashDigest,
    UntrustedPasswordAlgorithmArgon2id,
)
from parsec.components.account import (
    AccountVaultItemRecoveryList,
    AuthMethod,
    VaultItemRecoveryList,
    VaultItemRecoveryVault,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_account = Q("""
SELECT account._id AS account_internal_id
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

_q_get_all_vaults_for_account = Q("""
SELECT vault._id AS vault_internal_id
FROM vault
WHERE vault.account = $account_internal_id
""")

_q_get_all_auth_methods_for_account = Q("""
SELECT
    vault AS vault_internal_id,
    auth_method_id,
    created_on,
    created_by_ip,
    created_by_user_agent,
    vault_key_access,
    password_algorithm,
    password_algorithm_argon2id_opslimit,
    password_algorithm_argon2id_memlimit_kb,
    password_algorithm_argon2id_parallelism,
    disabled_on
FROM vault_authentication_method
WHERE vault = ANY($vault_internal_ids::INTEGER [])
""")

_q_get_all_vault_items_for_account = Q("""
SELECT
    vault AS vault_internal_id,
    fingerprint,
    data
FROM vault_item
WHERE vault = ANY($vault_internal_ids::INTEGER [])
""")


async def vault_item_recovery_list(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
) -> VaultItemRecoveryList | AccountVaultItemRecoveryList:
    # 1) Retrieve the account

    row = await conn.fetchrow(*_q_get_account(auth_method_id=auth_method_id))
    if not row:
        return AccountVaultItemRecoveryList.ACCOUNT_NOT_FOUND

    match row["account_internal_id"]:
        case int() as account_internal_id:
            pass
        case _:
            assert False, row

    # 2) Get all vaults for this account (current and previous)

    vault_rows = await conn.fetch(
        *_q_get_all_vaults_for_account(account_internal_id=account_internal_id)
    )
    # An account must have at least one vault, and we have in fact already accessed
    # the account's current vault in the previous step (since the auth method is
    # related to the vault).
    assert len(vault_rows) > 0

    vault_internal_ids = []
    for row in vault_rows:
        match row["vault_internal_id"]:
            case int() as vault_internal_id:
                vault_internal_ids.append(vault_internal_id)
            case _:
                assert False, row

    # 3) Fetch all auth methods for the vaults

    auth_method_rows = await conn.fetch(
        *_q_get_all_auth_methods_for_account(vault_internal_ids=vault_internal_ids)
    )
    auth_methods = []
    for row in auth_method_rows:
        match row["vault_internal_id"]:
            case int() as vault_internal_id:
                pass
            case _:
                assert False, row

        match row["auth_method_id"]:
            case str() as raw_auth_method_id:
                auth_method_id = AccountAuthMethodID.from_hex(raw_auth_method_id)
            case _:
                assert False, row

        match row["created_on"]:
            case DateTime() as created_on:
                pass
            case _:
                assert False, row

        match row["created_by_ip"]:
            case str() as created_by_ip:
                pass
            case _:
                assert False, row

        match row["created_by_user_agent"]:
            case str() as created_by_user_agent:
                pass
            case _:
                assert False, row

        match row["vault_key_access"]:
            case bytes() as vault_key_access:
                pass
            case _:
                assert False, row

        match row["disabled_on"]:
            case DateTime() | None as disabled_on:
                pass
            case _:
                assert False, row

        match row["password_algorithm"]:
            case None:
                password_algorithm = None

            case "ARGON2ID":
                match row["password_algorithm_argon2id_opslimit"]:
                    case int() as algorithm_argon2id_opslimit:
                        pass
                    case _:
                        assert False, row

                match row["password_algorithm_argon2id_memlimit_kb"]:
                    case int() as algorithm_argon2id_memlimit_kb:
                        pass
                    case _:
                        assert False, row

                match row["password_algorithm_argon2id_parallelism"]:
                    case int() as algorithm_argon2id_parallelism:
                        pass
                    case _:
                        assert False, row

                password_algorithm = UntrustedPasswordAlgorithmArgon2id(
                    opslimit=algorithm_argon2id_opslimit,
                    memlimit_kb=algorithm_argon2id_memlimit_kb,
                    parallelism=algorithm_argon2id_parallelism,
                )

            case _:
                assert False, row

        auth_method = AuthMethod(
            auth_method_id=auth_method_id,
            created_on=created_on,
            created_by_ip=created_by_ip,
            created_by_user_agent=created_by_user_agent,
            vault_key_access=vault_key_access,
            password_algorithm=password_algorithm,
            disabled_on=disabled_on,
        )

        auth_methods.append((vault_internal_id, auth_method))

    # 4) Fetch all vault items for the vaults

    vault_item_rows = await conn.fetch(
        *_q_get_all_vault_items_for_account(vault_internal_ids=vault_internal_ids)
    )
    vault_items = []
    for row in vault_item_rows:
        match row["vault_internal_id"]:
            case int() as vault_internal_id:
                pass
            case _:
                assert False, row

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

        vault_items.append((vault_internal_id, fingerprint, data))

    # 5) Merge everything into VaultItemRecoveryList

    vaults = {
        vault_internal_id: VaultItemRecoveryVault(
            auth_methods=[],
            vault_items={},
        )
        for vault_internal_id in vault_internal_ids
    }
    for vault_internal_id, auth_method in auth_methods:
        vaults[vault_internal_id].auth_methods.append(auth_method)
    for vault_internal_id, fingerprint, data in vault_items:
        vaults[vault_internal_id].vault_items[fingerprint] = data

    # 6) Finally order by vault creation date and return

    vaults_ordered = sorted(
        vaults.values(),
        # Note a vault is guaranteed to have at least one auth method
        key=lambda vault: vault.auth_methods[0].created_on,
    )

    return VaultItemRecoveryList(
        current_vault=vaults_ordered[-1],
        previous_vaults=vaults_ordered[:-1],
    )
