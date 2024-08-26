# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DeviceID,
    HumanHandle,
    OrganizationID,
    UserID,
    UserProfile,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_organization_internal_id,
)
from parsec.components.user import (
    UserDump,
)

_q_get_organization_users = Q(
    f"""
SELECT DISTINCT ON(user_._id)
    user_.user_id,
    user_.frozen,
    user_.created_on,
    user_.revoked_on,
    COALESCE(profile.profile, user_.initial_profile) AS current_profile,
    human.email AS human_email,
    human.label AS human_label
FROM user_
LEFT JOIN profile ON user_._id = profile.user_
INNER JOIN human ON human._id = user_.human
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
ORDER BY user_._id, profile.certified_on DESC
"""
)

_q_get_organization_devices = Q(
    f"""
SELECT
    user_.user_id,
    device.device_id,
    device.created_on
FROM device
INNER JOIN user_ ON user_._id = device.user_
WHERE
    device.organization = { q_organization_internal_id("$organization_id") }
"""
)


async def user_test_dump_current_users(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> dict[UserID, UserDump]:
    rows = await conn.fetch(*_q_get_organization_users(organization_id=organization_id.str))
    items: dict[UserID, UserDump] = {}
    for row in rows:
        user_id = UserID.from_hex(row["user_id"])
        human_handle = HumanHandle(email=row["human_email"], label=row["human_label"])
        items[user_id] = UserDump(
            user_id=user_id,
            human_handle=human_handle,
            created_on=row["created_on"],
            revoked_on=row["revoked_on"],
            devices=[],
            current_profile=UserProfile.from_str(row["current_profile"]),
        )
    rows = await conn.fetch(*_q_get_organization_devices(organization_id=organization_id.str))
    for row in sorted(rows, key=lambda row: row["created_on"]):
        user_id = UserID.from_hex(row["user_id"])
        device_id = DeviceID.from_hex(row["device_id"])
        items[user_id].devices.append(device_id)
    return items
