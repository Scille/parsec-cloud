# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    UserID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_user,
)
from parsec.components.realm import (
    RealmDumpRealmsGrantedRolesBadOutcome,
    RealmGrantedRole,
)

_q_get_org = Q(
    """
SELECT _id as organization_internal_id
FROM organization
WHERE
    organization_id = $organization_id
    -- Only consider bootstrapped organizations
    AND root_verify_key IS NOT NULL
LIMIT 1
"""
)


_q_get_realm_roles = Q(
    f"""
SELECT
    realm.realm_id,
    certificate,
    {q_user(_id="realm_user_role.user_", select="user_id")} AS user_id,
    role,
    {q_device(_id="realm_user_role.certified_by", select="device_id")} AS certified_by,
    certified_on
FROM realm_user_role LEFT JOIN realm ON realm_user_role.realm = realm._id
WHERE realm.organization = $organization_internal_id
ORDER BY realm, certified_on DESC
"""
)


async def realm_dump_realms_granted_roles(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> list[RealmGrantedRole] | RealmDumpRealmsGrantedRolesBadOutcome:
    row = await conn.fetchrow(*_q_get_org(organization_id=organization_id.str))
    assert row is not None

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return RealmDumpRealmsGrantedRolesBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    rows = await conn.fetch(*_q_get_realm_roles(organization_internal_id=organization_internal_id))
    granted_roles = []
    for row in rows:
        match row["realm_id"]:
            case str() as raw_realm_id:
                realm_id = VlobID.from_hex(raw_realm_id)
            case unknown:
                assert False, unknown

        match row["certificate"]:
            case bytes() as certificate:
                pass
            case unknown:
                assert False, unknown

        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case unknown:
                assert False, unknown

        match row["role"]:
            case str() as raw_role:
                role = RealmRole.from_str(raw_role)
            case unknown:
                assert False, unknown

        match row["certified_by"]:
            case str() as raw_certified_by:
                certified_by = DeviceID.from_hex(raw_certified_by)
            case unknown:
                assert False, unknown

        match row["certified_on"]:
            case DateTime() as certified_on:
                pass
            case unknown:
                assert False, unknown

        granted_roles.append(
            RealmGrantedRole(
                realm_id=realm_id,
                certificate=certificate,
                user_id=user_id,
                role=role,
                granted_by=certified_by,
                granted_on=certified_on,
            )
        )

    return granted_roles
