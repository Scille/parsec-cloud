# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import itertools
from triopg import UniqueViolationError
from pypika import Parameter

from parsec.api.protocol import OrganizationID
from parsec.backend.user import User, Device, UserError, UserNotFoundError, UserAlreadyExistsError
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.utils import Query, query
from parsec.backend.postgresql.tables import (
    t_device,
    t_user,
    q_organization_internal_id,
    q_user_internal_id,
    q_device_internal_id,
)


_q_get_user_devices = (
    Query.from_(t_device)
    .select(t_device.device_id)
    .where(
        t_device.user_
        == q_user_internal_id(organization_id=Parameter("$1"), user_id=Parameter("$2"))
    )
    .get_sql()
)


_q_insert_user = (
    Query.into(t_user)
    .columns(
        "organization", "user_id", "is_admin", "user_certificate", "user_certifier", "created_on"
    )
    .insert(
        q_organization_internal_id(Parameter("$1")),
        Parameter("$2"),
        Parameter("$3"),
        Parameter("$4"),
        q_device_internal_id(organization_id=Parameter("$1"), device_id=Parameter("$5")),
        Parameter("$6"),
    )
    .get_sql()
)


_q_insert_device = (
    Query.into(t_device)
    .columns(
        "organization", "user_", "device_id", "device_certificate", "device_certifier", "created_on"
    )
    .insert(
        q_organization_internal_id(Parameter("$1")),
        q_user_internal_id(organization_id=Parameter("$1"), user_id=Parameter("$2")),
        Parameter("$3"),
        Parameter("$4"),
        q_device_internal_id(organization_id=Parameter("$1"), device_id=Parameter("$5")),
        Parameter("$6"),
    )
    .get_sql()
)


async def _create_user(
    conn, organization_id: OrganizationID, user: User, first_device: Device
) -> None:
    try:
        result = await conn.execute(
            _q_insert_user,
            organization_id,
            user.user_id,
            user.is_admin,
            user.user_certificate,
            user.user_certifier,
            user.created_on,
        )
    except UniqueViolationError:
        raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

    if result != "INSERT 0 1":
        raise UserError(f"Insertion error: {result}")

    await _create_device(conn, organization_id, first_device, first_device=True)

    await send_signal(
        conn,
        "user.created",
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
    await _create_user(conn, organization_id, user, first_device)


async def _create_device(
    conn, organization_id: OrganizationID, device: Device, first_device: bool = False
) -> None:
    if not first_device:
        existing_devices = await conn.fetch(_q_get_user_devices, organization_id, device.user_id)
        if not existing_devices:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        if device.device_id in itertools.chain(*existing_devices):
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

    try:
        result = await conn.execute(
            _q_insert_device,
            organization_id,
            device.user_id,
            device.device_id,
            device.device_certificate,
            device.device_certifier,
            device.created_on,
        )
    except UniqueViolationError:
        raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

    if result != "INSERT 0 1":
        raise UserError(f"Insertion error: {result}")


@query(in_transaction=True)
async def query_create_device(
    conn, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
) -> None:
    await _create_device(conn, organization_id, device, encrypted_answer)
    await send_signal(
        conn,
        "device.created",
        organization_id=organization_id,
        device_id=device.device_id,
        device_certificate=device.device_certificate,
        encrypted_answer=encrypted_answer,
    )
