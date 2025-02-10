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
    q_realm,
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
SELECT DISTINCT ON (realm)
    {q_realm(_id="realm_user_role.realm", select="realm_id")} AS realm_id,
    role,
    certified_on
FROM realm_user_role
WHERE user_ = $user_internal_id
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

        match row["realm_role_certificate"]:
            case bytes() as realm_role_certificate:
                pass
            case unknown:
                assert False, unknown

        match row["user_id"]:
            case bytes() as raw_user_id:
                user_id = UserID.from_bytes(raw_user_id)
            case unknown:
                assert False, unknown

        match row["role"]:
            case str() as raw_role:
                role = RealmRole.from_str(raw_role)
            case unknown:
                assert False, unknown

        match row["granted_by"]:
            case bytes() as raw_granted_by:
                granted_by = DeviceID.from_bytes(raw_granted_by)
            case unknown:
                assert False, unknown

        match row["granted_on"]:
            case DateTime() as granted_on:
                pass
            case unknown:
                assert False, unknown

        granted_roles.append(
            RealmGrantedRole(
                realm_id=realm_id,
                certificate=realm_role_certificate,
                user_id=user_id,
                role=role,
                granted_by=granted_by,
                granted_on=granted_on,
            )
        )

    return granted_roles
