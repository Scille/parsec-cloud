# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional
from pypika import Parameter
import pendulum

from parsec.types import OrganizationID, DeviceID
from parsec.backend.user import UserError, UserNotFoundError, UserAlreadyRevokedError
from parsec.backend.postgresql.utils import Query, query
from parsec.backend.postgresql.tables import (
    t_device,
    q_device,
    q_organization_internal_id,
    q_user_internal_id,
    q_device_internal_id,
)


_q_revoke_device = """
UPDATE device SET
    revoked_device_certificate = $3,
    revoked_device_certifier = ({}),
    revoked_on = $5
WHERE
    organization = ({})
    AND device_id = $2
    AND revoked_on IS NULL
""".format(
    q_device_internal_id(organization_id=Parameter("$1"), device_id=Parameter("$4")),
    q_organization_internal_id(Parameter("$1")),
)


_q_get_devices_revoked_on = (
    Query.from_(t_device)
    .where(
        t_device.user_
        == q_user_internal_id(organization_id=Parameter("$1"), user_id=Parameter("$2"))
    )
    .select(t_device.revoked_on)
    .get_sql()
)


@query(in_transaction=True)
async def query_revoke_device(
    conn,
    organization_id: OrganizationID,
    device_id: DeviceID,
    revoked_device_certificate: bytes,
    revoked_device_certifier: DeviceID,
    revoked_on: pendulum.Pendulum = None,
) -> Optional[pendulum.Pendulum]:
    result = await conn.execute(
        _q_revoke_device,
        organization_id,
        device_id,
        revoked_device_certificate,
        revoked_device_certifier,
        revoked_on or pendulum.now(),
    )

    if result != "UPDATE 1":
        # TODO: avoid having to do another query to find the error

        query = (
            q_device(organization_id=Parameter("$1"), device_id=Parameter("$2"))
            .select("revoked_on")
            .get_sql()
        )

        err_result = await conn.fetchrow(query, organization_id, device_id)
        if not err_result:
            raise UserNotFoundError(device_id)

        elif err_result[0]:
            raise UserAlreadyRevokedError()

        else:
            raise UserError(f"Update error: {result}")

    # Determine if the user has bee revoked (i.e. all
    # his devices are revoked)

    result = await conn.fetch(_q_get_devices_revoked_on, organization_id, device_id.user_id)

    revocations = [r[0] for r in result]
    if None in revocations:
        return None
    else:
        return sorted(revocations)[-1]
