# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import OrganizationID
from parsec.components.organization import WipeOrganizationBadOutcome
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_wipe_organization = Q("""
DELETE FROM organization
WHERE organization_id = $organization_id
""")


async def organization_wipe(
    conn: AsyncpgConnection, id: OrganizationID
) -> WipeOrganizationBadOutcome | None:
    res = await conn.execute(*_q_wipe_organization(organization_id=id.str))

    # Parse status `DELETE N`
    count = int(res.split()[-1])

    if count == 0:
        return WipeOrganizationBadOutcome.ORGANIZATION_NOT_FOUND

    return None
