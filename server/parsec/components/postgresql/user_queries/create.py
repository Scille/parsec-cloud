# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import itertools

import asyncpg
from asyncpg import UniqueViolationError

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    OrganizationID,
    UserCertificate,
)
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_human_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
    query,
)
from parsec.components.user import (
    UserCreateDeviceStoreBadOutcome,
    UserCreateUserStoreBadOutcome,
)

_q_check_active_users_limit = Q(
    """
    SELECT
        (
            organization.active_users_limit is NULL
            OR (
                SELECT
                    count(*)
                FROM
                    user_
                WHERE
                    user_.organization = organization._id AND
                    user_.revoked_on IS NULL
            ) < organization.active_users_limit
        ) as allowed
    FROM
        organization
    WHERE
        organization.organization_id = $organization_id
"""
)


_q_get_user_devices = Q(
    f"""
SELECT device_id
FROM device
WHERE user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
"""
)


_q_get_not_revoked_users_for_human = Q(
    f"""
SELECT user_id
FROM user_
WHERE
    human = { q_human_internal_id(organization_id="$organization_id", email="$email") }
    AND (
        revoked_on IS NULL
        OR revoked_on > $now
    )
"""
)


_q_insert_human_if_not_exists = Q(
    f"""
INSERT INTO human (organization, email, label)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $email,
    $label
)
ON CONFLICT DO NOTHING
"""
)


_q_insert_user = Q(
    f"""
INSERT INTO user_ (
    organization,
    user_id,
    profile,
    user_certificate,
    redacted_user_certificate,
    user_certifier,
    created_on
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $user_id,
    $profile,
    $user_certificate,
    $redacted_user_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$user_certifier") },
    $created_on
)
"""
)


_q_insert_user_with_human_handle = Q(
    f"""
INSERT INTO user_ (
    organization,
    user_id,
    profile,
    user_certificate,
    redacted_user_certificate,
    user_certifier,
    created_on,
    human
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $user_id,
    $profile,
    $user_certificate,
    $redacted_user_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$user_certifier") },
    $created_on,
    { q_human_internal_id(organization_id="$organization_id", email="$email") }
)
"""
)


_q_insert_device = Q(
    f"""
INSERT INTO device (
    organization,
    user_,
    device_id,
    device_label,
    device_certificate,
    redacted_device_certificate,
    device_certifier,
    created_on
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $device_id,
    $device_label,
    $device_certificate,
    $redacted_device_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$device_certifier") },
    $created_on
)
"""
)


_q_lock = Q(
    # Use 55 as magic number to represent user/device creation lock
    # (note this is not strictly needed right now given there is no other
    # advisory lock in the application, but may avoid weird error if we
    # introduce a new advisory lock while forgetting about this one)
    "SELECT pg_advisory_xact_lock(55, _id) FROM organization WHERE organization_id = $organization_id"
)


async def q_take_user_device_write_lock(
    conn: asyncpg.Connection, organization_id: OrganizationID
) -> None:
    """
    User/device creation is a complex procedure given it contains checks that
    cannot be enforced by PostgreSQL, e.g.:
    - `user_` table can contain multiple row with the same `human` value, but
      only one of them can be non-revoked
    - PKI-based enrollment also has to ensure no non-revoked `user_` exist

    So an easy way to lower complexity on this topic is to get rid of the
    concurrency (considering user/device creation is far from being performance
    intensive) by requesting a per-organization PostgreSQL Advisory Lock to be
    held for the duration of the user/device create/update transaction.
    """
    await conn.execute(*_q_lock(organization_id=organization_id.str))


async def _do_create_user(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    user_certificate_cooked: UserCertificate,
    user_certificate: bytes,
    user_certificate_redacted: bytes,
) -> None | UserCreateUserStoreBadOutcome:
    assert user_certificate_cooked.human_handle is not None
    # Create human handle if needed
    await conn.execute(
        *_q_insert_human_if_not_exists(
            organization_id=organization_id.str,
            email=user_certificate_cooked.human_handle.email,
            label=user_certificate_cooked.human_handle.label,
        )
    )

    # Now insert the new user
    try:
        user_certifier = (
            user_certificate_cooked.author.str if user_certificate_cooked.author else None
        )
        result = await conn.execute(
            *_q_insert_user_with_human_handle(
                organization_id=organization_id.str,
                user_id=user_certificate_cooked.user_id.str,
                profile=user_certificate_cooked.profile.str,
                user_certificate=user_certificate,
                redacted_user_certificate=user_certificate_redacted,
                user_certifier=user_certifier,
                created_on=user_certificate_cooked.timestamp,
                email=user_certificate_cooked.human_handle.email,
            )
        )

    except UniqueViolationError:
        return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS

    if result != "INSERT 0 1":
        assert False, f"Insertion error: {result}"

    # Finally make sure there is only one non-revoked user with this human handle
    now = DateTime.now()
    not_revoked_users = await conn.fetch(
        *_q_get_not_revoked_users_for_human(
            organization_id=organization_id.str,
            email=user_certificate_cooked.human_handle.email,
            now=now,
        )
    )
    if (
        len(not_revoked_users) != 1
        or not_revoked_users[0]["user_id"] != user_certificate_cooked.user_id.str
    ):
        # Exception cancels the transaction so the user insertion is automatically cancelled
        return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS


async def q_create_user(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    user_certificate_cooked: UserCertificate,
    user_certificate: bytes,
    user_certificate_redacted: bytes,
    device_certificate_cooked: DeviceCertificate,
    device_certificate: bytes,
    device_certificate_redacted: bytes,
    lock_already_held: bool = False,
) -> None | UserCreateUserStoreBadOutcome | UserCreateDeviceStoreBadOutcome:
    if not lock_already_held:
        await q_take_user_device_write_lock(conn, organization_id)

    record = await conn.fetchrow(*_q_check_active_users_limit(organization_id=organization_id.str))
    # Note with the user/device write lock held we have the guarantee the active users
    # limit won't change in our back.
    if not record["allowed"]:
        raise UserCreateUserStoreBadOutcome.ACTIVE_USERS_LIMIT_REACHED

    if not user_certificate_cooked.human_handle:
        assert False, "User creation without human handle is not supported anymore"

    match await _do_create_user(
        conn, organization_id, user_certificate_cooked, user_certificate, user_certificate_redacted
    ):
        case UserCreateUserStoreBadOutcome() as bad_outcome:
            return bad_outcome
    match await _create_device(
        conn,
        organization_id,
        device_certificate_cooked,
        device_certificate,
        device_certificate_redacted,
        first_device=True,
    ):
        case UserCreateDeviceStoreBadOutcome() as bad_outcome:
            return bad_outcome


@query(in_transaction=True)
async def query_create_user(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    user: User,
    first_device: Device,
) -> None:
    await q_create_user(conn, organization_id, user, first_device)


async def _create_device(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    device_certificate_cooked: DeviceCertificate,
    device_certificate: bytes,
    device_certificate_redacted: bytes,
    first_device: bool = False,
) -> None | UserCreateDeviceStoreBadOutcome:
    if not first_device:
        existing_devices = await conn.fetch(
            *_q_get_user_devices(
                organization_id=organization_id.str, user_id=device_certificate_cooked.user_id.str
            )
        )
        if not existing_devices:
            return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND

        if device_certificate_cooked.device_id in itertools.chain(*existing_devices):
            return UserCreateDeviceStoreBadOutcome.DEVICE_ALREADY_EXISTS

    try:
        device_certifier = (
            device_certificate_cooked.author.str if device_certificate_cooked.author else None
        )
        result = await conn.execute(
            *_q_insert_device(
                organization_id=organization_id.str,
                user_id=device_certificate_cooked.device_id.user_id.str,
                device_id=device_certificate_cooked.device_id.str,
                device_label=device_certificate_cooked.device_label.str
                if device_certificate_cooked.device_label
                else None,
                device_certificate=device_certificate,
                redacted_device_certificate=device_certificate_redacted,
                device_certifier=device_certifier,
                created_on=device_certificate_cooked.timestamp,
            )
        )
    except UniqueViolationError:
        return UserCreateDeviceStoreBadOutcome.DEVICE_ALREADY_EXISTS

    if result != "INSERT 0 1":
        assert False, f"Insertion error: {result}"


@query(in_transaction=True)
async def query_create_device(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    device: Device,
    encrypted_answer: bytes = b"",
) -> None:
    await q_take_user_device_write_lock(conn, organization_id)
    await _create_device(conn, organization_id, device, bool(encrypted_answer))
