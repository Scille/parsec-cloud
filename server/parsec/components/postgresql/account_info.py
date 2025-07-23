# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import AccountAuthMethodID, EmailAddress, HumanHandle
from parsec.components.account import AccountInfo, AccountInfoBadOutcome
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_account_info = Q(
    """
SELECT
    account.email,
    account.human_handle_label
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    vault_authentication_method.auth_method_id = $auth_method_id
    AND vault_authentication_method.disabled_on IS NULL
    -- Extra safety check since a deleted account should not have any active authentication method
    AND account.deleted_on IS NULL
LIMIT 1
"""
)


async def account_info(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
) -> AccountInfo | AccountInfoBadOutcome:
    row = await conn.fetchrow(*_q_get_account_info(auth_method_id=auth_method_id))
    if not row:
        return AccountInfoBadOutcome.ACCOUNT_NOT_FOUND

    match row["email"]:
        case str() as raw_email:
            email = EmailAddress(raw_email)
        case _:
            assert False, row

    match row["human_handle_label"]:
        case str() as label:
            pass
        case _:
            assert False, row

    human_handle = HumanHandle(email=email, label=label)

    return AccountInfo(human_handle=human_handle)
