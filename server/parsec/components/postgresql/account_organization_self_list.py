# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    ActiveUsersLimit,
    DateTime,
    OrganizationID,
    UserID,
    UserProfile,
)
from parsec.components.account import (
    AccountOrganizationListBadOutcome,
    AccountOrganizationSelfList,
    AccountOrganizationSelfListActiveUser,
    AccountOrganizationSelfListRevokedUser,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.config import AccountVaultStrategy, AllowedClientAgent

_q_get_account_email = Q("""
SELECT account.email
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

_q_get_organizations = Q(
    """
SELECT
    user_.user_id,
    user_.created_on AS user_created_on,
    user_.frozen AS user_is_frozen,
    user_.current_profile AS user_current_profile,
    organization.organization_id,
    organization.is_expired AS organization_is_expired,
    organization.user_profile_outsider_allowed AS organization_user_profile_outsider_allowed,
    organization.active_users_limit AS organization_active_users_limit,
    organization.allowed_client_agent AS organization_allowed_client_agent,
    organization.account_vault_strategy AS organization_account_vault_strategy
FROM user_
INNER JOIN human ON user_.human = human._id
INNER JOIN organization ON user_.organization = organization._id
WHERE
    human.email = $email
    AND user_.revoked_on IS NULL
-- There is at most a single user per organization with this email that is not revoked.
-- Note we don't use `LIMIT 1` here, because are looking across all organizations!
"""
)

_q_get_revoked_users = Q("""
SELECT
    organization.organization_id,
    user_.user_id,
    user_.created_on AS user_created_on,
    user_.revoked_on AS user_revoked_on,
    user_.current_profile AS user_current_profile
FROM user_
INNER JOIN human ON user_.human = human._id
INNER JOIN organization ON user_.organization = organization._id
WHERE
    human.email = $email
    AND user_.revoked_on IS NOT NULL
""")


async def organization_self_list(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
) -> AccountOrganizationSelfList | AccountOrganizationListBadOutcome:
    # 1) Get the account email

    row = await conn.fetchrow(*_q_get_account_email(auth_method_id=auth_method_id))
    if not row:
        return AccountOrganizationListBadOutcome.ACCOUNT_NOT_FOUND

    match row["email"]:
        case str() as raw_account_email:
            pass
        case _:
            assert False, row

    # 2) Find all organizations related to this email

    rows = await conn.fetch(*_q_get_organizations(email=raw_account_email))

    active = []
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case _:
                assert False, row

        match row["user_created_on"]:
            case DateTime() as user_created_on:
                pass
            case _:
                assert False, row

        match row["user_is_frozen"]:
            case bool() as user_is_frozen:
                pass
            case _:
                assert False, row

        match row["user_current_profile"]:
            case str() as user_current_profile_raw:
                user_current_profile = UserProfile.from_str(user_current_profile_raw)
                pass
            case _:
                assert False, row

        match row["organization_id"]:
            case str() as raw_organization_id:
                organization_id = OrganizationID(raw_organization_id)
            case _:
                assert False, row

        match row["organization_is_expired"]:
            case bool() as organization_is_expired:
                pass
            case _:
                assert False, row

        match row["organization_user_profile_outsider_allowed"]:
            case bool() as organization_user_profile_outsider_allowed:
                pass
            case _:
                assert False, row

        match row["organization_active_users_limit"]:
            case None as organization_active_users_limit:
                organization_active_users_limit = ActiveUsersLimit.NO_LIMIT
            case int() as organization_active_users_limit:
                organization_active_users_limit = ActiveUsersLimit.limited_to(
                    organization_active_users_limit
                )
            case _:
                assert False, row

        match row["organization_allowed_client_agent"]:
            case str() as organization_allowed_client_agent_raw:
                organization_allowed_client_agent = AllowedClientAgent(
                    organization_allowed_client_agent_raw
                )
            case _:
                assert False, row

        match row["organization_account_vault_strategy"]:
            case str() as organization_account_vault_strategy_raw:
                organization_account_vault_strategy = AccountVaultStrategy(
                    organization_account_vault_strategy_raw
                )
            case _:
                assert False, row

        active.append(
            AccountOrganizationSelfListActiveUser(
                user_id=user_id,
                created_on=user_created_on,
                current_profile=user_current_profile,
                is_frozen=user_is_frozen,
                organization_id=organization_id,
                organization_is_expired=organization_is_expired,
                organization_user_profile_outsider_allowed=organization_user_profile_outsider_allowed,
                organization_active_users_limit=organization_active_users_limit,
                organization_allowed_client_agent=organization_allowed_client_agent,
                organization_account_vault_strategy=organization_account_vault_strategy,
            )
        )

    # 3) Find revoked users across all organizations related to this email

    revoked = []
    rows = await conn.fetch(*_q_get_revoked_users(email=raw_account_email))
    for row in rows:
        match row["organization_id"]:
            case str() as raw_organization_id:
                organization_id = OrganizationID(raw_organization_id)
            case _:
                assert False, row

        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case _:
                assert False, row

        match row["user_created_on"]:
            case DateTime() as user_created_on:
                pass
            case _:
                assert False, row

        match row["user_revoked_on"]:
            case DateTime() as user_revoked_on:
                pass
            case _:
                assert False, row

        match row["user_current_profile"]:
            case str() as user_current_profile_raw:
                user_current_profile = UserProfile.from_str(user_current_profile_raw)
                pass
            case _:
                assert False, row

        revoked.append(
            AccountOrganizationSelfListRevokedUser(
                user_id=user_id,
                created_on=user_created_on,
                revoked_on=user_revoked_on,
                current_profile=user_current_profile,
                organization_id=organization_id,
            )
        )

    return AccountOrganizationSelfList(
        active=active,
        revoked=revoked,
    )
