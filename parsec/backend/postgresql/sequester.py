# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import List, Optional, Tuple

from triopg._triopg import TrioConnectionProxy
from parsec.api.data import DataError, SequesterServiceCertificate
from parsec.api.protocol import OrganizationID, RealmID, SequesterServiceID, VlobID
from parsec.backend.organization import SequesterAuthority
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.backend.sequester import (
    BaseSequesterComponent,
    SequesterCertificateOutOfBallparkError,
    SequesterCertificateValidationError,
    SequesterDisabledError,
    SequesterError,
    SequesterOrganizationNotFoundError,
    SequesterService,
    SequesterServiceAlreadyDeletedError,
    SequesterServiceAlreadyExists,
    SequesterServiceNotFoundError,
)
from parsec.crypto import CryptoError
from parsec.utils import timestamps_in_the_ballpark
from pendulum import DateTime
from pendulum import now as pendulum_now

"""


CREATE TABLE sequester_service(
    _id SERIAL PRIMARY KEY,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    service_certificate BYTEA NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    deleted_on TIMESTAMPTZ, -- NULL if not deleted
    webhook_url TEXT, -- NULL if service is not a WEBHOOK

    UNIQUE(organization, service_id)
);
"""


_q_get_organisation_sequester_authority = Q(
    f"""
    SELECT sequester_authority
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

_q_delete_sequester_service = Q(
    f"""
    UPDATE sequester_service
    SET deleted_on=$deleted_on
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

_q_get_sequester_service_deleted_for_update = Q(
    f"""
    SELECT service_id, deleted_on
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
    SELECT service_label, service_certificate, created_on, deleted_on
    FROM sequester_service
    WHERE
        service_id=$service_id
        AND organization={ q_organization_internal_id("$organization_id") }
    FOR UPDATE
    LIMIT 1"""
)

_q_get_organization_services = Q(
    f"""
    SELECT service_id, service_label, service_certificate, created_on, deleted_on
    FROM sequester_service
    WHERE
        organization={ q_organization_internal_id("$organization_id") }
    """
)


class PGPSequesterComponent(BaseSequesterComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create_service(
        self, organization_id: OrganizationID, service: SequesterService
    ) -> None:

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            row = await conn.fetchrow(
                *_q_get_organisation_sequester_authority(organization_id=organization_id.str)
            )
            if not row:
                raise SequesterOrganizationNotFoundError
            elif row[0] is None:
                raise SequesterDisabledError
            else:
                # TODO: Handle Error
                sequester_authority = SequesterAuthority.build_from_certificate(row[0])

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

            now = pendulum_now()
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
    ):
        row = await conn.fetchrow(
            *_q_get_organisation_sequester_authority(organization_id=organization_id.str)
        )
        if not row:
            raise SequesterOrganizationNotFoundError
        if row[0] is None:
            raise SequesterDisabledError

    async def delete_service(
        self,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
        deleted_on: Optional[DateTime] = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await self._assert_service_enabled(conn, organization_id)

            row = await conn.fetchrow(
                *_q_get_organisation_sequester_authority(organization_id=organization_id.str)
            )
            if not row:
                raise SequesterOrganizationNotFoundError
            if row[0] is None:
                raise SequesterDisabledError

            row = await conn.fetchrow(
                *_q_get_sequester_service_deleted_for_update(
                    organization_id=organization_id.str, service_id=service_id
                )
            )

            if not row:
                raise SequesterServiceNotFoundError
            if row[1] is not None:
                raise SequesterServiceAlreadyDeletedError

            result = await conn.execute(
                *_q_delete_sequester_service(
                    organization_id=organization_id.str,
                    service_id=service_id,
                    deleted_on=deleted_on,
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
            deleted_on=row[3],
        )

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> SequesterService:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            return await self._get_service(conn, organization_id, service_id)

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[SequesterService]:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
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
                    deleted_on=entry["deleted_on"],
                )
                for entry in entries
            ]

    async def dump_realm(
        self, organization_id: OrganizationID, service_id: SequesterServiceID, realm_id: RealmID
    ) -> List[Tuple[VlobID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn, conn.transacggtion():
            # Check organization and service exists
            await self._get_service(conn, organization_id, service_id)
            # TODO Dump !
        return []
