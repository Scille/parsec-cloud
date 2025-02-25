# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, OrganizationID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.user import UserInvitationInfo

_q_list_user_invitations = Q(
    """
SELECT
    invitation.organization AS organization_id,
    human.email AS created_by,
    invitation.created_on AS created_on
FROM invitation
INNER JOIN device ON device._id = invitation.created_by_device
INNER JOIN user_ ON user_._id = device.user_
INNER JOIN human ON human._id = user.human
WHERE
    invitation.deleted_on is NULL
    AND invitation.user_invitation_claimer_email = $email
    AND invitation.type = 'USER'
"""
)


async def list_user_invitations(conn: AsyncpgConnection, email: str) -> list[UserInvitationInfo]:
    rows = await conn.fetch(*_q_list_user_invitations(email=email))
    items = []
    for row in rows:
        match row["organization_id"]:
            case str() as raw_organization_id:
                organization_id = OrganizationID(raw_organization_id)
            case unknown:
                assert False, unknown

        match row["created_by"]:
            case str() as created_by:
                pass
            case unknown:
                assert False, unknown

        match row["created_on"]:
            case DateTime() as created_on:
                pass
            case unknown:
                assert False, unknown

        items.append(
            UserInvitationInfo(
                organization=organization_id,
                created_by=created_by,
                created_on=created_on,
            )
        )
    return [UserInvitationInfo(**row) for row in rows]
