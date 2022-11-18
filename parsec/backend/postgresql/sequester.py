# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import triopg
from collections import defaultdict
from typing import Any, List, Tuple

from parsec.sequester_crypto import SequesterVerifyKeyDer
from parsec.api.data import DataError, SequesterServiceCertificate
from parsec.api.protocol import OrganizationID, RealmID, SequesterServiceID, VlobID
from parsec.backend.organization import SequesterAuthority
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.utils import Q, q_organization_internal_id, q_realm_internal_id
from parsec.backend.sequester import (
    BaseSequesterComponent,
    BaseSequesterService,
    StorageSequesterService,
    WebhookSequesterService,
    SequesterServiceType,
    SequesterError,
    SequesterCertificateValidationError,
    SequesterDisabledError,
    SequesterOrganizationNotFoundError,
    SequesterServiceAlreadyDisabledError,
    SequesterServiceAlreadyEnabledError,
    SequesterServiceAlreadyExists,
    SequesterServiceNotFoundError,
    SequesterWrongServiceTypeError,
)
from parsec.crypto import CryptoError
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
    created_on,
    service_type,
    webhook_url
)
VALUES(
    $service_id,
    { q_organization_internal_id("$organization_id") },
    $service_certificate,
    $service_label,
    $created_on,
    $service_type,
    $webhook_url
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
SELECT service_id, service_label, service_certificate, created_on, disabled_on, service_type, webhook_url
FROM sequester_service
WHERE
    service_id=$service_id
    AND organization={ q_organization_internal_id("$organization_id") }
LIMIT 1
"""
)

_q_get_organization_services = Q(
    f"""
SELECT service_id, service_label, service_certificate, created_on, disabled_on, service_type, webhook_url
FROM sequester_service
WHERE
    organization={ q_organization_internal_id("$organization_id") }
ORDER BY _id
"""
)

_q_get_organization_services_without_disabled = Q(
    f"""
SELECT service_id, service_label, service_certificate, created_on, disabled_on, service_type, webhook_url
FROM sequester_service
WHERE
    organization={ q_organization_internal_id("$organization_id") }
    AND disabled_on IS NULL
ORDER BY _id
"""
)

_get_sequester_blob = Q(
    f"""
SELECT sequester_service_vlob_atom.blob, vlob_atom.vlob_id, vlob_atom._id
FROM sequester_service_vlob_atom
INNER JOIN vlob_atom ON vlob_atom._id = sequester_service_vlob_atom.vlob_atom
INNER JOIN realm_vlob_update ON vlob_atom._id = realm_vlob_update.vlob_atom

WHERE sequester_service_vlob_atom.service = (SELECT _id from sequester_service WHERE service_id=$service_id
AND organization={ q_organization_internal_id("$organization_id") })
AND realm_vlob_update.realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY sequester_service_vlob_atom._id
"""
)

_get_vlob_atom_ids = Q(
    f"""
    SELECT vlob_atom._id, vlob_encryption_revision, vlob_id
    FROM vlob_atom
    INNER JOIN realm_vlob_update ON vlob_atom._id = realm_vlob_update.vlob_atom

    WHERE organization={ q_organization_internal_id("$organization_id") }
    AND realm_vlob_update.realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    """
)


async def get_sequester_authority(
    conn: triopg._triopg.TrioConnectionProxy, organization_id: OrganizationID
) -> SequesterAuthority:
    row = await conn.fetchrow(
        *_q_get_organisation_sequester_authority(organization_id=organization_id.str)
    )
    if not row:
        raise SequesterOrganizationNotFoundError
    elif row[0] is None:
        raise SequesterDisabledError
    return SequesterAuthority(certificate=row[0], verify_key_der=SequesterVerifyKeyDer(row[1]))


async def get_sequester_services(
    conn: triopg._triopg.TrioConnectionProxy, organization_id: OrganizationID, with_disabled: bool
) -> List[BaseSequesterService]:
    q = (
        _q_get_organization_services
        if with_disabled
        else _q_get_organization_services_without_disabled
    )
    rows = await conn.fetch(*q(organization_id=organization_id.str))
    return [build_sequester_service_obj(row) for row in rows]


def build_sequester_service_obj(row: dict[str, Any]) -> BaseSequesterService:
    service_id = SequesterServiceID.from_hex(row["service_id"])
    service_type = SequesterServiceType(row["service_type"].lower())
    if service_type == SequesterServiceType.STORAGE:
        return StorageSequesterService(
            service_id=service_id,
            service_label=row["service_label"],
            service_certificate=row["service_certificate"],
            created_on=row["created_on"],
            disabled_on=row["disabled_on"],
        )
    else:
        assert service_type == SequesterServiceType.WEBHOOK
        webhook_url = row["webhook_url"]
        if webhook_url is None:
            raise ValueError(
                f"Database inconsistency: sequester service `{service_id.hex}` is of type WEBHOOK but has an empty `webhook_url` column"
            )
        return WebhookSequesterService(
            service_id=service_id,
            service_label=row["service_label"],
            service_certificate=row["service_certificate"],
            created_on=row["created_on"],
            disabled_on=row["disabled_on"],
            webhook_url=webhook_url,
        )


class PGPSequesterComponent(BaseSequesterComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create_service(
        self,
        organization_id: OrganizationID,
        service: BaseSequesterService,
    ) -> None:
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
                SequesterServiceCertificate.load(certif_dumped)

            except DataError as exc:
                raise SequesterCertificateValidationError(
                    f"Invalid certification data ({exc})."
                ) from exc

            # Note that unlike for other signed data, we don't check the certificate's
            # timestamp. This is because the certficate is allowed to be created long
            # before being inserted (see `generate_service_certificate` CLI command)

            row = await conn.fetchrow(
                *_q_get_sequester_service_exist(
                    organization_id=organization_id.str, service_id=service.service_id
                )
            )
            if row:
                raise SequesterServiceAlreadyExists
            webhook_url: str | None
            if isinstance(service, WebhookSequesterService):
                webhook_url = service.webhook_url
            else:
                webhook_url = None
            result = await conn.execute(
                *_q_create_sequester_service(
                    organization_id=organization_id.str,
                    service_id=service.service_id,
                    service_label=service.service_label,
                    service_certificate=service.service_certificate,
                    created_on=service.created_on,
                    service_type=service.service_type.value.upper(),
                    webhook_url=webhook_url,
                )
            )
            if result != "INSERT 0 1":
                raise SequesterError(f"Insertion Error: {result}")

    async def _assert_service_enabled(
        self, conn: triopg._triopg.TrioConnectionProxy, organization_id: OrganizationID
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
        disabled_on: DateTime | None = None,
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
        conn: triopg._triopg.TrioConnectionProxy,
        organization_id: OrganizationID,
        service_id: SequesterServiceID,
    ) -> BaseSequesterService:
        await self._assert_service_enabled(conn, organization_id)

        row = await conn.fetchrow(
            *_q_get_sequester_service(organization_id=organization_id.str, service_id=service_id)
        )

        if not row:
            raise SequesterServiceNotFoundError

        return build_sequester_service_obj(row)

    async def get_service(
        self, organization_id: OrganizationID, service_id: SequesterServiceID
    ) -> BaseSequesterService:
        async with self.dbh.pool.acquire() as conn:
            return await self._get_service(conn, organization_id, service_id)

    async def get_organization_services(
        self, organization_id: OrganizationID
    ) -> List[BaseSequesterService]:
        async with self.dbh.pool.acquire() as conn:
            await self._assert_service_enabled(conn, organization_id)
            return await get_sequester_services(
                conn=conn, organization_id=organization_id, with_disabled=True
            )

    async def dump_realm(
        self, organization_id: OrganizationID, service_id: SequesterServiceID, realm_id: RealmID
    ) -> List[Tuple[VlobID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            # Check organization and service exists
            service = await self._get_service(conn, organization_id, service_id)
            if service.service_type != SequesterServiceType.STORAGE:
                raise SequesterWrongServiceTypeError(
                    f"Service type {service.service_type} is not compatible with export"
                )

            # Exctract sequester blobs
            raw_sequester_data = await conn.fetch(
                *_get_sequester_blob(
                    organization_id=organization_id.str,
                    service_id=service_id,
                    realm_id=realm_id,
                )
            )

            # Sort data by postgresql table index
            sequester_blob = {
                data["_id"]: (VlobID.from_hex(data["vlob_id"]), data["blob"])
                for data in raw_sequester_data
            }
            # Get vlob ids
            sequester_vlob_ids = set(
                [VlobID.from_hex(data["vlob_id"]) for data in raw_sequester_data]
            )

            # Only vlob that contains sequester data has been downloaded.
            # A vlob_id can be in different vlob atoms.
            # Sometimes, those vlob atoms does not have sequester data.
            # We need to send all sequester vlob to the client and keep track of all updates
            # Sequester data has an update version.
            # To get this update version, we need to get all vlob atom (all updates)
            # for one vlob id.

            # Get all vlobs
            # Could be better to use "WHERE ... IN ..."" querry to get only vlobs that contains at least one sequester data.
            # It appears that triopg does not handle "WHERE vlob_id IN ($vlob_ids)" querries if items are UUIDs
            # No other choice to dump all vlobs.
            all_vlob_ids = await conn.fetch(
                *_get_vlob_atom_ids(
                    organization_id=organization_id.str,
                    realm_id=realm_id,
                )
            )

            vlobs_ids = defaultdict(list)
            for entry in all_vlob_ids:
                vlobs_ids[VlobID.from_hex(entry["vlob_id"])].append(entry["_id"])

            # Generate dump data
            dump = []

            for vlob_id in sequester_vlob_ids:
                # Get all vlob atom indexes
                vlob_atom_indexes = vlobs_ids[vlob_id]
                # Count versions
                for version, vlob_atom_index in enumerate(vlob_atom_indexes, start=1):
                    try:
                        sequester_vlob_id, blob = sequester_blob[vlob_atom_index]
                        assert sequester_vlob_id == vlob_id
                        dump.append((vlob_id, version, blob))
                    except KeyError:
                        pass

            return dump
