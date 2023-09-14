# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import triopg

from parsec._parsec import DateTime, DeviceID, RealmArchivingConfiguration, RealmID, RealmRole
from parsec.api.protocol import OrganizationID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.organization import OrganizationNotFoundError, _q_get_organization
from parsec.backend.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user_internal_id,
    query,
)
from parsec.backend.realm import (
    RealmAccessError,
    RealmArchivedError,
    RealmArchivingPeriodTooShortError,
    RealmConfiguredArchiving,
    RealmDeletedError,
    RealmNotFoundError,
    RealmRoleRequireGreaterTimestampError,
)
from parsec.backend.utils import OperationKind

_q_check_realm_exist = Q(
    q_realm(
        organization_id="$organization_id",
        realm_id="$realm_id",
        select="realm_id",
    )
)


_q_get_user_role = Q(
    f"""
SELECT role, certified_on
FROM realm_user_role
WHERE
    user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
    AND realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY certified_on DESC LIMIT 1
"""
)

_q_get_archiving_configuration = Q(
    f"""
SELECT configuration, deletion_date, certified_on
FROM realm_archiving
WHERE
    realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY certified_on DESC LIMIT 1
"""
)

_q_set_archiving_configuration = Q(
    f"""
INSERT INTO realm_archiving(
    realm,
    configuration,
    deletion_date,
    certificate,
    certified_by,
    certified_on
) SELECT
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    $configuration,
    $deletion_date,
    $certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on
"""
)


_q_get_archiving_configurations = Q(
    f"""
SELECT DISTINCT ON (realm)
    realm,
    configuration,
    deletion_date,
    certified_on,
    { q_device(_id="realm_archiving.certified_by", select="device_id") }
FROM realm_archiving
WHERE
    realm = ANY($internal_realm_ids)
ORDER BY realm, certified_on DESC
"""
)


_q_set_last_archiving_change = Q(
    f"""
INSERT INTO realm_user_change(realm, user_, last_role_change, last_vlob_update, last_archiving_change)
VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    NULL,
    NULL,
    $configured_on
)
ON CONFLICT (realm, user_)
DO UPDATE SET last_archiving_change = (
    SELECT GREATEST($configured_on, last_archiving_change)
    FROM realm_user_change
    WHERE realm={ q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    AND user_={ q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
    LIMIT 1
)
"""
)


@query()
async def query_get_archiving_configurations(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    realm_ids: dict[RealmID, int],
) -> list[tuple[RealmID, RealmArchivingConfiguration, DateTime | None, DeviceID | None]]:
    rep = await conn.fetch(*_q_get_archiving_configurations(internal_realm_ids=realm_ids.values()))
    mapping: dict[int, tuple[RealmArchivingConfiguration, DateTime | None, DeviceID | None]] = {}
    for item in rep:
        (
            internal_realm_id,
            configuration_str,
            deletion_date,
            archiving_certified_on,
            archiving_certified_by,
        ) = item
        configuration = RealmArchivingConfiguration.from_str(configuration_str, deletion_date)
        mapping[internal_realm_id] = (
            configuration,
            archiving_certified_on,
            DeviceID(archiving_certified_by),
        )
    default = RealmArchivingConfiguration.available(), None, None
    return [
        (realm_id, *mapping.get(internal_realm_id, default))
        for realm_id, internal_realm_id in realm_ids.items()
    ]


@query()
async def query_get_archiving_configuration(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    realm_id: RealmID,
) -> tuple[RealmArchivingConfiguration, DateTime | None]:
    rep = await conn.fetchrow(
        *_q_get_archiving_configuration(organization_id=organization_id.str, realm_id=realm_id)
    )
    if not rep:
        configuration = RealmArchivingConfiguration.available()
        archiving_certified_on = None
    else:
        configuration_raw, deletion_date, archiving_certified_on = rep
        configuration = RealmArchivingConfiguration.from_str(configuration_raw, deletion_date)
    return configuration, archiving_certified_on


async def check_archiving_configuration(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    realm_id: RealmID,
    operation_kind: OperationKind,
    now: DateTime,
) -> tuple[RealmArchivingConfiguration, DateTime | None]:
    (
        configuration,
        archiving_certified_on,
    ) = await query_get_archiving_configuration(conn, organization_id, realm_id)

    if configuration.is_deleted(now):
        raise RealmDeletedError()

    if operation_kind == OperationKind.DATA_WRITE:
        if configuration.is_archived() or configuration.is_deletion_planned():
            raise RealmArchivedError()

    return configuration, archiving_certified_on


@query(in_transaction=True)
async def query_update_archiving(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    archiving_configuration_request: RealmConfiguredArchiving,
    now: DateTime,
) -> None:
    """
    Raises:
        RealmAccessError
        RealmRoleRequireGreaterTimestampError
        RealmNotFoundError
        RealmDeletedError
        RealmArchivingPeriodTooShortError
    """
    realm_id = archiving_configuration_request.realm_id
    user_id = archiving_configuration_request.configured_by.user_id

    # TODO: Concurrency is not managed here.
    # It would be pointless to use a `FOR UPDATE` when getting the archiving configuration
    # since it would lock a row that remains unchanged during the transaction, since updating
    # the archiving merely inserts a new row. Plus, we also rely on information from other
    # tables such as `realm_user_roles`, so it's even more complicated to properly lock
    # information. For instance, having `query_update_archiving` and `query_update_roles`
    # both locking rows in different tables might end up in a deadlock if the `FOR UPDATE`
    # locks are not applied with care. A simpler solution might be to use a dedicated lock
    # for all transactions involving certificates for a given realm (or, for a given
    # organization).

    # 1. Check that the realm exist
    rep = await conn.fetchrow(
        *_q_check_realm_exist(organization_id=organization_id.str, realm_id=realm_id)
    )
    if not rep:
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    # 2. Check that the realm is not deleted
    (
        previous_configuration,
        previous_archiving_certified_on,
    ) = await check_archiving_configuration(
        conn, organization_id, realm_id, OperationKind.CONFIGURATION, now
    )
    if previous_configuration.is_deleted(now):
        raise RealmDeletedError()

    # 3. Check that the author role is owner
    rep = await conn.fetchrow(
        *_q_get_user_role(
            organization_id=organization_id.str,
            user_id=user_id.str,
            realm_id=realm_id,
        )
    )
    if not rep:
        raise RealmAccessError(f"Author `{user_id.str}` never had access to realm `{realm_id.hex}`")
    role, role_certified_on = rep
    if role is None:
        raise RealmAccessError(
            f"Author `{user_id.str}` had their access revoked from realm `{realm_id.hex}`"
        )
    if RealmRole.from_str(role) != RealmRole.OWNER:
        raise RealmAccessError(
            f"Author `{user_id.str}` requires owner right for realm `{realm_id.hex}`"
        )

    # 4. Check minimum archiving period
    row = await conn.fetchrow(*_q_get_organization(organization_id=organization_id.str))
    if not row:
        raise OrganizationNotFoundError()
    minimum_archiving_period = row["minimum_archiving_period"]
    if not archiving_configuration_request.is_valid_archiving_configuration(
        minimum_archiving_period
    ):
        raise RealmArchivingPeriodTooShortError()

    # 5. Check timestamp greater than last role certificate
    if role_certified_on >= archiving_configuration_request.configured_on:
        raise RealmRoleRequireGreaterTimestampError(role_certified_on)

    # 6. Check timestamp greater than last archiving certificate
    if (
        previous_archiving_certified_on is not None
        and previous_archiving_certified_on >= archiving_configuration_request.configured_on
    ):
        raise RealmRoleRequireGreaterTimestampError(previous_archiving_certified_on)

    # 7. Set last archiving change
    await conn.execute(
        *_q_set_last_archiving_change(
            organization_id=organization_id.str,
            realm_id=realm_id,
            user_id=user_id.str,
            configured_on=archiving_configuration_request.configured_on,
        )
    )

    # 8. Add archiving certificate
    configuration = archiving_configuration_request.configuration
    deletion_date = configuration.deletion_date if configuration.is_deletion_planned() else None
    await conn.execute(
        *_q_set_archiving_configuration(
            organization_id=organization_id.str,
            realm_id=realm_id,
            configuration=configuration.str,
            deletion_date=deletion_date,
            certificate=archiving_configuration_request.certificate,
            certified_by=archiving_configuration_request.configured_by.str,
            certified_on=archiving_configuration_request.configured_on,
        )
    )

    # 9. Send signal
    await send_signal(
        conn,
        BackendEvent.REALM_ARCHIVING_UPDATED,
        organization_id=organization_id,
        author=archiving_configuration_request.configured_by,
        realm_id=realm_id,
        configuration=configuration,
    )
