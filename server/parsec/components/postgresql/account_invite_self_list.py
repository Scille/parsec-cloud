# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    InvitationToken,
    InvitationType,
    OrganizationID,
)
from parsec.components.account import AccountInviteListBadOutcome
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

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


_q_get_invitations = Q("""
-- First retrieve the list of users across all organizations that have the given email
WITH active_users_with_this_email AS (
    SELECT user_._id AS user_internal_id
    FROM user_
    INNER JOIN human ON user_.human = human._id
    WHERE
        human.email = $email
        AND user_.revoked_on IS NULL
-- There is at most a single user per organization with this email that is not revoked.
-- Note we don't use `LIMIT 1` here, because are looking across all organizations!
)

SELECT
    organization.organization_id,
    invitation.token,
    invitation.type
FROM invitation
LEFT JOIN organization ON invitation.organization = organization._id
LEFT JOIN shamir_recovery_setup ON invitation.shamir_recovery = shamir_recovery_setup._id
WHERE
    (
        invitation.user_invitation_claimer_email = $email
        OR COALESCE(
            (
                SELECT TRUE
                FROM active_users_with_this_email
                WHERE
                    active_users_with_this_email.user_internal_id = invitation.device_invitation_claimer
                    OR active_users_with_this_email.user_internal_id = shamir_recovery_setup.user_
                LIMIT 1
            ),
            FALSE
        )
    )
    AND invitation.deleted_on IS NULL
""")


async def invite_self_list(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
) -> list[tuple[OrganizationID, InvitationToken, InvitationType]] | AccountInviteListBadOutcome:
    # 1) Get the account email

    row = await conn.fetchrow(*_q_get_account_email(auth_method_id=auth_method_id))
    if not row:
        return AccountInviteListBadOutcome.ACCOUNT_NOT_FOUND

    match row["email"]:
        case str() as raw_account_email:
            pass
        case _:
            assert False, row

    # 2) Find invitations related to this email across all organizations

    rows = await conn.fetch(*_q_get_invitations(email=raw_account_email))

    result = []
    for row in rows:
        match row["organization_id"]:
            case str() as raw_organization_id:
                organization_id = OrganizationID(raw_organization_id)
            case _:
                assert False, row

        match row["token"]:
            case str() as raw_invitation_token:
                invitation_token = InvitationToken.from_hex(raw_invitation_token)
            case _:
                assert False, row

        match row["type"]:
            case str() as raw_invitation_type:
                invitation_type = InvitationType.from_str(raw_invitation_type)
            case _:
                assert False, row

        result.append((organization_id, invitation_token, invitation_type))

    return result
