# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple
from pypika import Parameter

from parsec.types import UserID, DeviceID, OrganizationID
from parsec.backend.user import User, Device, UserNotFoundError
from parsec.backend.postgresql.utils import Query, query
from parsec.backend.postgresql.tables import (
    t_user,
    t_device,
    q_device,
    q_organization_internal_id,
    q_user_internal_id,
)


_q_get_user = (
    Query.from_(t_user)
    .select(
        "is_admin",
        "user_certificate",
        q_device(_id=t_user.user_certifier).select(t_device.device_id).as_("user_certifier"),
        "created_on",
    )
    .where(
        (t_user.organization == q_organization_internal_id(Parameter("$1")))
        & (t_user.user_id == Parameter("$2"))
    )
    .get_sql()
)


_t_d1 = t_device.as_("d1")
_t_d2 = t_device.as_("d2")
_t_d3 = t_device.as_("d3")


_q_get_device = (
    Query.from_(_t_d1)
    .select(
        "device_certificate",
        q_device(_id=_t_d1.device_certifier, table=_t_d2)
        .select(_t_d2.device_id)
        .as_("device_certifier"),
        "created_on",
        "revoked_on",
        "revoked_device_certificate",
        q_device(_id=_t_d1.revoked_device_certifier, table=_t_d3)
        .select(_t_d3.device_id)
        .as_("revoked_device_certifier"),
    )
    .where(
        (_t_d1.organization == q_organization_internal_id(Parameter("$1")))
        & (_t_d1.device_id == Parameter("$2"))
    )
    .get_sql()
)


_q_get_user_devices = (
    Query.from_(_t_d1)
    .select(
        "device_id",
        "device_certificate",
        q_device(_id=_t_d1.device_certifier, table=_t_d2)
        .select(_t_d2.device_id)
        .as_("device_certifier"),
        "created_on",
        "revoked_on",
        "revoked_device_certificate",
        q_device(_id=_t_d1.revoked_device_certifier, table=_t_d3)
        .select(_t_d3.device_id)
        .as_("revoked_device_certifier"),
    )
    .where(
        _t_d1.user_ == q_user_internal_id(organization_id=Parameter("$1"), user_id=Parameter("$2"))
    )
    .get_sql()
)


_q_get_trustchain = """
WITH nodes AS (
    WITH RECURSIVE nodes(node_id, certifier_id, revoker_id, parents, depth) AS (
        SELECT
            _id,
            device.device_certifier,
            device.revoked_device_certifier,
            ARRAY[device.device_certifier, device.revoked_device_certifier],
            1
        FROM device WHERE _id = ANY((SELECT _id FROM device WHERE organization = ({q_organization}) AND device_id = ANY($2::VARCHAR[])))

        UNION ALL

        SELECT
            _id,
            device.device_certifier,
            device.revoked_device_certifier,
            parents || device.device_certifier || device.revoked_device_certifier,
            nd.depth + 1
        FROM device, nodes as nd
        WHERE (
            (nd.certifier_id IS NOT NULL AND device._id = nd.certifier_id) OR
            (nd.revoker_id IS NOT NULL AND device._id = nd.revoker_id)
        )
    )
    SELECT * FROM (
        SELECT unnest(parents) as _id FROM nodes
        UNION
        SELECT _id FROM device WHERE organization = ({q_organization}) AND device_id = ANY($2::VARCHAR[])
    ) ss
    GROUP BY _id
)
SELECT
    device_id,
    device_certificate,
    ({q_device_certifier}) as device_certifier,
    created_on,
    revoked_on,
    revoked_device_certificate,
    ({q_revoked_device_certifier}) as revoked_device_certifier
FROM nodes
INNER JOIN device
ON nodes._id = device._id
""".format(
    q_organization=q_organization_internal_id(Parameter("$1")),
    q_device_certifier=q_device(_id=Parameter("device_certifier")).select("_id"),
    q_revoked_device_certifier=q_device(_id=Parameter("revoked_device_certifier")).select("_id"),
)


async def _get_user(conn, organization_id: OrganizationID, user_id: UserID) -> User:
    data = await conn.fetchrow(_q_get_user, organization_id, user_id)
    if not data:
        raise UserNotFoundError(user_id)

    return User(
        user_id=UserID(user_id),
        is_admin=data[0],
        user_certificate=data[1],
        user_certifier=data[2],
        created_on=data[3],
    )


async def _get_device(conn, organization_id: OrganizationID, device_id: DeviceID) -> Device:
    data = await conn.fetchrow(_q_get_device, organization_id, device_id)
    if not data:
        raise UserNotFoundError(device_id)

    return Device(device_id, *data)


async def _get_trustchain(
    conn, organization_id: OrganizationID, *device_ids: Tuple[DeviceID]
) -> Tuple[Device]:
    results = await conn.fetch(_q_get_trustchain, organization_id, device_ids)

    return tuple(Device(*result) for result in results)


async def _get_user_devices(
    conn, organization_id: OrganizationID, user_id: UserID
) -> Tuple[Device]:
    devices_results = await conn.fetch(_q_get_user_devices, organization_id, user_id)

    return tuple([Device(DeviceID(d_id), *d_data) for d_id, *d_data in devices_results])


@query()
async def query_get_user(conn, organization_id: OrganizationID, user_id: UserID) -> User:
    return await _get_user(conn, organization_id, user_id)


@query(in_transaction=True)
async def query_get_user_with_trustchain(
    conn, organization_id: OrganizationID, user_id: UserID
) -> Tuple[User, Tuple[Device]]:
    user = await _get_user(conn, organization_id, user_id)
    trustchain = await _get_trustchain(conn, organization_id, user.user_certifier)
    return user, trustchain


@query(in_transaction=True)
async def query_get_user_with_device_and_trustchain(
    conn, organization_id: OrganizationID, device_id: DeviceID
) -> Tuple[User, Device, Tuple[Device]]:
    user = await _get_user(conn, organization_id, device_id.user_id)
    user_device = await _get_device(conn, organization_id, device_id)
    trustchain = await _get_trustchain(
        conn,
        organization_id,
        user.user_certifier,
        user_device.device_certifier,
        user_device.revoked_device_certifier,
    )
    return user, user_device, trustchain


@query(in_transaction=True)
async def query_get_user_with_devices_and_trustchain(
    conn, organization_id: OrganizationID, user_id: UserID
) -> Tuple[User, Tuple[Device], Tuple[Device]]:
    user = await _get_user(conn, organization_id, user_id)
    user_devices = await _get_user_devices(conn, organization_id, user_id)
    trustchain = await _get_trustchain(
        conn,
        organization_id,
        user.user_certifier,
        *[device.device_certifier for device in user_devices],
        *[device.revoked_device_certifier for device in user_devices],
    )
    return user, user_devices, trustchain


@query(in_transaction=True)
async def query_get_user_with_device(
    conn, organization_id: OrganizationID, device_id: DeviceID
) -> Tuple[User, Device]:
    d_data = await conn.fetchrow(_q_get_device, organization_id, device_id)
    u_data = await conn.fetchrow(_q_get_user, organization_id, device_id.user_id)
    if not u_data or not d_data:
        raise UserNotFoundError(device_id)

    device = Device(device_id, *d_data)
    user = User(
        user_id=device_id.user_id,
        is_admin=u_data[0],
        user_certificate=u_data[1],
        user_certifier=u_data[2],
        created_on=u_data[3],
    )
    return user, device
