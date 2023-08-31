# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import triopg

from parsec._parsec import DateTime, RealmArchivingConfiguration, RealmRole
from parsec.api.protocol import OrganizationID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.organization import OrganizationNotFoundError, _q_get_organization
from parsec.backend.postgresql.realm_queries.update_roles import _q_get_realm_status
from parsec.backend.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_realm_internal_id,
    q_user_internal_id,
    query,
)
from parsec.backend.realm import (
    RealmAccessError,
    RealmArchivingConfigurationRequest,
    RealmArchivingPeriodTooShortError,
    RealmDeletedError,
    RealmNotFoundError,
    RealmRoleRequireGreaterTimestampError,
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

_q_get_archiving_configuration_for_update = Q(
    f"""
SELECT configuration, deletion_date, certified_on
FROM realm_archiving
WHERE
    realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY certified_on DESC LIMIT 1
FOR UPDATE
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
    { q_device_internal_id(organization_id="$organization_id", device_id="$granted_by") },
    $granted_on
"""
)

#     realm INTEGER REFERENCES realm (_id) NOT NULL,
#     configuration realm_archiving_configuration NOT NULL,
#     -- NULL if not DELETION_PLANNED
#     deletion_date TIMESTAMPTZ,
#     certificate BYTEA NOT NULL,
#     certified_by INTEGER REFERENCES device(_id) NOT NULL,
#     certified_on TIMESTAMPTZ NOT NULL
# );


@query(in_transaction=True)
async def query_update_archiving(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    archiving_configuration_request: RealmArchivingConfigurationRequest,
) -> None:
    """
    Raises:
        RealmAccessError
        RealmRoleRequireGreaterTimestampError
        RealmNotFoundError
        RealmDeletedError
        RealmArchivingPeriodTooShortError
    """
    now = DateTime.now()
    realm_id = archiving_configuration_request.realm_id
    user_id = archiving_configuration_request.configured_by.user_id

    # 1. Check that the realm exist
    rep = await conn.fetchrow(
        *_q_get_realm_status(organization_id=organization_id.str, realm_id=realm_id)
    )
    if not rep:
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    # 2. Check that the realm is not deleted
    rep = await conn.fetchrow(
        *_q_get_archiving_configuration_for_update(
            organization_id=organization_id.str, realm_id=realm_id
        )
    )
    if not rep:
        previous_configuration = RealmArchivingConfiguration.available()
        previous_archiving_certified_on = None
    else:
        previous_configuration_str, previous_deletion_date, previous_archiving_certified_on = rep
        previous_configuration = RealmArchivingConfiguration.from_str(
            previous_configuration_str, previous_deletion_date
        )
    if previous_configuration.is_deletion_planned() and previous_configuration.deletion_date <= now:
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

    # 7. Add archiving certificate
    configuration = archiving_configuration_request.configuration
    deletion_date = configuration.deletion_date if configuration.is_deletion_planned() else None
    await conn.execute(
        *_q_set_archiving_configuration(
            organization_id=organization_id.str,
            realm_id=realm_id,
            configuration=configuration.str,
            deletion_date=deletion_date,
            certificate=archiving_configuration_request.certificate,
            granted_by=archiving_configuration_request.configured_by.str,
            granted_on=archiving_configuration_request.configured_on,
        )
    )

    # 8. Send signal
    await send_signal(
        conn,
        BackendEvent.REALM_ARCHIVING_UPDATED,
        organization_id=organization_id,
        author=archiving_configuration_request.configured_by,
        realm_id=realm_id,
        configuration=configuration,
    )

    # realm = self._get_realm(organization_id, archiving_certificate.realm_id)

    # last_role = realm.get_last_role(archiving_certificate.author.user_id)

    # # Author should be an owner
    # if last_role is None or last_role.role != RealmRole.OWNER:
    #     raise RealmAccessError()

    # # Check minimum archiving period
    # assert self._organization_component is not None
    # organization = await self._organization_component.get(organization_id)
    # if not is_valid_archiving_configuration(
    #     archiving_certificate, organization.minimum_archiving_period
    # ):
    #     raise RealmArchivingPeriodTooShortError()

    # # Timestamp should be greater than last role
    # if last_role.granted_on >= archiving_certificate.timestamp:
    #     raise RealmRoleRequireGreaterTimestampError(last_role.granted_on)

    # # Timestamp should be greater than last archiving certificate
    # last_archiving_certificate = realm.get_last_archiving_certificate()
    # if (
    #     last_archiving_certificate is not None
    #     and last_archiving_certificate.timestamp >= archiving_certificate.timestamp
    # ):
    #     raise RealmRoleRequireGreaterTimestampError(last_archiving_certificate.timestamp)

    # # Update archiving configuration
    # realm.last_archiving_certificate = archiving_certificate

    # await self._send_event(
    #     BackendEvent.REALM_ARCHIVING_UPDATED,
    #     organization_id=organization_id,
    #     author=archiving_certificate.author.user_id,
    #     realm_id=archiving_certificate.realm_id,
    #     configuration=archiving_certificate.configuration,
    # )
