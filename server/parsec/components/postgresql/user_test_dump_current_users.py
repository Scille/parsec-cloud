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
SELECT
    user_.user_id,
    user_.frozen,
    user_.created_on,
    user_.revoked_on,
    user_.initial_profile,
    human.email AS human_email,
    human.label AS human_label
FROM user_
INNER JOIN human ON human._id = user_.human
WHERE
    user_.organization = {q_organization_internal_id("$organization_id")}
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
    device.organization = {q_organization_internal_id("$organization_id")}
ORDER BY device.created_on
"""
)

_q_get_organization_profile_updates = Q(
    f"""
SELECT
    user_.user_id,
    profile.profile,
    profile.certified_on
FROM profile
LEFT JOIN user_ ON user_._id = profile.user_
WHERE
    user_.organization = {q_organization_internal_id("$organization_id")}
ORDER BY profile.certified_on
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
            profile_history=[(row["created_on"], UserProfile.from_str(row["initial_profile"]))],
        )

    rows = await conn.fetch(*_q_get_organization_devices(organization_id=organization_id.str))
    for row in rows:
        user_id = UserID.from_hex(row["user_id"])
        device_id = DeviceID.from_hex(row["device_id"])
        try:
            user = items[user_id]
        except KeyError:
            # A concurrent operation may have created a new user we don't know about
            continue
        user.devices.append(device_id)

    rows = await conn.fetch(
        *_q_get_organization_profile_updates(organization_id=organization_id.str)
    )
    for row in rows:
        user_id = UserID.from_hex(row["user_id"])
        profile = UserProfile.from_str(row["profile"])
        try:
            user = items[user_id]
        except KeyError:
            # A concurrent operation may have created a new user we don't know about
            continue
        user.profile_history.append((row["certified_on"], profile))

    return items
