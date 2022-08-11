# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import List, Optional, Tuple
from triopg._triopg import TrioConnectionProxy

from parsec.sequester_crypto import SequesterVerifyKeyDer
from parsec.api.data import DataError, SequesterServiceCertificate
from parsec.api.protocol import OrganizationID, RealmID, SequesterServiceID, VlobID
from parsec.backend.organization import SequesterAuthority
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.utils import Q, q_organization_internal_id, q_realm_internal_id
from parsec.backend.sequester import (
    BaseSequesterComponent,
    SequesterCertificateOutOfBallparkError,
    SequesterCertificateValidationError,
    SequesterDisabledError,
    SequesterError,
    SequesterOrganizationNotFoundError,
    SequesterService,
    SequesterServiceAlreadyDisabledError,
    SequesterServiceAlreadyEnabledError,
    SequesterServiceAlreadyExists,
    SequesterServiceNotFoundError,
)
from parsec.crypto import CryptoError
from parsec.utils import timestamps_in_the_ballpark
from parsec._parsec import DateTime


# Sequester authority never gets modified past organization bootstrap, hence no need
# to lock the row with a `FOR UPDATE` even if other queries depend of this result
_q_get_organisation_sequester_authority = Q(
    f"""
SELECT sequester_authority_certificate, sequester_authority_verify_key_der
FROM organization
WHERE organization_id = $organization_id
"""
)

_q_create_sequester_service = Q(
    f"""
INSERT INTO sequester_service(
    service_id,
    organization,
    service_certificate,
    service_label,
    created_on
)
VALUES(
    $service_id,
    { q_organization_internal_id("$organization_id") },
    $service_certificate,
    $service_label,
    $created_on
)
"""
)

_q_disable_sequester_service = Q(
    f"""
UPDATE sequester_service
SET disabled_on=$disabled_on
WHERE
    service_id=$service_id
    AND organization={ q_organization_internal_id("$organization_id") }
"""
)

_q_enable_sequester_service = Q(
    f"""
UPDATE sequester_service
SET disabled_on=NULL
WHERE
    service_id=$service_id
    AND organization={ q_organization_internal_id("$organization_id") }
"""
)

_q_get_sequester_service_exist = Q(
    f"""
SELECT service_id
FROM sequester_service
WHERE
    service_id=$service_id
    AND organization={ q_organization_internal_id("$organization_id") }
LIMIT 1"""
)

_q_get_sequester_service_disabled_on_for_update = Q(
    f"""
SELECT disabled_on
FROM sequester_service
WHERE
    service_id=$service_id
    AND organization={ q_organization_internal_id("$organization_id") }
FOR UPDATE
LIMIT 1
"""
)

_q_get_sequester_service = Q(
    f"""
SELECT service_label, service_certificate, created_on, disabled_on
FROM sequester_service
WHERE
    service_id=$service_id
    AND organization={ q_organization_internal_id("$organization_id") }
LIMIT 1
"""
)

_q_get_organization_services = Q(
    f"""
SELECT service_id, service_label, service_certificate, created_on, disabled_on
FROM sequester_service
WHERE
    organization={ q_organization_internal_id("$organization_id") }
ORDER BY _id
"""
)

_get_sequester_blob = Q(
    f"""
SELECT sequester_service_vlob_atom.blob, vlob_atom.vlob_id
FROM sequester_service_vlob_atom
INNER JOIN vlob_atom ON vlob_atom._id = sequester_service_vlob_atom.vlob_atom
INNER JOIN realm_vlob_update ON vlob_atom._id = realm_vlob_update.vlob_atom

WHERE sequester_service_vlob_atom.service = (SELECT _id from sequester_service WHERE service_id=$service_id
AND organization={ q_organization_internal_id("$organization_id") })
AND realm_vlob_update.realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY sequester_service_vlob_atom._id
"""
)


async def get_sequester_authority(conn, organization_id: OrganizationID) -> SequesterAuthority:
    row = await conn.fetchrow(
        *_q_get_organisation_sequester_authority(organization_id=organization_id.str)
    )
    if not row:
        raise SequesterOrganizationNotFoundError
    elif row[0] is None:
        raise SequesterDisabledError
    return SequesterAuthority(certificate=row[0], verify_key_der=SequesterVerifyKeyDer(row[1]))


class PGPSequesterComponent(BaseSequesterComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create_service(
        self,
        organization_id: OrganizationID,
        service: SequesterService,
        now: Optional[DateTime] = None,
    ) -> None:
        now = now or DateTime.now()

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            sequester_authority = await get_sequester_authority(conn, organization_id)

            try:
                certif_dumped = sequester_authority.verify_key_der.verify(
                    service.service_certificate
                )
            except CryptoError as exc:
                raise SequesterCertificateValidationError(
                    f"Invalid certification data ({exc})."
                ) from exc

            try:
                certif_data = SequesterServiceCertificate.load(certif_dumped)

            except DataError as exc:
                raise SequesterCertificateValidationError(
                    f"Invalid certification data ({exc})."
                ) from exc

            if not timestamps_in_the_ballpark(certif_data.timestamp, now):
                raise SequesterCertificateOutOfBallparkError(
                    f"Invalid certification data (timestamp out of ballpark)."
                )

            row = await conn.fetchrow(
                *_q_get_sequester_service_exist(
                    organization_id=organization_id.str, service_id=service.service_id
                )
            )
            if row:
                raise SequesterServiceAlreadyExists

            result = await conn.execute(
                *_q_create_sequester_service(
                    organization_id=organization_id.str,
                    service_id=service.service_id,
                    service_label=service.service_label,
                    service_certificate=service.service_certificate,
                    created_on=service.created_on,
                )
            )
            if result != "INSERT 0 1":
                raise SequesterError(f"Insertion Error: {result}")

    async def _assert_service_enabled(
        self, conn: TrioConnectionProxy, organization_id: OrganizationID
    ) -> None:
        row = await conn.fetchrow(
            *_q_get_organisation_sequester_authority(organization_id=organization_id.str)
        )
        if not row:
            raise SequesterOrganizationNotFoundError
        if row[0] is None:
            raise SequesterDisabledError

    async def disable_service(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        disabled_on: Optional[DateTime] = None,
    ) -> None:

        disabled_on = disabled_on or DateTime.now()
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await self._assert_service_enabled(conn, organization_id)

            row = await conn.fetchrow(
                *_q_get_sequester_service_disabled_on_for_update(
                    organization_id=organization_id.str, service_id=service_id
                )
            )

            if not row:
                raise SequesterServiceNotFoundError

            if row["disabled_on"] is not None:
                raise SequesterServiceAlreadyDisabledError

            result = await conn.execute(
                *_q_disable_sequester_service(
                    organization_id=organization_id.str,
                    service_id=service_id,
                    disabled_on=disabled_on,
                )
            )
            if result != "UPDATE 1":
                raise SequesterError(f"Insertion Error: {result}")

    async def enable_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await self._assert_service_enabled(conn, organization_id)

            row = await conn.fetchrow(
                *_q_get_sequester_service_disabled_on_for_update(
                    organization_id=organization_id.str, service_id=service_id
                )
            )

            if not row:
                raise SequesterServiceNotFoundError

            if row["disabled_on"] is None:
                raise SequesterServiceAlreadyEnabledError

            result = await conn.execute(
                *_q_enable_sequester_service(
                    organization_id=organization_id.str, service_id=service_id
                )
            )
            if result != "UPDATE 1":
                raise SequesterError(f"Insertion Error: {result}")

    async def _get_service(
        self,
        conn: TrioConnectionProxy,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
    ) -> SequesterService:
        await self._assert_service_enabled(conn, organization_id)

        row = await conn.fetchrow(
            *_q_get_sequester_service(organization_id=organization_id.str, service_id=service_id)
        )

        if not row:
            raise SequesterServiceNotFoundError

        return SequesterService(
            service_id=service_id,
            service_label=row[0],
            service_certificate=row[1],
            created_on=row[2],
            disabled_on=row[3],
        )

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> SequesterService:
        async with self.dbh.pool.acquire() as conn:
            return await self._get_service(conn, organization_id, service_id)

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[SequesterService]:
        async with self.dbh.pool.acquire() as conn:
            await self._assert_service_enabled(conn, organization_id)
            entries = await conn.fetch(
                *_q_get_organization_services(organization_id=organization_id.str)
            )

            return [
                SequesterService(
                    service_id=SequesterServiceID(entry["service_id"]),
                    service_label=entry["service_label"],
                    service_certificate=entry["service_certificate"],
                    created_on=entry["created_on"],
                    disabled_on=entry["disabled_on"] if entry["disabled_on"] else None,
                )
                for entry in entries
            ]

    async def dump_realm(
        self, organization_id: OrganizationID, service_id: SequesterServiceID, realm_id: RealmID
    ) -> List[Tuple[VlobID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            # Check organization and service exists
            await self._get_service(conn, organization_id, service_id)
            data = await conn.fetch(
                *_get_sequester_blob(
                    organization_id=organization_id.str, service_id=service_id, realm_id=realm_id
                )
            )
            dump = []
            for version, entry in enumerate(data, start=1):
                dump.append((VlobID(entry["vlob_id"]), version, entry["blob"]))
            return dump
