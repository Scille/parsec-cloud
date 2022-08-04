# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from parsec.backend.backend_events import BackendEvent
import itertools
from triopg import UniqueViolationError
import pendulum
from libparsec.types import DateTime

from parsec.api.protocol import OrganizationID
from parsec.backend.user import (
    User,
    Device,
    UserError,
    UserNotFoundError,
    UserAlreadyExistsError,
    UserActiveUsersLimitReached,
)
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.utils import (
    Q,
    query,
    q_organization_internal_id,
    q_device_internal_id,
    q_user_internal_id,
    q_human_internal_id,
)


_q_check_active_users_limit = Q(
    f"""
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
    # (note this is not stricly needed right now given there is no other
    # advisory lock in the application, but may avoid weird error if we
    # introduce a new avisory lock while forgetting about this one)
    "SELECT pg_advisory_xact_lock(55, _id) FROM organization WHERE organization_id = $organization_id"
)


async def q_take_user_device_write_lock(conn, organization_id: OrganizationID):
    """
    User/device creation is a complexe procedure given it contains checks that
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


async def _do_create_user_with_human_handle(
    conn, organization_id: OrganizationID, user: User, first_device: Device
) -> None:
    assert user.human_handle is not None
    # Create human handle if needed
    await conn.execute(
        *_q_insert_human_if_not_exists(
            organization_id=organization_id.str,
            email=user.human_handle.email,
            label=user.human_handle.label,
        )
    )

    # Now insert the new user
    try:
        result = await conn.execute(
            *_q_insert_user_with_human_handle(
                organization_id=organization_id.str,
                user_id=user.user_id.str,
                profile=user.profile.value,
                user_certificate=user.user_certificate,
                redacted_user_certificate=user.redacted_user_certificate,
                user_certifier=user.user_certifier.str if user.user_certifier else None,
                created_on=pendulum.from_timestamp(user.created_on.timestamp()),
                email=user.human_handle.email,
            )
        )

    except UniqueViolationError:
        raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

    if result != "INSERT 0 1":
        raise UserError(f"Insertion error: {result}")

    # Finally make sure there is only one non-revoked user with this human handle
    now = DateTime.now()
    not_revoked_users = await conn.fetch(
        *_q_get_not_revoked_users_for_human(
            organization_id=organization_id.str,
            email=user.human_handle.email,
            now=pendulum.from_timestamp(now.timestamp()),
        )
    )
    if len(not_revoked_users) != 1 or not_revoked_users[0]["user_id"] != user.user_id.str:
        # Exception cancels the transaction so the user insertion is automatically cancelled
        raise UserAlreadyExistsError(
            f"Human handle `{user.human_handle}` already corresponds to a non-revoked user"
        )


async def _do_create_user_without_human_handle(
    conn, organization_id: OrganizationID, user: User, first_device: Device
) -> None:
    try:
        result = await conn.execute(
            *_q_insert_user(
                organization_id=organization_id.str,
                user_id=user.user_id.str,
                profile=user.profile.value,
                user_certificate=user.user_certificate,
                redacted_user_certificate=user.redacted_user_certificate,
                user_certifier=user.user_certifier.str if user.user_certifier else None,
                created_on=pendulum.from_timestamp(user.created_on.timestamp()),
            )
        )

    except UniqueViolationError:
        raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

    if result != "INSERT 0 1":
        raise UserError(f"Insertion error: {result}")


async def q_create_user(
    conn,
    organization_id: OrganizationID,
    user: User,
    first_device: Device,
    lock_already_held: bool = False,
):
    if not lock_already_held:
        await q_take_user_device_write_lock(conn, organization_id)

    record = await conn.fetchrow(*_q_check_active_users_limit(organization_id=organization_id.str))
    # Note with the user/device write lock held we have the guarantee the active users
    # limit won't change in our back.
    if not record["allowed"]:
        raise UserActiveUsersLimitReached()

    if user.human_handle:
        await _do_create_user_with_human_handle(conn, organization_id, user, first_device)
    else:
        await _do_create_user_without_human_handle(conn, organization_id, user, first_device)

    await _create_device(conn, organization_id, first_device, first_device=True)

    # TODO: should be no longer needed once APIv1 is removed
    await send_signal(
        conn,
        BackendEvent.USER_CREATED,
        organization_id=organization_id,
        user_id=user.user_id,
        user_certificate=user.user_certificate,
        first_device_id=first_device.device_id,
        first_device_certificate=first_device.device_certificate,
    )


@query(in_transaction=True)
async def query_create_user(
    conn, organization_id: OrganizationID, user: User, first_device: Device
) -> None:
    await q_create_user(conn, organization_id, user, first_device)


async def _create_device(
    conn, organization_id: OrganizationID, device: Device, first_device: bool = False
) -> None:
    if not first_device:
        existing_devices = await conn.fetch(
            *_q_get_user_devices(organization_id=organization_id.str, user_id=device.user_id.str)
        )
        if not existing_devices:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        if device.device_id in itertools.chain(*existing_devices):
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

    try:
        result = await conn.execute(
            *_q_insert_device(
                organization_id=organization_id.str,
                user_id=device.user_id.str,
                device_id=device.device_id.str,
                device_label=device.device_label.str if device.device_label else None,
                device_certificate=device.device_certificate,
                redacted_device_certificate=device.redacted_device_certificate,
                device_certifier=device.device_certifier.str if device.device_certifier else None,
                created_on=pendulum.from_timestamp(device.created_on.timestamp()),
            )
        )
    except UniqueViolationError:
        raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

    if result != "INSERT 0 1":
        raise UserError(f"Insertion error: {result}")


@query(in_transaction=True)
async def query_create_device(
    conn, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
) -> None:
    await q_take_user_device_write_lock(conn, organization_id)
    await _create_device(conn, organization_id, device, bool(encrypted_answer))
    await send_signal(
        conn,
        BackendEvent.DEVICE_CREATED,
        organization_id=organization_id,
        device_id=device.device_id,
        device_certificate=device.device_certificate,
        encrypted_answer=encrypted_answer,
    )
