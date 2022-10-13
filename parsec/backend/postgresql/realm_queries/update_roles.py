# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Optional, Tuple

from parsec.backend.backend_events import BackendEvent
from parsec.api.protocol import OrganizationID, RealmRole, UserProfile
from parsec.backend.realm import (
    RealmGrantedRole,
    RealmAccessError,
    RealmIncompatibleProfileError,
    RealmRoleAlreadyGranted,
    RealmNotFoundError,
    RealmInMaintenanceError,
    RealmRoleRequireGreaterTimestampError,
)
from parsec.backend.user import UserAlreadyRevokedError
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.message import send_message
from parsec.backend.postgresql.utils import (
    Q,
    query,
    q_user,
    q_user_internal_id,
    q_device_internal_id,
    q_realm,
    q_realm_internal_id,
)


_q_get_user_profile = Q(
    q_user(organization_id="$organization_id", user_id="$user_id", select="profile, revoked_on")
)


_q_get_realm_status = Q(
    q_realm(
        organization_id="$organization_id",
        realm_id="$realm_id",
        select="encryption_revision, maintenance_started_by, maintenance_started_on, maintenance_type",
    )
)


_q_get_roles = Q(
    f"""
SELECT
    { q_user_internal_id(organization_id="$organization_id", user_id="needle_user_id") },
    (
        SELECT ROW(role::text, certified_on)
        FROM realm_user_role
        WHERE
            user_ = { q_user_internal_id(organization_id="$organization_id", user_id="needle_user_id") }
            AND realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
        ORDER BY certified_on DESC LIMIT 1
    )
FROM UNNEST($users_ids::VARCHAR[]) AS needle_user_id
"""
)


_q_insert_realm_user_role = Q(
    f"""
INSERT INTO realm_user_role(
    realm,
    user_,
    role,
    certificate,
    certified_by,
    certified_on
) SELECT
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $role,
    $certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$granted_by") },
    $granted_on
"""
)


_q_get_last_vlob_update = Q(
    f"""
SELECT last_vlob_update
FROM realm_user_change
WHERE realm={ q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
AND user_={ q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
"""
)


_q_get_last_role_change = Q(
    f"""
SELECT last_role_change
FROM realm_user_change
WHERE realm={ q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
AND user_={ q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
"""
)


_q_set_last_role_change = Q(
    f"""
INSERT INTO realm_user_change(realm, user_, last_role_change, last_vlob_update)
VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $granted_on,
    NULL
)
ON CONFLICT (realm, user_)
DO UPDATE SET last_role_change = (
    SELECT GREATEST($granted_on, last_role_change)
    FROM realm_user_change
    WHERE realm={ q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    AND user_={ q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
    LIMIT 1
)
"""
)


@query(in_transaction=True)
async def query_update_roles(
    conn,
    organization_id: OrganizationID,
    new_role: RealmGrantedRole,
    recipient_message: Optional[bytes],
) -> None:
    assert new_role.granted_by is not None
    if new_role.granted_by.user_id == new_role.user_id:
        raise RealmAccessError("Cannot modify our own role")

    # Make sure user profile is compatible
    rep = await conn.fetchrow(
        *_q_get_user_profile(organization_id=organization_id.str, user_id=new_role.user_id.str)
    )
    if not rep:
        raise RealmNotFoundError(f"User `{new_role.user_id.str}` doesn't exist")
    if rep["profile"] == UserProfile.OUTSIDER.value and new_role.role in (
        RealmRole.MANAGER,
        RealmRole.OWNER,
    ):
        raise RealmIncompatibleProfileError("User with OUTSIDER profile cannot be MANAGER or OWNER")

    # Make the user is not revoked
    if rep["revoked_on"]:
        raise UserAlreadyRevokedError(f"User `{new_role.user_id.str}` is revoked")

    # Retrieve realm and make sure it is not under maintenance
    rep = await conn.fetchrow(
        *_q_get_realm_status(organization_id=organization_id.str, realm_id=new_role.realm_id.uuid)
    )
    if not rep:
        raise RealmNotFoundError(f"Realm `{new_role.realm_id.str}` doesn't exist")
    if rep["maintenance_type"]:
        raise RealmInMaintenanceError("Data realm is currently under maintenance")

    # Check access rights and user existance
    ((author_id, author_role), (user_id, existing_user_role)) = await conn.fetch(
        *_q_get_roles(
            organization_id=organization_id.str,
            realm_id=new_role.realm_id.uuid,
            users_ids=(new_role.granted_by.user_id.str, new_role.user_id.str),
        )
    )
    assert author_id
    assert user_id

    if author_role is not None:
        author_role, _ = author_role
        if author_role is not None:
            author_role = RealmRole.from_str(author_role)

    last_role_granted_on = None
    if existing_user_role is not None:
        existing_user_role, last_role_granted_on = existing_user_role
        if existing_user_role is not None:
            existing_user_role = RealmRole.from_str(existing_user_role)

    owner_only = (RealmRole.OWNER,)
    owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)

    needed_roles: Tuple[RealmRole, ...]
    if existing_user_role in owner_or_manager or new_role.role in owner_or_manager:
        needed_roles = owner_only
    else:
        needed_roles = owner_or_manager

    if author_role not in needed_roles:
        raise RealmAccessError()

    if existing_user_role == new_role.role:
        raise RealmRoleAlreadyGranted()

    # Timestamps for the role certificates of a given user should be striclty increasing
    if last_role_granted_on is not None and last_role_granted_on >= new_role.granted_on:
        raise RealmRoleRequireGreaterTimestampError(last_role_granted_on)

    # Perfrom extra checks when removing write rights
    if new_role.role in (RealmRole.READER, None):

        # The change of role needs to occur strictly after the last upload for this user
        rep = await conn.fetchrow(
            *_q_get_last_vlob_update(
                organization_id=organization_id.str,
                realm_id=new_role.realm_id.uuid,
                user_id=new_role.user_id.str,
            )
        )
        realm_last_vlob_update = None if not rep else rep[0]
        if realm_last_vlob_update is not None and realm_last_vlob_update >= new_role.granted_on:
            raise RealmRoleRequireGreaterTimestampError(realm_last_vlob_update)

    # Perform extra checks when removing management rights
    if new_role.role in (RealmRole.CONTRIBUTOR, RealmRole.READER, None):

        # The change of role needs to occur strictly after the last change of role performed by this user
        rep = await conn.fetchrow(
            *_q_get_last_role_change(
                organization_id=organization_id.str,
                realm_id=new_role.realm_id.uuid,
                user_id=new_role.user_id.str,
            )
        )
        realm_last_role_change = None if not rep else rep[0]
        if realm_last_role_change is not None and realm_last_role_change >= new_role.granted_on:
            raise RealmRoleRequireGreaterTimestampError(realm_last_role_change)

    await conn.execute(
        *_q_insert_realm_user_role(
            organization_id=organization_id.str,
            realm_id=new_role.realm_id.uuid,
            user_id=new_role.user_id.str,
            role=new_role.role.str if new_role.role else None,
            certificate=new_role.certificate,
            granted_by=new_role.granted_by.str,
            granted_on=new_role.granted_on,
        )
    )

    await conn.execute(
        *_q_set_last_role_change(
            organization_id=organization_id.str,
            realm_id=new_role.realm_id.uuid,
            user_id=new_role.granted_by.user_id.str,
            granted_on=new_role.granted_on,
        )
    )

    await send_signal(
        conn,
        BackendEvent.REALM_ROLES_UPDATED,
        organization_id=organization_id,
        author=new_role.granted_by,
        realm_id=new_role.realm_id,
        user=new_role.user_id,
        role=new_role.role,
    )

    if recipient_message:
        await send_message(
            conn,
            organization_id,
            new_role.granted_by,
            new_role.user_id,
            new_role.granted_on,
            recipient_message,
        )
