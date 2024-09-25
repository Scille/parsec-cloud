# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
)
from parsec.components.organization import (
    OrganizationGetTosBadOutcome,
    TermsOfService,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)

_q_get_tos = Q("""
SELECT
    is_expired,
    tos_updated_on,
    tos_per_locale_urls
FROM organization
WHERE organization_id = $organization_id
LIMIT 1
""")


async def organization_get_tos(
    conn: AsyncpgConnection,
    id: OrganizationID,
) -> TermsOfService | None | OrganizationGetTosBadOutcome:
    row = await conn.fetchrow(*_q_get_tos(organization_id=id.str))

    if not row:
        return OrganizationGetTosBadOutcome.ORGANIZATION_NOT_FOUND

    match row["is_expired"]:
        case True:
            return OrganizationGetTosBadOutcome.ORGANIZATION_EXPIRED
        case False:
            pass
        case unknown:
            assert False, unknown

    match (row["tos_updated_on"], row["tos_per_locale_urls"]):
        case (None, None):
            return None

        case (DateTime() as tos_updated_on, dict() as tos_per_locale_urls):
            # Sanity check to ensure the JSON on database have the expected
            # {<locale>: <url>} format.
            for key, value in tos_per_locale_urls.items():
                assert isinstance(key, str) and key, tos_per_locale_urls
                assert isinstance(value, str) and value, tos_per_locale_urls

            return TermsOfService(
                updated_on=tos_updated_on,
                per_locale_urls=tos_per_locale_urls,
            )

        case unknown:
            assert False, unknown
